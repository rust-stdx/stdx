use crate::{Error, ReceivedCertificate};

#[async_trait::async_trait]
pub trait CertificateVerifier {
    async fn verfiy_certificate(&self, cert: &ReceivedCertificate, server_name: Option<&str>) -> Result<(), Error>;
}
