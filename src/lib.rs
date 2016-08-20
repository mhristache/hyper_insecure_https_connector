extern crate hyper;
extern crate openssl;

use hyper::net::{SslClient, NetworkStream, HttpsConnector};
use openssl::ssl::{Ssl, SslContext, SslStream, SslMethod};


#[derive(Debug, Clone)]
pub struct InsecureOpensslClient(SslContext);

impl Default for InsecureOpensslClient {
    fn default() -> InsecureOpensslClient {
        InsecureOpensslClient(SslContext::new(SslMethod::Sslv23).unwrap())
    }
}

impl<T: NetworkStream + Send + Clone> SslClient<T> for InsecureOpensslClient {
    type Stream = SslStream<T>;

    fn wrap_client(&self, stream: T, host: &str) -> hyper::Result<Self::Stream> {
        let ssl = try!(Ssl::new(&self.0));
        try!(ssl.set_hostname(host));
        SslStream::connect(ssl, stream).map_err(From::from)
    }
}

pub fn insecure_https_connector() -> HttpsConnector<InsecureOpensslClient> {
    hyper::net::HttpsConnector::new(InsecureOpensslClient::default())
}
