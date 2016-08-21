## Overview

This crate provides the `insecure_https_connector()` function which can be used to generate a `hyper::net::HttpsConnector` with the underlying OpenSSL context configured to:
*   disable SSL certificate verification
*   allow `SSLv2` and `SSLv3`

Example:

```rust
extern crate hyper_insecure_https_connector;
extern crate hyper;

use hyper_insecure_https_connector::insecure_https_connector;
use std::io::Read;

fn main() {
    
    let client = hyper::Client::with_connector(insecure_https_connector());

    let mut res = client.get("http://httpbin.org/get").send().unwrap();

    let mut content = String::new();
    res.read_to_string(&mut content).unwrap();

    println!("Answer:\n{}", content);
}
```

_Note_: it only works with `hyper 0.9.*`

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.