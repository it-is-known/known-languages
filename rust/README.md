# Known Languages

[![License](https://img.shields.io/badge/license-Public%20Domain-blue.svg)](https://unlicense.org)
[![Compatibility](https://img.shields.io/badge/rust-1.85%2B-blue)](https://blog.rust-lang.org/2025/02/20/Rust-1.85.0/)
[![Package](https://img.shields.io/crates/v/known-languages)](https://crates.io/crates/known-languages)
[![Documentation](https://docs.rs/known-languages/badge.svg)](https://docs.rs/known-languages)

**Well-known languages for Rust.**

<sub>

[[Features](#-features)] |
[[Prerequisites](#%EF%B8%8F-prerequisites)] |
[[Installation](#%EF%B8%8F-installation)] |
[[Examples](#-examples)] |
[[Reference](#-reference)] |
[[Development](#%E2%80%8D-development)]

</sub>

## ✨ Features

- Exports an enum for all well-known languages.
- Plays nice with others: interoperates with [Serde] and [Clap].
- 100% pure and safe Rust with minimal dependencies and no bloat.
- Supports `no_std` environments from the get-go.
- Supports opting out of any feature using comprehensive [feature flags].
- Adheres to the Rust API Guidelines in its [naming conventions].
- Cuts red tape: 100% free and unencumbered public domain software.

## 🛠️ Prerequisites

- [Rust] 1.85+ (2024 edition)

## ⬇️ Installation

### Installation via Cargo

```bash
cargo add known-languages
```

### Installation in `Cargo.toml`

Enable all default features:

```toml
[dependencies]
known-languages = "0.0"
```

Enable only specific features:

```toml
[dependencies]
known-languages = { version = "0.0", default-features = false, features = ["serde"] }
```

## 👉 Examples

### Importing the Library

```rust
use known_languages;
```

## 📚 Reference

[docs.rs/known-languages](https://docs.rs/known-languages)

## 👨‍💻 Development

```bash
git clone https://github.com/known-facts/known-languages.git
```

---

[![Share on X](https://img.shields.io/badge/share%20on-x-03A9F4?logo=x)](https://x.com/intent/post?url=https://github.com/known-facts/known-languages&text=Known%20Languages)
[![Share on Reddit](https://img.shields.io/badge/share%20on-reddit-red?logo=reddit)](https://reddit.com/submit?url=https://github.com/known-facts/known-languages&title=Known%20Languages)
[![Share on Hacker News](https://img.shields.io/badge/share%20on-hn-orange?logo=ycombinator)](https://news.ycombinator.com/submitlink?u=https://github.com/known-facts/known-languages&t=Known%20Languages)
[![Share on Facebook](https://img.shields.io/badge/share%20on-fb-1976D2?logo=facebook)](https://www.facebook.com/sharer/sharer.php?u=https://github.com/known-facts/known-languages)
[![Share on LinkedIn](https://img.shields.io/badge/share%20on-linkedin-3949AB?logo=linkedin)](https://www.linkedin.com/sharing/share-offsite/?url=https://github.com/known-facts/known-languages)

[Clap]: https://crates.io/crates/clap
[Rust]: https://rust-lang.org
[Serde]: https://crates.io/crates/serde
[feature flags]: https://github.com/known-facts/known-languages/blob/master/lib/known-languages/Cargo.toml
[naming conventions]: https://rust-lang.github.io/api-guidelines/naming.html
