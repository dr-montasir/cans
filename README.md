<div align="center">
  <br>
  <a href="https://github.com/dr-montasir/cans">
      <img src="cans-logo.svg" width="100">
  </a>
  <br>

[<img alt="github" src="https://img.shields.io/badge/github-dr%20montasir%20/%20cans-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="22">](https://github.com/dr-montasir/cans)[<img alt="crates.io" src="https://img.shields.io/crates/v/cans.svg?style=for-the-badge&color=fc8d62&logo=rust" height="22">](https://crates.io/crates/cans)[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-cans-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="22">](https://docs.rs/cans)[<img alt="license" src="https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="22">](https://choosealicense.com/licenses/apache-2.0)

  <h1>CANS</h1>

  <p>An elegant and lightweight Rust-based literal template engine</p>
</div>

## Table of Contents

- [Overview](#overview)
- [Changelog](#changelog)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Documentation](#documentation)
- [License](#license)
- [Contributing](#contributing)
- [Donations](#donations)
- [Author](#author)

## Overview

The **CANS** templating engine is an elegant and efficient solution developed in Rust, designed for developers who prioritize simplicity and type safety. This crate allows you to create dynamic web pages and applications with minimal boilerplate code, ensuring that your templates are easy to read and maintain.

While **CANS** is optimized for seamless integration with the [GetPost](https://crates.io/crates/getpost) framework, it also provides the flexibility to work with any framework or content type, making it a versatile choice for a wide range of projects.

## Changelog

[![github](https://img.shields.io/badge/github-%20changelog-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dr-montasir/cans/blob/main/CHANGELOG.md)

## Features

- **Simplicity**: Easy-to-read syntax that minimizes boilerplate.
- **Type Safety**: Leverages Rust's type system for safe template rendering.
- **Dynamic Content**: Supports dynamic insertion of values, loops, and conditionals.
- **Integration**: Works seamlessly with various web frameworks.

## Installation

Run the following Cargo command in your project directory:

```shell
cargo add cans
```

or add `cans` to your `Cargo.toml` file:

```toml
[dependencies]
cans = "MAJOR.MINOR.PATCH" # Replace with the latest version
```

## Usage

### Template

CANS provides robust support for templating, including support for handling HTML, looping through collections, and rendering text. Below are some examples demonstrating how to use the `do_html` macro, the `do_forloop` macro, and the `do_text` function.

### Example: Using the do_html Macro

```rust
use cans::html::{do_html, do_text};

pub const HEAD: &str = r#"<head>
<meta charset="UTF-8">
    <title>{{page_title}} Page</title>
</head>"#;

pub const HOME_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
  {{HEAD}}
  <body>
     Home Page
  </body>
</html>"#;

pub fn do_home_page() -> String {
    do_html!(HOME_TEMPLATE, HEAD = HEAD, page_title = do_text("Home"))
}

pub const ABOUT_TEMPLATE: &str = r#"<!DOCTYPE html>
<html>
  {{HEAD}}
  <body>
     <p>About Page</p>
     {{component_if}}
  </body>
</html>"#;

pub fn do_about_page() -> String {
    let component_if: &str;
    let x = 3;

    if x == 1 {
        component_if = "<a href='#'><i>x = 1</i></a>";
    } else if x == 2 {
        component_if = r#"<a href='#'><i>x = {{x}}</i></a>"#;
    } else {
        component_if = "<a href=\"#\"><i>x ≠ 1 & x  ≠ 2. The 'x' value is ( {{x}} )</i></a>";
    };

    do_html!(
        ABOUT_TEMPLATE,
        HEAD = HEAD,
        page_title = do_text("About"),
        component_if = component_if,
        x = x // x must be defined after component_if.
    )
}

fn main() {
   let home_page = do_home_page();
   println!("{}", home_page);

   let about_page = do_about_page();
   println!("{}", about_page);
}
```

```html
<!-- home page output -->
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>Home Page</title>
  </head>
  <body>
    Home Page
  </body>
</html>

<!-- about page output -->
<!DOCTYPE html>
<html>
  <head>
    <meta charset="UTF-8" />
    <title>About Page</title>
  </head>
  <body>
    <p>About Page</p>
    <a href="#"><i>x ≠ 1 & x ≠ 2. The 'x' value is ( 3 )</i></a>
  </body>
</html>
```

### Example: Using the do_forloop Function

```rust
use cans::html::do_forloop;

fn main() {
    let items = vec!["Apple", "Banana", "Cherry"];
    let list_html = do_forloop(&items, "<ul>", "<li>", "</li>", "</ul>");
    println!("{}", list_html);
    // Output: <ul><li>Apple</li><li>Banana</li><li>Cherry</li></ul>

    let float_vector = vec![1.0, 2.0, 3.0];
    let forloop_float = do_forloop(&float_vector, "", "", "", "");
    println!("{}", forloop_float);
    // Output: 123
}
```

## Documentation

For a detailed API reference, visit the [CANS Documentation](https://docs.rs/cans/latest/cans).

## Contributing

Contributions are welcome! If you have suggestions or improvements, feel free to submit an issue or a pull request.

## License

This project is licensed under the MIT or Apache 2.0 License - see the LICENSE file for details.

## Conclusion

Whether you're building a small project or a large application, CANS provides the tools you need to create dynamic and flexible templates with ease. Unlock the potential of CANS to elevate your Rust web development experience!

## Donations

If you appreciate the work on CANS and would like to support its development, you can make a donation using USDT (TRC-20). Your contributions will help us continue to improve the project and maintain its features.

**Donate with USDT (TRC-20)**

- Wallet Address: [`TFtKw3aExk5fXvdmTobiVDBbkbJB66wc6D`](https://tronscan.org/#/address/TFtKw3aExk5fXvdmTobiVDBbkbJB66wc6D)

**Donate with USDC (TRC-20)**

- Wallet Address: [`TFtKw3aExk5fXvdmTobiVDBbkbJB66wc6D`](https://tronscan.org/#/address/TFtKw3aExk5fXvdmTobiVDBbkbJB66wc6D)

**Donate with TRON (TRX)**

- Wallet Address: [`TFtKw3aExk5fXvdmTobiVDBbkbJB66wc6D`](https://tronscan.org/#/address/TFtKw3aExk5fXvdmTobiVDBbkbJB66wc6D)

Thank you for your support!

---

## Author

[Dr. Montasir Mirghani](https://github.com/dr-montasir)
