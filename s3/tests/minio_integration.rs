#![cfg(feature = "reqwest")]

use std::time::{SystemTime, UNIX_EPOCH};

use bytes::Bytes;
use futures_util::TryStreamExt;
use s3::{Client, ClientConfig, CompletedPart, Error, ReqwestHttpClient, StaticCredentials, Tag};

fn integration_enabled() -> bool {
    std::env::var("S3_RUN_INTEGRATION").ok().as_deref() == Some("1")
}

fn env_or_default(name: &str, default: &str) -> String {
    std::env::var(name).unwrap_or_else(|_| default.to_string())
}

fn unique_suffix() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock before unix epoch")
        .as_nanos();
    nanos.to_string()
}

fn unique_bucket_name() -> String {
    let base = env_or_default("S3_TEST_BUCKET", "rust-stdx-s3-integration");
    let suffix = unique_suffix();
    let mut name = format!("{base}-{suffix}");
    if name.len() > 63 {
        name.truncate(63);
    }
    name
}

fn test_client() -> Client<ReqwestHttpClient> {
    crypto_rustls::install_default_rustls_provider();

    let endpoint = env_or_default("S3_ENDPOINT", "http://127.0.0.1:9000");
    let region = env_or_default("S3_REGION", "auto");
    let access_key_id = env_or_default("S3_ACCESS_KEY_ID", "minioadmin");
    let secret_access_key = env_or_default("S3_SECRET_ACCESS_KEY", "minioadmin");
    let session_token = std::env::var("S3_SESSION_TOKEN").unwrap_or_default();

    let cfg = ClientConfig {
        endpoint: &endpoint,
        credentials: StaticCredentials {
            access_key_id: &access_key_id,
            secret_access_key: &secret_access_key,
            session_token: &session_token,
        },
        region: &region,
        virtual_hosted: false,
    };

    Client::new(&cfg).expect("failed to build s3 client")
}

async fn collect_stream(body: s3::ByteStream) -> Vec<u8> {
    let chunks: Vec<Bytes> = body.try_collect().await.expect("failed to collect body stream");
    chunks.into_iter().flat_map(|b| b.into_iter()).collect()
}

async fn create_bucket_or_panic(client: &Client<ReqwestHttpClient>, bucket: &str) {
    match client.create_bucket(bucket, None).await {
        Ok(()) => {}
        Err(Error::Api {
            status: 409, ..
        }) => {}
        Err(err) => panic!("create_bucket failed: {err}"),
    }
}

#[tokio::test]
async fn minio_bucket_operations() {
    if !integration_enabled() {
        eprintln!("skipping integration test (set S3_RUN_INTEGRATION=1)");
        return;
    }

    let client = test_client();
    let bucket = unique_bucket_name();
    create_bucket_or_panic(&client, &bucket).await;

    client.head_bucket(&bucket).await.expect("head_bucket failed");

    let list_buckets = client.list_buckets().await.expect("list_buckets failed");
    assert!(list_buckets.buckets.iter().any(|b| b.name == bucket));

    let location = client
        .get_bucket_location(&bucket)
        .await
        .expect("get_bucket_location failed");
    assert!(location.location_constraint.is_none() || !location.location_constraint.unwrap().is_empty());

    client.delete_bucket(&bucket).await.expect("delete_bucket failed");
}

#[tokio::test]
async fn minio_object_operations() {
    if !integration_enabled() {
        eprintln!("skipping integration test (set S3_RUN_INTEGRATION=1)");
        return;
    }

    let client = test_client();
    let bucket = unique_bucket_name();
    create_bucket_or_panic(&client, &bucket).await;

    let root = format!("integration/{}/", unique_suffix());
    let key = format!("{root}hello.txt");
    let body = b"hello from stdx s3";

    client.put_object(&bucket, &key, body).await.expect("put_object failed");

    let head = client.head_object(&bucket, &key).await.expect("head_object failed");
    assert_eq!(head.content_length, Some(body.len() as u64));

    let got = client.get_object(&bucket, &key).await.expect("get_object failed");
    assert_eq!(collect_stream(got.body).await, body.as_ref());

    let listed = client
        .list_objects(&bucket, Some(&root), None, Some(1000))
        .await
        .expect("list_objects failed");
    assert!(listed.contents.iter().any(|obj| obj.key == key));

    client
        .put_object_tagging(
            &bucket,
            &key,
            &[
                Tag {
                    key: "env".to_string(),
                    value: "test".to_string(),
                },
                Tag {
                    key: "owner".to_string(),
                    value: "stdx".to_string(),
                },
            ],
        )
        .await
        .expect("put_object_tagging failed");

    let tags = client
        .get_object_tagging(&bucket, &key)
        .await
        .expect("get_object_tagging failed");
    assert!(tags.tags.iter().any(|t| t.key == "env" && t.value == "test"));
    assert!(tags.tags.iter().any(|t| t.key == "owner" && t.value == "stdx"));

    client
        .delete_object_tagging(&bucket, &key)
        .await
        .expect("delete_object_tagging failed");
    let tags_after_delete = client
        .get_object_tagging(&bucket, &key)
        .await
        .expect("get_object_tagging after delete failed");
    assert!(tags_after_delete.tags.is_empty());

    let key2 = format!("{root}bulk-a.txt");
    let key3 = format!("{root}bulk-b.txt");
    client
        .put_object(&bucket, &key2, b"a")
        .await
        .expect("put_object key2 failed");
    client
        .put_object(&bucket, &key3, b"b")
        .await
        .expect("put_object key3 failed");

    let deleted = client
        .delete_objects(&bucket, &[&key2, &key3])
        .await
        .expect("delete_objects failed");
    assert!(deleted.errors.is_empty());
    assert!(deleted.deleted.iter().any(|d| d.key == key2));
    assert!(deleted.deleted.iter().any(|d| d.key == key3));

    client.delete_object(&bucket, &key).await.expect("delete_object failed");
    client.delete_bucket(&bucket).await.expect("delete_bucket failed");
}

#[tokio::test]
async fn minio_multipart_operations() {
    if !integration_enabled() {
        eprintln!("skipping integration test (set S3_RUN_INTEGRATION=1)");
        return;
    }

    let client = test_client();
    let bucket = unique_bucket_name();
    create_bucket_or_panic(&client, &bucket).await;

    let root = format!("integration/{}/", unique_suffix());
    let complete_key = format!("{root}multipart-complete.bin");
    let abort_key = format!("{root}multipart-abort.bin");

    // S3 requires every part except the last to be at least 5 MiB.
    let part1: Vec<u8> = vec![b'A'; 5 * 1024 * 1024];
    let part2: Vec<u8> = vec![b'B'; 1024];

    let upload_id = client
        .create_multipart_upload(&bucket, &complete_key)
        .await
        .expect("create_multipart_upload failed");

    let uploads = client
        .list_multipart_uploads(&bucket)
        .await
        .expect("list_multipart_uploads failed");
    assert!(
        uploads
            .uploads
            .iter()
            .any(|u| u.key == complete_key && u.upload_id == upload_id)
    );

    let out1 = client
        .upload_part(&bucket, &complete_key, &upload_id, 1, &part1)
        .await
        .expect("upload_part 1 failed");
    let out2 = client
        .upload_part(&bucket, &complete_key, &upload_id, 2, &part2)
        .await
        .expect("upload_part 2 failed");

    let listed_parts = client
        .list_parts(&bucket, &complete_key, &upload_id)
        .await
        .expect("list_parts failed");
    assert!(listed_parts.parts.iter().any(|p| p.part_number == 1));
    assert!(listed_parts.parts.iter().any(|p| p.part_number == 2));

    let parts = vec![
        CompletedPart {
            part_number: 1,
            e_tag: out1.e_tag.expect("part 1 etag missing"),
        },
        CompletedPart {
            part_number: 2,
            e_tag: out2.e_tag.expect("part 2 etag missing"),
        },
    ];

    client
        .complete_multipart_upload(&bucket, &complete_key, &upload_id, &parts)
        .await
        .expect("complete_multipart_upload failed");

    let head = client
        .head_object(&bucket, &complete_key)
        .await
        .expect("head_object after multipart failed");
    let expected_len = (part1.len() + part2.len()) as u64;
    assert_eq!(head.content_length, Some(expected_len));

    let got = client
        .get_object(&bucket, &complete_key)
        .await
        .expect("get_object after multipart failed");
    let mut expected = part1;
    expected.extend_from_slice(&part2);
    assert_eq!(collect_stream(got.body).await, expected);

    client
        .delete_object(&bucket, &complete_key)
        .await
        .expect("delete_object after complete failed");

    let abort_upload_id = client
        .create_multipart_upload(&bucket, &abort_key)
        .await
        .expect("create_multipart_upload abort path failed");
    client
        .upload_part(&bucket, &abort_key, &abort_upload_id, 1, b"partial")
        .await
        .expect("upload_part abort path failed");
    client
        .abort_multipart_upload(&bucket, &abort_key, &abort_upload_id)
        .await
        .expect("abort_multipart_upload failed");

    let uploads_after_abort = client
        .list_multipart_uploads(&bucket)
        .await
        .expect("list_multipart_uploads after abort failed");
    assert!(
        !uploads_after_abort
            .uploads
            .iter()
            .any(|u| u.key == abort_key && u.upload_id == abort_upload_id)
    );

    client.delete_bucket(&bucket).await.expect("delete_bucket failed");
}
