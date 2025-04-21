mod blog;
mod components;
mod header;
mod hero;
mod pages;
mod router;
mod theme;

use crate::router::Route;

use dioxus::prelude::*;
use dioxus_logger::tracing::Level;
use std::collections::HashMap;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn static_dir() -> std::path::PathBuf {
    std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join("public")
}

fn main() {
    #[cfg(feature = "web")]
    {
        dioxus_logger::init(Level::INFO).expect("logger failed to init");
        LaunchBuilder::new()
            .with_cfg(server_only! {
                let mut cfg = ServeConfig::builder();

                if !cfg!(debug_assertions) {
                    cfg = cfg.incremental(
                        IncrementalRendererConfig::new()
                            .static_dir(static_dir())
                            .clear_cache(false)
                    );
                }

                cfg.build().expect("Unable to build ServeConfig")
            })
            .launch(App);
    }

    #[cfg(feature = "server")]
    {
        use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
        use axum::http::Method;
        use axum::{Extension, Router};
        use std::sync::Arc;
        use tower_http::cors::{Any, CorsLayer};

        tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(async move {
                let cors = CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
                    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

                let app = Router::new()
                    .layer(cors)
                    .serve_dioxus_application(ServeConfig::new().unwrap(), App);

                let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 3000));
                let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

                axum::serve(listener, app.into_make_service())
                    .await
                    .unwrap();
            });
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }

        document::Meta { name: "viewport", content: "width=device-width, initial-scale=1, maximum-scale=1" },
        document::Meta { name: "description", content: "Build type safe agents blazingly fast with Rust" },
        document::Meta { name: "robots", content: "index, follow" },
        document::Meta { name: "keywords", content: "Kevin, Rust, type safe agents, blazingly fast, web application" },
        document::Meta { name: "author", content: "Mahmoud Harmouch" },
        document::Meta { name: "copyright", content: "© 2025 Kevin RS. All rights reserved." },
        document::Meta { name: "revisit-after", content: "7 days" },
        document::Meta { name: "language", content: "English" },
        document::Meta { name: "rating", content: "General" },
        document::Meta { name: "designer", content: "Mahmoud Harmouch" },
        document::Meta { name: "reply-to", content: "ayo@kevin-rs.dev" },
        document::Meta { name: "target", content: "all" },
        document::Meta { name: "audience", content: "all" },
        document::Meta { name: "mobile-web-app-capable", content: "yes" },

        document::Meta { property: "og:title", content: "Kevin - Build type safe agents blazingly fast with Rust" },
        document::Meta { property: "og:description", content: "Build type safe agents blazingly fast with Rust" },
        document::Meta { property: "og:image", content: "https://kevin-rs.dev/assets/og-image.jpg" },
        document::Meta { property: "og:url", content: "https://kevin-rs.dev/" },
        document::Meta { property: "og:type", content: "website" },
        document::Meta { property: "og:site_name", content: "Kevin | Build type safe agents blazingly fast with Rust" },
        document::Meta { property: "og:locale", content: "en_US" },
        document::Meta { property: "og:image:width", content: "1200" },
        document::Meta { property: "og:image:height", content: "630" },

        document::Meta { name: "twitter:card", content: "summary_large_image" },
        document::Meta { name: "twitter:title", content: "Kevin - Build type safe agents blazingly fast with Rust" },
        document::Meta { name: "twitter:description", content: "Build type safe agents blazingly fast with Rust" },
        document::Meta { name: "twitter:image", content: "https://kevin-rs.dev/assets/og-image.jpg" },
        document::Meta { name: "twitter:site", content: "@wiseaidev" },
        document::Meta { name: "twitter:creator", content: "@wiseaidev" },
        document::Meta { name: "twitter:url", content: "https://kevin-rs.dev/" },

        document::Meta { name: "msapplication-TileColor", content: "#ffffff" },
        document::Meta { name: "msapplication-TileImage", content: "/assets/ms-icon-144x144.png" },
        document::Meta { name: "theme-color", content: "#ffffff" },
        document::Meta { name: "pinterest-rich-pin", content: "true" },
        document::Meta { name: "og:whatsapp", content: "Share on WhatsApp" },

        document::Title { "Kevin | Type Safe AGI" },

        document::Link { rel: "icon", type: "image/x-icon", href: "/assets/favicon.ico" },
        document::Link { rel: "apple-touch-icon", sizes: "57x57", href: "/assets/apple-icon-57x57.png" },
        document::Link { rel: "apple-touch-icon", sizes: "60x60", href: "/assets/apple-icon-60x60.png" },
        document::Link { rel: "apple-touch-icon", sizes: "72x72", href: "/assets/apple-icon-72x72.png" },
        document::Link { rel: "apple-touch-icon", sizes: "76x76", href: "/assets/apple-icon-76x76.png" },
        document::Link { rel: "apple-touch-icon", sizes: "114x114", href: "/assets/apple-icon-114x114.png" },
        document::Link { rel: "apple-touch-icon", sizes: "120x120", href: "/assets/apple-icon-120x120.png" },
        document::Link { rel: "apple-touch-icon", sizes: "144x144", href: "/assets/apple-icon-144x144.png" },
        document::Link { rel: "apple-touch-icon", sizes: "152x152", href: "/assets/apple-icon-152x152.png" },
        document::Link { rel: "apple-touch-icon", sizes: "180x180", href: "/assets/apple-icon-180x180.png" },
        document::Link { rel: "icon", type: "image/png", sizes: "192x192", href: "/assets/android-icon-192x192.png" },
        document::Link { rel: "icon", type: "image/png", sizes: "32x32", href: "/assets/favicon-32x32.png" },
        document::Link { rel: "icon", type: "image/png", sizes: "96x96", href: "/assets/favicon-96x96.png" },
        document::Link { rel: "icon", type: "image/png", sizes: "16x16", href: "/assets/favicon-16x16.png" },
        // document::Link { rel: "manifest", href: "/assets/manifest.json" },

        document::Script { src: "https://kit.fontawesome.com/62e08d355c.js", crossorigin: "anonymous", defer: true },
        document::Link { rel: "canonical", href: "https://kevin-rs.dev/" }
        document::Link { rel: "stylesheet", href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" }
        document::Link { rel: "stylesheet", href: "https://fonts.googleapis.com/css2?family=Lexend:wght@300;400;500;700;900&display=swap" }
        Router::<Route> {}
    }
}
