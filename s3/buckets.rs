use std::collections::BTreeMap;

use crate::{
    client::{
        Client, HttpClient, HttpMethod, bytes_to_string, canonical_bucket_uri, canonical_query_string, collect_body,
        consume_empty, xml_escape,
    },
    xml::Element,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListObjectsOutput {
    pub name: String,
    pub prefix: Option<String>,
    pub key_count: Option<u64>,
    pub max_keys: Option<u64>,
    pub is_truncated: bool,
    pub next_continuation_token: Option<String>,
    pub contents: Vec<ListObject>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListObject {
    pub key: String,
    pub last_modified: Option<String>,
    pub e_tag: Option<String>,
    pub size: Option<u64>,
    pub storage_class: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bucket {
    pub name: String,
    pub creation_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListBucketsOutput {
    pub buckets: Vec<Bucket>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetBucketLocationOutput {
    pub location_constraint: Option<String>,
}

impl<H: HttpClient> Client<H> {
    pub async fn list_buckets(&self) -> Result<ListBucketsOutput, crate::client::Error> {
        let response = self.execute(HttpMethod::Get, "/", "", b"", "").await?;
        let body = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let root = Element::parse(&body)?;

        Ok(ListBucketsOutput {
            buckets: root
                .child("Buckets")
                .into_iter()
                .flat_map(|buckets| buckets.children_named("Bucket"))
                .map(|entry| {
                    Ok(Bucket {
                        name: entry.required_text("Name")?.to_string(),
                        creation_date: entry.child_text("CreationDate").map(str::to_string),
                    })
                })
                .collect::<Result<_, crate::client::Error>>()?,
        })
    }

    pub async fn create_bucket(
        &self,
        bucket: &str,
        location_constraint: Option<&str>,
    ) -> Result<(), crate::client::Error> {
        let canonical_uri = canonical_bucket_uri(bucket);
        let body = match location_constraint {
            Some(region) => format!(
                "<CreateBucketConfiguration><LocationConstraint>{}</LocationConstraint></CreateBucketConfiguration>",
                xml_escape(region)
            )
            .into_bytes(),
            None => Vec::new(),
        };
        let response = self
            .execute_with_headers(HttpMethod::Put, &canonical_uri, "", &body, &[], bucket)
            .await?;
        consume_empty(response)
    }

    pub async fn head_bucket(&self, bucket: &str) -> Result<(), crate::client::Error> {
        let canonical_uri = canonical_bucket_uri(bucket);
        let response = self.execute(HttpMethod::Head, &canonical_uri, "", b"", bucket).await?;
        consume_empty(response)
    }

    pub async fn delete_bucket(&self, bucket: &str) -> Result<(), crate::client::Error> {
        let canonical_uri = canonical_bucket_uri(bucket);
        let response = self
            .execute(HttpMethod::Delete, &canonical_uri, "", b"", bucket)
            .await?;
        consume_empty(response)
    }

    pub async fn list_objects(
        &self,
        bucket: &str,
        prefix: Option<&str>,
        continuation_token: Option<&str>,
        max_keys: Option<u32>,
    ) -> Result<ListObjectsOutput, crate::client::Error> {
        let canonical_uri = canonical_bucket_uri(bucket);

        let mut params = BTreeMap::new();
        params.insert("list-type".to_string(), "2".to_string());
        if let Some(prefix) = prefix {
            params.insert("prefix".to_string(), prefix.to_string());
        }
        if let Some(token) = continuation_token {
            params.insert("continuation-token".to_string(), token.to_string());
        }
        if let Some(max_keys) = max_keys {
            params.insert("max-keys".to_string(), max_keys.to_string());
        }

        let canonical_query = canonical_query_string(&params);
        let response = self
            .execute(HttpMethod::Get, &canonical_uri, &canonical_query, b"", bucket)
            .await?;
        let body = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let root = Element::parse(&body)?;

        Ok(ListObjectsOutput {
            name: root.required_text("Name")?.to_string(),
            prefix: root.child_text("Prefix").map(str::to_string),
            key_count: parse_optional(&root, "KeyCount")?,
            max_keys: parse_optional(&root, "MaxKeys")?,
            is_truncated: matches!(root.child_text("IsTruncated"), Some("true")),
            next_continuation_token: root.child_text("NextContinuationToken").map(str::to_string),
            contents: root
                .children_named("Contents")
                .map(|entry| {
                    Ok(ListObject {
                        key: entry.required_text("Key")?.to_string(),
                        last_modified: entry.child_text("LastModified").map(str::to_string),
                        e_tag: entry.child_text("ETag").map(str::to_string),
                        size: parse_optional(entry, "Size")?,
                        storage_class: entry.child_text("StorageClass").map(str::to_string),
                    })
                })
                .collect::<Result<_, crate::client::Error>>()?,
        })
    }

    pub async fn get_bucket_location(&self, bucket: &str) -> Result<GetBucketLocationOutput, crate::client::Error> {
        let canonical_uri = canonical_bucket_uri(bucket);
        let response = self
            .execute(HttpMethod::Get, &canonical_uri, "location=", b"", bucket)
            .await?;
        let body = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let root = Element::parse(&body)?;
        let location = root.text().trim();
        Ok(GetBucketLocationOutput {
            location_constraint: (!location.is_empty()).then(|| location.to_string()),
        })
    }
}

fn parse_optional<T>(element: &Element, name: &str) -> Result<Option<T>, crate::client::Error>
where
    T: core::str::FromStr,
    T::Err: core::fmt::Display,
{
    element
        .child_text(name)
        .map(str::parse)
        .transpose()
        .map_err(|error| crate::client::Error::Xml(format!("invalid <{name}>: {error}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_list_objects_v2_xml() {
        let xml = r#"
<ListBucketResult>
  <Name>my-bucket</Name>
  <Prefix>photos/</Prefix>
  <KeyCount>1</KeyCount>
  <MaxKeys>1000</MaxKeys>
  <IsTruncated>false</IsTruncated>
  <Contents>
    <Key>photos/a.jpg</Key>
    <LastModified>2026-01-01T00:00:00.000Z</LastModified>
    <ETag>\"abc\"</ETag>
    <Size>42</Size>
    <StorageClass>STANDARD</StorageClass>
  </Contents>
</ListBucketResult>
"#;

        let root = Element::parse(xml).unwrap();
        assert_eq!(root.child_text("Name"), Some("my-bucket"));
        assert_eq!(root.child_text("Prefix"), Some("photos/"));
        assert_eq!(root.child_text("KeyCount"), Some("1"));
        assert_eq!(root.child_text("MaxKeys"), Some("1000"));
        assert_eq!(root.child_text("IsTruncated"), Some("false"));
        let contents: Vec<_> = root.children_named("Contents").collect();
        assert_eq!(contents.len(), 1);
        assert_eq!(contents[0].child_text("Key"), Some("photos/a.jpg"));
    }

    #[test]
    fn parses_list_buckets_xml() {
        let xml = r#"
<ListAllMyBucketsResult>
  <Buckets>
    <Bucket>
      <Name>bucket-a</Name>
      <CreationDate>2026-01-01T00:00:00.000Z</CreationDate>
    </Bucket>
    <Bucket>
      <Name>bucket-b</Name>
    </Bucket>
  </Buckets>
</ListAllMyBucketsResult>
"#;

        let root = Element::parse(xml).unwrap();
        let buckets: Vec<_> = root.child("Buckets").unwrap().children_named("Bucket").collect();
        assert_eq!(buckets.len(), 2);
        assert_eq!(buckets[0].child_text("Name"), Some("bucket-a"));
        assert_eq!(buckets[0].child_text("CreationDate"), Some("2026-01-01T00:00:00.000Z"));
        assert_eq!(buckets[1].child_text("Name"), Some("bucket-b"));
    }

    #[test]
    fn parses_bucket_location_xml() {
        let xml = "<LocationConstraint>auto</LocationConstraint>";
        let root = Element::parse(xml).unwrap();
        assert_eq!(root.text(), "auto");
    }
}
