use std::sync::Arc;

use rustls::{
    server::AllowAnyAuthenticatedClient, Certificate, ClientConfig, ClientConnection, Connection,
    Error as TlsError, PrivateKey, RootCertStore, ServerConfig, ServerConnection,
};

pub(crate) struct TlsFactory {
    ca_store: RootCertStore,
    certificate: Vec<Certificate>,
    private_key: PrivateKey,
}

impl TlsFactory {
    pub fn new(ca_cert: Vec<u8>, cert: Vec<u8>, key: Vec<u8>) -> Result<Self, TlsError> {
        let mut ca_store = RootCertStore::empty();
        ca_store.add(&Certificate(ca_cert))?;
        let certificate = vec![Certificate(cert)];
        let private_key = PrivateKey(key);
        Ok(TlsFactory {
            ca_store,
            certificate,
            private_key,
        })
    }
    pub fn create_client(&self, server_name: &str) -> Result<Connection, TlsError> {
        let config = ClientConfig::builder()
            .with_safe_defaults()
            .with_root_certificates(self.ca_store.clone())
            .with_client_auth_cert(self.certificate.clone(), self.private_key.clone())?;
        let client = ClientConnection::new(
            Arc::new(config),
            server_name
                .try_into()
                .or(Err(TlsError::General(String::from("Bad server_name"))))?,
        )?;
        Ok(Connection::Client(client))
    }
    pub fn create_server(&self) -> Result<Connection, TlsError> {
        let config = ServerConfig::builder()
            .with_safe_defaults()
            .with_client_cert_verifier(Arc::new(AllowAnyAuthenticatedClient::new(
                self.ca_store.clone(),
            )))
            .with_single_cert(self.certificate.clone(), self.private_key.clone())?;
        let server = ServerConnection::new(Arc::new(config))?;
        Ok(Connection::Server(server))
    }
}
