#[derive(
    Clone,
    Copy,
    dioxus_router::prelude::Routable,
    PartialEq,
    Eq,
    Hash,
    Debug,
    serde::Serialize,
    serde::Deserialize,
)]
pub enum BookRoute {
    #[route("/announcing-kevin-rs")]
    AnnouncingKevinRs {},
    #[route("/rethinking-arc-agi")]
    RethinkingArcAgi {},
}
impl BookRoute {
    pub fn sections(&self) -> &'static [use_mdbook::mdbook_shared::Section] {
        &self.page().sections
    }
    pub fn page(&self) -> &'static use_mdbook::mdbook_shared::Page<Self> {
        LAZY_BOOK.get_page(self)
    }
    pub fn page_id(&self) -> use_mdbook::mdbook_shared::PageId {
        match self {
            BookRoute::AnnouncingKevinRs {} => use_mdbook::mdbook_shared::PageId(0usize),
            BookRoute::RethinkingArcAgi {} => use_mdbook::mdbook_shared::PageId(1usize),
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::RethinkingArcAgi {}
    }
}
pub static LAZY_BOOK: use_mdbook::Lazy<use_mdbook::mdbook_shared::MdBook<BookRoute>> =
    use_mdbook::Lazy::new(|| {
        let mut page_id_mapping = ::std::collections::HashMap::new();
        let mut pages = Vec::new();
        pages
            .push((
                0usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 1 |---| Announcing Kevin RS 🚀 |---| announcement |---| announcing-kevin-rs |---| Apr 21 2025 |---| Kevin RS is a fully open-source Rust framework for building fast, autonomous AGI agents. Designed for reliability, performance, and general intelligence research, it supports zero-shot learning, multi-agent execution, and future-ready tooling - without relying on fragile stacks. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa"
                            .to_string(),
                        url: BookRoute::AnnouncingKevinRs {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why Kevin RS?".to_string(),
                                id: "why-kevin-rs?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Built in Pure Rust".to_string(),
                                id: "built-in-pure-rust".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The AutoGPT Framework".to_string(),
                                id: "the-autogpt-framework".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Kevin AGI: The Web App (Coming Soon)".to_string(),
                                id: "kevin-agi:-the-web-app-(coming-soon)".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why Rust for AGI?".to_string(),
                                id: "why-rust-for-agi?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Community & Contribution".to_string(),
                                id: "community-&-contribution".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Roadmap: AGI When?".to_string(),
                                id: "roadmap:-agi-when?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "License & Get Started".to_string(),
                                id: "license-&-get-started".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(0usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::AnnouncingKevinRs {},
            ::use_mdbook::mdbook_shared::PageId(0usize),
        );
        pages
            .push((
                1usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 2 |---| Rethinking ARC‑AGI 🧠 |---| analysis |---| rethinking-arc-agi |---| Apr 22 2025 |---| Francois Chollet's ARC‑AGI benchmark aimed to measure fluid intelligence in AI, but early versions were undermined by brute-force pattern-matching. |---| https://github.com/user-attachments/assets/5828a84d-a9c4-472d-a2b9-f097a3d9c840"
                            .to_string(),
                        url: BookRoute::RethinkingArcAgi {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "TLDR;".to_string(),
                                id: "tldr;".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "About the ARC‑AGI Benchmark".to_string(),
                                id: "about-the-arc‑agi-benchmark".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Flaws in Version 1".to_string(),
                                id: "flaws-in-version-1".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Version 2 Improvements".to_string(),
                                id: "version-2-improvements".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Misuse as an AGI Measure and Conceptual Misunderstandings"
                                    .to_string(),
                                id: "misuse-as-an-agi-measure-and-conceptual-misunderstandings"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Future Directions".to_string(),
                                id: "future-directions".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "References".to_string(),
                                id: "references".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(1usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::RethinkingArcAgi {},
            ::use_mdbook::mdbook_shared::PageId(1usize),
        );
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 1 |---| Announcing Kevin RS 🚀 |---| announcement |---| announcing-kevin-rs |---| Apr 21 2025 |---| Kevin RS is a fully open-source Rust framework for building fast, autonomous AGI agents. Designed for reliability, performance, and general intelligence research, it supports zero-shot learning, multi-agent execution, and future-ready tooling - without relying on fragile stacks. |---| https://github.com/user-attachments/assets/1f17ac99-d2c3-42fe-9d93-f84a5f5678fa"
                            .to_string(),
                        location: Some(BookRoute::AnnouncingKevinRs {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 2 |---| Rethinking ARC‑AGI 🧠 |---| analysis |---| rethinking-arc-agi |---| Apr 22 2025 |---| Francois Chollet's ARC‑AGI benchmark aimed to measure fluid intelligence in AI, but early versions were undermined by brute-force pattern-matching. |---| https://github.com/user-attachments/assets/5828a84d-a9c4-472d-a2b9-f097a3d9c840"
                            .to_string(),
                        location: Some(BookRoute::RethinkingArcAgi {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32]),
                        ),
                        nested_items: vec![],
                    }),
                ],
                suffix_chapters: vec![],
            },
            pages: pages.into_iter().collect(),
            page_id_mapping,
        }
    });
#[component(no_case_check)]
pub fn AnnouncingKevinRs() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey AI pioneers and Rustaceans," }
        p {
            "We've been told again and again that Artificial General Intelligence (AGI) is something for the distant future, maybe decades away, and only something giant companies with endless money and server farms can ever pull off. But here at Kevin RS, we're saying \"no thanks\" to that slow-moving mindset, because we believe AGI doesn't need to wait. Kevin RS is our mission, our manifesto, and our platform - all wrapped into one fully open-source toolkit built in pure Rust. This project is meant for people who care about doing things right, building agents that can think and act independently, and running them at a level of performance and safety that most modern stacks simply can't reach. Kevin RS isn't about chasing hype - it's about solving real problems, moving fast without breaking things, and building something that's meant to last. If you've ever been frustrated with janky Python scripts that break when one cloud service goes down, or tired of agents that need hand-holding to finish even simple tasks, or annoyed that your \"autonomous AI\" only works in one very specific context, Kevin RS is your way out of that mess. It's designed to be provider-agnostic, ultra-flexible, and completely free for anyone to build with. It's more than just a codebase - it's a movement. And the future of general intelligence belongs to builders like you."
        }
        h2 { id: "why-kevin-rs",
            a { href: "#why-kevin-rs", class: "header", "Why Kevin RS?" }
        }
        p {
            "Kevin RS exists because we got tired of toy-like \"AI agents\" that only work in demos or need everything hard-coded just to get through a basic task. This isn't just a tool - it's a framework for people who want to build AGI seriously, and that means we've taken a different approach from the ground up. Most agent frameworks lock you into specific cloud providers or APIs, and when those services change, your entire stack breaks - but Kevin RS is provider-agnostic, which means you can switch between OpenAI, Anthropic, or even self-hosted models without rewriting your app. That's not just a nice-to-have; it's essential if you care about long-term resilience and flexibility. It also supports fault-tolerant, multi-tenant execution, meaning you can spin up and run dozens (or even hundreds) of AI agents at once without them crashing each other or interfering with your system. The agents use zero-shot reasoning, which means you don't need to manually tell them how to do every little task - they learn what they need to do from the prompt and available tools. Rust helps make all this fast and reliable, because the language gives us \"zero-cost abstractions\", so we don't pay any speed penalty for using powerful tools like async tasks, memory-safe types, and advanced trait systems. If you've ever waited ten seconds for a Python bot to do something you know should take half a second - or if you've watched a container crash because some library silently failed - then you already know why Kevin RS matters. It gives you control, speed, and stability, all in one toolkit, and it's finally time for AI agents to grow up and be treated like real software - not toys."
        }
        h2 { id: "built-in-pure-rust",
            a { href: "#built-in-pure-rust", class: "header", "Built in Pure Rust" }
        }
        p {
            "We are building Kevin RS in Rust not because it's trendy or cool (although it is), but because it solves problems that other languages just don't touch. Rust is a systems programming language that gives you the kind of raw power you'd get from C or C++, but with a safety model that helps you catch mistakes before you ever run your code. That's incredibly important when you're building complex systems like AGI agents that might be running for hours, days, or even weeks without stopping. In Rust, you can't have null pointer bugs, buffer overflows, or race conditions unless you really go out of your way to ignore the rules - and those rules are built into the compiler. When you write in Rust, the language actually helps you think through memory management, ownership, concurrency, and more, which makes your code safer and easier to maintain in the long run. Rust also allows for \"fearless concurrency\", meaning you can write multi-threaded applications without worrying about data races, and that makes it ideal for running lots of AI agents at the same time without everything slowing down or crashing. One of the most amazing things about Rust is that all these safety features don't slow your code down - in fact, it compiles into extremely fast machine code that often beats C in benchmarks. And because of its \"zero-cost abstraction\" philosophy, you can use high-level tools without any performance hit. Rust also has a package manager called  "
            code { "Cargo" }
            ", which is incredibly easy to use and lets you build, test, and share your code with almost no setup. In a world where agents need to be fast, robust, and capable of learning on their own, Rust gives us the foundation we need to make that happen - and that's why Kevin RS is being built on it from the ground up."
        }
        h2 { id: "the-autogpt-framework",
            a { href: "#the-autogpt-framework", class: "header", "The AutoGPT Framework" }
        }
        p {
            "At the heart of Kevin RS is a powerful and flexible AutoGPT framework that turns language models into real agents that can act, learn, and evolve. Unlike many frameworks where the \"autonomy\" is just a loop that calls an LLM over and over, our AutoGPT engine is built for real-world workloads, with a full execution engine that supports retrying failed tasks, managing long-running state, and selecting tools dynamically based on what's needed. When you build an agent in Kevin RS, you can easily plug in the tools it needs - like web search, math solvers, or code execution - and those tools can be written in Rust or exposed over a local or remote API. Here's how easy it is to get started:"
        }
        CodeBlock {
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#8c8c8c;\">// TODO: Release\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> agent </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">AutoGPTBuilder::default()\n</span><span style=\"color:#f8f8f2;\">    .</span><span style=\"color:#66d9ef;\">with_provider</span><span style=\"color:#f8f8f2;\">(Provider::Gemini)\n</span><span style=\"color:#f8f8f2;\">    .</span><span style=\"color:#66d9ef;\">with_model</span><span style=\"color:#f8f8f2;\">(Model::Flash20)\n</span><span style=\"color:#f8f8f2;\">    .</span><span style=\"color:#66d9ef;\">tools</span><span style=\"color:#f8f8f2;\">(vec![Tool::Search, Tool::Calculator])\n</span><span style=\"color:#f8f8f2;\">    .</span><span style=\"color:#66d9ef;\">build</span><span style=\"color:#f8f8f2;\">()</span><span style=\"color:#f92672;\">?</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> task </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Message::User {{\n</span><span style=\"color:#f8f8f2;\">    content: Content::Text(</span><span style=\"color:#ffee99;\">&quot;Create a weather forecast website for global cities.&quot;</span><span style=\"color:#f8f8f2;\">.</span><span style=\"color:#66d9ef;\">to_string</span><span style=\"color:#f8f8f2;\">()),\n</span><span style=\"color:#f8f8f2;\">    name: </span><span style=\"font-style:italic;color:#66d9ef;\">None</span><span style=\"color:#f8f8f2;\">,\n</span><span style=\"color:#f8f8f2;\">}};\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> agent.</span><span style=\"color:#66d9ef;\">run</span><span style=\"color:#f8f8f2;\">(vec![task]).await {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(response) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, response);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">        eprintln!(</span><span style=\"color:#ffee99;\">&quot;Agent error: {{:?}}&quot;</span><span style=\"color:#f8f8f2;\">, err);\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
        }
        p {
            "That's it. No boilerplate, no messy Python glue, no fragile shell scripts. Everything runs fast and smoothly right inside your app, with full error handling, and safety guards built in. And because the entire stack is written in Rust, you get tight control over performance and safety, which matters a lot when your agents are running long workflows or managing sensitive data. The best part is that AutoGPT in Kevin RS is not just a proof of concept - it's production-grade software, ready to be used in everything from AGI research labs to developer tools to autonomous robots. It's not just smarter, it's powerfully built."
        }
        h2 { id: "kevin-agi-the-web-app-coming-soon",
            a { href: "#kevin-agi-the-web-app-coming-soon", class: "header",
                "Kevin AGI: The Web App (Coming Soon)"
            }
        }
        p {
            "We know not everyone wants to live in a terminal or build their own interface, so we're working on something truly exciting: a full-featured web app called Kevin that will let you interact with your AI agents through an easy-to-use interface. This app will make it possible to create, run, and monitor autonomous agents right from your browser, with tools for task chaining, visual debugging, and live feedback. It's designed to make AGI research feel more like using an IDE - and it will support everything from natural language task submission to plug-and-play tool integration. You'll be able to assign tasks like \"generate a research paper\", \"monitor my open-source issues\", or \"auto-deploy my web app\" and have the agents not only understand but "
            em { "act" }
            " on those requests. The app is fully open-source and will be easy to self-host or extend, so whether you're a solo hacker or running a whole AGI team, you'll be able to make Kevin your own. We're targeting an early alpha by summer 2025, and we're building it with the same values that power the Rust ecosystem: speed, reliability, safety, and full control for the user."
        }
        h2 { id: "why-rust-for-agi",
            a { href: "#why-rust-for-agi", class: "header", "Why Rust for AGI?" }
        }
        p {
            "People often ask: \"Why not Python for AGI? Isn't that what everyone else uses?\" And yes, Python is popular - but for AGI, we need more than just popularity. We need performance, predictability, and reliability, and that's exactly where Rust shines. With tools like Cargo, you get a single command that can compile, test, lint, and deploy your code, all with consistent results every time. Unlike Python, which often fails at runtime due to import errors, version mismatches, or memory leaks, Rust gives you compile-time confidence that your agents will behave the way you expect. It also lets you pick and choose features at build time, so you don't end up with bloated binaries or unexpected dependencies. There's no garbage collector, so you don't get random pauses in your agents when memory gets tight, which is a big deal for real-time systems or high-speed loops. And the Rust ecosystem is full of mature, reliable libraries - from Tokio for async web servers to Yew for modern frontends - that make it easy to build complete systems without leaving the Rust universe. At the end of the day, Rust gives you control, clarity, and confidence - three things you definitely want when building general intelligence."
        }
        h2 { id: "community--contribution",
            a { href: "#community--contribution", class: "header", "Community & Contribution" }
        }
        p {
            "Kevin RS isn't a closed project or a solo effort - it's a community-driven platform that grows through the ideas, contributions, and experiments of people like you. Whether you're an experienced Rustacean or someone just learning how agents work, there's a place for you in this ecosystem. The best way to get involved is to visit our "
            a { href: "https://github.com/kevin-rs/autogpt", "GitHub repo" }
            ", where you can find open issues, submit pull requests, or start discussions about new tools and features. We also have an active Discord where you can chat with other developers, share ideas, and ask questions in real time. If you're doing research or publishing papers on agent behavior, cognition, or AGI theory, we'd love to collaborate and highlight your work. Kevin RS was built with transparency in mind - we believe that the future of intelligence should be open, fair, and built for everyone, not locked behind paywalls or controlled by tech giants. By joining this project, you're not just writing code - you're shaping the future of how intelligence works."
        }
        h2 { id: "roadmap-agi-when",
            a { href: "#roadmap-agi-when", class: "header", "Roadmap: AGI When?" }
        }
        p {
            "Our big goal is bold, but we believe in it: we want to show off a working, general-purpose AI agents - capable of real-world learning, research synthesis, and self-improvement - by summer 2025. To do that, we're focusing on four major milestones. First, we're shipping AutoGPT 0.2.0, which will include a richer tool ecosystem, smarter decision-making loops, and better memory handling. Second, we're releasing Kevin - the web app UI that will make it easy to build, test, and share AGI workflows without touching code. Third, we're launching a benchmark suite that will allow researchers to test their agents on real tasks and compare results on global leaderboards. And finally, we're developing new kinds of specialized agents - ones that can write code, prove theorems, debug other agents, and even build their own internal models of the world. These milestones aren't just dreams - they're all in progress. And if we hit them, we'll be one giant step closer to open, safe, usable AGI for everyone."
        }
        h2 { id: "license--get-started",
            a { href: "#license--get-started", class: "header", "License & Get Started" }
        }
        p {
            "Kevin RS is 100% open source and licensed under the MIT license, which means you can do pretty much anything you want with it - fork it, modify it, use it in commercial products, or build your own version. Getting started is easy, and if you already have Rust installed, you can clone the repo and build it right now:"
        }
        CodeBlock { contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f8f8f2;\">git clone https:</span><span style=\"color:#8c8c8c;\">//github.com/kevin-rs/autogpt.git\n</span><span style=\"color:#f8f8f2;\">cd autogpt\n</span><span style=\"color:#f8f8f2;\">cargo build </span><span style=\"color:#f92672;\">--</span><span style=\"color:#f8f8f2;\">release</span></pre>\n" }
        p {
            "From there, you're off to the races. Build your own agents, contribute tools to the framework, or just explore how the system works. The only limit is your imagination."
        }
        blockquote {
            p {
                "Let's build the future of intelligence - "
                strong { "together" }
                ", and in Rust."
            }
        }
    }
}
#[component(no_case_check)]
pub fn RethinkingArcAgi() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h2 { id: "tldr",
            a { href: "#tldr", class: "header", "TLDR;" }
        }
        p {
            "Francois Chollet's ARC‑AGI benchmark was introduced to measure fluid intelligence in AI systems and to spotlight key bottlenecks on the path to AGI (^11). However, version 1 quickly went under brute‑force pattern‑matching strategies that bypass genuine reasoning and render the benchmark less effective (^4). In response, version 2 introduced multi‑step compositional reasoning and contextual rule application to raise the bar and mitigate brute forcing (^9). Yet, despite these improvements, persistent limitations remain, such as narrow task domains, rule ambiguity, and susceptibility to overfitting strategies customized to the test set (^10). Moreover, many stakeholders mistakenly treat ARC‑AGI as a definitive measure of AGI rather than a diagnostic tool for research insights (^7). This misuse has fueled conceptual misunderstandings regarding intelligence definitions and benchmark overreach (^6). Alternative methods, such as dynamic environment simulations, broad curriculum learning, and neurosymbolic integration, offer promising directions to complement or replace ARC‑AGI. Additionally, community‑driven benchmarks aiming at open‑ended task generalization may better reflect real‑world AGI demands (^13). Future research should balance designing challenging puzzles and avoiding artifacts that invite hacky solutions (^3). At kevin RS, we embrace these challenges by building open tools and frameworks to push AI development toward scalable, generalizable automation that exceeds narrow benchmarks (^12)."
        }
        h2 { id: "about-the-arcagi-benchmark",
            a { href: "#about-the-arcagi-benchmark", class: "header",
                "About the ARC‑AGI Benchmark"
            }
        }
        p {
            "The Abstraction and Reasoning Corpus for Artificial General Intelligence (ARC‑AGI) was proposed by Francois Chollet in 2019 as a benchmark to evaluate fluid intelligence in AI systems (^11). Inspired by childhood intelligence tests and the principle of program induction, ARC‑AGI presents visual reasoning puzzles that require pattern recognition, transformation, and abstract rule inference (^5). Unlike traditional benchmarks focusing on narrow‑domain performance, ARC‑AGI aims to assess an agent's ability to generalize to novel, unseen tasks with minimal examples (^11). The public evaluation set contains 800 tasks, while a private test set of similar complexity is used for scoring and competition purposes (^5). Early participants in ARC‑AGI‑1 relied on deep‑learning‑guided program synthesis techniques to achieve modest success rates around 33% on the private evaluation set (^5). Given the open‑ended nature of the problem space, human solvers reportedly achieved up to 85% accuracy under time constraints, demonstrating a significant performance gap between humans and machines. Despite the intuitive appeal of this test, version 1 of the benchmark was soon undermined by brute‑force algorithms that exhaustively searched combinations of primitive operations to find valid solutions (^4). These brute‑force strategies exploited the limited operations vocabulary and the finite search space of simple puzzle dimensions, allowing automated solutions with massive compute to dominate the leaderboard (^4). As a result, ARC‑AGI‑1 lost its ability to discriminate truly general reasoning from computationally expensive pattern‑matching hacks, reducing its diagnostic value for AGI research (^6). This shortcoming prompted the design of ARC‑AGI‑2, which aimed to patch these issues by introducing more complex task scaffolding and compositional rule requirements (^10)."
        }
        h2 { id: "flaws-in-version-1",
            a { href: "#flaws-in-version-1", class: "header", "Flaws in Version 1" }
        }
        p {
            "Version 1 of ARC‑AGI was designed with a fixed set of primitive operations that AI agents could combine to transform input grids into output grids, encompassing actions such as rotation, reflection, and color replacement. While this operational vocabulary captured the essence of many abstract puzzles, it also created a bounded search space that brute‑force techniques could traverse with sufficient computational resources (^4). Researchers quickly demonstrated that by systematically enumerating all possible compositions of operations up to a certain depth, AI agents could solve a large fraction of ARC‑AGI‑1 tasks without any genuine pattern understanding (^4). For example, a simple Python script leveraging recursive search can iterate through operation sequences and validate them against the provided training examples."
        }
        p {
            "This brute‑force technique demonstrates how the lack of puzzle diversity made it feasible to bypass the intended challenge through exhaustive search (^4). As solver implementations grew more sophisticated, they incorporated heuristics to prune the search tree, further boosting performance and highlighting design weaknesses in version 1 (^5). Consequently, version 1 ceased to serve as a meaningful barometer for developing flexible reasoning systems and instead reflected optimization of search algorithms (^1). This experience underscores the threats of static benchmarks on evolving computational landscapes, where solver ingenuity can outpace test designers' assumptions (^6). In response, the ARC‑AGI team moved to strengthen the benchmark's resilience by introducing version 2 later that year (^3)."
        }
        h2 { id: "version-2-improvements",
            a { href: "#version-2-improvements", class: "header", "Version 2 Improvements" }
        }
        p {
            "ARC‑AGI‑2 was released with a suite of modifications intended to thwart brute‑force exploitation by expanding the operational vocabulary and task scaffolding complexity (^10). Key enhancements included multi‑step compositional reasoning rules that require solvers to apply sequences of transformations conditioned on intermediate results (^1). Contextual rule application was introduced to ensure that puzzles could not be decoupled into independent subproblems, thereby blocking isolated brute‑force searches (^10). Additionally, version 2 incorporated randomized rule embedding, where certain elements of a puzzle's rule set were obfuscated until specific criteria were met during execution (^3). These measures aimed to force solvers to develop genuine inferential strategies instead of relying solely on exhaustive enumeration. Despite these improvements, developers discovered that by integrating heuristic pruning and dynamic rule inference, many tasks remained susceptible to pattern‑collision bypasses. The added complexity also raised the technical bar for participant engagement, potentially deterring researchers without extensive engineering resources (^9). Furthermore, some puzzles exhibited ambiguous solution paths, leading to multiple valid interpretations and complicating automatic evaluation (^10). These residual issues highlight the challenge of designing puzzles that are simultaneously open‑ended, unambiguous, and resistant to non‑human strategies (^5). As a result, ARC‑AGI‑2, while a marked improvement, still falls short of an ideal AGI benchmark and invites continued iteration (^8)."
        }
        h2 { id: "misuse-as-an-agi-measure-and-conceptual-misunderstandings",
            a {
                href: "#misuse-as-an-agi-measure-and-conceptual-misunderstandings",
                class: "header",
                "Misuse as an AGI Measure and Conceptual Misunderstandings"
            }
        }
        p {
            "Although F. Chollet has repeatedly emphasized that ARC‑AGI is not a definitive test for AGI, many in the community interpret high scores as AGI milestones (^8). This conflation overlooks the fundamental difference between solving a closed set of puzzles and exhibiting broad, human‑like adaptability across diverse tasks. Defining AGI itself remains a complex task, with no consensus on whether intelligence should be gauged by task breadth, learning efficiency, or cognitive flexibility (^13). Benchmark overreach may lead to premature claims of AGI achievement, driven more by competition and publicity than by scientific rigor (^1). Indeed, OpenAI staff members have argued that surpassing human performance on certain tasks could qualify as AGI under loose definitions, further puzzling the discourse (^1). Such debates underscore the need for clearer conceptual frameworks that distinguish between domain‑specific competence and true general intelligence (^6). Moreover, an overreliance on any single benchmark risks promoting overfitting of research efforts to that metric at the expense of broader innovation (^5). A holistic assessment of AGI progress should incorporate multiple evaluation modalities, including interactive simulations, real‑world robotics, and language understanding. By diversifying benchmarks, the field can mitigate the risk of tunnel vision and encourage development of versatile, robust AI systems (^9). Therefore, while ARC‑AGI provides valuable insights, it should be positioned as one tool among many in the AGI evaluation toolkit (^11)."
        }
        h2 { id: "future-directions",
            a { href: "#future-directions", class: "header", "Future Directions" }
        }
        p {
            "In a recent video presentation, F. Chollet outlines the motivations and developments behind ARC‑AGI and its iterations (^11). In light of ARC‑AGI's challenges, researchers are exploring dynamic environment benchmarks that mimic real‑world task variability and interactivity (^13). Projects such as "
            a { href: "https://github.com/openai/gym", "OpenAI's AI Gym" }
            ", "
            a { href: "https://arxiv.org/abs/1711.09883", "DeepMind's AI Safety Gridworlds" }
            ", and "
            a { href: "https://research.facebook.com/publications/habitat-a-platform-for-embodied-ai-research/",
                "Meta's Habitat"
            }
            " offer simulated worlds where agents learn through trial, error, and adaptive planning. Broad curriculum learning pipelines propose progressively increasing task difficulty to scaffold AI capabilities in a manner analogous to human education (^5). Neurosymbolic integration combines neural networks with symbolic reasoning modules, aiming to capture pattern recognition and logical inference in a unified architecture (^12). Program synthesis approaches leverage language models to generate candidate programs, offering a bridge between LLM fluency and precise rule‑based task execution (^3). Community‑led benchmark initiatives emphasize open‑source evaluation suites, transparent leaderboards, and reproducibility to foster collaborative progress (^9). Advanced puzzle designs are experimenting with procedurally generated tasks, random perturbations, and adversarial examples to further resist brute forcing (^10). Ultimately, the path to AGI will likely require hybrid evaluation strategies, combining analytical puzzles, simulated environments, and real‑world performance metrics (^13). At kevin RS, our vision is to unify these methodologies by building modular APIs that allow seamless integration of diverse benchmark types, from visual reasoning to autonomous control (^12)."
        }
        h2 { id: "references",
            a { href: "#references", class: "header", "References" }
        }
        ul {
            li {
                "(^1): Wiggers K. A test for AGI is closer to being solved - but it may be flawed. TechCrunch. https://techcrunch.com/2024/12/09/a-test-for-agi-is-closer-to-being-solved-but-it-may-be-flawed/. Published December 10, 2024."
            }
            li {
                "(^2): If an LLM solves this then we'll have AGI - Francois Chollet : r/singularity. https://www.reddit.com/r/singularity/comments/1df3bi9/if_an_llm_solves_this_then_well_have_agi_francois/."
            }
            li {
                "(^3): OpenAI o3 Breakthrough High Score on ARC-AGI-Pub. ARC Prize. https://arcprize.org/blog/oai-o3-pub-breakthrough."
            }
            li {
                "(^4): Pfister R, Jud H. Understanding and Benchmarking Artificial Intelligence: OpenAI's o3 Is Not AGI. arXiv.org. https://arxiv.org/abs/2501.07458. Published January 13, 2025."
            }
            li {
                "(^5): Chollet F, Knoop M, Kamradt G, Landers B. ARC Prize 2024: Technical report. arXiv.org. https://arxiv.org/abs/2412.04604. Published December 5, 2024."
            }
            li {
                "(^6): Lee K. ARC-AGI is a genuine AGI test but o3 cheated :(. December 2024. https://www.lesswrong.com/posts/KHCyituifsHFbZoAC/arc-agi-is-a-genuine-agi-test-but-o3-cheated/."
            }
            li {
                "(^7): Wiggers K. A test for AGI is closer to being solved - but it may be flawed. Yahoo Finance. https://finance.yahoo.com/news/test-agi-closer-being-solved-013640000.html?guccounter=1. Published December 10, 2024."
            }
            li {
                "(^8): Wong M. The man out to prove how dumb AI still is. The Atlantic. https://www.theatlantic.com/technology/archive/2025/04/arc-agi-chollet-test/682295/. Published April 4, 2025."
            }
            li {
                "(^9): (2) Is AGI Closer Than We Think? Insights from the ARC-AGI Test | LinkedIn. https://www.linkedin.com/pulse/agi-closer-than-we-think-insights-from-arc-agi-test-r-pillai-ukqoe/. Published December 14, 2024."
            }
            li {
                "(^10): Lolade. François Chollet Creates a Foundation that Will Make Benchmarks for AGI. AutoGPT. https://autogpt.net/francois-chollet-creates-a-foundation-that-creates-benchmarks-for-agi/. Published January 9, 2025."
            }
            li { "(^11): ARC Prize - What is ARC-AGI? ARC Prize. https://arcprize.org/arc-agi." }
            li {
                "(^12): github/profile/README.adoc at main · kevin-rs/.github. https://github.com/kevin-rs/.github/blob/main/profile/README.adoc.."
            }
            li {
                "(^13): Wikipedia contributors. Artificial general intelligence. Wikipedia. https://en.wikipedia.org/wiki/Artificial_general_intelligence. Published April 22, 2025."
            }
            li {
                "(^14): ARC Prize. ARC-AGI-2 overview with Francois Chollet. YouTube. April 2025. https://www.youtube.com/watch?v=TWHezX43I-4."
            }
        }
    }
}

use dioxus::prelude::*;
use crate::components::blog::code::CodeBlock;
