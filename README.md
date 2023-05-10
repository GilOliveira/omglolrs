# omglol crate for Rust
An asynchronous Rust wrapper for the omg.lol API by [Gil](https://gil.lol).

LICENSE: MPL 2.0 (see [`LICENSE`](/LICENSE))

> **Warning**: **This is a work in progress**
>
> Unstable and bug-prone. Endpoints may change. Upstream endpoints under development.

## Contribute

[Project homepage](https://sr.ht/~gpo/omglolrs/)

Repos: [sourcehut (canonical)](https://git.sr.ht/~gpo/omglolrs) |
       [GitLab](https://gitlab.com/GilOliveira/omglolrs) |
       [GitHub](https://github.com/GilOliveira/omglolrs) |
       [Codeberg](https://codeberg.org/Gil/omglolrs)

Caught a bug? [Mail in a ticket](mailto:~gpo/omglolrs@todo.sr.ht) after checking the
[bug tracker](https://todo.sr.ht/~gpo/omglolrs).

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
