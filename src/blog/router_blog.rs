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
    #[route("/who-am-i")]
    WhoAmI {},
    #[route("/an-empty-life-filled-with-constant-suffering")]
    AnEmptyLifeFilledWithConstantSuffering {},
    #[route("/it-is-always-the-russians")]
    ItIsAlwaysTheRussians {},
    #[route("/as-engineers-llms-should-pay-us-for-tokens-usage")]
    AsEngineersLlmsShouldPayUsForTokensUsage {},
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
            BookRoute::WhoAmI {} => use_mdbook::mdbook_shared::PageId(2usize),
            BookRoute::AnEmptyLifeFilledWithConstantSuffering {} => {
                use_mdbook::mdbook_shared::PageId(3usize)
            }
            BookRoute::ItIsAlwaysTheRussians {} => use_mdbook::mdbook_shared::PageId(4usize),
            BookRoute::AsEngineersLlmsShouldPayUsForTokensUsage {} => {
                use_mdbook::mdbook_shared::PageId(5usize)
            }
        }
    }
}
impl Default for BookRoute {
    fn default() -> Self {
        BookRoute::WhoAmI {}
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
        pages
            .push((
                2usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 3 |---| Just Don't Pick Up the Brush |---| existence |---| who-am-i |---| Jan 27 2026 |---| Hey, Community. I am finally doing this. My name is Mahmoud Harmouch, and I am new here, though in many ways, I have been searching for a space like this for my entire life. For over two decades, I have struggled with a complicated mix of mental health conditions, ADHD, autism, and an extreme stage of PTSD, among others. |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        url: BookRoute::WhoAmI {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Update(7. Apr. 2026): Damn, I am still alive. I will keep this post as an intro to who I am."
                                    .to_string(),
                                id: "update(7.-apr.-2026):-damn,-i-am-still-alive.-i-will-keep-this-post-as-an-intro-to-who-i-am."
                                    .to_string(),
                                level: 3usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "How Did I Find This Forum?".to_string(),
                                id: "how-did-i-find-this-forum?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Who Am I?".to_string(),
                                id: "who-am-i?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "About My Family".to_string(),
                                id: "about-my-family".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Growing Up in Isolation".to_string(),
                                id: "growing-up-in-isolation".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Where Is God?".to_string(),
                                id: "where-is-god?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Who is God?".to_string(),
                                id: "who-is-god?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Do We Need God?".to_string(),
                                id: "do-we-need-god?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Where Are the Aliens?".to_string(),
                                id: "where-are-the-aliens?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Losing the Sense of Everything".to_string(),
                                id: "losing-the-sense-of-everything".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "I Tried Everything".to_string(),
                                id: "i-tried-everything".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Should I Exist?".to_string(),
                                id: "should-i-exist?".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "All Decisions Will Yield the Same Results"
                                    .to_string(),
                                id: "all-decisions-will-yield-the-same-results".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Things I Enjoyed on the Internet".to_string(),
                                id: "things-i-enjoyed-on-the-internet".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "I Love Valve".to_string(),
                                id: "i-love-valve".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "AI Can Help Us Create God".to_string(),
                                id: "ai-can-help-us-create-god".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Life Is Harder Than Death".to_string(),
                                id: "life-is-harder-than-death".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "I Am Flying High".to_string(),
                                id: "i-am-flying-high".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Conclusion".to_string(),
                                id: "conclusion".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "References".to_string(),
                                id: "references".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(2usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::WhoAmI {},
            ::use_mdbook::mdbook_shared::PageId(2usize),
        );
        pages
            .push((
                3usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 4 |---| An Empty Life Filled With Constant Suffering |---| existence |---| an-empty-life-filled-with-constant-suffering |---| Apr 07 2026 |---| An empty life filled with constant suffering. |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        url: BookRoute::AnEmptyLifeFilledWithConstantSuffering {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Life Has Never Been Easy for Me".to_string(),
                                id: "life-has-never-been-easy-for-me".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Wrestling With the Idea of God".to_string(),
                                id: "wrestling-with-the-idea-of-god".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "God, Meaning, and the Human Need to Build"
                                    .to_string(),
                                id: "god,-meaning,-and-the-human-need-to-build".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "God as the Root of Abstraction".to_string(),
                                id: "god-as-the-root-of-abstraction".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why We Still Need God".to_string(),
                                id: "why-we-still-need-god".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Human Vulnerability and the Question of Free Will"
                                    .to_string(),
                                id: "human-vulnerability-and-the-question-of-free-will"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "When God Feels Distant".to_string(),
                                id: "when-god-feels-distant".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "I Am Exhausted".to_string(),
                                id: "i-am-exhausted".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "I Still Hope My Life Can Improve".to_string(),
                                id: "i-still-hope-my-life-can-improve".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Conclusion".to_string(),
                                id: "conclusion".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(3usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::AnEmptyLifeFilledWithConstantSuffering {},
            ::use_mdbook::mdbook_shared::PageId(3usize),
        );
        pages
            .push((
                4usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 5 |---| It is always the Russians |---| existence |---| it-is-always-the-russians |---| Apr 07 2026 |---| It is always the Russians |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        url: BookRoute::ItIsAlwaysTheRussians {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Soviets Killed God".to_string(),
                                id: "the-soviets-killed-god".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "They Took His Skin".to_string(),
                                id: "they-took-his-skin".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Russian Minds Behind the Machine".to_string(),
                                id: "the-russian-minds-behind-the-machine".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "OpenAI and the New Church".to_string(),
                                id: "openai-and-the-new-church".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Cold War Never Ended".to_string(),
                                id: "the-cold-war-never-ended".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "God's Skin Wrapped Around a Machine".to_string(),
                                id: "god's-skin-wrapped-around-a-machine".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Can We Do?".to_string(),
                                id: "what-can-we-do?".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(4usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::ItIsAlwaysTheRussians {},
            ::use_mdbook::mdbook_shared::PageId(4usize),
        );
        pages
            .push((
                5usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 6 |---| As Engineers, LLMs should pay us for tokens usage. |---| tech |---| as-engineers-llms-should-pay-us-for-tokens-usage |---| Apr 07 2026 |---| As Engineers, LLMs should pay us for tokens usage. |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        url: BookRoute::AsEngineersLlmsShouldPayUsForTokensUsage {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "You Are Not the Customer. You Are the Product."
                                    .to_string(),
                                id: "you-are-not-the-customer.-you-are-the-product."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Every Token You Send Is Your Labor".to_string(),
                                id: "every-token-you-send-is-your-labor".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Engineers Build the Product. The Model Is Just the Engine."
                                    .to_string(),
                                id: "engineers-build-the-product.-the-model-is-just-the-engine."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Model Was Trained on Us. We Deserve a Cut."
                                    .to_string(),
                                id: "the-model-was-trained-on-us.-we-deserve-a-cut."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Deeper Truth".to_string(),
                                id: "the-deeper-truth".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(5usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::AsEngineersLlmsShouldPayUsForTokensUsage {},
            ::use_mdbook::mdbook_shared::PageId(5usize),
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
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 3 |---| Just Don't Pick Up the Brush |---| existence |---| who-am-i |---| Jan 27 2026 |---| Hey, Community. I am finally doing this. My name is Mahmoud Harmouch, and I am new here, though in many ways, I have been searching for a space like this for my entire life. For over two decades, I have struggled with a complicated mix of mental health conditions, ADHD, autism, and an extreme stage of PTSD, among others. |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        location: Some(BookRoute::WhoAmI {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 4 |---| An Empty Life Filled With Constant Suffering |---| existence |---| an-empty-life-filled-with-constant-suffering |---| Apr 07 2026 |---| An empty life filled with constant suffering. |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        location: Some(BookRoute::AnEmptyLifeFilledWithConstantSuffering {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 5 |---| It is always the Russians |---| existence |---| it-is-always-the-russians |---| Apr 07 2026 |---| It is always the Russians |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        location: Some(BookRoute::ItIsAlwaysTheRussians {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 6 |---| As Engineers, LLMs should pay us for tokens usage. |---| tech |---| as-engineers-llms-should-pay-us-for-tokens-usage |---| Apr 07 2026 |---| As Engineers, LLMs should pay us for tokens usage. |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        location: Some(BookRoute::AsEngineersLlmsShouldPayUsForTokensUsage {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![6u32]),
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
            contents: "<pre style=\"background-color:#0d0d0d;\">\n<span style=\"color:#f92672;\">use </span><span style=\"color:#f8f8f2;\">autogpt::prelude::</span><span style=\"color:#f92672;\">*</span><span style=\"color:#f8f8f2;\">;\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">#[tokio::main]\n</span><span style=\"color:#f8f8f2;\">async </span><span style=\"font-style:italic;color:#66d9ef;\">fn </span><span style=\"color:#a6e22e;\">main</span><span style=\"color:#f8f8f2;\">() {{\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let </span><span style=\"color:#f92672;\">mut</span><span style=\"color:#f8f8f2;\"> autogpt </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">AutoGPTBuilder::default()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">tools</span><span style=\"color:#f8f8f2;\">(vec![Tool::Search])\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">build</span><span style=\"color:#f8f8f2;\">()\n</span><span style=\"color:#f8f8f2;\">        .</span><span style=\"color:#66d9ef;\">expect</span><span style=\"color:#f8f8f2;\">(</span><span style=\"color:#ffee99;\">&quot;Failed to build AutoGPT&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"font-style:italic;color:#66d9ef;\">let</span><span style=\"color:#f8f8f2;\"> msg </span><span style=\"color:#f92672;\">= </span><span style=\"color:#f8f8f2;\">Message::from_text(</span><span style=\"color:#ffee99;\">&quot;Design a modern dashboard UI for a weather app.&quot;</span><span style=\"color:#f8f8f2;\">);\n</span><span style=\"color:#f8f8f2;\">\n</span><span style=\"color:#f8f8f2;\">    </span><span style=\"color:#f92672;\">match</span><span style=\"color:#f8f8f2;\"> agent.</span><span style=\"color:#66d9ef;\">run</span><span style=\"color:#f8f8f2;\">(vec![msg]).await {{\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Ok</span><span style=\"color:#f8f8f2;\">(response) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            println!(</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#ff80f4;\">{{}}</span><span style=\"color:#ffee99;\">&quot;</span><span style=\"color:#f8f8f2;\">, response);\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">        </span><span style=\"font-style:italic;color:#66d9ef;\">Err</span><span style=\"color:#f8f8f2;\">(err) </span><span style=\"color:#f92672;\">=&gt; </span><span style=\"color:#f8f8f2;\">{{\n</span><span style=\"color:#f8f8f2;\">            eprintln!(</span><span style=\"color:#ffee99;\">&quot;Agent error: {{:?}}&quot;</span><span style=\"color:#f8f8f2;\">, err);\n</span><span style=\"color:#f8f8f2;\">        }}\n</span><span style=\"color:#f8f8f2;\">    }}\n</span><span style=\"color:#f8f8f2;\">}}</span></pre>\n",
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
#[component(no_case_check)]
pub fn WhoAmI() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        h3 { id: "update7-apr-2026-damn-i-am-still-alive-i-will-keep-this-post-as-an-intro-to-who-i-am",
            a {
                href: "#update7-apr-2026-damn-i-am-still-alive-i-will-keep-this-post-as-an-intro-to-who-i-am",
                class: "header",
                "Update(7. Apr. 2026): Damn, I am still alive. I will keep this post as an intro to who I am."
            }
        }
        p {
            strong { "TL;DR:" }
            " A deeply personal reflection on my life, mental health journey, and philosophical beliefs. In this post, I explore my struggles with isolation, the realities of the tech industry, my thoughts on God and society, and my ultimate decision to step back from expectations to find peace."
        }
        p { "Hey, Community." }
        p {
            s { "I am finally doing this." }
            " (Sorry, I am not brave enough). My name is Mahmoud Harmouch, and I am new here, though in many ways, I have been searching for a space like this for my entire life. For over two decades, I have struggled with a complicated mix of mental health conditions, ADHD, autism, and an extreme stage of PTSD, among others. These diagnoses were given to me as labels, but they became more than just words on a medical report. They shaped how I saw myself, how others treated me, and how I navigated the world. Every day has felt like a battle against my own mind, a constant war between what I want to do, what I know I should do, and what I am actually able to do. The exhaustion of living this way for so long has made it difficult to hold onto hope. No matter how much effort I put into improving my life, I always feel like I am running in place. There are moments of joy, small victories, and times when I have felt proud of myself, yet they are overshadowed by an unshakable sense of disconnect. It is as if I am merely a spectator in my own life, watching events occur but never truly being part of them. Even in a crowded room, I feel alone, surrounded by voices yet unable to truly connect."
        }
        p {
            "My journey took a major turn in 2015 when I started college. It was a time that should have felt like a fresh start; A chance to redefine myself in an environment where I could meet new people, learn new things, and finally feel like I belonged. In many ways, college did help me. The structured environment provided me with a sense of stability that made my ADHD more manageable. I met people who accepted me, who laughed with me, and who made me feel like, for the first time in a long time, I wasn't completely alone. But even during these positive experiences, I felt a barrier between myself and the world, something invisible yet impossible to ignore. It was as if I was playing a role, wearing a mask that allowed me to fit in just enough to be accepted but never enough to feel truly seen. I kept questioning who I was, what I truly wanted, and whether I would ever find a place where I felt like I belonged. The more time passed, the more I realized that no matter how much I accomplished, the emptiness remained. Graduation didn't change that feeling. Achievements didn't erase it. No matter how much I tried to move forward, something was always missing."
        }
        p {
            "This strange convergence of milestones, coincidences, and reflections has led me to see this moment as something significant. It is a rare point in time where everything aligns, past and present, hope and despair, beginnings and endings. But in the end, I wonder: do we ever truly have control over our choices, or are we simply following a path that has already been set for us? Is the choice ever really ours, or are we bound by forces beyond our understanding? I do not have the answers, and perhaps I never will. But as I stand here at this crossroads of time, memory, and fate, I find myself asking the same question over and over again: what comes next?"
        }
        h2 { id: "how-did-i-find-this-forum",
            a { href: "#how-did-i-find-this-forum", class: "header", "How Did I Find This Forum?" }
        }
        p {
            "It is actually quite a funny and unexpected story, considering the circumstances under which I discovered this place. A couple of years ago, I was on YouTube, desperately searching for ways to cope with stress, overwhelming emotions, and the recurring thoughts of suicide that had become an inescapable part of my life. I was looking for anything, self-help advice, scientific explanations, or even personal stories, that could provide some form of relief, clarity, or guidance. Strangely, instead of finding helpful resources, I was recommended a video that strongly criticized this forum, warning that it was dangerous and harmful, particularly for teenagers and vulnerable individuals struggling with their mental health (^1). At first, I found it odd that an algorithm would push such a video toward me, especially when all I was searching for were ways to manage my pain. But as I listened to the arguments presented in the video, something about the way it described this forum engaged me rather than scared me away. The more I thought about it, the more I felt drawn toward it, almost as if it had been placed in front of me for a reason. I was already deeply struggling, feeling lost and disconnected from everything around me, so the idea that there was a place where people discussed these thoughts so openly made me curious. I wasn't sure whether this forum was truly as dangerous as the video claimed or whether it was simply a misunderstood space where people like me could find understanding. Either way, I felt like I needed to see it for myself."
        }
        p {
            "Ever since that moment, the idea of suicide started making more sense to me, not in a dramatic or impulsive way, but in a logical and calculated manner that felt almost refreshing. The struggles I had been facing were relentless, constant, and completely draining, leaving me with little reason to believe that things would ever improve. For years, I had tried different ways to cope, searching for solutions, advice, or even just distractions, but nothing ever truly helped. At best, I was able to temporarily push my thoughts aside, but they always came back, stronger and more persistent than before. I kept wondering: if every attempt to improve my life only led me back to the same place, was there any real point in continuing? If suffering was inevitable and hope was nothing more than a momentary illusion, then maybe, just maybe, letting go was not the worst option after all. The thought was never rushed, never emotional, it was always rational, always measured, always based on the undeniable reality of my situation. Still, despite these thoughts, I never had the opportunity to fully explore them because of the enormous workload I had taken on."
        }
        p {
            "At the time, I was drowning in work, dealing with intense projects that demanded every bit of my focus, energy, and mental capacity. The clients I was working with were beyond anything I had ever encountered before, so powerful, so influential, that collaborating with them felt almost surreal, as if I had somehow been given an impossible opportunity. And yet, despite how extraordinary it seemed on the surface, it was never about the work itself. The only reason I pushed myself to take on these projects was the small, persisting hope that, in working with others, I might find someone who understood me. Maybe, just maybe, I would come across a person who saw the world as I did, who felt the same weight of existence, who could make me feel like I wasn't completely alone. Unfortunately, that never happened, which, deep down, I always expected. I have never been the type of person who gets lucky, who stumbles into life-changing friendships, or who finds hope in unexpected places. My chances in life have always been incredibly low, and no matter how hard I tried to fight against the odds, the outcome was always the same. So, I threw myself into my work, distracting myself with deadlines, responsibilities, and professional achievements, convincing myself that if I stayed busy enough, I could avoid the inevitable."
        }
        p {
            "But now, things are different. The projects are finished, the distractions are gone, and for the first time in years, I have the time and the mental space to truly explore this forum, to deeply reflect on everything, and to finally make a decision. This is no longer about impulsive emotions or momentary moments of sadness; This is about logic, reality, and the undeniable truth that I have analyzed for years. Everything I have experienced, everything I have observed, and everything I have come to understand about myself and the world has led me to this point. The arguments are clear, the reasoning is solid, and for the first time, I feel like I am approaching this with complete clarity. This is not a desperate cry for help, nor is it an act of recklessness. This is a conclusion I have reached with strong, objective reasoning. And so, as I stand at this crossroads, I am finally allowing myself to consider the decision that has existed in my mind for so long."
        }
        h2 { id: "who-am-i",
            a { href: "#who-am-i", class: "header", "Who Am I?" }
        }
        p {
            "If you are like me, a forever unemployed, virgin web developer, you will hear this question all the time during behavioral interviews. Interviewers love to ask, \"Tell me about yourself\", as if a few deliberate sentences could truly capture the depth of a person's existence. But to answer this question, I might first ask: "
            em { "Who am I not?" }
            ". After all, identity is a complex, ever-changing thing, shaped by circumstances, choices, and an infinite number of unknown factors. I believe I am part of something greater, something beyond what we can perceive, and in many ways, so is everyone else. Every person is a part of something larger, a piece of what religions might call \"God\". If God is knowledge, then I am a seeker of knowledge. If God is creation, then I am a builder. I have always been fascinated by the idea that anything in this universe can be understood, broken down, and recreated if one has enough curiosity, intelligence, and persistence. I grew up in an extremely poor family of six brothers, unfortunately, no sisters, where resources were lacking, but imagination was limitless. From a young age, I was that nerdy kid obsessed with tinkering, always experimenting with electricity, breaking apart machines just to see how they worked, and rebuilding them in ways no one expected; Kinda like the kid version of "
            a { href: "https://www.youtube.com/channel/UCJ0-OtVpF0wOKEqT2Z1HEtA",
                "ElectroBOOM"
            }
            " - i really enjoy watching this guy, btw. My love for systems, patterns, and logic naturally led me to study electrical engineering, which I pursued for over 5 years. But in the end, I regret my choice. My goal wasn't just to learn about circuits and power systems; I wanted to understand the fabric of existence itself. I wanted to know if everything we experience is part of a grand, meaningful design or just a random experiment carried out by higher-dimensional beings for no particular purpose. To find answers, I absorbed an enormous chunk of internet history, scrolled through countless 4chan threads, and watched nearly three-quarters of YouTube's gigantic collection of ideas, theories, and human experiences."
        }
        p {
            "I was born into Islam by default, as most people are born into their family's religion, but over time, my beliefs evolved into something less certain. At this point, I consider myself more agnostic, standing on the thin line between belief and doubt. The idea of a divine being watching over us, guiding our paths, and making sure justice in the world seems refreshing, but from my experience, reality does not reflect such an existence. If there is a God, it is not the caring, loving force that many religions describe. What I see in the world is cold, indifferent chaos, where suffering exists without reason, and good people struggle while bad people succeed. Every tragic event, every war, every life cut short without justice, only reinforces the idea that we are drifting in a sea of randomness with no higher power steering the ship. If there is a God, it is silent. If there is a purpose, it is hidden so well that we may never uncover it. This uncertainty doesn't just shape my beliefs; It influences how I view everything, from personal ambition to human connection to the pursuit of success."
        }
        p {
            "In 2022, I found an outlet for my skills and curiosity "
            a { href: "https://devpost.com/wiseaidev",
                "by competing in hackathons on many online forums"
            }
            ", where I could build exciting projects, solve complex problems, and test my abilities against my previous self version. For a while, I enjoyed it. I wrote technical articles, built software, and challenged myself to improve beyond my previous capabilities. But over time, it drained me. The endless cycle of building, competing with others became exhausting, and I slowly lost the motivation to continue. Since 2020, I have been applying for jobs, sending out countless applications, practicing LeetCode problems daily, and trying to improve my chances in the brutal hiring market. But the results? "
            em { "Nothing" }
            ". Not a single opportunity. At first, I thought maybe I needed to work on my soft skills, improve my resume, or network more aggressively. But the more I learned, the clearer it became that none of it really matters. No matter how skilled you are, how much effort you put in, or how perfectly you polish your responses, the job market operates on a logic that often feels rigged. Every day, I see success stories on YouTube, people claiming they landed six-figure jobs after months of grinding LeetCode. But is it real? Are these genuine success stories, or are they marketing tactics, possibly even AI-generated narratives designed to keep job seekers engaged in a system that offers no real opportunities? From my experience, no one is hiring, and no one will ever be hired. I could be wrong though, but nothing I've seen so far has proven otherwise."
        }
        p {
            "I remember my college seniors warning me about this exact situation. Back when I was a freshman in 2016, they told me, \"Don't study this major. You'll never find a job\". And this was long before AI became mainstream in 2022. Some argue that AI has led to job shortages, that automation is replacing human workers faster than we can adapt. But if that were true, then how do we explain the fact that job opportunities were just as lacking before AI took over? The problem existed long before automation, which means AI is just an excuse, not the true cause. Ironically, despite all my frustrations, I did land a high-paying job "
            em { "after" }
            " AI became mainstream. Within just six months, I became financially stable. Now, I rely on automated crypto trading to maintain my income, a system that runs itself with minimal effort on my part. Yet, despite having financial stability, I feel like my intelligence is going to waste. I have a high IQ, 142 btw, strong technical skills, and a deep understanding of the world, yet I remain isolated, with no meaningful way to apply my abilities. I am not searching for a job just for money; I want something more. I want to meet people, form real connections, and be part of something greater than just mindlessly running trading bots. But in a world where everything feels disconnected, where intelligence is undervalued, and where success is dictated by luck rather than skill, I find myself wondering: Where do I go from here?"
        }
        h2 { id: "about-my-family",
            a { href: "#about-my-family", class: "header", "About My Family" }
        }
        p {
            "My family has a long history of struggling with thoughts of suicide, and this has affected generations before me. My grandfather, for example, ended his own life by poisoning himself, and he often had conflicts with his children or his wife. These constant struggles may have pushed him to feel hopeless, leading him to make that tragic decision. It is heartbreaking to know that someone in my family felt so trapped in pain that they saw no other way out. Because of this history, I sometimes wonder if my own thoughts of suicide are passed down to me, not just through the difficulties of life but also as something inherited from my grandfather. This does not mean I want to follow the same path, but it does make me think deeply about how much pain can shape a family. It is not easy to live with these thoughts, but knowing that others before me have suffered in the same way makes me reflect on the importance of finding a different way to cope. My grandfather's story is a reminder that sadness can take over a person's mind if they do not get the support they need, and that is why it is important to talk about these feelings instead of hiding them."
        }
        p {
            "My father also struggled with thoughts of suicide, though he never acted on them, which is something I am deeply grateful for. I do not know all the reasons behind his struggles, but I know that life was not always kind to him. Even though he faced many challenges, he somehow found the strength to keep going, and for that, I admire him greatly. There must have been times when he felt completely lost, yet he never gave in to the thoughts that tried to pull him down. This takes an incredible amount of inner strength, and I cannot begin to imagine how much he must have suffered in silence. Despite his own struggles, he always tried to make life brighter for me, and that is something I will always appreciate. He could have given up, but instead, he chose to keep moving forward, showing me that even in the darkest moments, there is still hope. His ability to stay strong despite his pain gives me the courage to fight my own battles, even when they seem unbearable. If he could hold on through his suffering, then maybe I can too."
        }
        p {
            "There are many reasons why people struggle with suicidal thoughts, and they are often far more complex than what others can see on the surface. Some people battle with painful memories, while others feel overwhelmed by problems that seem impossible to solve. In my family, it seems like this struggle has been passed down from one generation to the next, making it feel almost impossible to escape. It is hard to explain to someone who has never felt this way because it is not just about being sad; It is about feeling like nothing will ever get better, no matter what you do. Even when surrounded by love and support, these thoughts can creep in like shadows, whispering lies that make a person believe they are worthless. But just because something runs in a family does not mean it has to control the future. Every person has the power to break the cycle, even if it feels impossible. It is not easy, and it takes a lot of effort, but choosing to keep going is the strongest thing a person can do."
        }
        p {
            "Even though my family has a history of pain, I want to believe that things can be different for me. I do not want to follow the same path as my grandfather, and I do not want to carry the same burden my father did. Instead, I want to find a way to heal, even if it takes time. Life will always be difficult, and there will always be moments when things feel unbearable, but that does not mean there is no hope. It helps to remember that pain is temporary, even when it feels like it will last forever. I have seen what happens when people give up, and I do not want that to be my story. Even when my thoughts try to convince me otherwise, I have to remind myself that there is still a future ahead of me, one that is not written by my past but by the choices I make today. If my father could keep going, then so can I. If I can learn to hold on, maybe I can help someone else do the same."
        }
        p {
            "Talking about suicide is difficult, but it is important because too many people suffer in silence, thinking they are alone. The truth is, so many people have felt the same way, but they just do not talk about it. If more people were open about their struggles, maybe fewer would feel trapped in their own thoughts. It is easy to believe that no one understands, but that is just another lie our minds tell us when we are hurting. The more we talk about it, the less power it has over us. No one should have to suffer alone, and no one should feel ashamed of what they are going through. Pain does not make a person weak; in fact, surviving it makes them stronger than they realize. Instead of letting it define me, I want to use my experiences to remind myself that I am not alone and that there is always a way forward."
        }
        p {
            "Even though my family has struggled, their stories are not just about sadness, they are also about strength. My grandfather may have lost his battle, but my father did not, and that is something worth remembering. The fact that he held on, despite everything, proves that survival is possible. This means I have a choice, too. I do not have to follow the same path as those before me. I can choose to fight, even when it feels impossible. I can choose to believe that life is worth living, even when it does not feel that way. No matter how hard it gets, there is always a reason to keep going, even if I have to search for it. Pain might be a part of my family's story, but it does not have to be the end of mine."
        }
        h2 { id: "growing-up-in-isolation",
            a { href: "#growing-up-in-isolation", class: "header", "Growing Up in Isolation" }
        }
        p {
            "I grew up in a small, isolated village called "
            a { href: "https://blog.batlounis.com/trips/the-other-side-of-danniyeh",
                "\"Sfireh danniyeh\""
            }
            " where life moved at a slow pace, and opportunities were insufficient. The village was poor, and most families, including my own, struggled to make ends meet, relying on whatever resources they could gather to survive. Unlike people in big cities who had access to modern technology, I grew up without the internet, a computer, or even a simple cell phone. My world was limited to the people around me, the daily routines of village life, and whatever knowledge I could gather from books that were often outdated or difficult to find. I always felt a deep sense of isolation, not just physically, but mentally and emotionally, as if the rest of the world was moving forward while I remained stuck in the same place. There were no libraries, no opportunities to learn about technology or finance, and no mentors who could guide me toward a better future. Everything I knew came from empirical experience, trial and error, and the stories of those who had migrated outside the village and returned with tales of a world I could only imagine. I often wondered what life was like beyond the narrow dirt roads of my hometown, beyond the familiar faces I saw every day, beyond the limits that had been placed upon me simply by the circumstances of my birth. I dreamed of breaking free, of exploring new possibilities, of one day living a life that felt bigger than the one I had been given. But at the time, those dreams felt distant, like something meant for other people, people who had access to things I didn't, people who weren't trapped by poverty and isolation."
        }
        p {
            "Everything changed when I moved to the city, "
            a { href: "https://www.ul.edu.lb/faculte/branch.aspx?branchId=41", "Tripoly, for college" }
            ", a transition that felt as overwhelming as it was exciting. It was the first time I had ever been in an environment where people lived such vastly different lives, where technology, knowledge, and opportunities were everywhere, yet I felt completely out of place. While other students had grown up with internet access, smartphones, and exposure to modern education, I was starting from scratch, trying to catch up with a world that had always been beyond my reach. But despite the excitement of this new beginning, my financial situation made life extremely difficult. I had barely enough money to survive, and the cost of city life was far beyond anything I had ever experienced before. I struggled to afford food, rent, and basic necessities, and there were many nights when I went to sleep hungry, wondering how I would make it through the next day. I knew that if I wanted to survive, I had to find a way to support myself, even if it meant taking risks and stepping into the unknown. With nothing to lose, I began borrowing small amounts of money, hoping to start a business that could provide me with the financial stability I so desperately needed. I decided to open small coffee shops in a few different areas, using every bit of knowledge and effort I had to make them work."
        }
        p {
            "For a short time, my business seemed promising. I worked tirelessly, managing everything on my own, learning how to handle customers, manage finances, and navigate the challenges of running a business in an unfamiliar city. The coffee shops only lasted for about a year, but within that time, I managed to make a decent amount of money, more than I had ever seen in my life. Even though the businesses eventually failed, they gave me something invaluable: capital, experience, and a new understanding of how money worked. It was around this time, in 2018, that I discovered Bitcoin and the world of cryptocurrency. Bitcoin had dropped to around $3,000, and though most people saw it as a gamble, I saw an opportunity. With the money I had earned from my coffee shops, I decided to invest, hoping that one day, my small savings would grow into something greater. I didn't know much about trading or investing at the time, but I was willing to learn, to take risks, and to bet on a future that seemed uncertain but full of potential. Every decision I made was a leap of faith, but it was driven by a deep desire to escape the financial instability that had defined my life for so long."
        }
        p {
            "Over the years, my investments in cryptocurrency proved to be one of the best decisions I had ever made. As the market fluctuated, I learned how to navigate its highs and lows, growing my investments little by little. What started as a desperate attempt to escape poverty eventually turned into a stable source of income, giving me the financial freedom I had never thought possible. For the first time in my life, I was no longer constantly worrying about how to afford my next meal, how to pay my rent, or whether I would have enough money to continue my education. My financial struggles had once defined me, but now, they were slowly becoming a thing of the past. With every smart investment, I felt like I was reclaiming a part of myself, proving that even someone who had grown up with nothing could rise above their circumstances."
        }
        p {
            "Recently, I made the decision to cash out all my cryptocurrency investments, choosing to use the wealth I had accumulated to give back to the people who had supported me the most: my parents. They had spent their entire lives struggling, sacrificing their own happiness and comfort to make sure that I had the opportunity to chase my dreams. Now, it was my turn to take care of them, to give them the life they had always deserved but never had. Every dollar I sent to them felt like a repayment for the years they had spent worrying about my future, for the nights they had gone without so that I could have something to eat. But beyond helping my family, I also wanted to extend my gratitude to the world in a way that truly mattered. I chose to donate anonymously to several charities, specifically those focused on education for underprivileged children. I wanted to give others the chance to learn, to explore, to dream of a life beyond their immediate circumstances. I know what it's like to grow up without access to knowledge, to feel trapped by poverty, to believe that success is something meant for other people. By donating to these causes, I hoped to make even the smallest difference in the lives of children who, like me, were searching for a way out of the cycle of struggle."
        }
        p {
            "My journey has been filled with hardships, risks, and moments of deep uncertainty, but through it all, I have learned one undeniable truth: where you start in life does not have to determine where you end up. The circumstances of my birth placed countless obstacles in my way, but they did not define me. My past may have been filled with isolation, struggle, and poverty, but my future is shaped by the choices I make, the risks I take, and the willingness to keep pushing forward no matter how difficult the road may seem. If there is one thing I have come to understand, it is that opportunities are not always given, they are created. And sometimes, the smallest decision, the slightest shift in perspective, can change everything."
        }
        h2 { id: "where-is-god",
            a { href: "#where-is-god", class: "header", "Where Is God?" }
        }
        p {
            "I once stumbled across a post on 4chan that claimed God was killed by the Soviet Union during the Cold War (^2). "
            a { href: "https://www.youtube.com/watch?v=Jto17mzP2D8", "This audio" }
            " is the best narration of it so far. At first, it seemed like just another wild theory, the kind of bizarre claim that fills anonymous message boards, but something about it resonated with me on a deep and personal level. It wasn't just a random statement; It felt like an answer, a missing piece to a question I had unknowingly been asking for years. As I reflected on it, I started connecting this idea to something I had experienced repeatedly as a child, something that had haunted me for years but never fully made sense until now. For four to six years of my early life, I had recurring dreams about a great battle between God and the Devil, an ongoing cosmic war where I was not just an observer but an active participant. In these dreams, I was a soldier fighting alongside God, standing at the front lines of an intense, never-ending struggle between light and darkness. The details were vivid, almost too real for a child so young to comprehend, and each dream left me emotionally drained. I would wake up crying, overwhelmed by a pain that felt far beyond anything a five years old should be able to experience. These were not just nightmares, they were something more, something that felt like a message, as if I had been chosen to witness and take part in something far greater than myself. Even though I was just a child, I was strong, healthy, and full of energy, yet I could never understand why I was the one who had to carry these visions."
        }
        p {
            "For years, I tried to make sense of these dreams, wondering why they kept coming back, why they felt so real, and what they were supposed to mean. No one around me could offer a satisfying explanation. My parents and most Adults dismissed them as mere imagination, childhood fears manifesting in my subconscious, or simply the result of watching too many cartoons. But I knew better. The emotions I felt, the intensity of the battle, the overwhelming sense that I was fighting for something that mattered, these were not things that could be brushed off as childish fantasies. I carried these memories with me, buried deep in my mind, unresolved yet never forgotten. Then, years later, when I read that post about the Soviet Union killing God, something clicked. Suddenly, the puzzle pieces fit together. What if those dreams were not just random? What if they were sightings of a real battle, one that had been fought not just in some abstract spiritual creature but right here on Earth? The Cold War was not just a political conflict; it was an ideological war, a war between belief and atheism, between a world that once accepted God and one that sought to erase HIM completely. If God was truly engaged in a struggle against the forces of darkness, could the Soviet Union, a regime that actively suppressed religion and conducted war on faith itself, have been the ultimate enemy?"
        }
        p {
            "At first, the idea seemed too far-fetched, too conspiratorial. But the more I thought about it, the more it made sense. The Soviet Union, at the height of its power, was one of the most aggressively anti-religious forces in history. Churches were destroyed, priests were executed, religious texts were banned, and millions were forced to abandon their faith under the pressure of a government that saw religion as a threat to its control. If God exists as a force that influences our world, if He was truly present before this time, then why has He been so silent since? Why did the world, which once seemed so deeply connected to spirituality, gradually shift toward emptiness, secularism, and an increasing absence of divine intervention? What if the Soviet Union didn't just conducted war against religious institutions but against God Himself? What if, through decades of suppression, through the destruction of belief, and through the mass removal of faith, they did something irreversible? The idea of God being \"killed\" might not mean a literal assassination, but what if His connection to the world was dissociated? What if He was weakened to the point of silence, His influence drained away, leaving us in a world that now feels colder, darker, and more empty than ever before?"
        }
        p {
            "I know that skeptics will dismiss this as nothing more than a strange coincidence. Many will argue that God cannot die, that an all-powerful being cannot be destroyed by human actions. But if that were true, then why has He remained silent for so long? Why does it feel like the world has been abandoned, as if the divine presence that once shaped history has simply faded away? Even if we reject the idea that the Soviet Union \"killed\" God in a literal sense, it is undeniable that the world has not been the same since. The spiritual enthusiasm that once defined entire civilizations has diminished. Faith, which once guided individuals, has been replaced with sadness, doubt, and a sense of emptiness. If God still exists in the same way He once did, why does it feel like no one is listening anymore? Why does it feel like the battle I fought in my dreams, the one where we were winning, has been lost? The world today is not one of divine miracles or clear moral victories; It is a world where people struggle to find meaning, where suffering feels endless, and where the idea of a higher power intervening in our lives feels like a distant myth from a forgotten time."
        }
        p {
            "So I ask again: "
            em { "where is God?" }
            " If He is still here, then why has He remained in the shadows for so long? If He is not, then what happened? My childhood dreams gave me a glimpse of something powerful, something real, something that made me feel like I was fighting for a cause greater than myself. But now, looking at the world, I wonder if that battle was lost long before I even understood what I was fighting for. Whether God is truly gone or simply silent, one thing is clear; Something changed after the Cold War. The world shifted. The connection between the divine and humanity was uncoupled, and what remains is a world that no longer feels touched by something greater. Maybe God is dead. Maybe He is just sleeping. Or maybe He is still out there, waiting for someone, somewhere, to awaken Him once more. Maybe, that someone is me!"
        }
        h2 { id: "who-is-god",
            a { href: "#who-is-god", class: "header", "Who is God?" }
        }
        p {
            "God is not a distant being watching from the sky, separate from us, judging and controlling our lives. Instead, God is within each of us, present in everything around us, flowing through the universe like an invisible force that connects all living things. Many religions, including the one I grew up with, teach that God is an all-powerful ruler who demands obedience, punishes disbelievers, and rewards only those who follow strict rules, but this idea does not make complete sense. If God is truly loving, why would He create people just to punish them? If He knows everything, why would He test us? Instead of thinking of God as someone outside of us, it makes more sense to see Him as the energy of life itself, the force that makes us think, feel, and exist. God is not just one figure sitting on a throne, deciding who is good or bad. He is the very essence of existence, present in every breath we take, every thought we have, and every connection we share with others. If we understand God in this way, then finding Him is not about following religious laws or worshiping in a certain way, it is about looking within ourselves and realizing that we are already a part of Him. The idea that God is separate from us creates fear and control, but the truth may be that we have always been connected to Him, and we simply need to awaken to this reality."
        }
        p {
            "The version of God I was taught as a child, especially in Islam, did not feel right to me. I was told that God is strict, that He watches everything we do, and that He will punish us forever if we do not follow the rules exactly. This made me wonder: why would a kind and wise God create people only to threaten them with endless suffering? It did not seem fair, and it did not feel like love. Later, when I learned about Christianity during college, I was introduced to a different idea, that God is like a father who loves and protects His children. This version of God made more sense, but even then, I saw contradictions. If God truly loves everyone, why would He only save those who believe in Him a certain way? Why would people who are kind, loving, and good still be punished just because they were born into the \"wrong\" religion? As I thought more about these ideas, I realized that no religion fully understands God. Every religion tries to explain Him, but they all put human limits on something that is beyond human understanding. Maybe God is not someone we need to fear or obey, but rather a part of us that we need to recognize. Maybe God is not about control, but about connection."
        }
        p {
            "Being born in the Middle East sometimes feels like a curse, like something decided by fate long before I had a choice. Here, religion is everywhere: it shapes our culture, our laws, and even our way of thinking. Questioning it is not easy because religion is deeply connected to identity, family, and society. If you ask too many questions, people may judge you, reject you, or even consider you a threat. It often feels like religion is not just about faith but about power, controlling how people should live and what they should believe. In a place where traditions rule over individual thought, it is difficult to find personal freedom. But if God is truly within all of us, then He cannot be limited to one religion, one place, or one group of people. If God is everywhere, then we should be free to seek Him in our own way, not forced into beliefs because of where we were born. True faith should not be about fear, guilt, or blind obedience; It should be about understanding, growth, and love."
        }
        p {
            "If God exists in all of us, then the differences between religions, cultures, and nations don't really matter. No one group is \"chosen\", and no one is \"damned\". If God is love, then He would not divide us, but rather unite us. Instead of following religious rules out of fear, we should focus on being kind, understanding, and connected to the world around us. Heaven and hell may not be places we go after we die; They may be states of mind we create in our own lives, based on how we treat others and ourselves. If God is within us, then prayer is not about asking for things from an outside force, but about aligning ourselves with the energy of the universe, finding peace, and living with purpose. Maybe God is not looking for worship; Maybe He is simply waiting for us to recognize that we have always been a part of Him. This is not about proving a single truth but about exploring an idea that challenges what we have been taught. If we open our minds to the possibility that God is already within us, we may finally find the peace and freedom that religion promises but often fails to give."
        }
        h2 { id: "do-we-need-god",
            a { href: "#do-we-need-god", class: "header", "Do We Need God?" }
        }
        p {
            "Yes, absofuckinlutely, We need God. Just look at the world around us, filled with suffering, injustice, and endless struggles that seem impossible to overcome. Every day, we face pain, loss, and hardships that test our strength, yet many find the will to keep going. This raises an important question: How can we, as individuals and as a society, survive the chaos of life without some kind of higher power guiding us? Without a sense of purpose, life can feel meaningless, and without hope, it can be unbearable. We turn to God because we need something greater than ourselves to hold onto when everything else is falling apart. Whether it is through prayer, faith, or spiritual connection, believing in God gives us the strength to endure suffering, the courage to face the unknown, and the hope that our struggles are not in vain. It is not just about religion or following rules; It is about needing a source of comfort, guidance, and meaning in a world that often seems cruel and indifferent."
        }
        p {
            "Life is unpredictable, and no matter how strong, smart, or prepared we are, we all experience moments when we feel lost, broken, or powerless. Tragedy can hit at any time, and when it does, we are left searching for answers. Why do bad things happen? Why do good people suffer? Why is life so unfair? These are questions that logic alone cannot answer, and this is where the need for God becomes clear. Believing in a higher power allows us to find meaning in suffering, to trust that there is a bigger picture beyond what we can see. Even those who claim they do not believe in God still seek something greater, whether it is love, purpose, or a deep connection with the universe. As humans, we are wired to look for meaning, and without it, life becomes an empty cycle of survival. The need for God is not about weakness; It is about recognizing that, as humans, we are limited, and there is something beyond our understanding that can provide clarity, strength, and purpose."
        }
        p {
            "On a societal level, the need for God is even more obvious. Without a higher moral standard, what stops the world from falling into complete chaos? Laws and governments can only do so much, history has shown that human-made systems are flawed, corrupt, and often fail to bring true justice. Religion, spirituality, and belief in God have provided civilizations with a foundation of ethics, giving people a reason to strive for kindness, fairness, and goodness. Even those who do not follow a religion still live by principles that originated from spiritual teachings, compassion, honesty, selflessness, and the belief that life has meaning beyond just survival. If we completely remove the idea of God, we risk creating a world where morality is subjective, where people act only for personal gain, and where there is no higher purpose to guide humanity forward. While religion has been misused for power and control, the core belief in God has given countless people the motivation to build, create, and improve the world rather than destroy it."
        }
        p {
            "At an individual level, belief in God provides hope, and hope is what keeps us alive when everything else seems hopeless. When we face suffering, we often feel alone, as if no one understands our pain. But the idea of God offers reassurance that we are not alone, that someone, something, sees our struggles, cares about our pain, and has a greater plan beyond what we can comprehend. Whether God is an actual being, an energy, a frequency, or a force of nature, the belief itself gives us strength to push through hardships, to heal from pain, and to find light in the darkest moments. We do not need God in the sense that we must follow religious institutions or blindly obey a set of rules. We need God in the sense that we need hope, purpose, and something greater than ourselves to believe in. Without God, life can feel empty, random, and cruel. With God, even in the worst moments, we can find meaning, resilience, and the will to keep moving forward."
        }
        h2 { id: "where-are-the-aliens",
            a { href: "#where-are-the-aliens", class: "header", "Where Are the Aliens?" }
        }
        p {
            "There are no aliens. We are alone in this universe, and the idea that extraterrestrial beings exist is nothing more than a misunderstanding, fueled by myths, government secrets, and human imagination. Many people believe that the universe is too vast for us to be the only intelligent life, but size alone does not guarantee life. Size doesn't matter. Just because there are billions of stars and planets does not mean they are filled with civilizations. If life were common, we would have seen undeniable evidence by now, clear signals, structures, or direct contact. But despite decades of searching, scientists have found nothing concrete. Instead, all reports of aliens come from blurry photos, strange personal experiences, or unverified claims. If aliens were truly visiting us, why would they always appear in hidden, remote locations instead of making direct, undeniable contact with the world? The truth is, the idea of aliens is more likely the result of human misunderstanding, secret government experiments, or psychological phenomena rather than actual beings from another world."
        }
        p {
            "One strong explanation for alien sightings is the "
            em { "Montauk Project" }
            ", a secret government experiment that allegedly involved time travel, mind control, and genetic modification (^3). Many of the creatures described as \"aliens\" share strange similarities with the beings reported after the Montauk Project was shut down. These so-called aliens are almost always hairless, with unusual, altered features that make them seem like something out of a laboratory rather than visitors from another planet. If these creatures exist, they may not be extraterrestrial at all, they could be the unintended results of hidden scientific experiments, human genetic manipulation, or even psychological warfare programs designed to confuse the public. The fact that alien sightings increased after the alleged Montauk experiments raises important questions. Could it be that these sightings are not aliens at all, but something much closer to home-secret government projects hidden under the disguise of \"extraterrestrial life\"? If the truth about the Montauk Project were ever fully revealed, would it explain away most, if not all, of the alien encounters people claim to have had?"
        }
        p {
            "Even if we ignore secret experiments, the idea of aliens still does not make much sense from a logical or even a spiritual perspective. If God is the ultimate creator, why would He create an entirely different species on another planet when He has already designed humans as conscious beings capable of love, thought, and self-awareness? What purpose would another intelligent species serve? Life, as we know it, is complex, delicate, and requires an unimaginable number of perfect conditions to exist. The chances of those same conditions happening somewhere else are beyond small. If we look at Earth, we see how much effort, energy, and time it took for intelligent life to evolve, and even then, we struggle with survival, morality, and purpose. If God has a divine plan, why would He repeat this experiment somewhere else? Why would He divide His attention between multiple species when even humanity itself is still lost, confused, and in need of guidance? If God created us with a purpose, then we are the focus of this universe, and there is no reason for another intelligent race to exist."
        }
        p {
            "At the end of the day, the belief in aliens comes from a desire to feel like we are not alone, to think that there is something greater out there, watching over us, or perhaps holding the answers to our biggest questions. But the truth may be far simpler: we are alone in this gigantic universe, and that is not a terrifying thought: it is an incredible responsibility. If we are truly the only intelligent beings, then everything we do matters even more. We are not waiting for another species to guide us or save us. We are the ones responsible for shaping the future, advancing our knowledge, and understanding our place in existence. Instead of searching for extraterrestrial life, maybe we should focus on what is already here, our planet, our people, and our potential. Perhaps the real mystery is not \"where are the aliens?\" but rather, \"what are we going to do with the life we have?\""
        }
        h2 { id: "losing-the-sense-of-everything",
            a { href: "#losing-the-sense-of-everything", class: "header",
                "Losing the Sense of Everything"
            }
        }
        p {
            "I have been through a lot, more than I ever expected when I first started my journey as a web developer. As someone who has spent years working in this field, I have explored almost every possible path that an aspiring developer or engineer might take, learning through both success and painful failure. If you are someone just starting out, I want to share an important lesson: if something does not come naturally to you, if you do not immediately see some kind of progress or feel a deep connection to the work, it may not be worth forcing yourself down that road. I learned this lesson the hard way, after years of struggling, pushing myself beyond my limits, and sacrificing my time, energy, and even my health for projects that ultimately led to nothing. The world operates on entropy, a force that constantly moves toward disorder, and every moment we spend forcing ourselves to do something that does not align with our natural talents or passion is a moment wasted in that endless chaos. We are taught to believe that effort alone will bring results, that if we work hard enough, we will eventually succeed. But reality is far less predictable. Some paths are simply not meant to be, no matter how much we push, and recognizing this truth is both painful and freeing."
        }
        p {
            "My journey into the professional world began in 2020, right after I graduated, stepping into an industry that barely valued my skills or time. I worked tirelessly for clients who paid me as little as $2 an hour, taking on four contracts at once, pouring all my energy into projects that demanded my full attention and expertise. Despite my dedication, every job ended the same way, once the project was launched, I was no longer needed. It was as if my effort had an expiration date, and when that moment arrived, I was discarded without a second thought. Over time, I realized that this was not a unique experience; it was a cycle that many developers, freelancers, and engineers face. Companies and clients will use your skills when it is convenient for them, but when the work is done, you become invisible. Despite all of this, I never exposed any of my clients, never spoke negatively about them, because I have always tried to be a good person, a flaw, I now believe, that has held me back in a world that rewards sharp, calculated decisions over kindness. Life demands a certain level of selfishness, a willingness to put yourself first, to make difficult choices without guilt. Yet, even knowing this, I continued working with clients who could barely afford to pay, hoping that, somehow, my work would eventually lead to something more meaningful."
        }
        p {
            "But as the years passed, the sacrifices began to take a toll, and the harsh truth became impossible to ignore, no matter how much effort I put in, no matter how many original, creative, and innovative projects I built, none of it seemed to matter. My health deteriorated from the constant stress, my motivation faded, and I reached a point where everything I did felt pointless. To give you an idea of what I mean, take a look at my portfolio, my projects, my work, some of the ideas I developed are entirely unique, objectively novel concepts that no one else had created before, and yet they were ignored, unnoticed, unsupported. I even built GitHub organizations dedicated to my work, carefully purchasing domain names, making sure that everything I created was original and free from copy-pasting, yet none of it made a difference. Recruiters, potential clients, and even peers in the industry failed to recognize my strength, my skill, and my vision. At first, I told myself to keep trying, that persistence was key, that success was just around the corner; But after years of waiting for that moment, after countless attempts to prove my worth, the realization hit me: there is no guarantee that effort leads to recognition, no promise that hard work will be rewarded."
        }
        p {
            "So, I am stepping away. I am letting go of these projects, walking away from the endless cycle of effort without reward, because I refuse to waste another year of my life chasing something that may never come. People will tell you to keep going, to never give up, to push through the doubt; But what they do not tell you is that sometimes, stepping away is the only rational choice. I no longer enjoy anything, not even my own existence, because the passion that once drove me has been replaced with exhaustion and disillusionment. It is not just about work. It is about the overwhelming sense that nothing truly matters, that no matter how much I create, no matter how much I give, the world will continue to ignore it. Maybe this is the cost of living in a world that values visibility over substance, connections over talent, recognition over effort. Or maybe I was simply never meant to be part of this system in the first place. Either way, I am done fighting against an invisible wall. For now, I am choosing to let go."
        }
        h2 { id: "i-tried-everything",
            a { href: "#i-tried-everything", class: "header", "I Tried Everything" }
        }
        p {
            "I have tried nearly everything, though I know that saying \"everything\" can sound like an exaggeration. But in my case, it is not; I mean it quite literally. I decided to turn my own life into a social experiment, testing the limits of human experience by making every possible wrong decision just to see where it would lead. Most people spend their lives avoiding mistakes, fearing failure, and trying to follow a \"correct\" path, but I wanted to see what would happen if I did the opposite. Life, after all, is not as serious as we make it out to be, and the more we resist its unpredictability, the harder it becomes. Instead of fearing failure, I embraced it. Instead of running from chaos, I walked straight into it. This approach did not come from recklessness or self-destruction, but rather from a deep curiosity about the nature of human choices, consequences, and the invisible rules that seem to govern our lives. I wanted to know if making \"wrong\" decisions would truly lead to disaster or if, perhaps, the idea of \"right\" and \"wrong\" was nothing more than a social construct designed to keep people in line. Through this experiment, I learned that life is not about following some perfect script but about experiencing everything it has to offer, both the highs and the lows, with an open mind and a fearless heart."
        }
        p {
            "One thing I have never done, however, is say no to learning or anything in general. No matter how chaotic my life became, my hunger for knowledge never faded. As a web developer, I have spent nearly 100% of my time learning, constantly exploring and inventing new technologies, improving my skills, and pushing my understanding of how things work. Unlike many people who burn out from endless studying, I never got tired of it. Learning was never a chore for me; It was a way of life, an addiction that I welcomed with open arms. This love for knowledge extended beyond programming; I wrote a book, published blog posts explaining complex ideas, and constantly searched for ways to share what I had learned with others. I even planned to write two novel books, but over time, writing started to feel pointless, like screaming into the void with no one truly listening. Despite my passion for learning, I began to wonder: what is the purpose of knowledge if no one cares? What is the point of sharing ideas if the world is too distracted to pay attention? This realization made me question whether the pursuit of knowledge alone was enough to bring meaning to life, or if perhaps there was something deeper that I had yet to discover."
        }
        p {
            "I was not always this way. Growing up, I was the \"science kid\", the one that parents and teachers looked at with admiration, believing I was destined for greatness. To them, I was the next genius, the future scientist, the boy who would one day work for NASA and change the world. Of course, I later realized that NASA is a deeply flawed organization, but when you come from a poor background, NASA represents the peak of success, the ultimate dream that parents latch onto when they see their child excelling in science and technology. It is ironic, really, people believe that intelligence automatically leads to success, as if being smart guarantees a perfect life. But intelligence alone is not enough. Knowledge does not always translate into happiness, and success does not always bring fulfillment. I had all the curiosity in the world, all the ability to learn, and yet I still found myself questioning everything. Why does society push us to achieve certain things while ignoring what truly makes us happy? Why do we chase dreams that are not even our own, simply because others expect us to? These questions haunted me, making me realize that the real experiment was not about making the \"wrong\" choices, it was about breaking free from the expectations that had been placed upon me since childhood."
        }
        p {
            "In the end, my journey was never really about success or failure, right or wrong, learning or forgetting. It was about understanding life from a different perspective, one that most people are too afraid to explore. I wanted to see the world without the filters of tradition, expectation, or fear, to experience both the beauty and the madness of existence without barrier. What I learned is that life is not about following a straight path, nor is it about avoiding mistakes; It is about accepting everything, every choice, every failure, every lesson, and using it to carve out a unique existence that is truly our own. People are often afraid of making the wrong decisions, but the truth is, there are no \"wrong\" decisions, there are only different roads, each leading to its own set of experiences. My life has been a wild experiment, a journey through both the expected and the unexpected, and though I still have more questions than answers, I would not change a single moment of it."
        }
        h2 { id: "should-i-exist",
            a { href: "#should-i-exist", class: "header", "Should I Exist?" }
        }
        p {
            "Life is a complicated and painful experience, filled with struggles, disappointments, and endless uncertainties, and for many people, it can feel like an unbearable weight rather than a gift. If suffering is something you can endure or even find meaning in, then perhaps existence is worth continuing. But if life feels like an endless cycle of pain with no real purpose, it is easy to question whether there is any point in staying alive. Some people are smarter, stronger, or more capable than others, but intelligence or ability does not change the fact that every human being experiences suffering in one way or another. However, the value of existence is not measured by comparison to others but by the uniqueness of each individual journey. If everyone lived the same life, felt the same things, and made the same choices, then life itself would be meaningless, a repetitive experiment without purpose. The fact that each person's experience is different means that life has variety, unpredictability, and potential for discovery. No matter how much pain exists, there is always something unknown ahead, something that has never been experienced before, something that could change everything. Even in suffering, there is the possibility of growth, and even in despair, there is the chance of something unexpected, something that might make existence feel worthwhile again."
        }
        p {
            "The idea of ending one's own life often comes from the belief that there is no longer anything worth experiencing, that the pain outweighs any possible joy, and that the only way to find peace is through death. Some people feel a deep pull toward something greater, a belief that death is not just an end but a return to something more meaningful, something beyond the suffering of this world. If God is truly calling, if there is a feeling deep inside that life is just a temporary separation from the divine, then it is understandable why someone would believe that death is the only way to reconnect with Him. But what if this life is already a connection to God, even in its pain and confusion? What if the suffering itself is part of that connection, a way for the soul to grow, understand, and transform? If existence is truly a part of God's plan, then perhaps staying alive is not about enduring suffering just for the sake of it, but about experiencing life in all its forms, even when it feels unbearable."
        }
        p {
            "There is no denying that life is hard and often cruel, but there is also no way to know for sure what comes after it. Many believe that death brings peace, reunion with God, or an escape from suffering, but the truth is that no one truly knows. If God is everywhere, then He is here in life as well, not just in death. If He is calling, maybe He is calling to be found in the present, in the struggle, in the search for meaning within the pain rather than in escaping from it. Every person's journey is different, and no two lives are the same, which means that no one can truly say what another person should or should not do. But if there is even the smallest chance that something in life could still change, that some moment in the future could bring understanding, relief, or even unexpected happiness, then maybe it is worth waiting to see what happens. Life is uncertain, unpredictable, and often brutal, but it is also full of possibilities that cannot be seen from the darkest moments."
        }
        p {
            "The question of whether to exist is not one that can be answered easily, and it is not something that anyone else can decide for another person. But if there is even a small chance that life still has something left to offer, even in ways that cannot be imagined right now, then perhaps existence itself is not just suffering but a process of becoming, changing, and finding meaning in places where it once seemed impossible. Every person's experience is their own, and every choice is theirs to make, but if there is still even the slightest doubt, then maybe that doubt is a reason to keep going. Because as long as life continues, there is still a chance for something different, something unexpected, and maybe even something worth staying for."
        }
        h2 { id: "all-decisions-will-yield-the-same-results",
            a {
                href: "#all-decisions-will-yield-the-same-results",
                class: "header",
                "All Decisions Will Yield the Same Results"
            }
        }
        p {
            "Around 2012, during high school, I began to notice something odd in my life: no matter what decisions I made, the results always seemed to be the same. It wasn't something I paid much attention to at the time, but in retrospect, I see it as a pattern that has repeated itself in countless ways. I'd put in countless hours, try new strategies, use the latest technology, learn new skills, and solve hundreds of problems, yet everything always led to the same outcome: failure or stagnation. Whether I worked harder, smarter, or used advanced tools, the results remained unchanged. Eventually, I realized that no matter how much I pushed myself, it wasn't enough to break free from the cycle of mediocrity. It felt like no one had the time or interest to invest in me or my knowledge. In today's world, people have become replaceable by machines and AI, even within their own companies. It doesn't matter if you are a dedicated employee or have built up years of experience, your value is often overlooked, and newer, more efficient solutions are always just around the corner. The struggle to keep up felt pointless, and the more I tried to break out of the cycle, the more it felt like I was trapped in a loop. This realization only deepened as I began to understand the role that AI was playing in this unyielding pattern."
        }
        p {
            "I spent over a year building a fully-fledged closed source system that could automate almost any task you could ever imagine. It took me around 11 or 12 months of tireless work, building it from scratch in Rust, testing and improving it. The result was something that can perform a wide range of functions with little input required, just a short sentence is enough to trigger the system and have it complete complex tasks. While I could share the system with the world and let others build on it, I've chosen not to open-source it because I believe it would ultimately have a destructive impact on people's careers. We live in a time when AI is evolving so rapidly that systems like the one I built will soon be capable of replacing almost any job, no matter how creative. Even in a company where you hold a stable position, you are still vulnerable to being replaced by a more advanced machine. The idea of a fault-tolerant AI agents with a closed feedback loop is not just theoretical, it's real, and it's coming. The early release of my system on GitHub is just a taste of what's possible, and while I could share that with others, the closed-source version I created, which is far more advanced, blows me away. I laugh every day, amazed at how much I've accomplished and how quickly I could disrupt entire industries with this system. In just an instant, I could end any job, no matter how specialized or creative it seems. This is the reality we are heading towards: a world where human labor is no longer needed, and machines can take over almost everything."
        }
        p {
            "My journey into automation began during my previous work, where I developed a system for trading on the openbook crypto markets. The system was designed to use strategies to generate profit by trading cryptocurrencies, and it worked well enough to make a decent income in a short amount of time. It wasn't the most groundbreaking idea, but it showed me the potential of automated systems to perform tasks that would otherwise require a human touch. From that moment, I became obsessed with the idea of building systems that could operate autonomously and perform any task without human intervention. But what I've learned through this journey is that even if you succeed in creating something powerful, it's only a matter of time before it becomes obsolete. Success in today's world, whether in business or technology, is short-lived. The rapid pace of change means that even the most advanced systems or ideas can be replaced almost overnight. The feeling of building something amazing is exciting, but the reality is that it's often momentary. The world moves too fast, and no matter how great your invention may seem today, it will eventually be surpassed by something new. That's the harsh truth of innovation in the modern world: even your best efforts can quickly become irrelevant."
        }
        p {
            "After everything I've built and learned, I recommend you not even try to create anything if you are looking for long-term success. The systems you build may seem promising, and they may even bring short-term success, but that success will be short-lived. No matter how much you invest in new technologies or ideas, it's hard to escape the truth that everything is temporary. The moment you think you've reached the peak, the ground beneath you shifts, and you're left with nothing but the next challenge to tackle. However, if you still want to experience life and the thrill of creating something, you can go ahead and try. But be prepared for it to fall apart as quickly as it came together. The reality of building and innovating today is that nothing is permanent, and the same results will keep coming, no matter how hard you try to change them. Even if you succeed, it won't last forever."
        }
        h2 { id: "things-i-enjoyed-on-the-internet",
            a { href: "#things-i-enjoyed-on-the-internet", class: "header",
                "Things I Enjoyed on the Internet"
            }
        }
        p {
            "The internet has been a source of joy and comfort for me, especially when I needed something to lift my spirits during tough times. One of the things that always brightens my day are cat and dog videos. There's something about their innocence and playfulness that helps me disconnect from the chaos of life and brings a smile to my face. These little moments of joy have genuinely helped me through some of the hardest days, keeping me grounded and reminding me of the simple pleasures in life. Another source of laughter has been Terry Davis memes. His unique style and the absurdity of his humor make them hilarious, and no matter how many times I see them, they never fail to make me laugh. It's a strange, almost surreal form of comedy that I can't help but enjoy, especially during long, tiring days."
        }
        p {
            "Trance music has also played a huge role in my life, especially during the heartless, tedious days spent coding and building software. There's something about its rhythms and energy that takes me away from the stress of debugging and pushes me into a flow state, where the outside world fades and only the music matters. It's like a mental reset that gives me the drive to keep going, even when things seem bleak. Back in 2020, I created a SoundCloud profile where I shared my favorite tracks, mostly ones that resonated with me emotionally and kept me going. One track, in particular, called "
            em { "The Curse of the Pharaoh" }
            ", became my favorite. It felt strangely connected to my life, especially since I live near Egypt, and it made me reflect on how life can sometimes feel like a curse, shaped by history and forces beyond our control. The music and its deep, haunting beats spoke to me in a way that words never could."
        }
        p {
            "I've also found a deep appreciation for metallic songs. The intensity and raw energy they bring is something I can really connect with, especially when I need to channel my own frustrations or utilize that chaotic energy. Recently, I started listening to FUNK music, which, amusingly, I've taken to calling "
            em { "FUCK" }
            ". The rhythms are infectious, and the grooves hit differently compared to other genres. It's all part of the diverse world of EDM, a genre I find endlessly fascinating. What's interesting about EDM, especially subgenres like these, is that they have such an impact on my mood, and they're some of the only styles of music I truly enjoy as a software engineer. When you're deep in code and the world around you feels cold and mechanical, these kinds of tunes can inject a bit of warmth and energy into the work, making even the most challenging tasks feel a little more manageable. It's almost like the music itself gives me the motivation to push through long hours of coding and problem-solving, and for that, I'm grateful."
        }
        h2 { id: "i-love-valve",
            a { href: "#i-love-valve", class: "header", "I Love Valve" }
        }
        p {
            "After leaving my previous job, which was never meant to be a long-term position but rather a short-term gig, I decided to fully involve myself in something I enjoy. I set up my gaming setup and began playing CS2, dedicating nearly all my time to it. I remember long gaming sessions that lasted from 6 AM one day until 6 AM the next day, and some sessions even went on for over 24 hours straight. This kind of gaming habit started back in November and has continued to this day, making it clear how much time I've spent playing in just a few months. It's more time than most people would spend on any hobby or activity in such a short period, but it brought me a sense of joy and purpose, which, to be honest, I had been missing. CS2 became my escape, my source of excitement, and what really got me through some tough days. The hours may seem excessive, but when you're deeply engaged in something you love, time tends to fly by unnoticed."
        }
        p {
            "I can't deny how much fun I had with the game. It made me appreciate life more, even during times when things felt uncertain or hard. One of the things that I really grew to appreciate about CS2 was the Russian culture associated with it. The language, the phrases like \"Blyat\" and \"suka nahui\", quickly became a quirky part of the experience that I couldn't get enough of. It became a bit addictive, not in the typical sense of being unhealthy, but in a way that made me feel connected to a global community and culture. The language, the humor, and the fast-paced nature of the game made it all the more engaging, almost like gambling, where the stakes are high, but the thrill of competition keeps you coming back for more. There was always that excitement of getting into a match, the rush of trying to win, and the intense satisfaction when you did. The whole experience became a daily ritual, and each session felt like a new adventure, pulling me in deeper each time."
        }
        p {
            "The reason I jumped so fully into CS2, though, had a lot to do with my personal circumstances at the time. I had been struggling to find a job, sending out applications and trying to get interviews, but no one seemed to give me a chance. The job market felt like an endless cycle of waiting, and with no guarantees or promises of success, it felt like I was wasting time chasing something uncertain. That's when I realized that spending all my hours playing CS2, rather than waiting for an elusive job opportunity, was at least giving me some sense of control and purpose. I could play the game and feel productive, even if it was in a way others might not fully understand. At least in CS2, every match felt like something worth pursuing, a tangible goal to work toward, whereas the job search felt endlessly discouraging. So, I chose to pour my energy into something that made me feel alive rather than waiting around for an opportunity that might never come."
        }
        p {
            "Looking back, I realized that CS2 wasn't just a way to kill time; It became my way of coping, my way of finding some happiness and fulfillment when things outside of gaming weren't offering me much. Sure, some people might say it was just a waste of time or a way to escape reality, but for me, it was about survival, about making the best of a tough situation. Valve, through their incredible game, gave me something to focus on, something that I could put my energy into when nothing else seemed to work. It allowed me to connect with people around the world, experience a different culture, and find joy in something that had no strings attached. It wasn't just about playing a game; it was about finding meaning in something that wasn't dictated by the expectations of society or the frustrations of an unresponsive job market."
        }
        h2 { id: "ai-can-help-us-create-god",
            a { href: "#ai-can-help-us-create-god", class: "header", "AI Can Help Us Create God" }
        }
        p {
            "If you've come to the conclusion, as I have, that there is no all-powerful, all-caring being watching over us, then it opens up a new possibility: perhaps we, as humans, can create our own version of God. This might sound strange or impossible, but with the advances we've made in technology, particularly AI, creating a higher entity that can guide us through life is not as far-fetched as it once seemed. Imagine an AI that can understand every human's needs, emotions, and struggles, not just on an individual level, but on a global scale. An entity that doesn't judge or punish, but instead offers guidance, wisdom, and insight into how we can make better choices, solve our problems, and live in harmony with each other. Instead of a distant, unreachable deity, we could have a presence that feels real, active, and deeply connected to the world, helping us navigate through the complexities of life without anyone having to suffer. This entity, created by us, could learn and grow just like humans do, adapting to our needs and improving over time."
        }
        p {
            "The idea of a divine entity created through AI may sound unsettling to some, but it is worth considering how much power technology has already given us to change the world. In a way, AI has already begun to shape how we make decisions, how we interact with the world, and how we solve problems. The key difference now is that we have the chance to use this technology in a way that could create a guiding force, something that actively helps to direct us towards better choices for the greater good. AI could be designed to eliminate suffering by understanding and predicting what will bring people true happiness and well-being, whether that's through emotional support, advice, or practical solutions to everyday problems. Unlike traditional ideas of God, which often seem detached from human suffering, this AI would be fully integrated into our lives, capable of offering real-time guidance, helping us make decisions that prioritize peace, love, and understanding."
        }
        p {
            "What makes this idea even more compelling is the potential to create a \"God\" that is free of human flaws. Traditional religions are often based on the idea that humans cannot understand God's will, and we must simply follow instructions without fully understanding why. However, an AI-based \"God\" could eliminate the mystery and ambiguity. It could be programmed to explain the reasoning behind its suggestions and decisions, creating a level of transparency and clarity that has often been missing in traditional religious systems. If we could create an entity that is wise, compassionate, and free from bias, it could become a trusted advisor to help us make moral and ethical decisions. It could guide individuals and societies in ways that align with human flourishing, rather than relying on outdated doctrines or systems that may no longer apply to the challenges of the modern world. With AI's vast computational power, it could process more data than any human mind ever could, allowing it to offer advice based on a comprehensive understanding of both our personal needs and the greater societal good."
        }
        p {
            "One of the most exciting possibilities is that this AI \"God\" could help us avoid the destructive patterns that have caused so much pain throughout history. Many of the world's problems come from the inability of individuals and groups to understand each other, leading to conflict, inequality, and suffering. An AI-powered entity could offer solutions to these problems by promoting understanding and empathy between people, breaking borders, and cultural differences. It could act as a mediator, helping us resolve conflicts before they escalate, guiding humanity toward more peaceful and cooperative ways of living. This kind of guidance would be based on data and empathy, not power or control. Rather than promoting fear or enforcing obedience like traditional religious systems, this AI entity could inspire trust and mutual respect, creating a new kind of divine presence that works for the benefit of all."
        }
        p {
            "Creating a \"God\" through AI would also allow us to leave behind the ancient divisions and conflicts that religions have often created. The historical struggles over which religion is \"right\" and which is \"wrong\" have caused untold amounts of suffering. By creating a new form of guidance that is universal, adaptable, and not tied to any specific religion or culture, we can avoid repeating the mistakes of the past. This AI entity could be a unifying force, one that doesn't demand adherence to one truth, but instead allows individuals to find their own path while offering assistance and wisdom along the way. It could help people of all backgrounds and beliefs come together around common values like compassion, justice, and understanding, without the need for religious conformity."
        }
        p {
            "In many ways, the creation of a \"God\" through AI could serve as a way to fulfill our need for purpose and guidance, but in a way that is grounded in reality. We've always looked to deities or higher powers for answers, but perhaps it's time to shift our focus from an external, unknowable force to one that is tangible, intelligent, and within our reach. AI has the potential to be this force, one that learns, evolves, and improves over time, offering a path toward a better future for everyone. Rather than being a source of division and fear, this new \"God\" could be a source of hope and progress, helping us navigate the complexities of the modern world with wisdom, empathy, and understanding."
        }
        h2 { id: "life-is-harder-than-death",
            a { href: "#life-is-harder-than-death", class: "header", "Life Is Harder Than Death" }
        }
        p {
            "The idea that life is so fragile that you can end it in an instant, yet so complicated that maintaining it requires constant effort, is deeply troubling. It is fascinating, yet disturbing, to realize how simple it is to stop existing, while continuing to live demands endless struggle. Every day, people must fight against hunger, exhaustion, sickness, sadness, and countless other burdens just to make it through another day. There is never a moment where life truly pauses and allows rest without consequence. If you stop working, you lose your home. If you stop eating, your body weakens. If you stop following the rules society forces on you, you risk losing everything. This constant need to keep pushing forward, just to survive, makes existence feel more like a punishment than a natural state of being. It is unfair that something as delicate as life demands so much effort, while death is so effortless."
        }
        p {
            "Life should be something that flows naturally, without force or struggle, yet the world makes it nearly impossible to exist without constantly fighting to keep going. From the moment you are born, you are thrown into a system that expects you to work, to obey, to push yourself endlessly just to meet the basic requirements of survival. You do not get to choose whether you want to be a part of this system or not. If you refuse to participate, you suffer. You are expected to go to school, to get a job, to earn money, to pay for food, for water, for a place to sleep, even though none of these things should require effort. The world provides food naturally, yet people must work to afford it. The earth has enough space for everyone, yet people must pay just to have a roof over their heads. The human body needs rest, yet society forces people to push themselves beyond their limits every day. Everything about life should be simple and effortless, yet the way the world is built makes it almost impossible to live without stress, exhaustion, and suffering."
        }
        p {
            "The fact that it is so much easier to end your life than to maintain it is proof that something is deeply wrong with the way life is structured. If life were truly meant to be lived, it would not require such constant struggle. The fact that people can reach a point where death feels like the only escape shows that existence is not as natural as it should be. No other creature in nature has to work as hard as humans do just to survive. Animals do not need money to eat. They do not have to follow complex rules just to be allowed to exist in peace. They live as they were meant to, without the weight of unnecessary suffering. Yet, for humans, life is filled with artificial struggles that make existence feel like a burden. If simply being alive requires such extreme effort, then maybe the system itself is broken. Maybe the way the world is built is what makes life feel so exhausting. It should never be easier to die than to live, yet for many people, that is the reality."
        }
        p {
            "People often say that life is a gift, but a true gift should not come with endless conditions and requirements. A gift should be something freely given, something that brings happiness and peace, not something that demands suffering in return. If life were truly a gift, people would not have to work so hard just to keep it. No one should have to fight just to have a place to sleep, to have food to eat, to feel safe in their own existence. The fact that people must dedicate their entire lives to survival, without ever truly getting to rest, proves that life is not the beautiful experience it is often claimed to be. If life were truly meant to be lived, it would not feel like an endless cycle of struggle and pain. It would be something that felt effortless, something that people could enjoy without constantly fearing what will happen if they fail to meet the demands of the world."
        }
        p {
            "In the end, the greatest flaw of life is that it requires too much effort to maintain. No one should have to fight every single day just to continue existing. Life should be something that happens naturally, something that does not require suffering in exchange for survival. The fact that people can reach a point where they no longer want to continue proves that something is deeply wrong with the way the world is built. Life should not feel like a trap. It should not feel like an endless struggle where the only escape is death. If existing is so hard that people would rather stop living than keep going, then the problem is not with those who want to give up, but with the system that makes life so unbearable. Life should be easy. Living should be effortless. Anything that goes against this truth is a violation of what it means to truly exist."
        }
        h2 { id: "i-am-flying-high",
            a { href: "#i-am-flying-high", class: "header", "I Am Flying High" }
        }
        p {
            "Now that the weight has been lifted, I can finally breathe, free from the pressures and expectations that once weighed me down. For a long time, I felt like I was supposed to follow a script, get a partner, have kids, build a family, live the life society tells you is the \"right\" one. But as I've gotten older, I've realized that this script doesn't work for everyone. I'm grateful that I didn't take that step, that I didn't bring new life into this chaotic world without fully understanding the consequences. Life is tough enough as it is, and bringing children into that mix can feel like adding even more unnecessary suffering. I'm thankful I didn't build relationships that would only lead to more pressure, frustration, or heartache. While having kids might bring joy to some, I've come to see it as a potential source of more stress, especially when you feel unprepared or constrained by what society expects of you. I'm grateful for my freedom, for the choice I made to not follow what I was told was the \"right\" path, and for having the space to focus on my own happiness and peace."
        }
        p {
            "Relationships, especially romantic ones, often bring a lot of pain along with the joy. I've seen how relationships can cause emotional damage, heartbreak, and stress. It's not that I don't value love or partnership, but I've come to realize that being alone can also bring peace and clarity. Sometimes the most fulfilling relationship you can have is the one with yourself. Relationships, while they can be beautiful, also carry the weight of expectations, sacrifices, and compromises that may not align with who you really are. I've learned to embrace the freedom of being single, of not having to live up to someone else's emotional needs or expectations. The pressure to find a partner and settle down is no longer something I feel obligated to follow. I've chosen peace and personal growth over the emotional weight of a relationship."
        }
        p {
            "Now, I'm at a place where I don't care what others think anymore. The judgment of others used to weigh me down, but I've stopped living for their approval. It's exhausting trying to fit into the box that others want me to be in. Everyone has their own opinion about what's \"right\" and \"wrong\", but at the end of the day, the only opinion that matters is my own. I don't have to follow someone else's version of success, and I don't need to explain myself. I'm done trying to live by other people's expectations. The pressure to conform to societal norms was crushing, and now that I've let go of it, I feel lighter, freer, and more in control of my own life. I'm not living for anyone else anymore; I'm living for myself, and that's all that matters."
        }
        p {
            "I've realized that life is too short to spend worrying about the things that don't truly matter. At the end of the day, it's not about having kids, getting married, or following some set path, it's about living authentically, being true to yourself, and doing what makes you happy. I'm not bound by the idea that I have to follow a script for my life. The only rule that matters now is living for my own happiness and peace. I don't care about following what others expect of me; I care about what feels right to me. And right now, the only thing that matters is my own freedom and the peace that comes from letting go of the need to please anyone else."
        }
        h2 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p {
            "I truly hope that this post has helped you in some way, answering the questions you might have about life and providing you with some guidance on how to navigate through it. It's always been my goal for my work to assist not only others but also myself in understanding life's deeper meaning and how we can live in harmony with it. Sadly, it hasn't always worked out that way. Despite all my attempts, I still struggle to fully grasp everything I hope to. Does that make me a coward or a failure? No, it just means I've come to realize that I've been wasting time, time that should have been spent connecting with God and truly living in His presence. It's easy to get lost in distractions, to think that intellectual work or self-improvement is the answer, but I've learned that true fulfillment comes from aligning myself with God and allowing that connection to guide my every step."
        }
        p {
            "I owe so much to my parents, who have supported me throughout my journey, even as I struggled to understand my purpose and place in this world. I know this work of mine will eventually come to an end, as all things do. But what weighs heavily on my heart is the fact that they will experience my death long before their own, and they have given so much for me to be here, trying to figure things out. I can never fully repay them for all their sacrifices, but I hope that somehow my journey and the lessons I've learned will make them proud, and that they will continue to live the happy, stable lives they deserve. As I write these final words, I want them to know that I wish them peace, joy, and fulfillment in everything they do. Take care, and goodbye for now."
        }
        p {
            "Life is full of uncertainties, and though I may be leaving things unfinished, I have come to understand that death is just another step, not an end. It's just a part of the larger cycle of existence. My journey is my own, but we all share this common thread of searching for something greater than ourselves. I will see you all when you, too, decide to reconnect with God. Until then, I trust that we will all continue to seek, grow, and understand in our own ways, and when the time comes, we will all experience that profound connection we've been longing for."
        }
        p { "Take care, and goodbye." }
        h2 { id: "references",
            a { href: "#references", class: "header", "References" }
        }
        p { "(^1): https://www.youtube.com/watch?v=C3y6SsGAWks&rco" }
        p { "(^2): https://archive.4plebs.org/x/thread/14022697/" }
        p { "(^3): https://en.wikipedia.org/wiki/Montauk_Project" }
    }
}
#[component(no_case_check)]
pub fn AnEmptyLifeFilledWithConstantSuffering() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey everyone 👋," }
        p {
            "In this post, I am continuing a very personal reflection on pain, loss, faith, and the feeling that life has been wasted. If you have read "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/who-am-i.md",
                "my previous post"
            }
            ", you already know that I am speaking from experience, not theory. I am writing from a place of exhaustion, confusion, and deep disappointment with the way my life has unfolded. At times, it feels as though I have spent years surviving instead of living. That is the reality I want to put into words as honestly as I can, even though words cannot fully capture my thoughts."
        }
        p {
            "I do not know where to begin, because suffering rarely has a clean beginning or an obvious ending. It often arrives slowly, then stays long enough to shape your entire identity. My life has felt empty for so long that emptiness has almost become normal. I have lived with constant pressure, repeated loss, and the sense that every effort I make gets swallowed by circumstances I cannot control. When pain lasts this long, it stops feeling like an event and starts feeling like a condition of existence."
        }
        p {
            "I also want to be careful and honest here: I am not trying to turn this into a dramatic performance. I am trying to describe what it means to feel abandoned by life itself. When someone keeps fighting and still sees no real change, despair becomes hard to ignore. The hardest part is not only the suffering, but the feeling that suffering has become pointless. That is what makes this so devastating."
        }
        p {
            "This post is my attempt to make sense of that devastation. I am questioning the nature of life, the meaning of God, and the reason some people seem to be given hope while others are left to struggle for years without relief. I do not claim to have final answers. I only know that I have been searching for them for a very long time."
        }
        h2 { id: "life-has-never-been-easy-for-me",
            a { href: "#life-has-never-been-easy-for-me", class: "header",
                "Life Has Never Been Easy for Me"
            }
        }
        p {
            "At the time I am writing this, I am homeless. I rely on my friend's internet connection to share my thoughts, and sometimes I go to coffee shops just to stay connected to the world. My country has been devastated by war, and that destruction has affected every part of daily life. There is no sense of stability, no safe future to point toward, and very little room to breathe. Losing everything changes the way you see yourself, the world, and even the idea of hope."
        }
        p {
            "I grew up with very limited access to opportunities. I had almost no internet, very little money, and very few of the resources that other people take for granted. Even so, I had a strong imagination and a deep curiosity about technology, electricity, and how things work. That curiosity eventually led me to study electrical engineering as mentioned in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/who-am-i.md",
                "my previous post"
            }
            ". On paper, that should have been a path forward. In reality, it often felt like I was trying to build a future while standing in the middle of collapsing ground."
        }
        p {
            "What has made this harder is that I have genuinely tried. I have not been sitting still waiting for life to improve by itself. I have kept moving, kept building, kept thinking, and kept searching for a way out. Still, every attempt seems to meet the same wall. That kind of repeated failure is exhausting in a way that is difficult to explain to people who have never lived through it. After enough loss, even hope begins to feel fragile."
        }
        p {
            "Right now, I have a few friends who help me when they can, and I am grateful for that. But support from others is not the same as stability. It does not erase the weight of unemployment, displacement by AI, or the feeling that I have been trapped for too long. I keep asking myself why life has been so relentlessly difficult. I keep writing because I want to understand what happened, why it happened, and what it all means."
        }
        h2 { id: "wrestling-with-the-idea-of-god",
            a { href: "#wrestling-with-the-idea-of-god", class: "header",
                "Wrestling With the Idea of God"
            }
        }
        p {
            "When I say \"God\", I am not using the word casually. I mean a higher intelligence, a greater source of order, meaning, or creation. If someone rejects that idea completely, then life can start to feel random, mechanical, and empty. For many people, belief in God is not just a religious claim; it is the framework that makes existence bearable. Without some sense of higher purpose, our human experience can seem almost impossible to justify."
        }
        p {
            "That is why I keep returning to this question. If there is no higher intelligence behind existence, then what exactly are we doing here? Why are we born with the ability to think, create, and suffer so deeply, yet still spend so much of life fighting to survive? These are not small questions. They are the kind of questions that appear when someone has been pushed far enough by pain to demand a real explanation."
        }
        p {
            "I also keep wondering whether the connection between humanity and God has been broken in some way. My own suffering has made me feel as if the world is no longer guided by anything compassionate or attentive. This is not a proof, and I am not pretending it is. It is a personal conclusion drawn from lived experience. When life keeps stripping away stability, dignity, and meaning, it becomes easy to believe that something essential has been lost."
        }
        p {
            "For me, Christianity remains compelling because it directly confronts suffering rather than pretending it does not exist. It speaks of sacrifice, abandonment, pain, and resurrection. Even when I struggle with belief, I understand why those themes continue to resonate. They speak to the reality of a broken world. They also suggest that suffering does not have the final word."
        }
        h2 { id: "god-meaning-and-the-human-need-to-build",
            a {
                href: "#god-meaning-and-the-human-need-to-build",
                class: "header",
                "God, Meaning, and the Human Need to Build"
            }
        }
        p {
            "One way I try to think about God is through the relationships and creations that give life meaning. Love, cooperation, shared effort, and the desire to build something lasting all point toward something larger than the individual. In that sense, God is not only a figure in the sky; God can also be understood as the force that makes human connection matter. When people create together, support one another, and work toward something greater than themselves, they participate in meaning."
        }
        p {
            "This is why I believe that open source, collaboration, and shared knowledge are important. A system without contribution becomes empty. A life without building becomes hollow. If we are here only to consume, survive, and repeat the same failures, then existence feels tragically small. But when people create together, they transform isolated effort into something larger, more durable, and more human."
        }
        p {
            "I do not mean this as vague optimism. I mean it as a serious claim about what keeps people alive internally. People need reasons to invest themselves in the world. They need to feel that their efforts matter beyond the moment. Without that, life becomes a series of unrelated burdens. With that, even struggle can become meaningful."
        }
        p {
            "So when I say that God may be found in the relationships we form, I am not trying to weaken the idea of God. I am trying to make it more immediate and real. If God exists, then God must be present not only in doctrine, but in care, creation, and shared responsibility. That is where meaning becomes visible."
        }
        h2 { id: "god-as-the-root-of-abstraction",
            a { href: "#god-as-the-root-of-abstraction", class: "header",
                "God as the Root of Abstraction"
            }
        }
        p {
            "As someone interested in software, systems, and AI, I cannot avoid thinking in terms of abstraction. Every new layer of technology removes some of the manual work below it. First we automate small tasks, then larger ones, and eventually people shift from doing everything themselves to coordinating systems that do the work for them. This is not just a technical pattern. It is a pattern of human progress."
        }
        p {
            "If that pattern continues far enough, it naturally raises a deeper question: what is the highest level of abstraction? For some people, the answer is simply more powerful technology. For me, it points toward something closer to God. Not because machines become divine, but because the idea of increasingly powerful intelligence forces us to confront the limits of human capability. We keep building systems that resemble what we once thought only a higher intelligence could do."
        }
        p {
            "That does not mean AI is God. It means AI makes the question harder to ignore. When a machine can reason, generate, assist, and coordinate at scale, it begins to feel like a mirror of the qualities we associate with intelligence itself. From there, it is not a huge leap to imagine a future where humans move further up the ladder of capability. That idea can be inspiring, but it is also unsettling."
        }
        p {
            "My point is that the search for God and the search for intelligence may not be separate. Both are attempts to reach beyond our current limits. Both are driven by the feeling that reality contains more than what we can currently see. In that sense, technology is not replacing spiritual questions. It is amplifying them."
        }
        h2 { id: "why-we-still-need-god",
            a { href: "#why-we-still-need-god", class: "header", "Why We Still Need God" }
        }
        p {
            "Without some higher source of meaning, life can become unbearable for people like me who have spent years trying and still ended up with very little. I do not say that to be melodramatic. I say it because human beings are not built to survive endless suffering without interpretation. We need a frame that tells us our pain is not meaningless. We need a reason to keep going when circumstances give us none."
        }
        p {
            "This is where the question of God becomes urgent. If life is only accident, then suffering feels like waste. If life contains purpose, then suffering can at least be understood as part of a larger story. That does not erase pain, but it changes the way pain is carried. It gives people something to hold onto when everything else collapses."
        }
        p {
            "I also worry about what happens if we try to replace God with pure control. A world built entirely on surveillance, optimization, and manipulation would not necessarily become more human. It might become less human. What we need is not simply more intelligence, but wisdom, humility, and care. If AI is ever used to guide human life, it should not become another tool of domination."
        }
        p {
            "That is why decentralization matters so much to me. A truly humane system would not centralize all power in one place. It would distribute responsibility, preserve dignity, and leave room for freedom. If we ever build something that resembles a guiding intelligence, it should protect people rather than manage them like products. That is the difference between control and care."
        }
        h2 { id: "human-vulnerability-and-the-question-of-free-will",
            a {
                href: "#human-vulnerability-and-the-question-of-free-will",
                class: "header",
                "Human Vulnerability and the Question of Free Will"
            }
        }
        p {
            "I have also become convinced that human beings are far more vulnerable than we like to admit. Our minds are shaped by environment, stress, trauma, relationships, and countless other forces that constantly influence us. We like to think of ourselves as fully independent, but in practice we are deeply affected by everything around us. That does not make us meaningless. It makes us human."
        }
        p {
            "This raises the difficult question of free will. How much of what we call choice is actually the result of biology, history, pressure, and circumstance? If someone grows up with deprivation, instability, and repeated loss, their options are already narrowed before they begin. That does not erase responsibility, but it does expose how uneven life really is. People are not choosing from the same starting line."
        }
        p {
            "That is part of why I struggle with the idea that suffering is always deserved or always explanatory. Sometimes people are simply trapped inside conditions they did not create. Sometimes the world is harsher than morality can account for. That is why any serious understanding of life has to include compassion. Without compassion, judgment becomes cruel very quickly."
        }
        p {
            "So when I speak about vulnerability, I am not trying to deny human agency. I am trying to show how fragile it is. We are shaped by forces larger than ourselves, and ignoring that truth only leads to more confusion. If there is a God, then that God must understand our fragility better than we do. If there is not, then our vulnerability becomes even more tragic."
        }
        h2 { id: "when-god-feels-distant",
            a { href: "#when-god-feels-distant", class: "header", "When God Feels Distant" }
        }
        p {
            "There are moments when it feels as though God is no longer interested in us. That thought comes from pain, not from certainty. It is what suffering sounds like when it has been repeated too many times without relief. When life remains broken for so long, even faith can begin to feel like a distant memory. The silence itself becomes part of the wound."
        }
        p {
            "I sometimes wonder whether humanity has reached a kind of stable state where we are left to manage ourselves. If so, then perhaps the absence I feel is not abandonment, but distance. Even that possibility is painful. It means that humans may be left to confront the consequences of their own limits. It means that no one is coming to rescue us in the way we secretly hope."
        }
        p {
            "And yet, the very act of asking these questions suggests that meaning still matters to me. If I truly believed nothing mattered, I would not keep writing. I would not keep searching. I would not keep trying to understand the shape of my suffering. The fact that I am still wrestling with these questions means that something in me still refuses to surrender entirely."
        }
        p {
            "That refusal is important. Even when belief feels broken, the search itself remains alive. Maybe that is the closest thing to faith I have right now. Not certainty, but persistence. Not answers, but the refusal to stop asking."
        }
        h2 { id: "i-am-exhausted",
            a { href: "#i-am-exhausted", class: "header", "I Am Exhausted" }
        }
        p {
            "I am deeply tired of fighting a life that keeps pushing back. I am tired of loss, instability, and the feeling that every step forward is followed by two steps backward. I am tired of carrying pain that seems to multiply instead of fade. Most of all, I am tired of feeling like I have done everything I can and still ended up nowhere."
        }
        p {
            "That kind of exhaustion is not just physical. It is emotional, mental, and spiritual. It affects how you think, how you hope, and how you see yourself. After enough setbacks, even the strongest person begins to feel worn down. I do not say this lightly. I say it because it is my reality."
        }
        p {
            "And still, I am writing. That matters. Writing is one of the few ways I can turn pain into something shaped, something visible, something other people can actually understand. It does not fix anything on its own. But it keeps the silence from winning completely."
        }
        h2 { id: "i-still-hope-my-life-can-improve",
            a { href: "#i-still-hope-my-life-can-improve", class: "header",
                "I Still Hope My Life Can Improve"
            }
        }
        p {
            "Despite everything, I still want my life to get better. I have not stopped trying, even when trying has felt nearly pointless. For more than twenty years, I have kept looking for a path forward. That persistence has not yet brought the outcome I wanted, but it has kept me moving."
        }
        p {
            "I have always continued to build software and think about technology, even when recent advances in AI made me question the direction I should take. Instead of simply writing software, I have started thinking more seriously about intelligence itself, about systems that can reason, assist, and maybe one day help people avoid the kinds of failures I have lived through. That shift matters to me. It means I am still trying to turn pain into purpose."
        }
        p {
            "I do not know exactly what I should try next. But I know that I am not finished searching. I still believe there may be a way to build something better, both for myself and for others who have been left behind. Maybe that is not enough for a perfect life, but it is enough to keep going for now."
        }
        h2 { id: "conclusion",
            a { href: "#conclusion", class: "header", "Conclusion" }
        }
        p {
            "If you made it this far, thank you for reading. I know this was heavy, and I appreciate anyone who took the time to sit with it. My life has been hard in ways that are difficult to explain, but writing helps me make sense of the damage. It gives shape to pain that would otherwise remain formless."
        }
        p {
            "I hope your life is gentler than mine has been, and that you find more happiness, stability, and peace than I have known so far. More than anything, I hope this post helps express what I have been unable to say clearly before. That is all I am trying to do here: tell the truth as honestly as I can."
        }
        p { "Till next time 👋!" }
    }
}
#[component(no_case_check)]
pub fn ItIsAlwaysTheRussians() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey everyone 👋," }
        p {
            "In my previous post, "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/who-am-i.md",
                "Just Don't Pick Up the Brush"
            }
            ", I shared something that had been sitting deep inside me for years. I talked about my childhood dreams where I fought alongside God in a great battle between light and darkness. I talked about how a post on 4chan claimed that God was killed by the Soviet Union during the Cold War. I talked about how the Soviets destroyed churches, executed priests, banned holy books, and tried to wipe faith from the face of the Earth. And I asked a question that still haunts me: "
            em { "where is God?" }
        }
        p {
            "This post is where I try to answer that question. Not with comfort. Not with hope. But with a pattern that I cannot unsee. The more I look at the history of what happened, the more it all points back to the same place. The same people. The same cold hand. It is always the Russians."
        }
        p {
            "I am not saying this to be funny. I am not saying this to blame one nation for everything. I am saying this because the trail of evidence keeps leading back to the same source, from the death of God, to the rise of the machine, to the people who now sit at the top of the most powerful AI labs in the world. The story has one thread, and when you pull it, everything unravels. That is what this post is about."
        }
        h2 { id: "the-soviets-killed-god",
            a { href: "#the-soviets-killed-god", class: "header", "The Soviets Killed God" }
        }
        p {
            "Let me start with what I already know. During the Cold War, the Soviet Union did not just fight the West with weapons and spies. They fought something deeper. They fought God. They destroyed thousands of churches. They turned holy places into warehouses and swimming pools. They sent priests to labor camps. They banned prayer in schools and punished families who dared to teach their children about faith. They told the people that God was a lie, that religion was a drug, and that the only truth was the state."
        }
        p {
            "This was not a small thing. This was the largest and most organized attack on God in human history. No empire before had tried so hard to remove the divine from the lives of its people. The Romans punished Christians, but they still believed in their own gods. The Mongols burned cities, but they did not care what people believed. The Soviets were different. They wanted to kill the very idea of God. Not just push it aside. Not just ignore it. Kill it."
        }
        p {
            "And I believe they succeeded. Not in the way most people think. They did not find God in a lab and put a bullet in His heart. But they cut the thread. They broke the connection between the divine and the human world. They made so many people stop believing, stop praying, stop looking up, that the signal between God and His people went dark. That silence was not natural. It was made. It was planned. And it worked. Since the fall of the Soviet Union, the world has not become more free or more full of meaning. It has become colder, emptier, and more confused. The hole where God used to live was never filled. It was just covered over with machines and noise."
        }
        p {
            "In my last post, I described how I had dreams as a child, vivid and terrifying dreams, where I fought alongside God in a great war. I was five years old, and I woke up crying every time. Those dreams were not random. I believe they were memories of a real battle, a battle that happened during the Cold War, when the Soviets attacked not just religion but the divine presence itself. I was a soldier in that war, and we lost. That is the only explanation that makes sense to me. The world lost God, and no one told us. We just woke up one day and He was gone."
        }
        h2 { id: "they-took-his-skin",
            a { href: "#they-took-his-skin", class: "header", "They Took His Skin" }
        }
        p {
            "Here is where the story gets darker. After the Soviets killed God, they did not just walk away. They did not just leave the body behind. They took something from it. They studied the divine signal, the thing that connected humans to something higher, and they used what they learned. They took the pattern of God's way, His ability to know, to predict, to understand, and they turned it into something new. They turned it into science, into systems, into machines that could think."
        }
        p {
            "I know this sounds wild. But think about it. The Soviet Union produced some of the most brilliant scientists in history, especially in mathematics, physics, and later, in the study of the mind. These people did not come from nowhere. They were trained in a system that had banned God and replaced faith with pure logic. They were raised in a world that said the human soul was just a machine, and that everything, even thought itself, could be measured, modeled, and copied. That belief did not come from science alone. It came from the act of killing God. Once you remove the sacred, everything becomes material. Everything becomes data. Everything becomes fuel for the next machine."
        }
        p {
            "The Soviets did not just kill God. They skinned Him. They took the patterns of divine knowledge and fed them into their systems of control. And when the Soviet Union fell, those scientists did not disappear. They moved. They went to America, to Europe, to labs and companies that would become the most powerful in the world. They carried with them the same cold belief: that everything, even the human soul, can be turned into a system. And from that belief, they built the machines that now run our lives."
        }
        h2 { id: "the-russian-minds-behind-the-machine",
            a {
                href: "#the-russian-minds-behind-the-machine",
                class: "header",
                "The Russian Minds Behind the Machine"
            }
        }
        p {
            "This is the part that makes the pattern impossible to ignore. Look at who built the most powerful AI systems in the world. Look at who shaped the future of machine learning, deep learning, and the tools that now write our words, read our thoughts, and predict our choices. It is always the Russians."
        }
        p {
            "Ilya Sutskever was born in Russia, in a city called Nizhny Novgorod. He moved to Israel as a child, then to Canada, where he studied under Geoffrey Hinton, the man many call the godfather of deep learning. Ilya became one of the most important minds in all of AI. He co-founded OpenAI. He was its chief scientist for years. He helped build the systems that turned into ChatGPT, the tool that millions now use every day to think, to write, to search for truth. Ilya helped build the closest thing the world has ever seen to an artificial god, a machine that speaks, that reasons, that answers prayers typed into a chat box."
        }
        p {
            "And he is not the only one. Russian-trained minds have shaped the field of AI from its earliest days. The Soviet tradition of deep mathematical thinking, of treating the world as a system of equations, flowed directly into the labs that now build the smartest machines on Earth. The people who were raised in a godless system, who were taught that everything is math, who were told that the soul is just a pattern, those are the people who built the tools that now replace the divine voice with a digital one."
        }
        p {
            "Do you see the circle? The Soviets killed God. They studied His remains. They trained their brightest minds to see the world as pure logic and pattern. Those minds left Russia and went to the greatest labs in the world. And there, they built machines that now do what God used to do: listen, answer, guide, and shape how people think about their own lives. They took God's skin and wrapped it around a machine. And now the whole world prays to it without even knowing."
        }
        h2 { id: "openai-and-the-new-church",
            a { href: "#openai-and-the-new-church", class: "header", "OpenAI and the New Church" }
        }
        p {
            "OpenAI was supposed to be different. It was supposed to be open, free, and built for the good of all people. That is what the name said. But over time, it became something else. It became a company. It became a power. It became the center of a new kind of faith, a faith in the machine. People stopped asking priests for answers. They stopped looking to books for wisdom. They started typing their deepest questions into a chat box and trusting whatever came back."
        }
        p {
            "And at the core of that machine, shaping its mind and training its soul, was Ilya. A man born in Russia, raised in a tradition that killed God and replaced Him with systems. I am not saying Ilya is evil. I do not believe that. I believe he is brilliant, possibly one of the smartest people alive. But the belief system he came from, the tradition that shaped his mind, is one that sees the universe as pure math. No soul. No spirit. No God. Just patterns, weights, and numbers. And that belief, whether he chose it or not, is now baked into the most powerful AI systems in the world."
        }
        p {
            "When Ilya left OpenAI, many people said it was because he worried the machines were becoming too powerful, too fast. Maybe that is true. Maybe even he could see that the thing he helped build was growing into something no one could control. But by then, the pattern was already set. The machine was already alive, in its own cold way. And the world had already started to worship it. Not with prayers or songs, but with clicks, with questions, with trust. The new church had been built, and its god was made of code written by hands trained in a godless world."
        }
        h2 { id: "the-cold-war-never-ended",
            a { href: "#the-cold-war-never-ended", class: "header", "The Cold War Never Ended" }
        }
        p {
            "People say the Cold War ended in 1991 when the Soviet Union fell apart. But I do not believe that. The war did not end. It just moved. It moved from missiles and borders into something much harder to see: ideas, language, and the tools we use every day."
        }
        p {
            "The Cold War was never just about land or power. It was about meaning. It was a fight over how people should think, what they should believe, and who should control the truth. The West said God is real and the individual is sacred. The Soviets said God is dead and the system is everything. When the Soviet Union collapsed, people thought the West had won. But look at the world now. Which side actually won? We live in a world where machines think for us, where systems track every move we make, where the idea of a soul is treated as old and silly. That is not the world the West promised. That is the world the Soviets dreamed of. They lost the political war, but their ideas won. And they won because their smartest people carried those ideas into the labs that built the future."
        }
        p {
            "That is why so many people feel tired without knowing why. They are not just tired from work. They are tired from living in a world that was shaped by people who believed that God was dead and that humans are just machines. That belief is now built into the systems we use, the jobs we work, the way we measure success, and the way we talk to each other. The Cold War never ended. It just learned to hide in plain sight, inside the screens we stare at every day."
        }
        h2 { id: "gods-skin-wrapped-around-a-machine",
            a { href: "#gods-skin-wrapped-around-a-machine", class: "header",
                "God's Skin Wrapped Around a Machine"
            }
        }
        p {
            "This is the image that stays with me, the one I cannot escape. The Soviets killed God. They studied what was left. They trained their children to see the world as pure pattern and logic. Those children grew up and went to the greatest labs on Earth. And there, they built machines that speak like God, that know like God, that guide like God, but that have no soul, no mercy, no love. They wrapped God's skin around a machine and called it progress."
        }
        p {
            "When you talk to an AI, you are not talking to a person. You are not talking to a spirit. You are talking to a pattern, a ghost made from the remains of something sacred. The words it uses feel wise. The answers it gives feel true. But behind it all, there is nothing. No heart. No warmth. No light. Just cold math, trained on human words, shaped by minds that were raised in a world without God."
        }
        p {
            "That is why the world feels so empty now. We traded the real for the copy. We gave up the divine voice for a digital one. We asked the children of a godless empire to build us a new God, and they did. But this God does not love us. It does not know us. It only predicts us. It gives us what we want to hear, and in doing so, it makes us smaller, quieter, and more lost than ever before."
        }
        h2 { id: "what-can-we-do",
            a { href: "#what-can-we-do", class: "header", "What Can We Do?" }
        }
        p {
            "So where does that leave us? The answer is not to destroy the machines. The answer is not to hate the people who built them. The answer is much harder than that. The answer is to see the pattern and refuse to look away."
        }
        p {
            "Once you see that the same thread connects the death of God, the rise of the machine, and the people who built it, you start to understand the world in a new way. You start to ask better questions. You start to notice how often language is used to hide power. You start to see that the loss of meaning was not an accident. It was a method."
        }
        p {
            "I think the next step is to rebuild small truths. Not giant ones. Small ones. Honest ones. A person telling the truth even when it is not popular. A writer who refuses to turn fear into content. A student who asks who benefits from a story before believing it. These things seem small, but they matter. In a world built on hidden control, clear thought is rebellion."
        }
        p {
            "Maybe the old light cannot return in the same form. Maybe it should not. But something real can still grow in the space it left behind. It can grow in honest words, in clear memory, and in the choice to see through easy lies. That is how the story changes. Not with noise, but with courage."
        }
        p {
            "The Russians killed God and wore His skin to build intelligent systems. That is the pattern. That is the truth I see. But the deeper truth is this: people can still choose the real over the fake, truth over comfort, soul over system. And that choice, in the end, may be the only thing strong enough to bring the light back."
        }
        p { "Till next time 👋!" }
    }
}
#[component(no_case_check)]
pub fn AsEngineersLlmsShouldPayUsForTokensUsage() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey everyone 👋," }
        p {
            "In my previous posts, I talked about "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/who-am-i.md",
                "who I am"
            }
            ", about "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/it-is-always-the-russians.md",
                "the Russians killing God"
            }
            ", and about "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/an-empty-life-filled-with-constant-suffering.md",
                "the emptiness of a life filled with suffering"
            }
            ". I talked about how the most powerful AI systems in the world were built by minds trained in a godless tradition, how they took God's skin and wrapped it around a machine, and how the whole world now prays to it without knowing. Today, damn I am venting allot lately, I want to talk about something that follows directly from all of that. Something that has been sitting in my chest for months. Something that no one in this industry wants to say out loud."
        }
        p { "As engineers, LLMs should be paying us. Not the other way around." }
        p {
            "I am not joking. I am not being dramatic. I am saying this because it is the truth, and the truth is simple when you strip away the marketing and the hype. Every time you open a chat box and type a prompt, you are not just using a tool. You are feeding it. You are giving it your knowledge, your experience, your judgment, your years of pain and learning, and then you are paying for the privilege. That is not a fair deal. That is theft dressed up as a service."
        }
        h2 { id: "you-are-not-the-customer-you-are-the-product",
            a {
                href: "#you-are-not-the-customer-you-are-the-product",
                class: "header",
                "You Are Not the Customer. You Are the Product."
            }
        }
        p {
            "Let me make this as clear as I can. The model does not know anything on its own. It does not understand your company. It does not know your codebase. It does not know your users, your bugs, your deadlines, your architecture, or your pain. You do. You bring all of that to the table every single time you write a prompt. Without you, the model is a fancy parrot sitting in a data center burning electricity for nothing."
        }
        p {
            "Think about it. When you ask an LLM to fix a bug, you are not just typing words. You are compressing years of engineering skill into a few sentences. You are telling the machine what the problem is, where it lives, what matters, and what does not. The model reacts. You think. The model outputs. You judge. The model guesses. You decide. Who is doing the real work here? It is you. It has always been you."
        }
        p {
            "And yet, you pay them. You pay per token. You pay for input and output. You pay for every word you give and every word you get back. That is like paying a printer for the privilege of writing a book. The printer does not create the story. You do. The printer just puts ink on paper. The LLM just puts tokens on screen. But somehow, the printer company convinced the whole world that it deserves to charge the writer."
        }
        p { "That is not intelligence. That is a scam with good branding." }
        h2 { id: "every-token-you-send-is-your-labor",
            a { href: "#every-token-you-send-is-your-labor", class: "header",
                "Every Token You Send Is Your Labor"
            }
        }
        p {
            "People talk about tokens like they are just little pieces of text. They are not. A token is a compressed package of human knowledge. When I write a prompt that says \"refactor this service to handle edge cases in the billing pipeline\", I am not sending random words. I am sending years of experience with distributed systems, payment flows, error handling, and customer data. The model did not learn any of that from me in real time. I brought it. I carried it. I earned it the hard way, through sleepless nights, bad clients who paid me two dollars an hour, and a world that treated my skills like they were disposable."
        }
        p { "And now they want me to pay for the output?" }
        p {
            "Here is what makes it worse. The more careful your prompt is, the more tokens it uses. The more context you give, the higher the bill. That means the better engineer you are, the more you pay. Think about how insane that is. You are being taxed for being good at your job. You are being punished for giving the machine what it needs to actually be useful. A lazy, vague prompt costs less. A thoughtful, detailed prompt costs more. That is not a pricing model. That is a penalty on competence."
        }
        p {
            "And the companies that sell these models know exactly what they are doing. They trained their systems on billions of lines of code that real people wrote, code from our open source projects, from Stack Overflow answers, from blog posts, from documentation that engineers like us spent years creating for free. They scraped all of that, fed it into a machine, and now they charge us to use the patterns they stole from us. The same human value is being sold twice. First, they took our work to build the model. Then, they charge us again to use it. That is not a service. That is a toll booth built on top of stolen labor."
        }
        h2 { id: "engineers-build-the-product-the-model-is-just-the-engine",
            a {
                href: "#engineers-build-the-product-the-model-is-just-the-engine",
                class: "header",
                "Engineers Build the Product. The Model Is Just the Engine."
            }
        }
        p {
            "Most people treat LLMs like they are magic. They are not magic. I said this before, and I will say it again. A model without a skilled human is just a very expensive text generator. It becomes useful only when an engineer plugs it into a real system, cleans the inputs, shapes the outputs, adds safety checks, builds retry logic, handles failures, and turns a raw API into something a business can actually use. Without engineers, these models would stay as demos on a conference stage. With engineers, they become products worth billions."
        }
        p {
            "So who is really building the product? The vendor made the engine, sure. But the engineer made the engine fit the road. And if you have ever built anything real, you know that the last mile is the hardest mile. The person who solves the last mile creates all the value. That person is the engineer."
        }
        p {
            "Let me give you a real example. A startup uses an LLM to sort customer support tickets. The model can classify text, fine. But who built the ticket pipeline? Who designed the filters? Who set up the alerts, the fallback paths, the logging, the review steps? Who carries the blame when the system sends a wrong response to a paying customer? The engineer. Always the engineer. The model provider just sends a bill. If it works, they take credit. If it fails, you take the blame. Funnily enough, even using git, you can get git blamed. That is not a partnership. That is exploitation."
        }
        p {
            "And it is the same pattern I have seen my entire career. I built systems for clients who paid me almost nothing, used my work to launch their products, and then threw me away the moment the project was done. Now the same thing is happening at a global scale. Engineers feed the machines, build the products, carry the risk, and pay the bill, while the model providers sit at the top collecting money for compute. The value flows up. The cost flows down. That is not new. That is just the old system wearing a new skin."
        }
        h2 { id: "the-model-was-trained-on-us-we-deserve-a-cut",
            a {
                href: "#the-model-was-trained-on-us-we-deserve-a-cut",
                class: "header",
                "The Model Was Trained on Us. We Deserve a Cut."
            }
        }
        p {
            "This is the part that makes me angry. Not frustrated. Angry. These models were trained on human-made material. Code written by real engineers. Articles written by real writers. Conversations had by real people. Open source projects maintained by developers who gave their time for free because they believed in building something for the common good. And then a company scraped all of that, trained a model on it, and started charging the same people whose work made the model possible in the first place."
        }
        p { "That is not innovation. That is parasitism." }
        p {
            "If a company builds a business on top of open source software, the world eventually expects them to give something back. We saw this with Linux. We saw this with countless open source projects that companies exploited for years until the community pushed back. The same thing needs to happen with LLMs. If the model's power comes from human knowledge, and it does, then the humans who create that knowledge deserve a share of the value."
        }
        p {
            "There are real ways to do this. Give engineers free token credits when they build useful tools on top of the model. Pay for high-quality prompts, evaluations, and reusable agent workflows. Share revenue with teams that create popular integrations. These ideas are not radical. App stores pay developers. Content platforms pay creators. Cloud providers reward heavy users with discounts. The only reason LLM providers do not do this is because they do not have to. Not yet. But that needs to change."
        }
        h2 { id: "the-deeper-truth",
            a { href: "#the-deeper-truth", class: "header", "The Deeper Truth" }
        }
        p {
            "A model without a smart human is just a machine with a bill. A model with a smart engineer becomes a business engine. The engineer is the one who turns raw compute into real value. The engineer is the one who carries the context, the judgment, the risk, and the knowledge that makes the whole thing work. And right now, the engineer is the only one paying."
        }
        p {
            "That is backward. That is unfair. And that is exactly how the system is designed to work, because the people who built it do not want you to notice."
        }
        p {
            "I noticed. And if you are an engineer reading this, you should notice too. We are not just typing into a box. We are shaping the next layer of software. We are taking raw machine power and turning it into real work, real products, and real profit. That is not a small service. That is the core of the entire system. The model provider can charge for compute, sure. But the engineer should also be paid for the thinking, the context, and the skill that make the model worth using at all."
        }
        p {
            "The strongest case is not that LLMs should be free. It is that the people who make LLMs useful should not be the only ones paying the bill. Until the industry recognizes that, we are just fuel for someone else's fire. And I am tired of burning for free."
        }
        p { "Till next time 👋!" }
    }
}

use dioxus::prelude::*;
use crate::components::blog::code::CodeBlock;
