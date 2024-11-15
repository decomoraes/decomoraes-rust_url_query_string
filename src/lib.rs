//! # `url_query_string` Procedural Macro Crate
//!
//! The `url_query_string` crate provides a procedural macro `#[derive(ToQueryString)]`
//! for automatically generating methods to serialize Rust structs into URL query strings.
//!
//! This crate leverages `serde_qs` for serialization, ensuring compatibility with
//! `serde` and its powerful features. By deriving `ToQueryString`, two methods are
//! added to your struct:
//!
//! - `to_query_string`: Converts the struct to a query string, ignoring errors and returning
//!   an empty string in case of failure.
//! - `try_to_query_string`: Returns a `Result` containing either the query string or
//!   a serialization error from `serde_qs`.
//!
//! ## Features
//!
//! - **Automatic Query String Generation**: Simplifies converting structs into URL-friendly query strings.
//! - **Compatible with `serde`**: Supports serialization conventions like `camelCase` or `snake_case`
//!   through `serde` attributes.
//! - **Error Handling**: Offers `try_to_query_string` for cases where error handling is necessary.
//!
//! ## Usage
//!
//! ### Add the Crate
//!
//! Add the crate to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! url_query_string = "0.1"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_qs = "0.7"
//! ```
//!
//! ### Example
//!
//! ```rust
//! use serde::Serialize;
//! use url_query_string::ToQueryString;
//!
//! #[derive(Serialize, ToQueryString)]
//! #[serde(rename_all = "camelCase")]
//! struct TestStruct {
//!     pub page: Option<u32>,
//!     pub page_size: Option<u32>,
//!     pub id: Option<String>,
//!     pub user_id: Option<String>,
//! }
//!
//! fn main() {
//!     let instance = TestStruct {
//!         page: Some(1),
//!         page_size: Some(20),
//!         id: Some("test_id".to_string()),
//!         user_id: Some("user_123".to_string()),
//!     };
//!
//!     // Generate query string (ignores errors).
//!     let query_string = instance.to_query_string();
//!     println!("Query String: {}", query_string);
//!
//!     // Generate query string with error handling.
//!     match instance.try_to_query_string() {
//!         Ok(qs) => println!("Query String (with Result): {}", qs),
//!         Err(e) => eprintln!("Error: {}", e),
//!     }
//! }
//! ```
//!
//! ### Output
//!
//! Running the example above will produce:
//!
//! ```text
//! Query String: page=1&pageSize=20&id=test_id&userId=user_123
//! Query String (with Result): page=1&pageSize=20&id=test_id&userId=user_123
//! ```
//!
//! ## Integration with `serde`
//!
//! The crate uses `serde` attributes to control the serialization behavior. For example, you can
//! use `#[serde(rename_all = "snake_case")]` to adjust the case of field names in the query string.
//!
//! ### Example with `snake_case`
//!
//! ```rust
//! use serde::Serialize;
//! use url_query_string::ToQueryString;
//!
//! #[derive(Serialize, ToQueryString)]
//! #[serde(rename_all = "snake_case")]
//! struct AnotherStruct {
//!     pub user_name: Option<String>,
//!     pub access_token: Option<String>,
//! }
//! ```
//!
//! This will generate query strings like:
//!
//! ```text
//! user_name=test_user&access_token=abcd1234
//! ```
//!
//! ## Generated Methods
//!
//! When deriving `ToQueryString`, the following methods are added to your struct:
//!
//! - `pub fn to_query_string(&self) -> String`: Converts the struct into a query string. Returns
//!   an empty string if serialization fails.
//!
//! - `pub fn try_to_query_string(&self) -> Result<String, serde_qs::Error>`: Converts the struct into
//!   a query string. Returns a `Result` with either the query string or an error.
//!
//! ## Contribution
//!
//! Contributions are welcome! If you encounter any bugs or have feature requests, please
//! open an issue or submit a pull request on [GitHub](https://github.com/your-repo/url_query_string).

extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

/// Procedural macro to derive query string serialization methods for structs.
///
/// This macro generates two methods for the struct:
///
/// - `to_query_string`: Converts the struct to a query string as a `String`.
///   It ignores errors, returning an empty string (`""`) if serialization fails.
/// - `try_to_query_string`: Converts the struct to a query string as a `Result<String, serde_qs::Error>`.
///   This method allows handling errors during serialization.
///
/// ## Usage
///
/// The struct must implement the `serde::Serialize` trait, as the macro relies on
/// `serde_qs` for query string serialization. You can use `serde` attributes like
/// `#[serde(rename_all = "camelCase")]` to customize the query string format.
///
/// ### Example
///
/// ```rust
/// use serde::Serialize;
/// use url_query_string::ToQueryString;
///
/// #[derive(Serialize, ToQueryString)]
/// #[serde(rename_all = "camelCase")]
/// struct ExampleStruct {
///     pub user_id: Option<String>,
///     pub page: Option<u32>,
/// }
///
/// let instance = ExampleStruct {
///     user_id: Some("user_123".to_string()),
///     page: Some(1),
/// };
///
/// // Generate query string
/// assert_eq!(instance.to_query_string(), "userId=user_123&page=1");
/// ```
#[proc_macro_derive(ToQueryString)]
pub fn to_query_string_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let gen = quote! {
        impl #name {
            /// Converts the struct into a query string, ignoring errors.
            ///
            /// # Returns
            ///
            /// A `String` containing the query string. If serialization fails,
            /// it returns an empty string (`""`).
            pub fn to_query_string(&self) -> String {
                serde_qs::to_string(self).unwrap_or_default()
            }

            /// Converts the struct into a query string, returning a `Result`.
            ///
            /// # Returns
            ///
            /// A `Result` containing either:
            /// - `Ok(String)`: The generated query string.
            /// - `Err(serde_qs::Error)`: An error encountered during serialization.
            pub fn try_to_query_string(&self) -> Result<String, serde_qs::Error> {
                serde_qs::to_string(self)
            }
        }
    };

    gen.into()
}
