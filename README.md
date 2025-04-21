# 💻 Kevin RS Landing Page

[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/wiseaidev)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Made With Rust](https://img.shields.io/badge/Made%20with-Rust-1f425f.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)

## ⚙️ Building and Running

1. Fork/Clone the GitHub repository.

   ```bash
   git clone https://github.com/kevin-rs/blog
   ```

1. Navigate to the application directory.

   ```bash
   cd blog
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

## 🚀 Deploying to CloudFlare

1. Install [Wrangler CLI](https://developers.cloudflare.com/workers/wrangler/get-started/):

   To get started with Cloudflare Workers, you'll need to install the Wrangler CLI, which is a powerful tool for managing your deployments. Open up your command-line interface and enter the following command to globally install Wrangler:

   ```sh
   npm i -g wrangler
   ```

1. Login to Cloudflare Account from the CLI:

   To seamlessly interact with your Cloudflare account, you'll need to log in using Wrangler. Run the following command in your terminal:

   ```sh
   wrangler login
   ```

1. Run Your Build Command:

   Before deployment, you need to build the project. Execute the following command to share the app with the world:

   ```sh
   make build
   ```

1. Create a New Deployment:

   Execute the following command to deploy the app with Wrangler Pages:

   ```sh
   wrangler pages deploy target/dx/kevin/release/web/public
   ```

## 🤝 Contributing

We welcome contributions to enhance the kevin-rs landing page repository! Thank you for helping make this project better!

## 📜 License

This project and the accompanying materials are made available under the terms and conditions of the [`MIT License`](LICENSE).
