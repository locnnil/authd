use authd::nss_client::NssClient;
use std::error::Error;
use tokio::net::UnixStream;
use tonic::transport::{Channel, Endpoint, Uri};
use tower::service_fn;

use crate::debug;

pub mod authd {
    tonic::include_proto!("authd");
}

/// new_client creates a new client connection to the gRPC server or returns an active one.
pub async fn new_client() -> Result<NssClient<Channel>, Box<dyn Error>> {
    debug!("Connecting to authd on {}...", super::socket_path());

    // The URL must have a valid format, even though we don't use it.
    let ch = Endpoint::try_from("https://not-used:404")?
        .connect_with_connector(service_fn(|_: Uri| {
            UnixStream::connect(super::socket_path())
        }))
        .await?;

    Ok(NssClient::new(ch))
}
