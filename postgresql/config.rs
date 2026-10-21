//! Connection configuration parsed from a `postgres://` URL.

use std::{fmt, time::Duration};

use zeroize::Zeroize;

use crate::error::Error;

/// Default number of bytes of decoded rows (or `COPY` chunks) buffered per
/// reader before the actor applies backpressure.
const DEFAULT_MAX_BUFFERED_BYTES: usize = 16 * 1024 * 1024;

/// How the client verifies the server certificate.
///
/// TLS is always required: `disable`, `prefer` and `allow` (which permit
/// plaintext) are rejected. The default is [`SslMode::Require`], which encrypts
/// the connection but does **not** authenticate the server — an active attacker
/// can impersonate it. Use [`SslMode::VerifyFull`] unless the network is
/// trusted. Note that libpq's `require` verifies the chain when a root CA is
/// present; this client never does.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SslMode {
    /// Require TLS. The server certificate is **not** verified, so the
    /// connection is encrypted but the server is unauthenticated (like
    /// libpq's `require`).
    #[default]
    Require,
    /// Require TLS and verify the certificate chain, but not the hostname.
    VerifyCa,
    /// Require TLS and verify both the certificate chain and the hostname.
    VerifyFull,
}

impl SslMode {
    fn parse(value: &str) -> Result<Self, Error> {
        match value {
            "require" => Ok(SslMode::Require),
            "verify-ca" => Ok(SslMode::VerifyCa),
            "verify-full" => Ok(SslMode::VerifyFull),
            "disable" | "prefer" | "allow" => Err(Error::Config(format!(
                "sslmode `{value}` is not supported: this client always requires TLS"
            ))),
            other => Err(Error::Config(format!("unsupported sslmode `{other}`"))),
        }
    }
}

/// A parsed `postgres://` connection URL.
#[derive(Clone)]
pub struct Config {
    /// Host name or IP address.
    pub host: String,
    /// TCP port.
    pub port: u16,
    /// User name.
    pub user: String,
    /// Password, if any.
    pub password: Option<String>,
    /// Database name, if any.
    pub database: Option<String>,
    /// `application_name` reported to the server.
    pub application_name: Option<String>,
    /// Extra `options` passed to the server.
    pub options: Option<String>,
    /// TLS mode.
    pub sslmode: SslMode,
    /// Path to a PEM file of trusted root certificates.
    pub sslrootcert: Option<String>,
    /// Timeout applied to establishing a connection.
    pub connect_timeout: Duration,
    /// IANA time zone used to interpret zone-less `timestamp`, `date` and
    /// `time` values. Defaults to UTC.
    pub timezone: Option<String>,
    /// Maximum number of prepared statements cached per connection (`0`
    /// disables caching, for transaction-mode poolers).
    pub statement_cache_size: usize,
    /// Largest accepted backend message, in bytes. Defaults to 128 MiB. A
    /// hostile server cannot make the client buffer more than this per message.
    pub max_message_len: usize,
    /// Maximum number of bytes of decoded rows (or `COPY` chunks) buffered on
    /// behalf of a reader that is not consuming them. Defaults to 16 MiB.
    ///
    /// A single row larger than this is still delivered (one row is always
    /// allowed); the bound applies to the queue.
    pub max_buffered_bytes: usize,
    /// Largest SCRAM iteration count accepted from the server. Defaults to
    /// 1,000,000. The server chooses this value and it is pure client CPU
    /// work, so it bounds the work an unauthenticated server can force.
    pub max_scram_iterations: u32,
    /// Pool settings parsed from the URL (overridden by an explicit
    /// [`PoolConfig`]).
    pub pool: PoolConfig,
}

impl fmt::Debug for Config {
    /// Formats the configuration with the password redacted, so a config
    /// logged or printed for diagnostics cannot leak the credentials.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("host", &self.host)
            .field("port", &self.port)
            .field("user", &self.user)
            .field("password", &self.password.as_ref().map(|_| "<redacted>"))
            .field("database", &self.database)
            .field("application_name", &self.application_name)
            .field("options", &self.options)
            .field("sslmode", &self.sslmode)
            .field("sslrootcert", &self.sslrootcert)
            .field("connect_timeout", &self.connect_timeout)
            .field("timezone", &self.timezone)
            .field("statement_cache_size", &self.statement_cache_size)
            .field("max_message_len", &self.max_message_len)
            .field("max_buffered_bytes", &self.max_buffered_bytes)
            .field("max_scram_iterations", &self.max_scram_iterations)
            .field("pool", &self.pool)
            .finish()
    }
}

fn decode(input: &str) -> Result<String, Error> {
    Ok(percent_encoding::percent_decode_str(input)
        .decode_utf8()
        .map_err(|e| Error::Config(format!("invalid percent-encoding in connection URL: {e}")))?
        .into_owned())
}

/// Rejects a value containing a NUL byte.
///
/// Startup parameters are NUL-delimited strings on the wire, so a NUL in one of
/// them would split it and inject extra startup parameters (for example a
/// different `user`). Refusing it at the configuration boundary keeps the
/// connection string from being able to alter the login.
fn reject_nul(name: &str, value: &str) -> Result<(), Error> {
    if value.contains('\0') {
        return Err(Error::Config(format!("`{name}` must not contain a NUL byte")));
    }
    Ok(())
}

/// Percent-decodes one URL component and rejects a decoded NUL byte.
fn decode_checked(name: &str, input: &str) -> Result<String, Error> {
    let decoded = decode(input)?;
    reject_nul(name, &decoded)?;
    Ok(decoded)
}

impl Drop for Config {
    /// Zeroizes the password. Best-effort: the string may have been copied
    /// elsewhere (for example into an error message) outside our control.
    fn drop(&mut self) {
        if let Some(password) = self.password.as_mut() {
            password.zeroize();
        }
    }
}

impl Config {
    /// Parses a `postgres://` or `postgresql://` URL.
    ///
    /// Supported query parameters: `sslmode`, `sslrootcert`, `connect_timeout`
    /// (seconds), `application_name`, `options`, `timezone`,
    /// `statement_cache_size`, `max_message_len` (bytes),
    /// `max_buffered_bytes` (bytes), `max_scram_iterations`, `pool_size`,
    /// `min_connections`, `acquire_timeout`, `idle_timeout` and
    /// `max_lifetime`. Unknown parameters are rejected so that typos surface
    /// immediately.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] for a malformed URL, a missing user, or an
    /// unsupported parameter.
    pub fn parse(url: &str) -> Result<Self, Error> {
        let parsed = url::Url::parse(url).map_err(|e| Error::Config(format!("invalid URL: {e}")))?;

        let scheme = parsed.scheme();
        if scheme != "postgres" && scheme != "postgresql" {
            return Err(Error::Config(format!(
                "unsupported URL scheme `{scheme}` (expected `postgres` or `postgresql`)"
            )));
        }

        // Normalize the host: `url::Url::host_str` keeps the brackets around an
        // IPv6 literal (`[::1]`), which is not a valid IP address for
        // `TcpStream::connect` nor a valid TLS server name.
        let host = match parsed.host() {
            Some(url::Host::Ipv6(ip)) => ip.to_string(),
            Some(url::Host::Ipv4(ip)) => ip.to_string(),
            Some(url::Host::Domain(domain)) => domain.to_string(),
            None => "localhost".to_string(),
        };
        reject_nul("host", &host)?;
        let port = parsed.port().unwrap_or(5432);

        let mut user = decode_checked("user", parsed.username())?;
        if user.is_empty() {
            user = std::env::var("PGUSER")
                .or_else(|_| std::env::var("USER"))
                .or_else(|_| std::env::var("LOGNAME"))
                .unwrap_or_default();
            reject_nul("user", &user)?;
        }
        if user.is_empty() {
            return Err(Error::Config("connection URL is missing a user".into()));
        }

        let password = parsed.password().map(|p| decode_checked("password", p)).transpose()?;

        let database = {
            let path = parsed.path().trim_start_matches('/');
            if path.is_empty() {
                None
            } else {
                Some(decode_checked("database", path)?)
            }
        };

        let mut sslmode = SslMode::default();
        let mut sslrootcert = None;
        let mut application_name = None;
        let mut options = None;
        let mut connect_timeout = Duration::from_secs(10);
        let mut timezone = None;
        let mut statement_cache_size = 256usize;
        let mut max_message_len = postgresql_protocol::DEFAULT_MAX_MESSAGE_LEN;
        let mut max_buffered_bytes = DEFAULT_MAX_BUFFERED_BYTES;
        let mut max_scram_iterations = postgresql_protocol::scram::DEFAULT_MAX_SCRAM_ITERATIONS;
        let mut pool = PoolConfig::default();

        for (key, value) in parsed.query_pairs() {
            match key.as_ref() {
                "sslmode" => sslmode = SslMode::parse(&value)?,
                "sslrootcert" => sslrootcert = Some(value.into_owned()),
                "application_name" => application_name = Some(value.into_owned()),
                "options" => options = Some(value.into_owned()),
                "connect_timeout" => connect_timeout = Duration::from_secs(parse_secs("connect_timeout", &value)?),
                "timezone" => timezone = Some(value.into_owned()),
                "statement_cache_size" => {
                    statement_cache_size = parse_count("statement_cache_size", &value)? as usize;
                }
                "max_message_len" => max_message_len = parse_len("max_message_len", &value)?,
                "max_buffered_bytes" => max_buffered_bytes = parse_buffered_bytes(&value)?,
                "max_scram_iterations" => max_scram_iterations = parse_iterations("max_scram_iterations", &value)?,
                "pool_size" => pool.pool_size = parse_count("pool_size", &value)?,
                "min_connections" => pool.min_connections = parse_count("min_connections", &value)?,
                "acquire_timeout" => pool.acquire_timeout = Duration::from_secs(parse_secs("acquire_timeout", &value)?),
                "idle_timeout" => pool.idle_timeout = Duration::from_secs(parse_secs("idle_timeout", &value)?),
                "max_lifetime" => pool.max_lifetime = Duration::from_secs(parse_secs("max_lifetime", &value)?),
                other => {
                    return Err(Error::Config(format!("unsupported connection parameter `{other}`")));
                }
            }
        }

        // The startup packet carries `application_name` and `options` as
        // NUL-delimited strings; a NUL would inject extra startup parameters.
        for (name, value) in [
            ("sslrootcert", &sslrootcert),
            ("application_name", &application_name),
            ("options", &options),
            ("timezone", &timezone),
        ] {
            if let Some(value) = value {
                reject_nul(name, value)?;
            }
        }

        // `connect_timeout` bounds the whole connection attempt. A zero
        // duration would make every attempt fail immediately, so it is
        // rejected instead of silently producing an unusable connection.
        if connect_timeout.is_zero() {
            return Err(Error::Config(
                "`connect_timeout` must be greater than zero; omit it to use the 10 second default".into(),
            ));
        }
        pool.validate()?;

        Ok(Config {
            host,
            port,
            user,
            password,
            database,
            application_name,
            options,
            sslmode,
            sslrootcert,
            connect_timeout,
            timezone,
            statement_cache_size,
            max_message_len,
            max_buffered_bytes,
            max_scram_iterations,
            pool,
        })
    }
}

/// Parses a byte length, rejecting values that are too small to be useful or
/// too large to fit the protocol's 32-bit length prefix.
fn parse_len(name: &str, value: &str) -> Result<usize, Error> {
    let n: usize = value
        .parse()
        .map_err(|_| Error::Config(format!("`{name}` must be an integer number of bytes")))?;
    check_len(name, n)?;
    Ok(n)
}

fn check_len(name: &str, n: usize) -> Result<(), Error> {
    if !(64 * 1024..=i32::MAX as usize).contains(&n) {
        return Err(Error::Config(format!("`{name}` must be between 65536 and {} bytes", i32::MAX)));
    }
    Ok(())
}

/// Parses the row-buffering budget (larger than one message is allowed).
fn parse_buffered_bytes(value: &str) -> Result<usize, Error> {
    const MAX: usize = 1024 * 1024 * 1024;
    let n: usize = value
        .parse()
        .map_err(|_| Error::Config("`max_buffered_bytes` must be an integer number of bytes".into()))?;
    if !(64 * 1024..=MAX).contains(&n) {
        return Err(Error::Config(format!(
            "`max_buffered_bytes` must be between 65536 and {MAX} bytes"
        )));
    }
    Ok(n)
}

/// Parses a positive iteration count.
fn parse_iterations(name: &str, value: &str) -> Result<u32, Error> {
    let n: u32 = value
        .parse()
        .map_err(|_| Error::Config(format!("`{name}` must be a positive integer")))?;
    if n == 0 {
        return Err(Error::Config(format!("`{name}` must be greater than zero")));
    }
    Ok(n)
}

fn parse_secs(name: &str, value: &str) -> Result<u64, Error> {
    value
        .parse()
        .map_err(|_| Error::Config(format!("`{name}` must be an integer number of seconds")))
}

fn parse_count(name: &str, value: &str) -> Result<u32, Error> {
    value
        .parse()
        .map_err(|_| Error::Config(format!("`{name}` must be a non-negative integer")))
}

/// Connection pool tuning.
///
/// These values can also be supplied in the connection URL (`pool_size`,
/// `min_connections`, `acquire_timeout`, `idle_timeout`, `max_lifetime`); a
/// [`PoolConfig`] passed to [`crate::Pool::connect`] overrides the URL.
#[derive(Debug, Clone)]
pub struct PoolConfig {
    /// Maximum number of connections in the pool.
    pub pool_size: u32,
    /// Connections to open eagerly when the pool is created, and the floor of
    /// live connections the idle reaper maintains.
    ///
    /// This is best-effort: if some connections cannot be established the pool
    /// starts with fewer. Expired connections kept only to satisfy the floor
    /// are replaced by the reaper (best-effort) rather than kept past
    /// `max_lifetime`.
    pub min_connections: u32,
    /// How long [`crate::Pool::get`] waits for a connection before failing.
    ///
    /// Establishing a new connection can additionally take up to
    /// `connect_timeout`.
    pub acquire_timeout: Duration,
    /// Close a connection after this long without use. Checked at each
    /// [`PoolConfig::reap_interval`].
    pub idle_timeout: Duration,
    /// Close a connection this long after it was created. Checked at each
    /// [`PoolConfig::reap_interval`].
    pub max_lifetime: Duration,
    /// Largest accepted backend message, in bytes, for connections created by
    /// this pool.
    ///
    /// `None` (the default) keeps the URL's `max_message_len`, so passing a
    /// `PoolConfig` for unrelated settings cannot silently raise a limit that
    /// the connection URL deliberately lowered.
    pub max_message_len: Option<usize>,
    /// How often the idle reaper runs. Defaults to 30 seconds.
    pub reap_interval: Duration,
    /// Verify an idle connection with a round trip before handing it out.
    ///
    /// Disabled by default. Enable it when connections may be killed by a
    /// proxy or a network partition that the idle actor cannot observe.
    pub test_on_checkout: bool,
    /// Only verify connections that have been idle for at least this long when
    /// [`PoolConfig::test_on_checkout`] is enabled. Defaults to 30 seconds;
    /// [`Duration::ZERO`] verifies every checkout.
    pub test_on_checkout_after: Duration,
}

impl Default for PoolConfig {
    fn default() -> Self {
        PoolConfig {
            pool_size: 80,
            min_connections: 0,
            acquire_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(600),
            max_lifetime: Duration::from_secs(1800),
            max_message_len: None,
            reap_interval: Duration::from_secs(30),
            test_on_checkout: false,
            test_on_checkout_after: Duration::from_secs(30),
        }
    }
}

impl PoolConfig {
    /// Rejects settings that cannot work (or that would hang a checkout).
    ///
    /// # Errors
    ///
    /// Returns [`Error::Config`] for a zero `pool_size`, a `min_connections`
    /// larger than `pool_size`, a zero `reap_interval`, or a
    /// `max_message_len` outside the range accepted by the URL parameter.
    pub fn validate(&self) -> Result<(), Error> {
        if self.pool_size == 0 {
            return Err(Error::Config("`pool_size` must be greater than zero".into()));
        }
        if self.min_connections > self.pool_size {
            return Err(Error::Config(format!(
                "`min_connections` ({}) must not exceed `pool_size` ({})",
                self.min_connections, self.pool_size
            )));
        }
        if self.reap_interval.is_zero() {
            return Err(Error::Config("`reap_interval` must be greater than zero".into()));
        }
        if let Some(max_message_len) = self.max_message_len {
            check_len("max_message_len", max_message_len)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_full_url() {
        let config = Config::parse(
            "postgres://alice:s3cret@db.example.com:6543/app?sslmode=verify-full&application_name=web&connect_timeout=5",
        )
        .unwrap();
        assert_eq!(config.host, "db.example.com");
        assert_eq!(config.port, 6543);
        assert_eq!(config.user, "alice");
        assert_eq!(config.password.as_deref(), Some("s3cret"));
        assert_eq!(config.database.as_deref(), Some("app"));
        assert_eq!(config.application_name.as_deref(), Some("web"));
        assert_eq!(config.sslmode, SslMode::VerifyFull);
        assert_eq!(config.connect_timeout, Duration::from_secs(5));
    }

    #[test]
    fn defaults() {
        let config = Config::parse("postgres://bob@localhost/db").unwrap();
        assert_eq!(config.port, 5432);
        assert_eq!(config.sslmode, SslMode::Require);
        assert_eq!(config.password, None);
    }

    #[test]
    fn rejects_plaintext_sslmode() {
        assert!(Config::parse("postgres://bob@localhost/db?sslmode=disable").is_err());
        assert!(Config::parse("postgres://bob@localhost/db?sslmode=prefer").is_err());
    }

    #[test]
    fn parses_pool_parameters() {
        let config = Config::parse(
            "postgres://bob@localhost/db?pool_size=5&min_connections=2&acquire_timeout=3&idle_timeout=60&max_lifetime=120",
        )
        .unwrap();
        assert_eq!(config.pool.pool_size, 5);
        assert_eq!(config.pool.min_connections, 2);
        assert_eq!(config.pool.acquire_timeout, Duration::from_secs(3));
        assert_eq!(config.pool.idle_timeout, Duration::from_secs(60));
        assert_eq!(config.pool.max_lifetime, Duration::from_secs(120));
    }

    #[test]
    fn rejects_unknown_parameter() {
        assert!(Config::parse("postgres://bob@localhost/db?nope=1").is_err());
    }

    #[test]
    fn parses_limits() {
        let config =
            Config::parse("postgres://bob@localhost/db?max_message_len=1048576&max_scram_iterations=5000").unwrap();
        assert_eq!(config.max_message_len, 1_048_576);
        assert_eq!(config.max_scram_iterations, 5000);
        // A `PoolConfig` does not set `max_message_len` by default, so the URL
        // value is kept when one is supplied.
        assert_eq!(config.pool.max_message_len, None);
    }

    #[test]
    fn rejects_bad_limits() {
        assert!(Config::parse("postgres://bob@localhost/db?max_message_len=100").is_err());
        assert!(Config::parse("postgres://bob@localhost/db?max_message_len=99999999999").is_err());
        assert!(Config::parse("postgres://bob@localhost/db?max_scram_iterations=0").is_err());
    }

    #[test]
    fn rejects_zero_connect_timeout() {
        // A zero timeout would make every connection attempt fail instantly.
        let err = Config::parse("postgres://bob@localhost/db?connect_timeout=0").unwrap_err();
        assert!(err.to_string().contains("greater than zero"), "{err}");
    }

    #[test]
    fn rejects_impossible_pool_configs() {
        let err = Config::parse("postgres://bob@localhost/db?pool_size=0").unwrap_err();
        assert!(err.to_string().contains("pool_size"), "{err}");
        let err = Config::parse("postgres://bob@localhost/db?pool_size=1&min_connections=2").unwrap_err();
        assert!(err.to_string().contains("min_connections"), "{err}");

        let mut config = PoolConfig::default();
        config.min_connections = config.pool_size + 1;
        assert!(config.validate().is_err());
        config.min_connections = 0;
        config.pool_size = 0;
        assert!(config.validate().is_err());
        config.pool_size = 1;
        config.reap_interval = Duration::ZERO;
        assert!(config.validate().is_err());
        config.reap_interval = Duration::from_secs(1);
        config.max_message_len = Some(1024);
        assert!(config.validate().is_err());
        assert!(PoolConfig::default().validate().is_ok());
    }

    #[test]
    fn rejects_unknown_scheme() {
        assert!(Config::parse("mysql://bob@localhost/db").is_err());
    }

    #[test]
    fn debug_redacts_the_password() {
        let config = Config::parse("postgres://alice:s3cret@localhost/db").unwrap();
        let rendered = format!("{config:?}");
        assert!(!rendered.contains("s3cret"), "the password leaked in Debug: {rendered}");
        assert!(rendered.contains("alice"));
        assert!(rendered.contains("<redacted>"));
    }

    #[test]
    fn rejects_nul_in_connection_parameters() {
        // Startup parameters are NUL-delimited; a NUL must never reach them.
        assert!(Config::parse("postgres://us%00er@localhost/db").is_err());
        assert!(Config::parse("postgres://bob:pa%00ss@localhost/db").is_err());
        assert!(Config::parse("postgres://bob@localhost/db%00name").is_err());
        assert!(Config::parse("postgres://bob@localhost/db?application_name=a%00user%00admin").is_err());
        assert!(Config::parse("postgres://bob@localhost/db?options=-c%00x=1").is_err());
        assert!(Config::parse("postgres://bob@localhost/db?timezone=UTC%00").is_err());
    }

    #[test]
    fn normalizes_ipv6_hosts() {
        // `host_str()` keeps the brackets, which are not valid for
        // `TcpStream::connect` or for a TLS server name.
        let config = Config::parse("postgres://u@[::1]:5432/db").unwrap();
        assert_eq!(config.host, "::1");
        assert_eq!(config.port, 5432);
        let config = Config::parse("postgres://u@[2001:db8::1]/db").unwrap();
        assert_eq!(config.host, "2001:db8::1");
        assert_eq!(config.port, 5432);
    }

    #[test]
    fn decodes_percent_encoding() {
        let config = Config::parse("postgres://a%40b:p%3Ass@localhost/db%20name").unwrap();
        assert_eq!(config.user, "a@b");
        assert_eq!(config.password.as_deref(), Some("p:ss"));
        assert_eq!(config.database.as_deref(), Some("db name"));
    }
}
