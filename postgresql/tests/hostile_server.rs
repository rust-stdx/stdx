//! Robustness tests against a hostile PostgreSQL server.
//!
//! Every test starts a fake server that speaks just enough of the protocol to
//! authenticate, and then answers with malformed or malicious backend
//! messages: impossible column counts, declared lengths far beyond the
//! payload, truncated rows, non-ASCII SQLSTATE codes and truncated `COPY`
//! streams. The client must report an error — never panic, never return a
//! silently wrong result, and never wedge the connection.
//!
//! The fake server needs no database and no certificate: the client is
//! connected with `sslmode=require` (the default), which accepts any server
//! certificate, and the server's `CertifiedKey` is built directly from a
//! generated key.

use std::{net::SocketAddr, sync::Arc, time::Duration};

use bytes::{Buf, BufMut, BytesMut};
use rustls::{
    ServerConfig,
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer},
    sign::{CertifiedKey, SingleCertAndKey},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

/// How a fake server answers the second `Sync` (the one that follows `Bind`).
#[derive(Clone, Copy, Debug)]
enum Case {
    DataRowNegativeCount,
    DataRowCountMismatch,
    DataRowTruncated,
    RowDescriptionNegativeCount,
    ParameterDescriptionNegativeCount,
    OversizedMessageLength,
    ErrorNonAsciiCode,
    CopyOutTruncated,
    CopyOutHugeFieldLength,
    CopyOutSplitRow,
    CopyBothResponse,
    CopyInResponseDuringQuery,
    AuthMessageFlood,
    NotificationDuringQuery,
}

#[tokio::test]
async fn data_row_with_negative_count_is_an_error() {
    run(Case::DataRowNegativeCount, |conn| async move {
        let result = query(&conn).await;
        assert!(
            result.is_err(),
            "a malformed row must not look like an empty result: {result:?}"
        );
    })
    .await;
}

#[tokio::test]
async fn data_row_count_mismatch_is_an_error() {
    run(Case::DataRowCountMismatch, |conn| async move {
        let result = query(&conn).await;
        assert!(
            result.is_err(),
            "a row with more columns than the statement must be rejected: {result:?}"
        );
    })
    .await;
}

#[tokio::test]
async fn truncated_data_row_is_an_error() {
    run(Case::DataRowTruncated, |conn| async move {
        let result = query(&conn).await;
        assert!(result.is_err(), "a truncated row must be rejected: {result:?}");
    })
    .await;
}

#[tokio::test]
async fn row_description_with_negative_count_is_an_error() {
    run(Case::RowDescriptionNegativeCount, |conn| async move {
        let result = query(&conn).await;
        assert!(result.is_err(), "a malformed RowDescription must be rejected: {result:?}");
    })
    .await;
}

#[tokio::test]
async fn parameter_description_with_negative_count_is_an_error() {
    run(Case::ParameterDescriptionNegativeCount, |conn| async move {
        let result = query(&conn).await;
        assert!(result.is_err(), "a malformed ParameterDescription must be rejected: {result:?}");
    })
    .await;
}

#[tokio::test]
async fn oversized_message_length_is_an_error_without_buffering() {
    run(Case::OversizedMessageLength, |conn| async move {
        let result = query(&conn).await;
        assert!(
            result.is_err(),
            "a message claiming a 2 GiB body must be rejected up front: {result:?}"
        );
    })
    .await;
}

#[tokio::test]
async fn error_with_non_ascii_sqlstate_does_not_panic() {
    run(Case::ErrorNonAsciiCode, |conn| async move {
        let err = query(&conn).await.expect_err("the server reports an error");
        let postgresql::Error::Server(db) = &err else {
            panic!("expected a server error, got {err:?}");
        };
        // `class()` slices the (server controlled) code; it must not panic on
        // a code that is not ASCII.
        assert_eq!(db.code, "\u{20ac}");
        // `class()` slices the server-controlled code; it must not panic and
        // must not pretend to recognise a class it cannot parse.
        assert_eq!(db.class(), "");
        assert!(!db.is_data_exception());
    })
    .await;
}

#[tokio::test]
async fn truncated_copy_stream_is_an_error_not_zero_rows() {
    run(Case::CopyOutTruncated, |conn| async move {
        let out = postgresql::__private::copy_out::<(i32, String)>(&conn, "COPY t TO STDOUT BINARY", &[])
            .await
            .expect("copy_out starts");
        let result = out.collect_all().await;
        assert!(
            result.is_err(),
            "a truncated COPY stream must not look like an empty result: {result:?}"
        );
    })
    .await;
}

#[tokio::test]
async fn copy_out_with_an_impossible_field_length_is_an_error() {
    run(Case::CopyOutHugeFieldLength, |conn| async move {
        let out = postgresql::__private::copy_out::<(i32,)>(&conn, "COPY t TO STDOUT BINARY", &[23])
            .await
            .expect("copy_out starts");
        let result = out.collect_all().await;
        assert!(
            result.is_err(),
            "a ~2 GiB declared field must be rejected, not buffered: {result:?}"
        );
    })
    .await;
}

#[tokio::test]
async fn copy_out_row_split_across_many_messages_is_reassembled() {
    run(Case::CopyOutSplitRow, |conn| async move {
        let out = postgresql::__private::copy_out::<(i32, String)>(&conn, "COPY t TO STDOUT BINARY", &[23, 25])
            .await
            .expect("copy_out starts");
        let rows = out.collect_all().await.expect("a well-formed split stream decodes");
        assert_eq!(rows, vec![(7, "xy".to_string())]);
    })
    .await;
}

#[tokio::test]
async fn copy_both_response_is_rejected() {
    // 'W' (CopyBothResponse, streaming replication) must not be mistaken for a
    // readable COPY stream.
    run(Case::CopyBothResponse, |conn| async move {
        let result = query(&conn).await;
        assert!(result.is_err(), "CopyBothResponse must be rejected: {result:?}");
    })
    .await;
}

#[tokio::test]
async fn copy_in_response_during_a_query_is_an_error_not_a_hang() {
    run(Case::CopyInResponseDuringQuery, |conn| async move {
        // A server that enters COPY-in mode for a parameterized query must not
        // wedge the connection: the client sends CopyFail, reports the error,
        // and stays usable.
        let result = query(&conn).await;
        assert!(result.is_err(), "CopyInResponse must be rejected: {result:?}");

        let again = query(&conn).await;
        assert!(again.is_ok(), "the connection must recover after COPY mode: {again:?}");
    })
    .await;
}

#[tokio::test]
async fn a_parameter_status_flood_during_startup_is_rejected() {
    let addr = serve(Case::AuthMessageFlood).await;
    let url = format!("postgres://probe@{addr}/db");
    let result = tokio::time::timeout(Duration::from_secs(20), postgresql::Connection::connect(&url)).await;
    assert!(
        matches!(result, Ok(Err(_))),
        "a startup message flood must be rejected, got {result:?}"
    );
}

#[tokio::test]
async fn connect_times_out_when_the_server_stalls() {
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local addr");
    tokio::spawn(async move {
        // Accept the TCP connection and never reply to the SSLRequest.
        let mut held = Vec::new();
        while let Ok((sock, _)) = listener.accept().await {
            held.push(sock);
        }
    });

    let url = format!("postgres://probe@{addr}/db?connect_timeout=1&sslmode=require");
    let start = std::time::Instant::now();
    let result = postgresql::Connection::connect(&url).await;
    assert!(result.is_err(), "a stalling server must not hang connect");
    assert!(
        start.elapsed() < Duration::from_secs(5),
        "connect took {:?}; the timeout was not applied to the handshake",
        start.elapsed()
    );
}

#[tokio::test]
async fn notification_arriving_during_a_query_is_delivered() {
    run(Case::NotificationDuringQuery, |conn| async move {
        let mut notifications = conn.notifications();
        let result = query(&conn).await;
        assert!(result.is_ok(), "the query itself succeeds: {result:?}");
        let got = tokio::time::timeout(Duration::from_secs(5), notifications.recv()).await;
        let notification = got.expect("notification timed out").expect("channel closed");
        assert_eq!(notification.channel, "probe_chan");
        assert_eq!(notification.payload, "during-query");
    })
    .await;
}

// ---------------------------------------------------------------------------

async fn query(conn: &postgresql::Connection) -> Result<Vec<postgresql::Row>, postgresql::Error> {
    tokio::time::timeout(
        Duration::from_secs(10),
        postgresql::Query::new("SELECT 1", &[]).fetch_all::<postgresql::Connection, postgresql::Row>(conn),
    )
    .await
    .expect("the client must not hang on a malformed message")
}

/// Starts a fake server for `case`, connects a real client to it, and runs
/// `check`. The connection is always torn down afterwards.
async fn run<F, Fut>(case: Case, check: F)
where
    F: FnOnce(postgresql::Connection) -> Fut,
    Fut: std::future::Future<Output = ()>,
{
    let addr = serve(case).await;
    let url = format!("postgres://probe@{addr}/db");
    let conn = tokio::time::timeout(Duration::from_secs(20), postgresql::Connection::connect(&url))
        .await
        .expect("handshake timed out")
        .expect("connect");
    check(conn).await;
}

// --------------------------------------------------------------- fake server

fn frame(tag: u8, payload: &[u8]) -> Vec<u8> {
    let mut out = vec![tag];
    out.extend_from_slice(&((payload.len() + 4) as i32).to_be_bytes());
    out.extend_from_slice(payload);
    out
}

fn cstr(value: &str) -> Vec<u8> {
    let mut out = value.as_bytes().to_vec();
    out.push(0);
    out
}

fn ready() -> Vec<u8> {
    frame(b'Z', &[b'I'])
}

fn authenticate() -> Vec<u8> {
    let mut payload = 0i32.to_be_bytes().to_vec();
    payload.extend_from_slice(&ready()); // placeholder, replaced below
    frame(b'R', &0i32.to_be_bytes())
}

fn row_description() -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&1i16.to_be_bytes());
    payload.extend_from_slice(&cstr("x"));
    payload.extend_from_slice(&0u32.to_be_bytes()); // table oid
    payload.extend_from_slice(&0i16.to_be_bytes()); // column attr
    payload.extend_from_slice(&23u32.to_be_bytes()); // int4
    payload.extend_from_slice(&4i16.to_be_bytes());
    payload.extend_from_slice(&(-1i32).to_be_bytes());
    payload.extend_from_slice(&1i16.to_be_bytes()); // binary
    frame(b'T', &payload)
}

fn data_row(fields: &[Option<&[u8]>]) -> Vec<u8> {
    let mut payload = Vec::new();
    payload.extend_from_slice(&(fields.len() as i16).to_be_bytes());
    for field in fields {
        match field {
            None => payload.extend_from_slice(&(-1i32).to_be_bytes()),
            Some(bytes) => {
                payload.extend_from_slice(&(bytes.len() as i32).to_be_bytes());
                payload.extend_from_slice(bytes);
            }
        }
    }
    frame(b'D', &payload)
}

fn command_complete(tag: &str) -> Vec<u8> {
    frame(b'C', &cstr(tag))
}

/// Answer to `Parse` + `Describe` + `Sync`.
fn prepare_response() -> Vec<u8> {
    let mut out = frame(b'1', &[]);
    let payload = 0i16.to_be_bytes().to_vec();
    out.extend_from_slice(&frame(b't', &payload));
    out.extend_from_slice(&row_description());
    out.extend_from_slice(&ready());
    out
}

/// Answer to `Bind` + `Execute` + `Sync`.
fn case_response(case: Case) -> Vec<u8> {
    let mut out = frame(b'2', &[]);
    match case {
        Case::DataRowNegativeCount => {
            let mut payload = (-1i16).to_be_bytes().to_vec();
            payload.extend_from_slice(&4i32.to_be_bytes());
            out.extend_from_slice(&frame(b'D', &payload));
            out.extend_from_slice(&command_complete("SELECT 1"));
        }
        Case::DataRowCountMismatch => {
            out.extend_from_slice(&data_row(&[Some(&1i32.to_be_bytes()), Some(&2i32.to_be_bytes())]));
            out.extend_from_slice(&command_complete("SELECT 1"));
        }
        Case::DataRowTruncated => {
            let mut payload = 1i16.to_be_bytes().to_vec();
            payload.extend_from_slice(&100i32.to_be_bytes()); // claims 100 bytes
            payload.extend_from_slice(b"short");
            out.extend_from_slice(&frame(b'D', &payload));
            out.extend_from_slice(&command_complete("SELECT 1"));
        }
        Case::RowDescriptionNegativeCount => {
            out.extend_from_slice(&frame(b'T', &(-1i16).to_be_bytes()));
        }
        Case::ParameterDescriptionNegativeCount => {
            out.extend_from_slice(&frame(b't', &(-1i16).to_be_bytes()));
        }
        Case::OversizedMessageLength => {
            out.push(b'D');
            out.extend_from_slice(&i32::MAX.to_be_bytes());
        }
        Case::ErrorNonAsciiCode => {
            let mut payload = Vec::new();
            payload.extend_from_slice(&cstr("SERROR"));
            payload.extend_from_slice("C\u{20ac}".as_bytes());
            payload.push(0);
            payload.extend_from_slice(&cstr("Mweird code"));
            payload.push(0);
            out.extend_from_slice(&frame(b'E', &payload));
        }
        Case::CopyOutTruncated => {
            // CopyOutResponse (binary), one field per column.
            let mut payload = vec![1u8];
            payload.extend_from_slice(&1i16.to_be_bytes());
            payload.extend_from_slice(&1i16.to_be_bytes());
            out.extend_from_slice(&frame(b'H', &payload));
            // The file header plus a row whose field runs past the end, then
            // CopyDone without the -1 trailer.
            let mut data = b"PGCOPY\n\xff\r\n\0".to_vec();
            data.extend_from_slice(&0i32.to_be_bytes());
            data.extend_from_slice(&0i32.to_be_bytes());
            data.extend_from_slice(&1i16.to_be_bytes());
            data.extend_from_slice(&1000i32.to_be_bytes());
            data.extend_from_slice(&[0u8; 8]);
            out.extend_from_slice(&frame(b'd', &data));
            out.extend_from_slice(&frame(b'c', &[]));
            out.extend_from_slice(&command_complete("COPY 1"));
        }
        Case::CopyOutHugeFieldLength => {
            let mut payload = vec![1u8];
            payload.extend_from_slice(&1i16.to_be_bytes());
            payload.extend_from_slice(&1i16.to_be_bytes());
            out.extend_from_slice(&frame(b'H', &payload));
            // The header plus a row that claims a ~2 GiB field: the client must
            // refuse to buffer it rather than wait for the bytes.
            let mut data = b"PGCOPY\n\xff\r\n\0".to_vec();
            data.extend_from_slice(&0i32.to_be_bytes());
            data.extend_from_slice(&0i32.to_be_bytes());
            data.extend_from_slice(&1i16.to_be_bytes());
            data.extend_from_slice(&i32::MAX.to_be_bytes());
            out.extend_from_slice(&frame(b'd', &data));
            out.extend_from_slice(&frame(b'c', &[]));
            out.extend_from_slice(&command_complete("COPY 1"));
        }
        Case::CopyOutSplitRow => {
            let mut payload = vec![1u8];
            payload.extend_from_slice(&1i16.to_be_bytes());
            payload.extend_from_slice(&1i16.to_be_bytes());
            out.extend_from_slice(&frame(b'H', &payload));
            let mut data = b"PGCOPY\n\xff\r\n\0".to_vec();
            data.extend_from_slice(&0i32.to_be_bytes());
            data.extend_from_slice(&0i32.to_be_bytes());
            data.extend_from_slice(&2i16.to_be_bytes()); // 2 fields
            data.extend_from_slice(&4i32.to_be_bytes());
            data.extend_from_slice(&7i32.to_be_bytes()); // int4 7
            data.extend_from_slice(&2i32.to_be_bytes());
            data.extend_from_slice(b"xy"); // text "xy"
            data.extend_from_slice(&(-1i16).to_be_bytes()); // trailer
            // Split the whole stream into one-byte CopyData messages: the
            // client must reassemble the row without quadratic copying.
            for byte in data {
                out.extend_from_slice(&frame(b'd', &[byte]));
            }
            out.extend_from_slice(&frame(b'c', &[]));
            out.extend_from_slice(&command_complete("COPY 1"));
        }
        Case::CopyBothResponse => {
            // CopyBothResponse (streaming replication), which the client must
            // reject instead of treating as a readable COPY stream.
            let mut payload = vec![1u8];
            payload.extend_from_slice(&1i16.to_be_bytes());
            payload.extend_from_slice(&1i16.to_be_bytes());
            out.extend_from_slice(&frame(b'W', &payload));
            out.extend_from_slice(&command_complete("COPY 1"));
        }
        Case::CopyInResponseDuringQuery => {
            // A server that enters COPY-in mode in response to a parameterized
            // query. The client must send CopyFail (not hang waiting for COPY
            // data) and then reach the trailing ReadyForQuery.
            let mut payload = vec![1u8];
            payload.extend_from_slice(&1i16.to_be_bytes());
            payload.extend_from_slice(&1i16.to_be_bytes());
            out.extend_from_slice(&frame(b'G', &payload));
        }
        Case::AuthMessageFlood => {
            // Handled during startup; nothing to send for an `Execute`.
        }
        Case::NotificationDuringQuery => {
            let mut payload = Vec::new();
            payload.extend_from_slice(&1234i32.to_be_bytes());
            payload.extend_from_slice(&cstr("probe_chan"));
            payload.extend_from_slice(&cstr("during-query"));
            out.extend_from_slice(&frame(b'A', &payload));
            out.extend_from_slice(&data_row(&[Some(&1i32.to_be_bytes())]));
            out.extend_from_slice(&command_complete("SELECT 1"));
        }
    }
    out.extend_from_slice(&ready());
    out
}

/// A `CertifiedKey` backed by a generated key and a placeholder certificate.
///
/// The tests connect with `sslmode=require`, which does not verify the server
/// certificate, so nothing has to be signed.
fn server_config() -> Arc<ServerConfig> {
    let provider = Arc::new(crypto_rustls::default_provider());
    let key = crypto::p256::SecretKey::generate().expect("generate key");
    let der = crypto::encoding::pkcs8::encode_p256_pkcs8_der(&key).expect("encode key");
    let signing_key = provider
        .key_provider
        .load_private_key(PrivateKeyDer::Pkcs8(PrivatePkcs8KeyDer::from(der.to_vec())))
        .expect("load key");
    let certified = CertifiedKey::new(vec![CertificateDer::from(vec![0x30, 0x00])], signing_key);

    Arc::new(
        ServerConfig::builder_with_provider(provider)
            .with_protocol_versions(&[&rustls::version::TLS13])
            .expect("TLS 1.3")
            .with_no_client_auth()
            .with_cert_resolver(Arc::new(SingleCertAndKey::from(certified))),
    )
}

async fn serve(case: Case) -> SocketAddr {
    let config = server_config();
    let listener = TcpListener::bind("127.0.0.1:0").await.expect("bind");
    let addr = listener.local_addr().expect("local addr");
    tokio::spawn(async move {
        while let Ok((sock, _)) = listener.accept().await {
            let config = config.clone();
            tokio::spawn(async move {
                let _ = handle(sock, config, case).await;
            });
        }
    });
    addr
}

async fn handle(mut sock: TcpStream, config: Arc<ServerConfig>, case: Case) -> std::io::Result<()> {
    // Answer `SSLRequest` with `S` and take the connection over TLS.
    let mut head = [0u8; 8];
    sock.read_exact(&mut head).await?;
    sock.write_all(&[b'S']).await?;
    sock.flush().await?;
    let mut sock = tokio_rustls::TlsAcceptor::from(config).accept(sock).await?;

    // Startup message (untagged).
    let mut len = [0u8; 4];
    sock.read_exact(&mut len).await?;
    let mut body = vec![0u8; i32::from_be_bytes(len) as usize - 4];
    sock.read_exact(&mut body).await?;

    if let Case::AuthMessageFlood = case {
        // Far more startup messages than the client accepts: it must give up
        // with an error rather than reading forever.
        for i in 0..5000u32 {
            let mut payload = Vec::new();
            payload.extend_from_slice(&cstr("flood"));
            payload.extend_from_slice(&cstr(&i.to_string()));
            sock.write_all(&frame(b'S', &payload)).await?;
        }
        sock.flush().await?;
        tokio::time::sleep(Duration::from_secs(1)).await;
        return Ok(());
    }

    sock.write_all(&authenticate()).await?;
    sock.write_all(&ready()).await?;
    sock.flush().await?;

    // Answer each `Sync`: the first one belongs to `Parse`/`Describe`.
    let mut buf = BytesMut::new();
    let mut saw_bind = false;
    let mut sent_case = false;
    loop {
        while buf.len() >= 5 {
            let tag = buf[0];
            let len = i32::from_be_bytes([buf[1], buf[2], buf[3], buf[4]]) as usize;
            if buf.len() < len + 1 {
                break;
            }
            buf.advance(len + 1);
            match tag {
                b'S' => {
                    let response = if !saw_bind {
                        prepare_response()
                    } else if !sent_case {
                        sent_case = true;
                        case_response(case)
                    } else {
                        ready()
                    };
                    sock.write_all(&response).await?;
                    sock.flush().await?;
                }
                b'B' => saw_bind = true,
                b'Q' => {
                    if !sent_case {
                        sent_case = true;
                        sock.write_all(&case_response(case)).await?;
                    } else {
                        sock.write_all(&ready()).await?;
                    }
                    sock.flush().await?;
                }
                b'X' => return Ok(()),
                _ => {}
            }
        }
        let mut chunk = [0u8; 8192];
        let read = sock.read(&mut chunk).await?;
        if read == 0 {
            return Ok(());
        }
        buf.put_slice(&chunk[..read]);
    }
}
