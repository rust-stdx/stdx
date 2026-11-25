use std::collections::BTreeMap;

use crypto::{Hasher, sha2::Sha256};

use crate::{
    client::{
        ByteStream, Client, Error, HttpClient, HttpMethod, bytes_to_string, canonical_bucket_uri, canonical_object_uri,
        canonical_query_string, collect_body, consume_empty, header_to_string, header_to_u64, xml_escape,
    },
    xml::Element,
};

#[derive(Debug, Clone)]
pub struct CompletedPart {
    pub part_number: u32,
    pub e_tag: String,
}

#[derive(Debug, Clone)]
pub struct UploadPartOutput {
    pub e_tag: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CompleteMultipartUploadOutput {
    pub e_tag: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeletedObject {
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct DeleteObjectsError {
    pub key: Option<String>,
    pub code: Option<String>,
    pub message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct DeleteObjectsOutput {
    pub deleted: Vec<DeletedObject>,
    pub errors: Vec<DeleteObjectsError>,
}

#[derive(Debug, Clone)]
pub struct MultipartUpload {
    pub key: String,
    pub upload_id: String,
    pub initiated: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ListMultipartUploadsOutput {
    pub uploads: Vec<MultipartUpload>,
}

#[derive(Debug, Clone)]
pub struct UploadedPart {
    pub part_number: u32,
    pub e_tag: Option<String>,
    pub size: Option<u64>,
    pub last_modified: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ListPartsOutput {
    pub parts: Vec<UploadedPart>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct GetObjectTaggingOutput {
    pub tags: Vec<Tag>,
}

pub struct GetObjectOutput {
    pub body: ByteStream,
    pub e_tag: Option<String>,
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct HeadObjectOutput {
    pub e_tag: Option<String>,
    pub content_type: Option<String>,
    pub content_length: Option<u64>,
}

impl<H: HttpClient> Client<H> {
    pub async fn put_object(&self, bucket: &str, key: &str, body: &[u8]) -> Result<(), crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let response = self.execute(HttpMethod::Put, &canonical_uri, "", body, bucket).await?;
        consume_empty(response)
    }

    pub async fn get_object(&self, bucket: &str, key: &str) -> Result<GetObjectOutput, crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let response = self.execute(HttpMethod::Get, &canonical_uri, "", b"", bucket).await?;
        let e_tag = header_to_string(&response, "etag");
        let content_type = header_to_string(&response, "content-type");
        let content_length = header_to_u64(&response, "content-length");

        Ok(GetObjectOutput {
            body: response.body,
            e_tag,
            content_type,
            content_length,
        })
    }

    pub async fn head_object(&self, bucket: &str, key: &str) -> Result<HeadObjectOutput, crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let response = self.execute(HttpMethod::Head, &canonical_uri, "", b"", bucket).await?;

        Ok(HeadObjectOutput {
            e_tag: header_to_string(&response, "etag"),
            content_type: header_to_string(&response, "content-type"),
            content_length: header_to_u64(&response, "content-length"),
        })
    }

    pub async fn delete_object(&self, bucket: &str, key: &str) -> Result<(), crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let response = self
            .execute(HttpMethod::Delete, &canonical_uri, "", b"", bucket)
            .await?;
        consume_empty(response)
    }

    pub async fn delete_objects(
        &self,
        bucket: &str,
        keys: &[&str],
    ) -> Result<DeleteObjectsOutput, crate::client::Error> {
        let canonical_uri = canonical_bucket_uri(bucket);
        let body = delete_body_xml(keys);
        let checksum_headers = delete_objects_checksum_headers(&body);
        let response = self
            .execute_with_headers(HttpMethod::Post, &canonical_uri, "delete=", &body, &checksum_headers, bucket)
            .await?;
        let xml_text = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let root = Element::parse(&xml_text)?;
        Ok(DeleteObjectsOutput {
            deleted: root
                .children_named("Deleted")
                .map(|entry| {
                    Ok(DeletedObject {
                        key: entry.required_text("Key")?.to_string(),
                    })
                })
                .collect::<Result<_, Error>>()?,
            errors: root
                .children_named("Error")
                .map(|entry| DeleteObjectsError {
                    key: entry.child_text("Key").map(str::to_string),
                    code: entry.child_text("Code").map(str::to_string),
                    message: entry.child_text("Message").map(str::to_string),
                })
                .collect(),
        })
    }

    pub async fn create_multipart_upload(&self, bucket: &str, key: &str) -> Result<String, crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let response = self
            .execute(HttpMethod::Post, &canonical_uri, "uploads=", b"", bucket)
            .await?;
        let xml_text = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let root = Element::parse(&xml_text)?;
        Ok(root.required_text("UploadId")?.to_string())
    }

    pub async fn upload_part(
        &self,
        bucket: &str,
        key: &str,
        upload_id: &str,
        part_number: u32,
        body: &[u8],
    ) -> Result<UploadPartOutput, crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let mut params = BTreeMap::new();
        params.insert("partNumber".to_string(), part_number.to_string());
        params.insert("uploadId".to_string(), upload_id.to_string());
        let canonical_query = canonical_query_string(&params);
        let response = self
            .execute(HttpMethod::Put, &canonical_uri, &canonical_query, body, bucket)
            .await?;
        let e_tag = header_to_string(&response, "etag");
        Ok(UploadPartOutput {
            e_tag,
        })
    }

    pub async fn complete_multipart_upload(
        &self,
        bucket: &str,
        key: &str,
        upload_id: &str,
        parts: &[CompletedPart],
    ) -> Result<CompleteMultipartUploadOutput, crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let mut params = BTreeMap::new();
        params.insert("uploadId".to_string(), upload_id.to_string());
        let canonical_query = canonical_query_string(&params);
        let xml_body = complete_multipart_body_xml(parts);
        let response = self
            .execute(HttpMethod::Post, &canonical_uri, &canonical_query, &xml_body, bucket)
            .await?;
        let xml_text = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let root = Element::parse(&xml_text)?;
        Ok(CompleteMultipartUploadOutput {
            e_tag: root.child_text("ETag").map(str::to_string),
        })
    }

    pub async fn abort_multipart_upload(
        &self,
        bucket: &str,
        key: &str,
        upload_id: &str,
    ) -> Result<(), crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let mut params = BTreeMap::new();
        params.insert("uploadId".to_string(), upload_id.to_string());
        let canonical_query = canonical_query_string(&params);
        let response = self
            .execute(HttpMethod::Delete, &canonical_uri, &canonical_query, b"", bucket)
            .await?;
        consume_empty(response)
    }

    pub async fn list_multipart_uploads(
        &self,
        bucket: &str,
    ) -> Result<ListMultipartUploadsOutput, crate::client::Error> {
        let canonical_uri = canonical_bucket_uri(bucket);
        let response = self
            .execute(HttpMethod::Get, &canonical_uri, "uploads=", b"", bucket)
            .await?;
        let xml_text = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let root = Element::parse(&xml_text)?;
        Ok(ListMultipartUploadsOutput {
            uploads: root
                .children_named("Upload")
                .map(|entry| {
                    Ok(MultipartUpload {
                        key: entry.required_text("Key")?.to_string(),
                        upload_id: entry.required_text("UploadId")?.to_string(),
                        initiated: entry.child_text("Initiated").map(str::to_string),
                    })
                })
                .collect::<Result<_, Error>>()?,
        })
    }

    pub async fn list_parts(
        &self,
        bucket: &str,
        key: &str,
        upload_id: &str,
    ) -> Result<ListPartsOutput, crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let mut params = BTreeMap::new();
        params.insert("uploadId".to_string(), upload_id.to_string());
        let canonical_query = canonical_query_string(&params);
        let response = self
            .execute(HttpMethod::Get, &canonical_uri, &canonical_query, b"", bucket)
            .await?;
        let xml_text = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let root = Element::parse(&xml_text)?;
        Ok(ListPartsOutput {
            parts: root
                .children_named("Part")
                .map(|entry| {
                    Ok(UploadedPart {
                        part_number: entry
                            .required_text("PartNumber")?
                            .parse()
                            .map_err(|error| Error::Xml(format!("invalid PartNumber: {error}")))?,
                        e_tag: entry.child_text("ETag").map(str::to_string),
                        size: entry
                            .child_text("Size")
                            .map(str::parse)
                            .transpose()
                            .map_err(|error| Error::Xml(format!("invalid Size: {error}")))?,
                        last_modified: entry.child_text("LastModified").map(str::to_string),
                    })
                })
                .collect::<Result<_, Error>>()?,
        })
    }

    pub async fn put_object_tagging(&self, bucket: &str, key: &str, tags: &[Tag]) -> Result<(), crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let body = tagging_body_xml(tags);
        let response = self
            .execute(HttpMethod::Put, &canonical_uri, "tagging=", &body, bucket)
            .await?;
        consume_empty(response)
    }

    pub async fn get_object_tagging(
        &self,
        bucket: &str,
        key: &str,
    ) -> Result<GetObjectTaggingOutput, crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let response = self
            .execute(HttpMethod::Get, &canonical_uri, "tagging=", b"", bucket)
            .await?;
        let xml_text = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let root = Element::parse(&xml_text)?;
        Ok(GetObjectTaggingOutput {
            tags: root
                .child("TagSet")
                .into_iter()
                .flat_map(|set| set.children_named("Tag"))
                .map(|entry| {
                    Ok(Tag {
                        key: entry.required_text("Key")?.to_string(),
                        value: entry.required_text("Value")?.to_string(),
                    })
                })
                .collect::<Result<_, Error>>()?,
        })
    }

    pub async fn delete_object_tagging(&self, bucket: &str, key: &str) -> Result<(), crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let response = self
            .execute(HttpMethod::Delete, &canonical_uri, "tagging=", b"", bucket)
            .await?;
        consume_empty(response)
    }
}

fn delete_objects_checksum_headers(body: &[u8]) -> [(String, String); 2] {
    [
        ("x-amz-sdk-checksum-algorithm".to_string(), "SHA256".to_string()),
        ("x-amz-checksum-sha256".to_string(), checksum_sha256(body)),
    ]
}

fn checksum_sha256(body: &[u8]) -> String {
    base64::encode(Sha256::hash(body).as_ref(), base64::Alphabet::Standard)
}

fn delete_body_xml(keys: &[&str]) -> Vec<u8> {
    let mut body = String::from("<Delete>");
    for key in keys {
        body.push_str("<Object><Key>");
        body.push_str(&xml_escape(key));
        body.push_str("</Key></Object>");
    }
    body.push_str("</Delete>");
    body.into_bytes()
}

fn complete_multipart_body_xml(parts: &[CompletedPart]) -> Vec<u8> {
    let mut body = String::from("<CompleteMultipartUpload>");
    for part in parts {
        body.push_str("<Part><PartNumber>");
        body.push_str(&part.part_number.to_string());
        body.push_str("</PartNumber><ETag>");
        body.push_str(&xml_escape(&part.e_tag));
        body.push_str("</ETag></Part>");
    }
    body.push_str("</CompleteMultipartUpload>");
    body.into_bytes()
}

fn tagging_body_xml(tags: &[Tag]) -> Vec<u8> {
    let mut body = String::from("<Tagging><TagSet>");
    for tag in tags {
        body.push_str("<Tag><Key>");
        body.push_str(&xml_escape(&tag.key));
        body.push_str("</Key><Value>");
        body.push_str(&xml_escape(&tag.value));
        body.push_str("</Value></Tag>");
    }
    body.push_str("</TagSet></Tagging>");
    body.into_bytes()
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use tokio::runtime::Runtime;

    use super::*;
    use crate::client::{HttpRequest, HttpResponseData, StaticCredentials};

    #[test]
    fn builds_delete_objects_body() {
        let xml = String::from_utf8(delete_body_xml(&["a", "b/c"])).unwrap();
        assert!(xml.contains("<Key>a</Key>"));
        assert!(xml.contains("<Key>b/c</Key>"));
    }

    #[test]
    fn parses_delete_objects_response() {
        let xml = r#"
<DeleteResult>
  <Deleted><Key>a.txt</Key></Deleted>
  <Error><Key>b.txt</Key><Code>AccessDenied</Code><Message>Denied</Message></Error>
</DeleteResult>
"#;
        let root = Element::parse(xml).unwrap();
        let deleted: Vec<_> = root.children_named("Deleted").collect();
        assert_eq!(deleted.len(), 1);
        assert_eq!(deleted[0].child_text("Key"), Some("a.txt"));
        let errors: Vec<_> = root.children_named("Error").collect();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].child_text("Code"), Some("AccessDenied"));
    }

    #[test]
    fn delete_objects_sets_sha256_checksum_headers() {
        #[derive(Clone)]
        struct CapturingHttpClient {
            request: Arc<Mutex<Option<HttpRequest>>>,
        }

        impl crate::client::HttpClient for CapturingHttpClient {
            async fn send(&self, request: HttpRequest) -> Result<HttpResponseData, crate::client::HttpError> {
                *self.request.lock().unwrap() = Some(request);
                Ok(HttpResponseData {
                    status_code: 200,
                    headers: Vec::new(),
                    body: Box::pin(futures_util::stream::once(async {
                        Ok(bytes::Bytes::from_static(b"<DeleteResult />"))
                    })),
                })
            }
        }

        let captured = Arc::new(Mutex::new(None));
        let http = CapturingHttpClient {
            request: Arc::clone(&captured),
        };
        let cfg = crate::client::ClientConfig {
            endpoint: "http://127.0.0.1:9000",
            credentials: StaticCredentials {
                access_key_id: "minioadmin",
                secret_access_key: "minioadmin",
                session_token: "",
            },
            region: "auto",
            virtual_hosted: false,
        };
        let client = Client::with_http_client(&cfg, http).unwrap();

        Runtime::new().unwrap().block_on(async {
            client.delete_objects("bucket", &["a", "b/c"]).await.unwrap();
        });

        let request = captured.lock().unwrap().clone().unwrap();
        let checksum_algorithm = request
            .headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("x-amz-sdk-checksum-algorithm"))
            .map(|(_, value)| value.as_str());
        let checksum_sha256 = request
            .headers
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case("x-amz-checksum-sha256"))
            .map(|(_, value)| value.as_str());

        assert_eq!(checksum_algorithm, Some("SHA256"));
        let expected_body = String::from_utf8(delete_body_xml(&["a", "b/c"])).unwrap();
        assert_eq!(checksum_sha256, Some(super::checksum_sha256(expected_body.as_bytes()).as_str()));
    }

    #[test]
    fn builds_and_parses_tagging_xml() {
        let body = String::from_utf8(tagging_body_xml(&[
            Tag {
                key: "env".to_string(),
                value: "dev".to_string(),
            },
            Tag {
                key: "team".to_string(),
                value: "infra".to_string(),
            },
        ]))
        .unwrap();
        assert!(body.contains("<Key>env</Key><Value>dev</Value>"));
        assert!(body.contains("<Key>team</Key><Value>infra</Value>"));

        let root =
            Element::parse("<Tagging><TagSet><Tag><Key>a</Key><Value>b</Value></Tag></TagSet></Tagging>").unwrap();
        let tags: Vec<_> = root.child("TagSet").unwrap().children_named("Tag").collect();
        assert_eq!(tags.len(), 1);
        assert_eq!(tags[0].child_text("Key"), Some("a"));
        assert_eq!(tags[0].child_text("Value"), Some("b"));
    }

    #[test]
    fn parses_list_parts_response() {
        let xml = r#"
<ListPartsResult>
  <Part>
    <PartNumber>1</PartNumber>
    <ETag>"etag-1"</ETag>
    <Size>5</Size>
    <LastModified>2026-01-01T00:00:00.000Z</LastModified>
  </Part>
</ListPartsResult>
"#;
        let root = Element::parse(xml).unwrap();
        let parts: Vec<_> = root.children_named("Part").collect();
        assert_eq!(parts.len(), 1);
        assert_eq!(parts[0].child_text("PartNumber"), Some("1"));
    }

    #[test]
    fn parses_list_multipart_uploads_response() {
        let xml = r#"
<ListMultipartUploadsResult>
  <Upload>
    <Key>big.bin</Key>
    <UploadId>upload-1</UploadId>
    <Initiated>2026-01-01T00:00:00.000Z</Initiated>
  </Upload>
</ListMultipartUploadsResult>
"#;
        let root = Element::parse(xml).unwrap();
        let uploads: Vec<_> = root.children_named("Upload").collect();
        assert_eq!(uploads.len(), 1);
        assert_eq!(uploads[0].child_text("Key"), Some("big.bin"));
        assert_eq!(uploads[0].child_text("UploadId"), Some("upload-1"));
    }
}
