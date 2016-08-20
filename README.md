This crate provides the `insecure_https_connector()` function which can be used to generate a `HttpsConnector` with the underlying Openssl context configured to:
*   disable hostname verification
*   allows `SSLv2` and `SSLv3`

Example:

```rust
extern crate hyper_insecure_https_connector;
extern crate hyper;

use hyper_insecure_https_connector::insecure_https_connector;

fn main() {
    
    let client = hyper::Client::with_connector(insecure_https_connector());

    let mut res = client.get("http://httpbin.org/get").send().unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content).unwrap();

    println!("Answer:\n{}", content);
}
```