# omglol crate for Rust
A asynchronous Rust wrapper for the omg.lol API by [Gil](https://gil.lol).

> **Warning**: **This is a work in progress**
>
> Unstable and bug-prone. Endpoints may change. Upstream endpoints under development.

Repos: [sourcehut (canonical)](https://git.sr.ht/~gpo/omglolrs) | [GitLab](https://gitlab.com/GilOliveira/omg.lol.rs) | [GitHub](https://github.com/GilOliveira/omglolrs)

LICENSE: MPL 2.0 (see [`LICENSE`](LICENSE))

## Examples

### Get service status

```rust
use omglol::client::OmglolClient;

fn main() {
  let response = OmglolClient::new(None())
                  .get_service_status()
                  .await;
  println!("{:#?}", response);
}
```

### Fetch a webpage
```rust
use omglol::client::OmglolClient;

fn main() {
  let response = OmglolClient::new(Some("YOUR_API_KEY_HERE"))
                  .get_web_page("your-address")
                  .await();
  println!("{:#?}", response);
}
```
