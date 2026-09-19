# cwd-scoper

![Stars](https://img.shields.io/github/stars/FishgameStudio/cwd-scoper-rs)
![Issues](https://img.shields.io/github/issues/FishgameStudio/cwd-scoper-rs)
![PRs](https://img.shields.io/github/issues-pr/FishgameStudio/cwd-scoper-rs)
[![Rust](https://img.shields.io/badge/rust-1.70+-orange?logo=rust)](rust-lang.org)
[![License](https://img.shields.io/github/license/FishgameStudio/cwd-scoper-rs)](LICENSE)
[![Crates.Io](https://img.shields.io/crates/v/cwd-scoper)](https://crates.io/crates/cwd-scoper)

**cwd-scoper** is a **lightweight & easy-to-use** proc macro library to scope your runtime directory to avoid issues about relative directories.

## 🚀 Getting Started

### 📦 Installation
```sh
# Download from crates.io
cargo add cwd-scoper
# Or install from forked repository
cargo add --path .
```

### 💡 Usage

You can import this crate and use it in your daily development. Here's a simple example to use it.

```rust
use cwd_scoper::cwd_src;
use std::path::PathBuf;
use std::env::current_dir;
#[cwd_src]
fn main() {
    let src_file = PathBuf::from(file!());
    let src_dir = src_file.parent().unwrap();
    let curr_dir = current_dir().unwrap();
    assert_eq!(src_dir, curr_dir);
}
```

## 🤝 Contributing

Contributions are **greatly appreciated**! 
If you have a suggestion that would make this better, please fork the repo and create a pull request. 

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### 🌟 Top contributors:

<a href="https://github.com/FishgameStudio/cwd-scoper-rs/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=FishgameStudio/cwd-scoper-rs" alt="contrib.rocks image" />
</a>

## 📃 License

[MIT](https://mit-license.org) Licensed. See [LICENSE](LICENSE) for more information.

## 📬 Contact

Nicola Grey - [popxh@outlook.com](mailto:popxh@outlook.com)
Project Link: [https://github.com/FishgameStudio/cwd-scoper](https://github.com/FishgameStudio/cwd-scoper)
