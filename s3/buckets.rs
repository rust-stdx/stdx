use std::collections::BTreeMap;

use quick_xml::de::from_str;
use serde::Deserialize;

use crate::client::{
    Client, HttpClient, HttpMethod, bytes_to_string, canonical_bucket_uri, canonical_query_string, collect_body,
    consume_empty, xml_escape,
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
        let xml: ListAllMyBucketsResultXml = from_str(&body)?;

        Ok(ListBucketsOutput {
            buckets: xml
                .buckets
                .bucket
                .into_iter()
                .map(|entry| Bucket {
                    name: entry.name,
                    creation_date: entry.creation_date,
                })
                .collect(),
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
        let xml: ListBucketResultXml = from_str(&body)?;

        Ok(ListObjectsOutput {
            name: xml.name,
            prefix: xml.prefix,
            key_count: xml.key_count,
            max_keys: xml.max_keys,
            is_truncated: xml.is_truncated,
            next_continuation_token: xml.next_continuation_token,
            contents: xml
                .contents
                .into_iter()
                .map(|entry| ListObject {
                    key: entry.key,
                    last_modified: entry.last_modified,
                    e_tag: entry.e_tag,
                    size: entry.size,
                    storage_class: entry.storage_class,
                })
                .collect(),
        })
    }

    pub async fn get_bucket_location(&self, bucket: &str) -> Result<GetBucketLocationOutput, crate::client::Error> {
        let canonical_uri = canonical_bucket_uri(bucket);
        let response = self
            .execute(HttpMethod::Get, &canonical_uri, "location=", b"", bucket)
            .await?;
        let body = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let xml: LocationConstraintXml = from_str(&body)?;
        Ok(GetBucketLocationOutput {
            location_constraint: xml.location_constraint.and_then(|s| {
                let trimmed = s.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            }),
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ListAllMyBucketsResult")]
struct ListAllMyBucketsResultXml {
    #[serde(rename = "Buckets")]
    buckets: BucketsXml,
}

#[derive(Debug, Deserialize)]
struct BucketsXml {
    #[serde(rename = "Bucket", default)]
    bucket: Vec<BucketXml>,
}

#[derive(Debug, Deserialize)]
struct BucketXml {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "CreationDate")]
    creation_date: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ListBucketResult")]
struct ListBucketResultXml {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Prefix")]
    prefix: Option<String>,
    #[serde(rename = "KeyCount")]
    key_count: Option<u64>,
    #[serde(rename = "MaxKeys")]
    max_keys: Option<u64>,
    #[serde(rename = "IsTruncated")]
    is_truncated: bool,
    #[serde(rename = "NextContinuationToken")]
    next_continuation_token: Option<String>,
    #[serde(rename = "Contents", default)]
    contents: Vec<ObjectXml>,
}

#[derive(Debug, Deserialize)]
struct ObjectXml {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "LastModified")]
    last_modified: Option<String>,
    #[serde(rename = "ETag")]
    e_tag: Option<String>,
    #[serde(rename = "Size")]
    size: Option<u64>,
    #[serde(rename = "StorageClass")]
    storage_class: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "LocationConstraint")]
struct LocationConstraintXml {
    #[serde(rename = "$text")]
    location_constraint: Option<String>,
}

#[cfg(test)]
mod tests {
    use quick_xml::de::from_str;

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

        let parsed: ListBucketResultXml = from_str(xml).unwrap();
        assert_eq!(parsed.name, "my-bucket");
        assert_eq!(parsed.prefix.as_deref(), Some("photos/"));
        assert_eq!(parsed.key_count, Some(1));
        assert_eq!(parsed.max_keys, Some(1000));
        assert!(!parsed.is_truncated);
        assert_eq!(parsed.contents.len(), 1);
        assert_eq!(parsed.contents[0].key, "photos/a.jpg");
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

        let parsed: ListAllMyBucketsResultXml = from_str(xml).unwrap();
        assert_eq!(parsed.buckets.bucket.len(), 2);
        assert_eq!(parsed.buckets.bucket[0].name, "bucket-a");
        assert_eq!(
            parsed.buckets.bucket[0].creation_date.as_deref(),
            Some("2026-01-01T00:00:00.000Z")
        );
        assert_eq!(parsed.buckets.bucket[1].name, "bucket-b");
    }

    #[test]
    fn parses_bucket_location_xml() {
        let xml = "<LocationConstraint>auto</LocationConstraint>";
        let parsed: LocationConstraintXml = from_str(xml).unwrap();
        assert_eq!(parsed.location_constraint.as_deref(), Some("auto"));
    }
}
