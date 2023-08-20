# omglol crate for Rust
An asynchronous Rust wrapper for the omg.lol API by [Gil](https://gil.lol).

LICENSE: MPL 2.0 (see [`LICENSE`](/LICENSE))

> **Warning**: **Heads up! This is a public alpha for adventurous folks!**
>
> Keep in mind:
> * All basic features should be implemented and functional
> * Not all features have been throughly tested
> * Endpoints are bound to change
> * Bugs and unresolved issues are bound to come up
> * This is available as-is and with no support or warranty
> * Upstream endpoints are bound to change and will break this crate
> * This crate is just provided for fun 🌈

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
  let response = OmglolClient::new()
                  .get_service_status()
                  .await;
  println!("{:#?}", response);
}
```

### Fetch a webpage
```rust
use omglol::client::OmglolClient;

fn main() {
  let response = OmglolClient::new()
                  .auth("YOUR_API_KEY_HERE")
                  .get_web_page("your-address")
                  .await();
  println!("{:#?}", response);
}
```
