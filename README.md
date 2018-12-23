# `lifxi`

[![Build Status](https://travis-ci.com/Aehmlo/lifxi.svg?branch=master)](https://travis-ci.com/Aehmlo/lifxi) [![Crates.io](https://img.shields.io/crates/v/lifxi.svg)](https://crates.io/crates/lifxi) [![Documentation](https://docs.rs/lifxi/badge.svg)](https://docs.rs/lifxi)

Control [LIFX](https://lifx.com) devices over (eventually LAN and) the internet.

## Getting Started

This crate currently only supports control via the web API. To get started, go to [the LIFX account settings page](https://cloud.lifx.com/settings) and create an access token.

[The `Client` struct](https://docs.rs/lifxi/*/lifxi/http/struct.Client.html) is the entry point for all functionality in this crate. It's advised to have a single instance of this client, as it holds a connection pool. Depending on your architecture, [the `lazy-static` crate](https://docs.rs/lazy_static) may be a good choice:

```rust
lazy_static! {
    static ref CLIENT: Client = Client::new("secret");
}
```

Here's a simple demo to ensure everything's working:

```rust
use lifxi::http::*;
fn main() {
    let client = Client::new("your secret here");
    let _result = client
        .select(Selector::All)
        .set_state()
        .power(true)
        .color(Color::Red)
        .brightness(0.4)
        .send();
}
```

If running that example results in all of your LIFX bulbs turning on and changing to red, you're in business! Head over to [the docs](https://docs.rs/lifxi) to see more.

## Contributing

Contributions are welcome! Submit a pull request, file an issue, or feel free to just discuss in the comments. The LIFX [HTTP API documentation](https://api.developer.lifx.com) and [LAN protocol documentation](https://lan.developer.lifx.com/) will likely be helpful in any development efforts.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as below, without any additional terms or conditions.

## License

Licensed under [the Apache License, Version 2.0](https://opensource.org/licenses/Apache-2.0) or [the MIT License](https://opensource.org/licenses/MIT), at your
option.