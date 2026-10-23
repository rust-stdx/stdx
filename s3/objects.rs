use std::collections::BTreeMap;

use crypto::{Hasher, sha2::Sha256};
use quick_xml::{de::from_str, se::to_string};
use serde::{Deserialize, Serialize};

use crate::client::{
    ByteStream, Client, Error, HttpClient, HttpMethod, bytes_to_string, canonical_bucket_uri, canonical_object_uri,
    canonical_query_string, collect_body, consume_empty, header_to_string, header_to_u64,
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
        let body = to_string(&DeleteBodyXml {
            object: keys
                .iter()
                .map(|k| DeleteObjectEntryXml {
                    key: k.to_string(),
                })
                .collect(),
        })
        .map_err(|e| Error::Xml(quick_xml::DeError::Custom(e.to_string())))?
        .into_bytes();
        let checksum_headers = delete_objects_checksum_headers(&body);
        let response = self
            .execute_with_headers(HttpMethod::Post, &canonical_uri, "delete=", &body, &checksum_headers, bucket)
            .await?;
        let xml_text = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let xml: DeleteResultXml = from_str(&xml_text)?;
        Ok(DeleteObjectsOutput {
            deleted: xml
                .deleted
                .into_iter()
                .map(|entry| DeletedObject {
                    key: entry.key,
                })
                .collect(),
            errors: xml
                .errors
                .into_iter()
                .map(|entry| DeleteObjectsError {
                    key: entry.key,
                    code: entry.code,
                    message: entry.message,
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
        let xml: InitiateMultipartUploadResultXml = from_str(&xml_text)?;
        Ok(xml.upload_id)
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
        let xml_body = to_string(&CompleteMultipartBodyXml {
            part: parts
                .iter()
                .map(|p| CompletePartEntryXml {
                    part_number: p.part_number,
                    e_tag: p.e_tag.clone(),
                })
                .collect(),
        })
        .map_err(|e| Error::Xml(quick_xml::DeError::Custom(e.to_string())))?
        .into_bytes();
        let response = self
            .execute(HttpMethod::Post, &canonical_uri, &canonical_query, &xml_body, bucket)
            .await?;
        let xml_text = bytes_to_string(collect_body(response.body, self.max_response_bytes).await?)?;
        let xml: CompleteMultipartUploadResultXml = from_str(&xml_text)?;
        Ok(CompleteMultipartUploadOutput {
            e_tag: xml.e_tag,
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
        let xml: ListMultipartUploadsResultXml = from_str(&xml_text)?;
        Ok(ListMultipartUploadsOutput {
            uploads: xml
                .uploads
                .into_iter()
                .map(|entry| MultipartUpload {
                    key: entry.key,
                    upload_id: entry.upload_id,
                    initiated: entry.initiated,
                })
                .collect(),
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
        let xml: ListPartsResultXml = from_str(&xml_text)?;
        Ok(ListPartsOutput {
            parts: xml
                .parts
                .into_iter()
                .map(|entry| UploadedPart {
                    part_number: entry.part_number,
                    e_tag: entry.e_tag,
                    size: entry.size,
                    last_modified: entry.last_modified,
                })
                .collect(),
        })
    }

    pub async fn put_object_tagging(&self, bucket: &str, key: &str, tags: &[Tag]) -> Result<(), crate::client::Error> {
        let canonical_uri = canonical_object_uri(bucket, key);
        let body = to_string(&TaggingBodyXml {
            tag_set: TagSetBodyXml {
                tag: tags
                    .iter()
                    .map(|t| TagEntryXml {
                        key: t.key.clone(),
                        value: t.value.clone(),
                    })
                    .collect(),
            },
        })
        .map_err(|e| Error::Xml(quick_xml::DeError::Custom(e.to_string())))?
        .into_bytes();
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
        let xml: TaggingXml = from_str(&xml_text)?;
        Ok(GetObjectTaggingOutput {
            tags: xml
                .tag_set
                .tag
                .into_iter()
                .map(|entry| Tag {
                    key: entry.key,
                    value: entry.value,
                })
                .collect(),
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

#[derive(Debug, Serialize)]
#[serde(rename = "Delete")]
struct DeleteBodyXml {
    #[serde(rename = "Object")]
    object: Vec<DeleteObjectEntryXml>,
}

#[derive(Debug, Serialize)]
struct DeleteObjectEntryXml {
    #[serde(rename = "Key")]
    key: String,
}

#[derive(Debug, Serialize)]
#[serde(rename = "CompleteMultipartUpload")]
struct CompleteMultipartBodyXml {
    #[serde(rename = "Part")]
    part: Vec<CompletePartEntryXml>,
}

#[derive(Debug, Serialize)]
struct CompletePartEntryXml {
    #[serde(rename = "PartNumber")]
    part_number: u32,
    #[serde(rename = "ETag")]
    e_tag: String,
}

#[derive(Debug, Serialize)]
#[serde(rename = "Tagging")]
struct TaggingBodyXml {
    #[serde(rename = "TagSet")]
    tag_set: TagSetBodyXml,
}

#[derive(Debug, Serialize)]
struct TagSetBodyXml {
    #[serde(rename = "Tag")]
    tag: Vec<TagEntryXml>,
}

#[derive(Debug, Serialize)]
struct TagEntryXml {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "InitiateMultipartUploadResult")]
struct InitiateMultipartUploadResultXml {
    #[serde(rename = "UploadId")]
    upload_id: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "CompleteMultipartUploadResult")]
struct CompleteMultipartUploadResultXml {
    #[serde(rename = "ETag")]
    e_tag: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "DeleteResult")]
struct DeleteResultXml {
    #[serde(rename = "Deleted", default)]
    deleted: Vec<DeletedXml>,
    #[serde(rename = "Error", default)]
    errors: Vec<DeleteErrorXml>,
}

#[derive(Debug, Deserialize)]
struct DeletedXml {
    #[serde(rename = "Key")]
    key: String,
}

#[derive(Debug, Deserialize)]
struct DeleteErrorXml {
    #[serde(rename = "Key")]
    key: Option<String>,
    #[serde(rename = "Code")]
    code: Option<String>,
    #[serde(rename = "Message")]
    message: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ListMultipartUploadsResult")]
struct ListMultipartUploadsResultXml {
    #[serde(rename = "Upload", default)]
    uploads: Vec<MultipartUploadXml>,
}

#[derive(Debug, Deserialize)]
struct MultipartUploadXml {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "UploadId")]
    upload_id: String,
    #[serde(rename = "Initiated")]
    initiated: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "ListPartsResult")]
struct ListPartsResultXml {
    #[serde(rename = "Part", default)]
    parts: Vec<PartXml>,
}

#[derive(Debug, Deserialize)]
struct PartXml {
    #[serde(rename = "PartNumber")]
    part_number: u32,
    #[serde(rename = "ETag")]
    e_tag: Option<String>,
    #[serde(rename = "Size")]
    size: Option<u64>,
    #[serde(rename = "LastModified")]
    last_modified: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename = "Tagging")]
struct TaggingXml {
    #[serde(rename = "TagSet")]
    tag_set: TagSetXml,
}

#[derive(Debug, Deserialize)]
struct TagSetXml {
    #[serde(rename = "Tag", default)]
    tag: Vec<TagXml>,
}

#[derive(Debug, Deserialize)]
struct TagXml {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "Value")]
    value: String,
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use quick_xml::de::from_str;
    use tokio::runtime::Runtime;

    use super::*;
    use crate::client::{HttpRequest, HttpResponseData, StaticCredentials};

    #[test]
    fn builds_delete_objects_body() {
        let xml = to_string(&DeleteBodyXml {
            object: vec![
                DeleteObjectEntryXml {
                    key: "a".to_string(),
                },
                DeleteObjectEntryXml {
                    key: "b/c".to_string(),
                },
            ],
        })
        .unwrap();
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
        let parsed: DeleteResultXml = from_str(xml).unwrap();
        assert_eq!(parsed.deleted.len(), 1);
        assert_eq!(parsed.deleted[0].key, "a.txt");
        assert_eq!(parsed.errors.len(), 1);
        assert_eq!(parsed.errors[0].code.as_deref(), Some("AccessDenied"));
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
        let expected_body = to_string(&DeleteBodyXml {
            object: vec![
                DeleteObjectEntryXml {
                    key: "a".to_string(),
                },
                DeleteObjectEntryXml {
                    key: "b/c".to_string(),
                },
            ],
        })
        .unwrap();
        assert_eq!(checksum_sha256, Some(super::checksum_sha256(expected_body.as_bytes()).as_str()));
    }

    #[test]
    fn builds_and_parses_tagging_xml() {
        let body = to_string(&TaggingBodyXml {
            tag_set: TagSetBodyXml {
                tag: vec![
                    TagEntryXml {
                        key: "env".to_string(),
                        value: "dev".to_string(),
                    },
                    TagEntryXml {
                        key: "team".to_string(),
                        value: "infra".to_string(),
                    },
                ],
            },
        })
        .unwrap();
        assert!(body.contains("<Key>env</Key><Value>dev</Value>"));
        assert!(body.contains("<Key>team</Key><Value>infra</Value>"));

        let parsed: TaggingXml =
            from_str("<Tagging><TagSet><Tag><Key>a</Key><Value>b</Value></Tag></TagSet></Tagging>").unwrap();
        assert_eq!(parsed.tag_set.tag.len(), 1);
        assert_eq!(parsed.tag_set.tag[0].key, "a");
        assert_eq!(parsed.tag_set.tag[0].value, "b");
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
        let parsed: ListPartsResultXml = from_str(xml).unwrap();
        assert_eq!(parsed.parts.len(), 1);
        assert_eq!(parsed.parts[0].part_number, 1);
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
        let parsed: ListMultipartUploadsResultXml = from_str(xml).unwrap();
        assert_eq!(parsed.uploads.len(), 1);
        assert_eq!(parsed.uploads[0].key, "big.bin");
        assert_eq!(parsed.uploads[0].upload_id, "upload-1");
    }
}
