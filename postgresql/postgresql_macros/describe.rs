//! Compile-time statement description.
//!
//! When `DATABASE_URL` is set, the query macros connect to the server over TLS
//! and `Parse`/`Describe` the statement to learn the inferred parameter OIDs
//! and result columns. Nothing is executed.

use std::{
    io::{Read, Write},
    net::{TcpStream, ToSocketAddrs},
    sync::Arc,
    time::Duration,
};

use bytes::BytesMut;
use postgresql_protocol::{
    backend::{BackendMessage, FieldDescription},
    frontend,
    scram::ScramClient,
};
use rustls::{
    ClientConfig, ClientConnection, DigitallySignedStruct, RootCertStore, SignatureScheme, StreamOwned,
    client::danger::{HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier},
    pki_types::{CertificateDer, ServerName, UnixTime},
};

/// One result column, as seen at compile time.
#[derive(Clone)]
#[allow(dead_code)]
pub struct ColumnInfo {
    pub name: String,
    pub type_oid: u32,
    pub nullable: bool,
    pub nullable_known: bool,
}

/// The inferred shape of a statement.
#[allow(dead_code)]
pub struct Describe {
    pub param_oids: Vec<u32>,
    pub columns: Vec<ColumnInfo>,
}

struct Config {
    host: String,
    port: u16,
    user: String,
    password: Option<String>,
    database: Option<String>,
    sslmode: String,
    sslrootcert: Option<String>,
    connect_timeout: Duration,
    max_scram_iterations: u32,
}

/// Percent-decodes one URL component, matching what `Config::parse` does at
/// runtime so that a URL works the same at build time and at run time.
fn decode(input: &str) -> Result<String, String> {
    percent_encoding::percent_decode_str(input)
        .decode_utf8()
        .map_err(|e| format!("invalid percent-encoding in DATABASE_URL: {e}"))
        .map(|s| s.into_owned())
}

fn parse_config(url: &str) -> Result<Config, String> {
    let parsed = url::Url::parse(url).map_err(|e| format!("invalid DATABASE_URL: {e}"))?;
    let host = parsed
        .host_str()
        .filter(|h| !h.is_empty())
        .unwrap_or("localhost")
        .to_string();
    let port = parsed.port().unwrap_or(5432);
    let user = decode(parsed.username())?;
    let password = parsed.password().map(decode).transpose()?;
    let database = {
        let path = parsed.path().trim_start_matches('/');
        if path.is_empty() { None } else { Some(decode(path)?) }
    };
    let mut sslmode = "require".to_string();
    let mut sslrootcert = None;
    let mut connect_timeout = Duration::from_secs(10);
    let mut max_scram_iterations = postgresql_protocol::scram::DEFAULT_MAX_SCRAM_ITERATIONS;
    for (key, value) in parsed.query_pairs() {
        match key.as_ref() {
            "sslmode" => sslmode = value.into_owned(),
            "sslrootcert" => sslrootcert = Some(value.into_owned()),
            // A stalled database must not hang the build forever.
            "connect_timeout" => {
                connect_timeout =
                    Duration::from_secs(value.parse().map_err(|_| "invalid `connect_timeout`".to_string())?);
            }
            "max_scram_iterations" => {
                max_scram_iterations = value
                    .parse()
                    .map_err(|_| "invalid `max_scram_iterations`".to_string())?;
            }
            _ => {}
        }
    }
    Ok(Config {
        host,
        port,
        user,
        password,
        database,
        sslmode,
        sslrootcert,
        connect_timeout,
        max_scram_iterations,
    })
}

trait ReadWrite: Read + Write + Send {}
impl<T: Read + Write + Send> ReadWrite for T {}

struct Connection {
    stream: Box<dyn ReadWrite>,
    buf: BytesMut,
}

impl Connection {
    fn send(&mut self, bytes: &[u8]) -> Result<(), String> {
        self.stream.write_all(bytes).map_err(|e| e.to_string())?;
        self.stream.flush().map_err(|e| e.to_string())
    }

    fn read_message(&mut self) -> Result<BackendMessage, String> {
        loop {
            match postgresql_protocol::decode_with_limit(&mut self.buf, postgresql_protocol::DEFAULT_MAX_MESSAGE_LEN) {
                Ok(Some(message)) => return Ok(message),
                Ok(None) => {}
                Err(postgresql_protocol::Error::Server(db)) => {
                    return Err(format!("server error: {db}"));
                }
                Err(err) => return Err(format!("protocol error: {err}")),
            }
            let mut chunk = [0u8; 8192];
            let read = self.stream.read(&mut chunk).map_err(|e| e.to_string())?;
            if read == 0 {
                return Err("connection closed by server".into());
            }
            self.buf.extend_from_slice(&chunk[..read]);
        }
    }
}

/// Connects over TLS and authenticates, returning a ready connection.
fn connect(url: &str) -> Result<Connection, String> {
    let config = parse_config(url)?;
    let addr = (config.host.as_str(), config.port)
        .to_socket_addrs()
        .map_err(|e| format!("cannot resolve {}:{}: {e}", config.host, config.port))?
        .next()
        .ok_or_else(|| format!("cannot resolve {}:{}", config.host, config.port))?;
    let tcp = TcpStream::connect_timeout(&addr, config.connect_timeout)
        .map_err(|e| format!("cannot connect to {addr}: {e}"))?;
    // Bound every read and write too: a server that accepts the connection and
    // then stalls must fail the build instead of hanging it.
    let _ = tcp.set_read_timeout(Some(config.connect_timeout));
    let _ = tcp.set_write_timeout(Some(config.connect_timeout));
    let stream = negotiate_tls(tcp, &config)?;
    let mut conn = Connection {
        stream,
        buf: BytesMut::with_capacity(8192),
    };
    startup(&mut conn, &config)?;
    authenticate(&mut conn, &config)?;
    Ok(conn)
}

/// Connects over TLS, authenticates, and describes `sql`.
pub fn describe(url: &str, sql: &str) -> Result<Describe, String> {
    with_connection(url, |conn| describe_statement(conn, sql))
}

/// Resolves the input/output columns of a binary `COPY` statement.
pub fn copy_columns(url: &str, sql: &str) -> Result<Vec<ColumnInfo>, String> {
    let spec = parse_copy(sql)?;
    with_connection(url, |conn| resolve_copy_columns(conn, &spec))
}

/// Runs `f` on a connection cached for the compiler process, reconnecting once
/// if a cached connection has gone stale.
///
/// The query macros expand once per invocation; opening a fresh TLS + SCRAM
/// connection for each one is slow and can exhaust the server's connection
/// limit during a large build. The connection is reused across invocations and
/// dropped (then reconnected on the next call) after any error.
fn with_connection<T>(url: &str, mut f: impl FnMut(&mut Connection) -> Result<T, String>) -> Result<T, String> {
    static CACHE: std::sync::Mutex<Option<(String, Connection)>> = std::sync::Mutex::new(None);
    let mut last_err = None;
    for _ in 0..2 {
        let mut guard = CACHE.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let stale = guard.as_ref().map(|(cached, _)| cached != url).unwrap_or(true);
        if stale {
            *guard = None;
        }
        if guard.is_none() {
            *guard = Some((url.to_string(), connect(url)?));
        }
        let conn = &mut guard.as_mut().expect("connected above").1;
        match f(conn) {
            Ok(value) => return Ok(value),
            Err(err) => {
                // The connection may be in an unknown state; drop it so the
                // next attempt reconnects from scratch.
                *guard = None;
                last_err = Some(err);
            }
        }
    }
    Err(last_err.unwrap_or_else(|| "postgresql: could not describe the statement".to_string()))
}

fn negotiate_tls(mut tcp: TcpStream, config: &Config) -> Result<Box<dyn ReadWrite>, String> {
    let mut request = BytesMut::new();
    frontend::ssl_request(&mut request);
    tcp.write_all(&request).map_err(|e| e.to_string())?;
    tcp.flush().map_err(|e| e.to_string())?;

    let mut response = [0u8; 1];
    tcp.read_exact(&mut response).map_err(|e| e.to_string())?;

    match response[0] {
        b'S' => {
            let provider = Arc::new(crypto_rustls::default_provider());
            let builder = ClientConfig::builder_with_provider(provider.clone())
                .with_protocol_versions(&[&rustls::version::TLS13])
                .map_err(|e| e.to_string())?;
            let client_config = match config.sslmode.as_str() {
                "verify-full" => {
                    let roots = roots(config)?;
                    builder.with_root_certificates(roots).with_no_client_auth()
                }
                "verify-ca" => builder
                    .dangerous()
                    .with_custom_certificate_verifier(Arc::new(ChainOnlyVerifier {
                        roots: roots(config)?,
                        algorithms: provider.signature_verification_algorithms,
                    }))
                    .with_no_client_auth(),
                _ => builder
                    .dangerous()
                    .with_custom_certificate_verifier(Arc::new(NoVerifier {
                        algorithms: provider.signature_verification_algorithms,
                    }))
                    .with_no_client_auth(),
            };
            let server_name = ServerName::try_from(config.host.clone()).map_err(|e| e.to_string())?;
            let connection = ClientConnection::new(Arc::new(client_config), server_name).map_err(|e| e.to_string())?;
            Ok(Box::new(StreamOwned::new(connection, tcp)))
        }
        b'N' => Err("the server does not support TLS; compile-time checking requires TLS (set \
             STDX_POSTGRESQL_CHECK=false to disable checking)"
            .into()),
        other => Err(format!("unexpected response to SSLRequest: {other}")),
    }
}

#[derive(Debug)]
struct NoVerifier {
    algorithms: rustls::crypto::WebPkiSupportedAlgorithms,
}

/// `verify-ca`: verify the chain, not the hostname.
#[derive(Debug)]
struct ChainOnlyVerifier {
    roots: RootCertStore,
    algorithms: rustls::crypto::WebPkiSupportedAlgorithms,
}

impl ServerCertVerifier for ChainOnlyVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        let cert = rustls::server::ParsedCertificate::try_from(end_entity)?;
        rustls::client::verify_server_cert_signed_by_trust_anchor(
            &cert,
            &self.roots,
            intermediates,
            now,
            self.algorithms.all,
        )?;
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls12_signature(message, cert, dss, &self.algorithms)
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        rustls::crypto::verify_tls13_signature(message, cert, dss, &self.algorithms)
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.algorithms.supported_schemes()
    }
}

impl ServerCertVerifier for NoVerifier {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::crypto::verify_tls12_signature(message, cert, dss, &self.algorithms)
            .unwrap_or(HandshakeSignatureValid::assertion()))
    }

    fn verify_tls13_signature(
        &self,
        message: &[u8],
        cert: &CertificateDer<'_>,
        dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(rustls::crypto::verify_tls13_signature(message, cert, dss, &self.algorithms)
            .unwrap_or(HandshakeSignatureValid::assertion()))
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        self.algorithms.supported_schemes()
    }
}

fn roots(config: &Config) -> Result<RootCertStore, String> {
    let mut store = RootCertStore::empty();
    if let Some(path) = &config.sslrootcert {
        let data = std::fs::read(path).map_err(|e| format!("cannot read sslrootcert `{path}`: {e}"))?;
        for cert in parse_pem_certificates(&data) {
            store
                .add(cert)
                .map_err(|e| format!("invalid certificate in `{path}`: {e}"))?;
        }
        return Ok(store);
    }
    for dir in [
        "/etc/ssl/certs",
        "/etc/pki/tls/certs",
        "/usr/local/share/certs",
        "/usr/share/ca-certificates/mozilla",
    ] {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                let is_cert = path
                    .extension()
                    .and_then(|e| e.to_str())
                    .map(|e| matches!(e, "pem" | "crt" | "cer"))
                    .unwrap_or(false);
                if !is_cert {
                    continue;
                }
                if let Ok(data) = std::fs::read(&path) {
                    for cert in parse_pem_certificates(&data) {
                        let _ = store.add(cert);
                    }
                }
            }
        }
    }
    if store.is_empty() {
        return Err("no trusted root certificates found; provide `sslrootcert`".into());
    }
    Ok(store)
}

fn parse_pem_certificates(data: &[u8]) -> Vec<CertificateDer<'static>> {
    const BEGIN: &[u8] = b"-----BEGIN CERTIFICATE-----";
    const END: &[u8] = b"-----END CERTIFICATE-----";
    let mut certs = Vec::new();
    let mut rest = data;
    while let Some(start) = find(rest, BEGIN) {
        let after = &rest[start + BEGIN.len()..];
        let Some(end) = find(after, END) else {
            break;
        };
        let body: Vec<u8> = after[..end]
            .iter()
            .copied()
            .filter(|b| !b.is_ascii_whitespace())
            .collect();
        if let Ok(der) = base64::decode(&body, base64::Alphabet::Standard) {
            certs.push(CertificateDer::from(der));
        }
        rest = &after[end + END.len()..];
    }
    certs
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

fn startup(conn: &mut Connection, config: &Config) -> Result<(), String> {
    let mut params: Vec<(&str, &str)> = vec![("client_encoding", "UTF8"), ("user", &config.user)];
    if let Some(database) = &config.database {
        params.push(("database", database));
    }
    let mut buf = BytesMut::new();
    frontend::startup(&mut buf, &params).map_err(|e| e.to_string())?;
    conn.send(&buf)
}

fn authenticate(conn: &mut Connection, config: &Config) -> Result<(), String> {
    let mut write = BytesMut::new();
    loop {
        match conn.read_message()? {
            BackendMessage::AuthenticationOk => {}
            BackendMessage::AuthenticationCleartextPassword => {
                write.clear();
                frontend::password(&mut write, config.password.as_deref().unwrap_or("")).map_err(|e| e.to_string())?;
                conn.send(&write)?;
            }
            BackendMessage::AuthenticationSasl(mechanisms) => {
                if !mechanisms.iter().any(|m| m == "SCRAM-SHA-256") {
                    return Err("server does not offer SCRAM-SHA-256".into());
                }
                let mut client = ScramClient::with_max_iterations(
                    config.password.as_deref().unwrap_or(""),
                    config.max_scram_iterations,
                )
                .map_err(|e| e.to_string())?;
                write.clear();
                frontend::sasl_initial_response(&mut write, "SCRAM-SHA-256", client.client_first_message().as_bytes())
                    .map_err(|e| e.to_string())?;
                conn.send(&write)?;

                match conn.read_message()? {
                    BackendMessage::AuthenticationSaslContinue(data) => {
                        client.parse_server_first_message(&data).map_err(|e| e.to_string())?;
                        let final_message = client.build_client_final_message().map_err(|e| e.to_string())?;
                        write.clear();
                        frontend::sasl_response(&mut write, &final_message).map_err(|e| e.to_string())?;
                        conn.send(&write)?;
                    }
                    other => return Err(format!("expected SASLContinue, got {other:?}")),
                }
                match conn.read_message()? {
                    BackendMessage::AuthenticationSaslFinal(data) => {
                        client.parse_server_final_message(&data).map_err(|e| e.to_string())?;
                    }
                    other => return Err(format!("expected SASLFinal, got {other:?}")),
                }
            }
            BackendMessage::AuthenticationMd5Password(_) => {
                return Err("MD5 authentication is not supported".into());
            }
            BackendMessage::ParameterStatus {
                ..
            } => {}
            BackendMessage::BackendKeyData {
                ..
            } => {}
            BackendMessage::NoticeResponse(_) => {}
            BackendMessage::ReadyForQuery(_) => break,
            other => return Err(format!("unexpected message during startup: {other:?}")),
        }
    }
    Ok(())
}

fn describe_statement(conn: &mut Connection, sql: &str) -> Result<Describe, String> {
    let mut write = BytesMut::new();
    frontend::parse(&mut write, "", sql, &[]).map_err(|e| e.to_string())?;
    frontend::describe(&mut write, b'S', "").map_err(|e| e.to_string())?;
    frontend::sync(&mut write).map_err(|e| e.to_string())?;
    conn.send(&write)?;

    let mut param_oids = Vec::new();
    let mut fields: Option<Vec<FieldDescription>> = None;
    loop {
        match conn.read_message()? {
            BackendMessage::ParseComplete => {}
            BackendMessage::ParameterDescription(oids) => param_oids = oids,
            BackendMessage::RowDescription(described) => fields = Some(described),
            BackendMessage::NoData => fields = Some(Vec::new()),
            BackendMessage::NoticeResponse(_) => {}
            BackendMessage::ReadyForQuery(_) => break,
            other => return Err(format!("unexpected message while describing: {other:?}")),
        }
    }

    // Resolve the nullability of every table column in a single round trip.
    let fields = fields.unwrap_or_default();
    let refs: Vec<(u32, i16)> = fields
        .iter()
        .filter(|f| f.table_oid != 0 && f.column_attr > 0)
        .map(|f| (f.table_oid, f.column_attr))
        .collect();
    let not_null = columns_not_null(conn, &refs)?;

    let mut columns = Vec::with_capacity(fields.len());
    let mut next = 0usize;
    for field in fields {
        let (nullable, nullable_known) = if field.table_oid != 0 && field.column_attr > 0 {
            let known = not_null.get(next).copied().unwrap_or(false);
            next += 1;
            (!known, true)
        } else {
            // An expression column: its nullability cannot be derived.
            (false, false)
        };
        columns.push(ColumnInfo {
            name: field.name,
            type_oid: field.type_oid,
            nullable,
            nullable_known,
        });
    }

    Ok(Describe {
        param_oids,
        columns,
    })
}

/// Returns `pg_attribute.attnotnull` for each `(attrelid, attnum)` pair, in
/// one round trip. Missing attributes report `false`.
fn columns_not_null(conn: &mut Connection, refs: &[(u32, i16)]) -> Result<Vec<bool>, String> {
    if refs.is_empty() {
        return Ok(Vec::new());
    }
    // The values are numbers taken from the server's own `RowDescription`, so
    // they cannot contain SQL.
    let list: Vec<String> = refs.iter().map(|(oid, attnum)| format!("({oid},{attnum})")).collect();
    let sql = format!(
        "SELECT a.attnotnull FROM pg_catalog.pg_attribute a          WHERE (a.attrelid, a.attnum) IN ({}) ORDER BY a.attrelid, a.attnum",
        list.join(", ")
    );
    let rows = simple_query_rows(conn, &sql)?;
    let mut out = vec![false; refs.len()];
    for (index, row) in rows.iter().enumerate() {
        if let Some(value) = row.first().and_then(|v| v.as_deref()) {
            if let Some(slot) = out.get_mut(index) {
                *slot = value == "t";
            }
        }
    }
    Ok(out)
}

struct CopySpec {
    relation: String,
    columns: Option<Vec<String>>,
}

/// Case-insensitively strips an ASCII `prefix`, never splitting a UTF-8
/// character boundary (a non-ASCII SQL string must not panic the macro).
fn strip_prefix_ci<'a>(s: &'a str, prefix: &str) -> Option<&'a str> {
    let head = s.get(..prefix.len())?;
    if head.eq_ignore_ascii_case(prefix) {
        s.get(prefix.len()..)
    } else {
        None
    }
}

fn unquote(s: &str) -> &str {
    let s = s.trim();
    if s.len() >= 2 && s.starts_with('"') && s.ends_with('"') {
        &s[1..s.len() - 1]
    } else {
        s
    }
}

/// Returns the end of the relation name, honouring double-quoted identifiers
/// (which may contain spaces, parentheses or case-sensitive names).
fn relation_end(s: &str) -> usize {
    let mut in_quotes = false;
    let mut escaped = false;
    for (index, c) in s.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        match c {
            '"' if in_quotes => escaped = true,
            '"' => in_quotes = true,
            '(' | ')' if !in_quotes => return index,
            c if c.is_whitespace() && !in_quotes => return index,
            _ => {}
        }
    }
    s.len()
}

/// Parses the documented subset of `COPY`: `COPY [schema.]table [(cols)] FROM
/// STDIN BINARY` or `... TO STDOUT BINARY`.
fn parse_copy(sql: &str) -> Result<CopySpec, String> {
    let rest = strip_prefix_ci(sql.trim(), "COPY").ok_or("not a COPY statement")?;
    let rest = rest.trim_start();

    let end = relation_end(rest);
    // Keep the identifier exactly as written (including its quotes) so that
    // `to_regclass` preserves case sensitivity: `COPY "Users"` must resolve to
    // `Users`, not to `users`.
    let relation = rest[..end].trim().to_string();
    if relation.is_empty() {
        return Err("COPY is missing a relation".into());
    }
    let after = rest[end..].trim_start();

    let (columns, after) = if let Some(stripped) = after.strip_prefix('(') {
        let close = stripped.find(')').ok_or("unterminated COPY column list")?;
        let cols: Vec<String> = stripped[..close]
            .split(',')
            .map(|c| unquote(c).to_string())
            .filter(|c| !c.is_empty())
            .collect();
        (Some(cols), stripped[close + 1..].trim_start())
    } else {
        (None, after)
    };

    let upper = after.to_ascii_uppercase();
    let is_in = upper.starts_with("FROM") && upper.contains("STDIN");
    let is_out = upper.starts_with("TO") && upper.contains("STDOUT");
    if !upper.contains("BINARY") || !(is_in || is_out) {
        return Err("only `COPY ... FROM STDIN BINARY` and `COPY ... TO STDOUT BINARY` are supported".into());
    }
    Ok(CopySpec {
        relation,
        columns,
    })
}

fn resolve_copy_columns(conn: &mut Connection, spec: &CopySpec) -> Result<Vec<ColumnInfo>, String> {
    let rel = spec.relation.replace('\'', "''");
    let sql = format!(
        "SELECT a.attname, a.atttypid::int8::text, a.attnotnull \
         FROM pg_attribute a \
         WHERE a.attrelid = to_regclass('{rel}') AND a.attnum > 0 AND NOT a.attisdropped \
         ORDER BY a.attnum"
    );
    let rows = simple_query_rows(conn, &sql)?;
    let mut all: Vec<ColumnInfo> = Vec::new();
    for row in rows {
        if row.len() < 3 {
            continue;
        }
        let name = row[0].clone().ok_or("missing column name")?;
        let oid: u32 = row[1]
            .as_deref()
            .unwrap_or("0")
            .parse()
            .map_err(|_| "invalid type oid")?;
        let not_null = row[2].as_deref() == Some("t");
        all.push(ColumnInfo {
            name,
            type_oid: oid,
            nullable: !not_null,
            nullable_known: true,
        });
    }
    if all.is_empty() {
        return Err(format!("relation `{}` was not found", spec.relation));
    }
    match &spec.columns {
        None => Ok(all),
        Some(cols) => {
            let mut out = Vec::with_capacity(cols.len());
            for col in cols {
                let found = all
                    .iter()
                    .find(|c| &c.name == col)
                    .ok_or_else(|| format!("column `{col}` does not exist in `{}`", spec.relation))?;
                out.push(found.clone());
            }
            Ok(out)
        }
    }
}

fn simple_query_rows(conn: &mut Connection, sql: &str) -> Result<Vec<Vec<Option<String>>>, String> {
    let mut write = BytesMut::new();
    frontend::query(&mut write, sql).map_err(|e| e.to_string())?;
    conn.send(&write)?;
    let mut rows = Vec::new();
    loop {
        match conn.read_message()? {
            BackendMessage::RowDescription(_) => {}
            BackendMessage::DataRow(payload) => rows.push(parse_text_data_row(&payload)?),
            BackendMessage::CommandComplete(_) => {}
            BackendMessage::NoticeResponse(_) => {}
            BackendMessage::EmptyQueryResponse => {}
            BackendMessage::ReadyForQuery(_) => break,
            other => return Err(format!("unexpected message in simple query: {other:?}")),
        }
    }
    Ok(rows)
}

fn parse_text_data_row(payload: &[u8]) -> Result<Vec<Option<String>>, String> {
    if payload.len() < 2 {
        return Err("truncated data row".into());
    }
    let raw_count = i16::from_be_bytes([payload[0], payload[1]]);
    let count = usize::try_from(raw_count).map_err(|_| format!("invalid data row column count {raw_count}"))?;
    // Each field costs at least a 4-byte length prefix.
    let mut pos = 2usize;
    let mut fields = Vec::with_capacity(count.min(payload.len() / 4));
    for _ in 0..count {
        if payload.len() < pos + 4 {
            return Err("truncated data row".into());
        }
        let raw_len = i32::from_be_bytes([payload[pos], payload[pos + 1], payload[pos + 2], payload[pos + 3]]);
        pos += 4;
        if raw_len == -1 {
            fields.push(None);
        } else {
            let len = usize::try_from(raw_len).map_err(|_| format!("invalid data row column length {raw_len}"))?;
            if payload.len() < pos + len {
                return Err("truncated data row".into());
            }
            fields.push(Some(String::from_utf8_lossy(&payload[pos..pos + len]).into_owned()));
            pos += len;
        }
    }
    Ok(fields)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_copy_headers() {
        let spec = parse_copy("COPY public.t (a, b) FROM STDIN BINARY").unwrap();
        assert_eq!(spec.relation, "public.t");
        assert_eq!(spec.columns.unwrap(), vec!["a".to_string(), "b".to_string()]);

        let spec = parse_copy("copy t to stdout binary").unwrap();
        assert_eq!(spec.relation, "t");
        assert!(spec.columns.is_none());

        assert!(parse_copy("SELECT 1").is_err());
        assert!(parse_copy("COPY t FROM 'file'").is_err());
        assert!(parse_copy("COPY t FROM STDIN").is_err());
    }

    #[test]
    fn strip_prefix_ci_handles_non_ascii_without_panicking() {
        // A multi-byte character at the prefix boundary must not split a char.
        assert_eq!(strip_prefix_ci("CO\u{20ac}PY t FROM STDIN BINARY", "COPY"), None);
        assert_eq!(strip_prefix_ci("copy t to stdout binary", "COPY"), Some(" t to stdout binary"));
        assert_eq!(strip_prefix_ci("COPY", "COPY"), Some(""));
        assert_eq!(strip_prefix_ci("COP", "COPY"), None);
    }
}
