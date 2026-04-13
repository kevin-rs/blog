# 💻 wise ai Landing Page

[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Made With Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)

## ⚙️ Building and Running

1. Fork/Clone the GitHub repository.

   ```bash
   git clone https://github.com/wiseaidotdev/blog
   ```

1. Navigate to the application directory.

   ```bash
   cd blog
   ```

1. Install Cargo Binstall:

   ```bash
   cargo install cargo-binstall
   ```

1. Install Dioxus 0.6.3:

   ```bash
   cargo binstall dioxus-cli@0.6.3
   ```

1. Run [the Tailwind CLI (v3)](https://v3.tailwindcss.com/docs/installation):

   ```sh
   npx tailwindcss@v3 -i ./assets/tailwind.css -o ./assets/output.css --watch
   ```

1. SSG the blogs:

   ```sh
   make ssg
   ```

1. Run the client:

   ```sh
   make run
   ```

Navigate to http://localhost:3000 to explore the landing page.

## 🤝 Contributing

We welcome contributions to enhance the wise ai landing page repository! Thank you for helping make this project better!

## 📜 License

This project and the accompanying materials are made available under the terms and conditions of the [`MIT License`](LICENSE).
