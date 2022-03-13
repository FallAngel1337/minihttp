# minihttp

`minihttp` is a simple, lightweight, easy to use and less dependent http client lib for rust.
Only provide basic http client feature, more like python request module.

Enjoy yourself...

## Online documention

[Document](https://docs.rs/minihttp)

## Usage

### Send a GET request

```rust
// Using the shortcut function
let content = minihttp::get("https://google.com").unwrap().text();
println!("{content}");

// Using the Client
let content = minihttp::Client::new("https://google.com").unwrap().get().send().unwrap().text();
println!("{content}");
```

### Send a POST request

```rust
// Using the shortcut funtion
let content = minihttp::post("https://google.com").unwrap().text();
println!("{content}");

// Using the Client
let content = minihttp::Client::new("https://google.com")
  .unwrap()
  .post()
  .send()
  .unwrap()
  .text();
println!("{content}");
```

### Custom headers
```rust
let content = minihttp::Client::new("https://google.com")
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
let content = minihttp::Client::new("http://www.google.com")
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
minihttp = { git = "https://github.com/FallAngel1337/minihttp", branch = "main" }
```
