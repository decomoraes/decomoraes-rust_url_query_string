# url_query_string

`url_query_string` is a Rust procedural macro crate that simplifies converting structs into URL query strings. By deriving `ToQueryString`, two methods are automatically generated for your structs:

- `to_query_string`: Converts the struct into a query string, returning a `String`. Ignores errors by returning an empty string if serialization fails.
- `try_to_query_string`: Converts the struct into a query string, returning a `Result<String, serde_qs::Error>`.

## Features

- **Easy Query String Generation**: No manual string concatenation; works out of the box with your structs.
- **Serde-Compatible**: Fully integrates with `serde`, allowing customization of query string formats.
- **Error Handling**: Choose between ignoring errors or handling them explicitly.

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
url_query_string = "0.1"
serde = { version = "1.0", features = ["derive"] }
serde_qs = "0.7"
```

## Usage

### Example

```rust
use serde::Serialize;
use url_query_string::ToQueryString;

#[derive(Serialize, ToQueryString)]
#[serde(rename_all = "camelCase")]
struct TestStruct {
    pub page: Option<u32>,
    pub page_size: Option<u32>,
    pub id: Option<String>,
    pub user_id: Option<String>,
}

fn main() {
    let instance = TestStruct {
        page: Some(1),
        page_size: Some(20),
        id: Some("test_id".to_string()),
        user_id: Some("user_123".to_string()),
    };

    // Generate query string (ignores errors).
    let query_string = instance.to_query_string();
    println!("Query String: {}", query_string);

    // Generate query string with error handling.
    match instance.try_to_query_string() {
        Ok(qs) => println!("Query String (with Result): {}", qs),
        Err(e) => eprintln!("Error: {}", e),
    }
}
```

### Output

Running the example above will produce:

```
Query String: page=1&pageSize=20&id=test_id&userId=user_123
Query String (with Result): page=1&pageSize=20&id=test_id&userId=user_123
```

## Customization with `serde`

You can use `serde` attributes like `#[serde(rename_all = "snake_case")]` to control the format of your query strings:

```rust
#[derive(Serialize, ToQueryString)]
#[serde(rename_all = "snake_case")]
struct AnotherStruct {
    pub user_name: Option<String>,
    pub access_token: Option<String>,
}
```

This will generate query strings like:

```
user_name=test_user&access_token=abcd1234
```

## Methods Generated

When deriving `ToQueryString`, the following methods are added to your struct:

- `to_query_string`: Returns a `String` with the query string. Errors are ignored.
- `try_to_query_string`: Returns a `Result<String, serde_qs::Error>` for explicit error handling.

## Contribution

Contributions are welcome! If you encounter any bugs or have feature requests, please open an issue or submit a pull request on [GitHub](https://github.com/your-repo/url_query_string).

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
