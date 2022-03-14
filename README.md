# smolhttp

This project is a fork of the original [`minihttp`](https://crates.io/crates/minihttp) that tries to improve the code and add more features, dont pushing aside the main purpose of the project thats is to be `simple and lightweight`.

## Copyright
All the credits go to the original author `p00s`

## Usage

### Send a GET request

```rust
// Using the shortcut function
let content = smolhttp::get("https://www.rust-lang.org").unwrap().text();
println!("{content}");

// Using the Client
let content = smolhttp::Client::new("https://www.rust-lang.org").unwrap().get().send().unwrap().text();
println!("{content}");
```

### Send a POST request

```rust
// Using the shortcut funtion
let content = smolhttp::post("https://www.rust-lang.org").unwrap().text();
println!("{content}");

// Using the Client
let content = smolhttp::Client::new("https://www.rust-lang.org")
  .unwrap()
  .post()
  .send()
  .unwrap()
  .text();
println!("{content}");
```

### Custom headers
```rust
let content = smolhttp::Client::new("https://www.rust-lang.org")
  .unwrap()
  .post()
  .headers(vec![("User-Agent".to_owned(), "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".to_owned())])
  .send()
  .unwrap()
  .text();
println!("{content}");
```

### Support proxy
```rust
let content = smolhttp::Client::new("http://www.google.com")
  .unwrap()
  .proxy("http://127.0.0.1:1080")
  .unwrap()
  .get()
  .send()
  .unwrap()
  .text();
println!("{content}");
```

### Adding it to your project
```toml
[dependencies]
smolhttp = "1.0"
```
