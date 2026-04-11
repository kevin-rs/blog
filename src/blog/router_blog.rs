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
    #[route("/technology-has-destroyed-my-livelihood")]
    TechnologyHasDestroyedMyLivelihood {},
    #[route("/language-is-limited-asi-is-impossible")]
    LanguageIsLimitedAsiIsImpossible {},
    #[route("/christianity-makes-perfect-sense")]
    ChristianityMakesPerfectSense {},
    #[route("/llms-are-usefull-lmms-will-break-reality")]
    LlmsAreUsefullLmmsWillBreakReality {},
    #[route("/mathematical-equations-are-multimodal-by-default")]
    MathematicalEquationsAreMultimodalByDefault {},
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
            BookRoute::TechnologyHasDestroyedMyLivelihood {} => {
                use_mdbook::mdbook_shared::PageId(6usize)
            }
            BookRoute::LanguageIsLimitedAsiIsImpossible {} => {
                use_mdbook::mdbook_shared::PageId(7usize)
            }
            BookRoute::ChristianityMakesPerfectSense {} => {
                use_mdbook::mdbook_shared::PageId(8usize)
            }
            BookRoute::LlmsAreUsefullLmmsWillBreakReality {} => {
                use_mdbook::mdbook_shared::PageId(9usize)
            }
            BookRoute::MathematicalEquationsAreMultimodalByDefault {} => {
                use_mdbook::mdbook_shared::PageId(10usize)
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
        pages
            .push((
                6usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 7 |---| Technology Has Destroyed My Livelihood |---| tech |---| technology-has-destroyed-my-livelihood |---| Apr 07 2026 |---| Technology Has Destroyed My Livelihood |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        url: BookRoute::TechnologyHasDestroyedMyLivelihood {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Biggest Lie in Modern History".to_string(),
                                id: "the-biggest-lie-in-modern-history".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "I Did Not Lose My Livelihood Because I Was Lazy"
                                    .to_string(),
                                id: "i-did-not-lose-my-livelihood-because-i-was-lazy"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Technology Punishes the People Who Need It Most"
                                    .to_string(),
                                id: "technology-punishes-the-people-who-need-it-most"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Job Market Is a Rigged Game".to_string(),
                                id: "the-job-market-is-a-rigged-game".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "War Broke the Ground. Technology Buried It."
                                    .to_string(),
                                id: "war-broke-the-ground.-technology-buried-it."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Learning Became a Trap".to_string(),
                                id: "learning-became-a-trap".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Industry Sold Us a Religion".to_string(),
                                id: "the-industry-sold-us-a-religion".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Technology Actually Owes Us".to_string(),
                                id: "what-technology-actually-owes-us".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "I Am Still Here".to_string(),
                                id: "i-am-still-here".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(6usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::TechnologyHasDestroyedMyLivelihood {},
            ::use_mdbook::mdbook_shared::PageId(6usize),
        );
        pages
            .push((
                7usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 8 |---| Language is Limited. ASI is Impossible. |---| tech |---| language-is-limited-asi-is-impossible |---| Apr 08 2026 |---| Language is Limited. ASI is Impossible. |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        url: BookRoute::LanguageIsLimitedAsiIsImpossible {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Words Are Not Thought".to_string(),
                                id: "words-are-not-thought".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Brain Is Not a Text Machine".to_string(),
                                id: "the-brain-is-not-a-text-machine".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Efficiency Is Not the Same as Intelligence"
                                    .to_string(),
                                id: "efficiency-is-not-the-same-as-intelligence"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "LLMs / LRMs Still Depend on Language".to_string(),
                                id: "llms-/-lrms-still-depend-on-language".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Induction Has a Ceiling".to_string(),
                                id: "induction-has-a-ceiling".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Some Things Cannot Be Represented Fully"
                                    .to_string(),
                                id: "some-things-cannot-be-represented-fully".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why ASI Keeps Moving Out of Reach".to_string(),
                                id: "why-asi-keeps-moving-out-of-reach".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Closing Thought".to_string(),
                                id: "closing-thought".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(7usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::LanguageIsLimitedAsiIsImpossible {},
            ::use_mdbook::mdbook_shared::PageId(7usize),
        );
        pages
            .push((
                8usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 9 |---| Christianity Makes Perfect Sense! |---| religion |---| christianity-makes-perfect-sense |---| Apr 08 2026 |---| Christianity Makes Perfect Sense! |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        url: BookRoute::ChristianityMakesPerfectSense {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Growing Up Under the Weight of Unanswered Questions"
                                    .to_string(),
                                id: "growing-up-under-the-weight-of-unanswered-questions"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why I Started Looking Beyond What I Had Been Taught"
                                    .to_string(),
                                id: "why-i-started-looking-beyond-what-i-had-been-taught"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why Christianity began to make more sense to me"
                                    .to_string(),
                                id: "why-christianity-began-to-make-more-sense-to-me"
                                    .to_string(),
                                level: 1usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Trinity and the idea of a relational God"
                                    .to_string(),
                                id: "the-trinity-and-the-idea-of-a-relational-god"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why forgiveness, grace, and redemption became real to me"
                                    .to_string(),
                                id: "why-forgiveness,-grace,-and-redemption-became-real-to-me"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why I ended up here, and why I still keep going"
                                    .to_string(),
                                id: "why-i-ended-up-here,-and-why-i-still-keep-going"
                                    .to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(8usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::ChristianityMakesPerfectSense {},
            ::use_mdbook::mdbook_shared::PageId(8usize),
        );
        pages
            .push((
                9usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 10 |---| LLMs are Usefull. LMMs will Break Reality |---| tech |---| llms-are-usefull-lmms-will-break-reality |---| Apr 10 2026 |---| LLMs are Usefull. LMMs will Break Reality |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        url: BookRoute::LlmsAreUsefullLmmsWillBreakReality {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "LLMs Are Useful And I Am Not Going to Pretend Otherwise"
                                    .to_string(),
                                id: "llms-are-useful-and-i-am-not-going-to-pretend-otherwise"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Language Is Still A Cage And I Already Proved It"
                                    .to_string(),
                                id: "language-is-still-a-cage-and-i-already-proved-it"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The World Is Not Made Of Words".to_string(),
                                id: "the-world-is-not-made-of-words".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why Mathematical Changes Everything".to_string(),
                                id: "why-mathematical-changes-everything".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Equations Are More Powerful Than Sentences"
                                    .to_string(),
                                id: "equations-are-more-powerful-than-sentences"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Simulation Is The Real Intelligence".to_string(),
                                id: "simulation-is-the-real-intelligence".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Danger Is Real And I Will Not Ignore It"
                                    .to_string(),
                                id: "the-danger-is-real-and-i-will-not-ignore-it"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Comes After Language".to_string(),
                                id: "what-comes-after-language".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "References".to_string(),
                                id: "references".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(9usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::LlmsAreUsefullLmmsWillBreakReality {},
            ::use_mdbook::mdbook_shared::PageId(9usize),
        );
        pages
            .push((
                10usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 11 |---| Mathematical Equations are Multimodal by default |---| tech |---| mathematical-equations-are-multimodal-by-default |---| Apr 11 2026 |---| Mathematical Equations are Multimodal by default |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        url: BookRoute::MathematicalEquationsAreMultimodalByDefault {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why I Think About Equations Differently Than Most People"
                                    .to_string(),
                                id: "why-i-think-about-equations-differently-than-most-people"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Compression Power That Nobody Talks About"
                                    .to_string(),
                                id: "the-compression-power-that-nobody-talks-about"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why Every Equation Is Already Multimodal"
                                    .to_string(),
                                id: "why-every-equation-is-already-multimodal".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Difference Between Correlation And Causation Is Not Academic"
                                    .to_string(),
                                id: "the-difference-between-correlation-and-causation-is-not-academic"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Simulation As The Ultimate Test Of Understanding"
                                    .to_string(),
                                id: "simulation-as-the-ultimate-test-of-understanding"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What This Means For The Future Of Intelligence"
                                    .to_string(),
                                id: "what-this-means-for-the-future-of-intelligence"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "References".to_string(),
                                id: "references".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(10usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::MathematicalEquationsAreMultimodalByDefault {},
            ::use_mdbook::mdbook_shared::PageId(10usize),
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
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 7 |---| Technology Has Destroyed My Livelihood |---| tech |---| technology-has-destroyed-my-livelihood |---| Apr 07 2026 |---| Technology Has Destroyed My Livelihood |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        location: Some(BookRoute::TechnologyHasDestroyedMyLivelihood {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![7u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 8 |---| Language is Limited. ASI is Impossible. |---| tech |---| language-is-limited-asi-is-impossible |---| Apr 08 2026 |---| Language is Limited. ASI is Impossible. |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        location: Some(BookRoute::LanguageIsLimitedAsiIsImpossible {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![8u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 9 |---| Christianity Makes Perfect Sense! |---| religion |---| christianity-makes-perfect-sense |---| Apr 08 2026 |---| Christianity Makes Perfect Sense! |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        location: Some(BookRoute::ChristianityMakesPerfectSense {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![9u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 10 |---| LLMs are Usefull. LMMs will Break Reality |---| tech |---| llms-are-usefull-lmms-will-break-reality |---| Apr 10 2026 |---| LLMs are Usefull. LMMs will Break Reality |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        location: Some(BookRoute::LlmsAreUsefullLmmsWillBreakReality {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![10u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 11 |---| Mathematical Equations are Multimodal by default |---| tech |---| mathematical-equations-are-multimodal-by-default |---| Apr 11 2026 |---| Mathematical Equations are Multimodal by default |---| https://images.pexels.com/photos/9933604/pexels-photo-9933604.jpeg"
                            .to_string(),
                        location: Some(BookRoute::MathematicalEquationsAreMultimodalByDefault {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![11u32]),
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
        p {
            strong { "🚨 Important Note:" }
            " Sushicide 🍣 is a serious problem that many among us struggle with. Software engineering is a heartless and lovely profession, and we should be more open about this topic so that we don't lose another innocent human soul."
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
            "It is actually quite a funny and unexpected story, considering the circumstances under which I discovered this place. A couple of years ago, I was on YouTube, desperately searching for ways to cope with stress, overwhelming emotions, and the recurring thoughts of sushicide 🍣 that had become an inescapable part of my life. I was looking for anything, self-help advice, scientific explanations, or even personal stories, that could provide some form of relief, clarity, or guidance. Strangely, instead of finding helpful resources, I was recommended a video that strongly criticized this forum, warning that it was dangerous and harmful, particularly for teenagers and vulnerable individuals struggling with their mental health (^1). At first, I found it odd that an algorithm would push such a video toward me, especially when all I was searching for were ways to manage my pain. But as I listened to the arguments presented in the video, something about the way it described this forum engaged me rather than scared me away. The more I thought about it, the more I felt drawn toward it, almost as if it had been placed in front of me for a reason. I was already deeply struggling, feeling lost and disconnected from everything around me, so the idea that there was a place where people discussed these thoughts so openly made me curious. I wasn't sure whether this forum was truly as dangerous as the video claimed or whether it was simply a misunderstood space where people like me could find understanding. Either way, I felt like I needed to see it for myself."
        }
        p {
            "Ever since that moment, the idea of sushicide 🍣 started making more sense to me, not in a dramatic or impulsive way, but in a logical and calculated manner that felt almost refreshing. The struggles I had been facing were relentless, constant, and completely draining, leaving me with little reason to believe that things would ever improve. For years, I had tried different ways to cope, searching for solutions, advice, or even just distractions, but nothing ever truly helped. At best, I was able to temporarily push my thoughts aside, but they always came back, stronger and more persistent than before. I kept wondering: if every attempt to improve my life only led me back to the same place, was there any real point in continuing? If suffering was inevitable and hope was nothing more than a momentary illusion, then maybe, just maybe, letting go was not the worst option after all. The thought was never rushed, never emotional, it was always rational, always measured, always based on the undeniable reality of my situation. Still, despite these thoughts, I never had the opportunity to fully explore them because of the enormous workload I had taken on."
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
            "My family has a long history of struggling with thoughts of sushicide 🍣, and this has affected generations before me. My grandfather, for example, ended his own life by poisoning himself, and he often had conflicts with his children or his wife. These constant struggles may have pushed him to feel hopeless, leading him to make that tragic decision. It is heartbreaking to know that someone in my family felt so trapped in pain that they saw no other way out. Because of this history, I sometimes wonder if my own thoughts of sushicide 🍣 are passed down to me, not just through the difficulties of life but also as something inherited from my grandfather. This does not mean I want to follow the same path, but it does make me think deeply about how much pain can shape a family. It is not easy to live with these thoughts, but knowing that others before me have suffered in the same way makes me reflect on the importance of finding a different way to cope. My grandfather's story is a reminder that sadness can take over a person's mind if they do not get the support they need, and that is why it is important to talk about these feelings instead of hiding them."
        }
        p {
            "My father also struggled with thoughts of sushicide 🍣, though he never acted on them, which is something I am deeply grateful for. I do not know all the reasons behind his struggles, but I know that life was not always kind to him. Even though he faced many challenges, he somehow found the strength to keep going, and for that, I admire him greatly. There must have been times when he felt completely lost, yet he never gave in to the thoughts that tried to pull him down. This takes an incredible amount of inner strength, and I cannot begin to imagine how much he must have suffered in silence. Despite his own struggles, he always tried to make life brighter for me, and that is something I will always appreciate. He could have given up, but instead, he chose to keep moving forward, showing me that even in the darkest moments, there is still hope. His ability to stay strong despite his pain gives me the courage to fight my own battles, even when they seem unbearable. If he could hold on through his suffering, then maybe I can too."
        }
        p {
            "There are many reasons why people struggle with sushicidal 🍣 thoughts, and they are often far more complex than what others can see on the surface. Some people battle with painful memories, while others feel overwhelmed by problems that seem impossible to solve. In my family, it seems like this struggle has been passed down from one generation to the next, making it feel almost impossible to escape. It is hard to explain to someone who has never felt this way because it is not just about being sad; It is about feeling like nothing will ever get better, no matter what you do. Even when surrounded by love and support, these thoughts can creep in like shadows, whispering lies that make a person believe they are worthless. But just because something runs in a family does not mean it has to control the future. Every person has the power to break the cycle, even if it feels impossible. It is not easy, and it takes a lot of effort, but choosing to keep going is the strongest thing a person can do."
        }
        p {
            "Even though my family has a history of pain, I want to believe that things can be different for me. I do not want to follow the same path as my grandfather, and I do not want to carry the same burden my father did. Instead, I want to find a way to heal, even if it takes time. Life will always be difficult, and there will always be moments when things feel unbearable, but that does not mean there is no hope. It helps to remember that pain is temporary, even when it feels like it will last forever. I have seen what happens when people give up, and I do not want that to be my story. Even when my thoughts try to convince me otherwise, I have to remind myself that there is still a future ahead of me, one that is not written by my past but by the choices I make today. If my father could keep going, then so can I. If I can learn to hold on, maybe I can help someone else do the same."
        }
        p {
            "Talking about sushicide 🍣 is difficult, but it is important because too many people suffer in silence, thinking they are alone. The truth is, so many people have felt the same way, but they just do not talk about it. If more people were open about their struggles, maybe fewer would feel trapped in their own thoughts. It is easy to believe that no one understands, but that is just another lie our minds tell us when we are hurting. The more we talk about it, the less power it has over us. No one should have to suffer alone, and no one should feel ashamed of what they are going through. Pain does not make a person weak; in fact, surviving it makes them stronger than they realize. Instead of letting it define me, I want to use my experiences to remind myself that I am not alone and that there is always a way forward."
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
        p {
            strong { "🚨 Important Note:" }
            " I know this post might make me sound like a conspiracy theorist, but I'm simply sharing my observations and asking you to keep an open mind. As a software engineer, I'm fascinated by the history of AI and its unexpected ties to the Cold War, and I want to share how my childhood dreams still haunt me today. I know ego is often criticized in engineering, but without it, we'd have no technological progress. I've already lost everything, so sharing this won't impact my livelihood "
        }
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
#[component(no_case_check)]
pub fn TechnologyHasDestroyedMyLivelihood() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey everyone 👋," }
        p {
            "In my previous posts, I talked about many topics. Every one of those posts was a piece of the same broken mirror. Today, damn it is the same day bro "
        }
        p {
            "I am not writing this as a critic. I am not writing this as a philosopher. I am writing this as someone whose livelihood was destroyed, slowly, quietly, and almost completely, while the entire world kept clapping for the thing that did it. Everyone talks about technology like it is oxygen. Like it is sunlight. Like the only possible response to it is gratitude. Well I have been breathing this oxygen for years and it has been poisoning me. I want to explain exactly how, and I want to make the case so clearly that no reasonable person can look away from it."
        }
        h2 { id: "the-biggest-lie-in-modern-history",
            a { href: "#the-biggest-lie-in-modern-history", class: "header",
                "The Biggest Lie in Modern History"
            }
        }
        p {
            "Here is the lie: technology is a great equalizer. Anyone with a laptop and an internet connection can change their life. That is the story they sell on every stage, in every keynote, in every inspirational YouTube video made by someone who already had money before they touched a keyboard. And the lie works because it contains just enough truth to be believable. Yes, a person with a laptop can learn to code. Yes, a person with internet can access information. But access is not opportunity. Information is not income. And a laptop does not feed you when every company on Earth is looking for someone cheaper, faster, or more connected than you."
        }
        p {
            "I grew up in a poor village with no internet, no computer, and no cell phone. I wrote about this in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/who-am-i.md",
                "my first post"
            }
            ". When I finally got access to the digital world, I did not waste it. I studied. I built. I wrote code, published projects, wrote blog posts, entered hackathons, created GitHub organizations, and pushed myself harder than most people I have ever met. And the result? Nothing. Not a damn thing. Years of effort, and the system treated me like I did not exist."
        }
        p {
            "So do not tell me technology is a great equalizer. A great equalizer does not leave people behind based on their name, their country, their accent, or how many LinkedIn connections they have. A great equalizer does not reward the person with an existing network and punish the person who is building from zero. What technology actually does is give the illusion of equal footing while the ground is tilted so far in one direction that half the people on it are sliding off the edge. That is not equality. That is performance."
        }
        h2 { id: "i-did-not-lose-my-livelihood-because-i-was-lazy",
            a {
                href: "#i-did-not-lose-my-livelihood-because-i-was-lazy",
                class: "header",
                "I Did Not Lose My Livelihood Because I Was Lazy"
            }
        }
        p {
            "Let me make this clear, because people love to assume things. I did not lose my career because I sat around doing nothing. I lost it while doing everything. I took on four contracts at once, working for clients who paid me 2 freakin dollars an hour. I built full systems, delivered them, and was discarded the moment the launch was done. I wrote a book. I built awesome projects. I maintained open source tools. I competed in hackathons. I stayed up coding through nights that turned into mornings that turned into more nights."
        }
        p {
            "And still, no recruiter hired me. Still, recruiters ignored my applications. Still, the industry looked right through me like I was transparent. I did not fail the system. The system failed me. And technology was the mechanism it used to do it."
        }
        p {
            "Think about that. The same tools I spent years mastering are the same tools the industry uses to filter me out before a human being ever sees my name. My resume goes into an automated system. An algorithm scans it. A keyword match decides whether I am worth a conversation. And just like that, years of real work, real skill, and real pain get reduced to a score that a machine spits out in less than a second. The person behind the resume does not matter. The machine does not care that I built an entire AI agent system from scratch in Rust. It does not care that I can think. It does not care that I am hungry, displaced, and fighting for my life. It just checks boxes. And if the boxes do not match the template, it throws me away."
        }
        p { "That is not efficiency. That is institutional cruelty wearing a mask of progress." }
        h2 { id: "technology-punishes-the-people-who-need-it-most",
            a {
                href: "#technology-punishes-the-people-who-need-it-most",
                class: "header",
                "Technology Punishes the People Who Need It Most"
            }
        }
        p {
            "The harder your life is, the more technology costs you. Not in money, although that too. It costs you in energy, in time, in emotional weight, in the relentless demand to keep up with a world that never slows down and never looks back."
        }
        p {
            "If you are stable, if you have a home, a job, a support system, a full stomach, and eight hours of sleep, then learning a new framework or updating your skills is an inconvenience. A weekend project. A fun challenge. But if you are fighting to survive, if you are displaced, if you are hungry, if your country has been destroyed by war, then every hour you spend learning a new tool is an hour taken from survival. Every sleepless night spent debugging code is a night your body does not recover from. Every rejected application is not just a missed opportunity. It is a blow to a foundation that was already cracked."
        }
        p {
            "I became someone who could function technically while falling apart internally. I could ship code and solve problems while running on nothing. And the industry saw that output and said, not enough. Learn more. Build more. Be faster. Be cheaper. Be grateful. That is what technology asks of people like me. Not to grow. To grind. To bleed into the machine and thank it for the privilege of being used."
        }
        p {
            "The cruelest part is this: the better you get, the more the system expects from you, and the less it gives in return. Skill does not protect you. Effort does not protect you. Discipline does not protect you. In a system that values connections over competence and branding over substance, the person who works the hardest can still end up with nothing. And they usually do. That is not a bug. That is the design."
        }
        h2 { id: "the-job-market-is-a-rigged-game",
            a { href: "#the-job-market-is-a-rigged-game", class: "header",
                "The Job Market Is a Rigged Game"
            }
        }
        p {
            "People love to say the best candidate gets the job. That has never been true, and technology has made it less true than ever. Hiring today is not a competition of skill. It is a competition of visibility, positioning, and algorithmic favor. A person can be the most capable engineer in the room and never get into the room because a machine decided their resume did not contain the right keywords. A person can have 10 years of real-world experience and be rejected by a system that prioritizes a degree from a specific university or a referral from someone already inside."
        }
        p {
            "I have applied for more jobs than I can count. I have tailored resumes, written cover letters, practiced interviews, cleaned up my GitHub, polished my portfolio, and done every single thing the advice columns and career coaches tell you to do. And the response, almost every single time, is silence. Not rejection. Silence. No reply. No explanation. Just the sound of effort disappearing into a void."
        }
        p {
            "And here is what makes it worse. The companies that reject me still use technologies I understand better than most of their teams. The frameworks I helped test, the tools I contributed to, the patterns I learned before they were trendy, all of that lives inside the products these companies sell. My work, and the work of millions of developers like me, is already built into the foundation. We just do not get to live in the house."
        }
        p {
            "Technology did not make the job market fairer. It made it faster. And speed without justice is just violence with better branding."
        }
        h2 { id: "war-broke-the-ground-technology-buried-it",
            a {
                href: "#war-broke-the-ground-technology-buried-it",
                class: "header",
                "War Broke the Ground. Technology Buried It."
            }
        }
        p {
            "I come from a country that was broken by war. I did not choose that. I did not cause it. But I carry it in everything I do. War does not just destroy buildings. It destroys continuity. It destroys the quiet assumption that if you work hard today, tomorrow will be a little better. When that assumption is gone, everything becomes heavier. Work becomes heavier. Hope becomes heavier. Getting out of bed becomes an act of defiance."
        }
        p {
            "Technology pretends war does not exist. The industry does not adjust for people whose educations were interrupted, whose infrastructure was destroyed, whose families were displaced. They do not build systems that account for instability. They build systems that assume everyone starts from the same place and punish anyone who does not."
        }
        p {
            "I watched the rest of the world adopt new tools, attend conferences, network in coffee shops, and build careers while I was trying to figure out where I would sleep next week. I had the same curiosity. I had the same intelligence. I had the same hunger to build. But I did not have the same ground to stand on. And technology does not give you ground. It gives you tools that only work when the ground is already there. Without stability, access is meaningless. Without safety, opportunity is just a word."
        }
        p {
            "The internet did not close the gap between me and someone in San Francisco or London. It made that gap Ultra HD. It let me see exactly how much I was missing, in real time, on a screen I could not always afford to charge. That is what technology did for people like me. It did not lift us up. It made the fall more visible."
        }
        h2 { id: "learning-became-a-trap",
            a { href: "#learning-became-a-trap", class: "header", "Learning Became a Trap" }
        }
        p {
            "I used to love learning. I used to sit with a problem for hours and feel genuinely alive when I solved it. That feeling kept me going through years of difficulty. But technology turned learning from joy into obligation, and then from obligation into pain."
        }
        p {
            "The pace of the industry is designed for people with margin. New framework every week. New paradigm every month. New set of tools, new way of thinking, new expectation to meet. If you do not keep up, you are irrelevant. If you do keep up, it costs you your health, your sleep, and whatever emotional reserves you had left. The treadmill never stops. And the people running it do not care that some of us started the race carrying two hundred pounds of trauma, poverty, and displacement on our backs."
        }
        p {
            "I kept learning because I believed it would eventually convert into stability. That was the promise. Learn and you will earn. Build and you will belong. Improve and you will be seen. But the conversion rate is broken. I have more skills now than I have ever had, and I have less stability than I have ever had. That is not a coincidence. That is the system working exactly as it was designed to work. It extracts learning from people, turns it into labor, and pays back in scraps. If you are lucky. If you are not lucky, it pays back in silence."
        }
        p {
            "The worst part is that technology has turned the love of learning itself into a liability. If you are the kind of person who genuinely loves to understand things, the industry will use that against you. It will give you harder problems, tighter deadlines, and less money, because it knows you will keep going anyway. Passion is exploited. Curiosity is monetized. And the people who feel the most are the ones who get burned the fastest."
        }
        h2 { id: "the-industry-sold-us-a-religion",
            a { href: "#the-industry-sold-us-a-religion", class: "header",
                "The Industry Sold Us a Religion"
            }
        }
        p {
            "Let me say something that will make people uncomfortable. The tech industry is a religion. It has prophets: founders who stand on stages and talk about changing the world. It has scripture: blog posts, documentation, and thought leadership that tells you how to think, what to build, and what matters. It has rituals: standups, sprints, hackathons, and conferences where people gather to celebrate the faith. And it has a promise of salvation: if you work hard enough, if you believe deeply enough, if you give enough of yourself to the machine, you will be rewarded."
        }
        p {
            "But like every religion that has ever existed, the rewards go to the priests, not the believers. The founders get rich. The investors get richer. And the engineers, the people who actually build the thing, get a salary if they are lucky and get nothing if they are not. The entire system runs on faith. Faith that the next project will lead to a better job. Faith that open source contribution will be recognized. Faith that effort will be rewarded. And that faith is what keeps people grinding while the people at the top keep extracting."
        }
        p {
            "I believed in that religion for years. I sacrificed my time, my sleep, my health, and my hope. And what did I get? The same thing every true believer gets when the church turns out to be a business: abandoned, confused, and too deep in to easily walk away. Technology is not neutral. It is not a tool. It is a system of belief that converts human labor into shareholder value and calls it progress."
        }
        h2 { id: "what-technology-actually-owes-us",
            a { href: "#what-technology-actually-owes-us", class: "header",
                "What Technology Actually Owes Us"
            }
        }
        p {
            "Technology was built on stolen labor. I said it in my "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/as-engineers-llms-should-pay-us-for-tokens-usage.md",
                "previous post about LLMs"
            }
            ", and I am saying it again. The models were trained on our code, our writing, our answers, our open source contributions. The platforms were built on content we created for free. The algorithms were refined using data we generated with every click, every search, every message. We are not the customers. We are the raw material. And the industry has gotten very good at extracting value from us while giving back just enough to keep us producing."
        }
        p {
            "If technology truly served people, it would not leave the most vulnerable behind. It would build systems that account for instability, displacement, and unequal starting positions. It would create hiring processes that actually evaluate human capability instead of running keyword matches on broken ATS software. It would pay the people whose knowledge it was trained on. It would measure progress not by how fast it moves but by who it leaves behind."
        }
        p {
            "But it does none of those things. Because it was never built to serve us. It was built to serve the people who own it. And we were always just fuel."
        }
        h2 { id: "i-am-still-here",
            a { href: "#i-am-still-here", class: "header", "I Am Still Here" }
        }
        p {
            "After everything, I am still here. That is not a victory. It is a fact. I did not overcome the system. The system is still running, and I am still getting crushed by it. But I have not disappeared, and I refuse to be quiet about what happened to me."
        }
        p {
            "My livelihood was not destroyed by laziness, by stupidity, or by lack of effort. It was destroyed by a system that promises access while delivering exclusion, that celebrates innovation while ignoring the people it displaces, and that wraps exploitation in the language of progress until no one can tell the difference anymore. Technology helped destroy my livelihood. And I know I am not the only one."
        }
        p {
            "If this post makes you uncomfortable, good. It should. Because the story of technology is not just the story of people who made it. It is also the story of people who were broken by it. And until that story is told with the same volume and the same urgency, nothing will change. The machines will keep getting smarter. The people at the bottom will keep getting quieter. And the ones at the top will keep calling it the future."
        }
        p {
            "I refuse to be quiet. And if you have been through what I have been through, you should refuse too. Because silence is not resilience. Silence is surrender. And I have surrendered enough."
        }
        p { "Till next time 👋!" }
    }
}
#[component(no_case_check)]
pub fn LanguageIsLimitedAsiIsImpossible() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey everyone 👋," }
        p {
            "In a previous post, "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/an-empty-life-filled-with-constant-suffering.md",
                "\"An Empty Life Filled With Constant Suffering\""
            }
            ", I mentioned that words cannot fully capture my thoughts. I have been circling the same wound for a while now, and I am not going to pretend otherwise. I keep coming back to the same ugly truth, which is that people keep mistaking words for reality. They keep acting like language is the final shape of thought, the final shape of meaning, and even the final shape of intelligence. I do not believe that for one second. And I think the whole dream of artificial superintelligence falls apart the moment you stop worshipping text and start looking at what thought really is."
        }
        h2 { id: "words-are-not-thought",
            a { href: "#words-are-not-thought", class: "header", "Words Are Not Thought" }
        }
        p {
            "A word is not a thought. A sentence is not a mind. A paragraph is not a living thing. People keep forgetting this because language is so useful, so smooth, and so easy to trust. But the truth is that words are only symbols, and symbols are always smaller than the thing they point to. A map can help you move through the world, but a map is not the land, and language is just a map made of sound and ink."
        }
        p {
            "When I think about something real, the thing in my head is always richer than the sentence I use to describe it. There is feeling in it, memory in it, fear in it, and a hundred tiny parts that never make it into speech. By the time I say the words out loud, the thing has already been cut down. It has already been shaped to fit inside a box that was never built for the full size of the thing itself. This is the nature of language itself."
        }
        p {
            "This is why people misunderstand each other all the time, even when they are being honest. They think they said enough, but they did not. Or they think they heard enough, but they only heard the shell. The real thing was bigger than the sentence, and it slipped away before anyone could hold it. We call that miscommunication, but I think that word is too soft. It is more like trying to pour the sea into a cup and then acting surprised when the cup overflows."
        }
        p {
            "A lot of people live their whole lives inside this mistake. They treat language like a perfect mirror of the mind, when it is really just a tool for getting close. That does not make language bad. It makes it limited. And once you admit that limitation, you have to admit that any machine built mainly on language is also limited in the same way."
        }
        p {
            "So when people tell me that more text will lead to more intelligence, I shake my head. More text only gives you more text. More symbols only gives you more symbols. A deeper world does not appear just because the pile of words gets taller. The world stays bigger than the words, and the gap stays there no matter how much we try to ignore it."
        }
        h2 { id: "the-brain-is-not-a-text-machine",
            a { href: "#the-brain-is-not-a-text-machine", class: "header",
                "The Brain Is Not a Text Machine"
            }
        }
        p {
            "The human brain does not work like a chat box. It does not sit there waiting for a prompt and then answer in neat lil lines. It is a living system, full of signals, loops, guesses, memory, instinct, pain, reward, and timing. It is doing many things at once, and most of them never become words. That is what makes it alive."
        }
        p {
            "I can know something before I can explain it. I can feel danger before I can name it. I can walk into a room and sense the mood before I have any clear reason for it. These things happen all the time, and they are real. They are part of thought, even if they never become a sentence. The brain is not only a speaker. It is a judge, a sensor, a learner, and a survivor."
        }
        p {
            "People who reduce intelligence to language always miss this. They look at speech and think they are looking at mind itself. They are not. They are looking at one thin layer on top of a much bigger machine. Under the words there is body, and under the body there is history, and under the history there is a living system trying to stay in the world. That system does not wait politely for grammar before it acts."
        }
        p {
            "This is why I find a lot of AI talk so shallow. People say a model is \"thinking\" because it can write a good answer. But a good answer is not the same thing as a good mind. A good answer can come from pattern, from imitation, from weight, from speed, from guesswork. It does not prove that the thing has the same kind of inner life that a human has."
        }
        p {
            "And that matters because the whole argument for ASI often rests on a false picture of what mind is. If you think mind is just a flow of text, then of course you think a big text machine could become godlike. But if mind is bigger than text, then the whole story changes. Then the machine is not climbing toward full intelligence. It is only getting better at one narrow channel of it."
        }
        h2 { id: "efficiency-is-not-the-same-as-intelligence",
            a {
                href: "#efficiency-is-not-the-same-as-intelligence",
                class: "header",
                "Efficiency Is Not the Same as Intelligence"
            }
        }
        p {
            "People love efficiency because machines are efficient in the places where humans are weak. A calculator can do arithmetic faster than I can. A database can store more than I can remember. A search engine can find a fact faster than I can dig it up by hand. That is all true. But none of that means the machine is smarter than me in the broad sense that matters in real life."
        }
        p {
            "This is where the hype gets slippery. People take one sharp skill and turn it into a story about total superiority. They see a system win one task and then act like it has won the whole game. That is worship and not logic. It is the same bad habit people have always had, where they take one bright trick and turn it into a god."
        }
        p {
            "Human beings are not efficient in a clean machine way, and I think that is part of our strength. We waste time. We doubt ourselves. We get tired. We make mistakes. We feel too much. We carry the past into the present. And yet we adapt in ways no simple machine does, because our intelligence is tied to the world, not just to output. We are living inside meaning and not just processing inputs."
        }
        p {
            "That is why I do not respect claims that a system is \"better\" just because it uses less energy or finishes a task faster. Better at what, exactly? Better at one narrow slice of a problem is not the same as better at being a mind. A machine can beat me at chess and still fail at understanding why I care about chess in the first place. It can finish the move and still miss the life around the move."
        }
        p {
            "People say efficiency scales. Fine. But human life is not only about scaling. It is about judgment, context, memory, purpose, and change. A system that can only win by being more efficient in a narrow frame is still trapped inside that frame. It has only become more polished inside its own limits, and has not crossed into a higher form of mind."
        }
        h2 { id: "llms--lrms-still-depend-on-language",
            a { href: "#llms--lrms-still-depend-on-language", class: "header",
                "LLMs / LRMs Still Depend on Language"
            }
        }
        p {
            "Large language models are built from text, trained on text, and judged by text. That is the whole game. They learn patterns in language and then produce new language that fits those patterns. Even when they seem to reason, they are still doing it through language-like steps. That is why I say they are trapped inside the medium that created them."
        }
        p {
            "A model can talk about grief without grieving. It can talk about love without loving. It can talk about pain without pain. It can describe fear, death, shame, joy, and longing in very convincing words, but it does not have to live any of it. That is the gap people keep trying to close with more scale, more data, and more polish. I do not think polish is the answer. I think it is a distraction."
        }
        p {
            "A lot of people confuse fluent speech with deep understanding. That is dangerous because fluent speech is easy to trust. When something sounds right, people relax. They stop asking what is under it. But language can be smooth and still empty. It can be beautiful and still hollow. It can look wise and still be only a mirror shaped by pattern."
        }
        p {
            "This is why I keep saying that an LLM is not a mind in the human sense. It is a machine that sits inside a symbolic world and moves symbols around. That is useful. That is powerful. That can even be dangerous in its own way. But it is still only working with representations, not with reality itself."
        }
        p {
            "And once you see that, the whole dream of \"just scale it and it will wake up\" starts to look weak. Scale does not magically turn a text engine into a living being. It makes a better text engine. That is not a small thing. But it is also not a leap across the wall into true superintelligence. The wall is still there."
        }
        h2 { id: "induction-has-a-ceiling",
            a { href: "#induction-has-a-ceiling", class: "header", "Induction Has a Ceiling" }
        }
        p {
            "LLMs learn by looking at what already exists. They study the past and guess the next token. That means they are always downstream from human work. Every sentence they produce is built on the shape of old human words, old human patterns, old human habits, and old human mistakes. They remix what came before, and they do it well, but they do not step outside the stream."
        }
        p {
            "That is a ceiling, whether people like it or not. A system that learns from what has already been said will always carry the weight of what has already been said. It can generalize. It can surprise us. It can even create things that look new. But the newness is still born inside a machine that feeds on prior expression. That is not the same as true freedom of mind."
        }
        p {
            "People talk about emergence like it is magic, but magic is not an argument. Saying \"something new might emerge\" does not solve the problem of the medium. If the medium is still language, then the new thing is still bounded by language. It may stretch the boundary. It may push it around. But it does not erase it. And if the boundary stays, the ceiling stays too."
        }
        p {
            "I think people underestimate how much reality resists clean representation. We do not live in a neat text file. We live in a world of touch, time, pressure, bodies, hunger, sickness, risk, and memory. Language can point at those things, but it cannot swallow them whole. A model trained on text can only ever learn a picture of the world that was already filtered through human words."
        }
        p {
            "I do not buy the story that more and more pattern completion will somehow turn into a new kind of god-like mind. Pattern completion is still pattern completion. It is useful. It is powerful. It can even help people do serious work. But it is not the same as crossing outside the frame that made it possible in the first place."
        }
        h2 { id: "some-things-cannot-be-represented-fully",
            a {
                href: "#some-things-cannot-be-represented-fully",
                class: "header",
                "Some Things Cannot Be Represented Fully"
            }
        }
        p {
            "There are moments in life that fight against language. Pain is one of them. Grief is one of them. Panic is one of them. Love is one of them. Sometimes you can speak for hours and still not get near the center of what you feel. The words can circle the thing, but they do not become the thing. The lived moment stays larger."
        }
        p {
            "I think this is one of the clearest signs that consciousness is not the same as language. If words were enough, then we would always be able to fully explain ourselves. We would always be able to write the exact thing we feel with no loss at all. But that is not what happens. What happens is that we reach for language, and language reaches back with something smaller than what we wanted to say."
        }
        p {
            "That means language is a tool, not a container for everything. It helps us share. It helps us build. It helps us remember. It helps us make order out of chaos. But it is still a tool with edges, and those edges cut off part of the truth every time. That is the cost of using symbols to stand in for living experience."
        }
        p {
            "This is also why I do not believe in the idea of a perfect language for thought. People like the dream of a complete code, a full map, a final system that can capture every part of mind without loss. I do not think that exists. To make such a thing, you would need total context, total memory, total meaning, and total flexibility all at once. At that point it would not really be language anymore. It would be reality itself."
        }
        p {
            "And that is the point people keep missing. The more complete you try to make the code, the less like code it becomes. The more you try to make representation match life, the more you discover that life keeps spilling out past the borders. There is always more in the thing than in the sign for the thing. There is always a crack where the real world leaks through."
        }
        h2 { id: "why-asi-keeps-moving-out-of-reach",
            a { href: "#why-asi-keeps-moving-out-of-reach", class: "header",
                "Why ASI Keeps Moving Out of Reach"
            }
        }
        p {
            "The dream of ASI assumes that intelligence can be scaled until it escapes human limits. I do not think that is true. I think scaling language makes a better language system, not a new kind of being. It makes a stronger pattern engine. It makes a smoother answer machine. It makes something that can help, impress, and sometimes even surprise us. But it does not automatically make a mind that has crossed into a higher order of life."
        }
        p {
            "Nowadays, I find so much of the current talk childish. People hear \"more parameters\" and act like they heard \"more soul\". They hear \"better reasoning\" and act like they heard \"new consciousness\". They hear \"agentic behavior\" and act like they heard \"selfhood\". But those are not the same things. A system can look alive from the outside and still be trapped in a narrow kind of symbolic motion."
        }
        p {
            "Human intelligence is broader than text. It is not only writing and speaking. It is feeling, moving, sensing, remembering, and choosing in a world that pushes back. It is a body in time. It is a mind shaped by loss, by care, by risk, and by contact with the real. A machine that stays inside text does not have that whole life. It has a piece of it. Maybe a very useful piece. But still only a piece."
        }
        p {
            "I do not think the future is some clean march toward an artificial god. I think the future is much more practical and much less holy. It is better tools. Better interfaces. Better support. Better automation. Better systems that help humans do more with less friction. That is already a huge deal. That is already enough to change whole industries. But it is not ASI."
        }
        p {
            "And maybe that is the honest answer. Maybe the whole fantasy of a text-based supermind is just another version of human pride, dressed up in math and code. Maybe we want a god because we are tired, lonely, and afraid. Maybe we want the machine to become more than us because we do not know how to live with our own limits. I get that. I really do. But wanting something badly does not make it true."
        }
        h2 { id: "closing-thought",
            a { href: "#closing-thought", class: "header", "Closing Thought" }
        }
        p {
            "Words are useful, but they are not the whole story. They are shadows of thought, not thought itself. They are bridges, not destinations. They help us reach across distance, but they do not become the thing on the other side. And once you admit that, a lot of the noise around AI starts to sound much smaller."
        }
        p {
            "I do not think the future belongs to some god machine made out of text. I think it belongs to tools that stay honest about what they are. Tools can be powerful without pretending to be alive. They can be useful without pretending to be wise. They can help us do more while still remaining just tools. That, to me, is a much better path than worshipping a model that only speaks in symbols."
        }
        p {
            "AI is not useless. I am saying its limits matter. I am saying language is not enough to become the full shape of mind. I am saying that if a system stays trapped inside symbols, then it stays trapped inside symbols. No amount of hype changes that. No amount of branding changes that. No amount of corporate talk changes that."
        }
        p { "And I think people need to hear that plainly." }
        p { "Till next time 👋!" }
    }
}
#[component(no_case_check)]
pub fn ChristianityMakesPerfectSense() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p {
            strong { "🚨 Important Note:" }
            " Discussing religions is considered a taboo/sensitive topic by some, and I want to make it clear that I am not here to offend anyone or to claim that one religion is superior to another. I am simply sharing my personal journey and my thoughts on the matter. I am not a theologian, but I am a software engineer."
        }
        p { "Hey everyone 👋," }
        p {
            "I want to begin with something I never thought I would say out loud, because for most of my life I assumed I would stay exactly where I started, believing what I was given without ever stepping outside of it. I was born and raised as a Muslim, and for many years I carried that identity the way a person carries a home they have never questioned, simply because it was always there. I was taught that faith should be simple, that certainty was strength, and that asking too many questions could pull you away from truth instead of bringing you closer to it. But even as a child, as I shared in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/who-am-i.md",
                "Just Don't Pick Up the Brush"
            }
            ", while repeating words I had memorized without fully understanding them, I could not stop feeling that real truth should not need silence or fear to survive. That quiet feeling was the first crack in something I once thought would never move."
        }
        p {
            "As I grew older, that crack did not go away, it slowly widened as I began to see the difference between repeating something and actually understanding it. I could say many things with confidence and still not know why they were true, and that created a tension in me that I could not ignore anymore. I started to feel that I was performing faith instead of living it, and that difference became too important for me to dismiss. Because if something is true, it should not need performance to appear real, it should stand on its own. I did not want a belief that depended on me avoiding hard questions; I wanted something honest enough to survive the cold logic of the world."
        }
        p {
            "The more I thought, the more I saw that fear keeps many people from questioning what they have always known. Fear of being wrong, fear of disappointing others, and fear of losing belonging can quietly shape how far a person is willing to think. I felt that fear too, but I also felt something stronger, which was the need to know what is actually true. Living with unanswered questions began to feel heavier than facing difficult answers. I started to understand that avoiding questions does not protect truth, it only protects comfort, and comfort, while useful in small ways, can never replace the solid ground of reality when the stakes are this high. That realization pushed me forward, even when I did not yet know where the search would lead."
        }
        h2 { id: "growing-up-under-the-weight-of-unanswered-questions",
            a {
                href: "#growing-up-under-the-weight-of-unanswered-questions",
                class: "header",
                "Growing Up Under the Weight of Unanswered Questions"
            }
        }
        p {
            "When I was younger, I believed that religion was mainly about obedience, about doing what you are told and staying within clearly defined boundaries that were already accepted by everyone around me. I learned early on that saying the right words and following the expected actions mattered more than fully understanding what those words meant. At the time, I did not question this deeply, because when you are surrounded by certainty, it feels natural to trust it without resistance. But even then, there was a quiet part of my mind that kept noticing the gap between knowing something and simply repeating it. That gap did not feel dangerous at first, but it did feel unfinished, like a signal that had been sent but never received. Over time, that unfinished feeling began to grow, because understanding is something the mind naturally seeks whether you allow it or not, even if the signal has gone dark."
        }
        p {
            "I was often taught, directly or indirectly, that doubt was something to avoid, something that could weaken faith rather than strengthen it. But that idea never fully made sense to me, because if something is true, then questioning it should not destroy it, but clarify it. I started to feel that doubt was not the opposite of belief, but a step toward deeper understanding. Because when you ask a real question, you are not trying to destroy truth, you are trying to find it more clearly. Ignoring doubt does not remove it, it only hides it temporarily, allowing it to grow in the background. I experienced this firsthand, because the more I avoided my questions, the stronger they returned later. It felt like trying to hold something underwater that naturally wants to rise to the surface, an attempt to silence a voice that refused to be forgotten."
        }
        p {
            "As I entered my teenage years, those questions became more serious and more difficult to ignore. I began to think deeply about things like suffering, fairness, and the structure of reality itself, as I've explored in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/an-empty-life-filled-with-constant-suffering.md",
                "An Empty Life Filled With Constant Suffering"
            }
            ". I could not understand how a just and powerful God could exist in a world where pain seemed so unevenly distributed. Some people struggle endlessly despite their efforts, while others seem to move forward with little resistance, and that imbalance felt impossible to ignore. I wanted answers that did not simply explain away the problem, but truly engaged with the weight of it. Because real problems require real explanations, not quick responses designed to end the conversation. I found myself thinking about these questions alone, often without anyone around me who could answer them in a way that satisfied both my mind and my sense of reality. And that isolation made the search feel even more personal and necessary."
        }
        p {
            "The more I searched for answers, the more I noticed that many responses I received were designed to protect belief rather than explain truth. They often felt like they were closing the door instead of opening it. This created a kind of internal conflict, because I wanted to believe, but I also wanted to understand. And when those two desires are not aligned, it creates a tension that is difficult to live with. I began to feel that I was being asked to accept conclusions without fully examining the reasoning behind them, a situation that felt like being trapped in a system of words with no connection to the real thing. If something is true, it should become stronger when examined in the light of day, not weaker. That idea stayed with me and continued to shape how I approached everything afterward."
        }
        p {
            "At the same time, my personal life was not easy, and that made these questions feel even more urgent. I experienced homelessness, unemployment, and the effects of living in a country affected by war and instability. On top of that, I was dealing with ADHD and autism, which made it harder for me to fit into systems that already felt difficult to navigate, as I detailed in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/technology-has-destroyed-my-livelihood.md",
                "Technology Has Destroyed My Livelihood"
            }
            ". These experiences were not abstract problems; they were real and constant pressures that shaped how I saw the world. I needed a belief system that could make sense of suffering in a way that was honest and meaningful. Because when life becomes difficult, shallow answers are no longer enough. You need something that can carry real weight, not just something that sounds comforting for a moment. And that need pushed me to keep searching even when it was exhausting."
        }
        p {
            "I started to understand that belief is not strong simply because it is widely accepted or inherited from others. A belief becomes strong when it is tested, examined, and still remains meaningful after that process. This realization changed how I approached everything I had been taught. I no longer wanted to rely on authority alone; I wanted to understand the reasoning behind what I believed. Because without understanding, belief can easily become fragile, it may appear stable on the surface, but it can collapse when faced with the cold, mathematical logic of a system that sees no soul. I did not want that kind of fragile belief, especially when dealing with something as important as truth. So I made a decision, even if I did not say it out loud at the time, that I would follow the questions wherever they led."
        }
        p {
            "That decision marked a turning point in my life, even though it did not lead to immediate answers. Instead, it led to more questions, more searching, and more moments of uncertainty. But those moments were different from before, because they were honest. I was no longer avoiding my doubts; I was engaging with them directly. And even though that process was difficult, it also felt more real. Because there is a difference between comfortable certainty and honest uncertainty. One feels stable but shallow, while the other feels difficult but meaningful. I chose the second path, not because it was easier, but because it was true to what I was experiencing, as I've always said, truth doesn't need a performance."
        }
        h2 { id: "why-i-started-looking-beyond-what-i-had-been-taught",
            a {
                href: "#why-i-started-looking-beyond-what-i-had-been-taught",
                class: "header",
                "Why I Started Looking Beyond What I Had Been Taught"
            }
        }
        p {
            "During my college years, something began to shift in a way that felt both natural and unavoidable, because I was no longer only surrounded by people who thought the same way I did. I was studying electrical engineering in a country that was slowly falling apart, and that environment itself forced me to think more seriously about reality. I started meeting people from different backgrounds, different beliefs, and different ways of understanding life. These were not imaginary opponents or simplified ideas, but real people with real thoughts and real experiences. That alone challenged many assumptions I did not even realize I was holding. Because it is easy to believe something about others when you have never truly listened to them. But once you sit with someone and hear them honestly, your perspective begins to expand whether you want it to or not."
        }
        p {
            "What surprised me most was not that people believed differently, but that many of them were thoughtful, kind, and deeply serious about truth. This directly challenged the simplified image I had been given about people outside my belief system. I expected confusion or carelessness, but instead I found reflection and sincerity. That forced me to reconsider how I understood belief itself. Because if good and thoughtful people can arrive at different conclusions, then the process of belief must be more complex than I was taught. It means truth is not just inherited, but discovered through experience, reflection, and reasoning. And once that realization settles in, it becomes impossible to ignore."
        }
        p {
            "For the first time in my life, I listened without preparing a response in my head. I did not try to defend my position immediately or correct the other person before they finished speaking. I simply listened to understand, and that alone changed something deep inside me. Because true listening removes the illusion that you already know everything important. It opens a space where real learning can happen. And in that space, I began to see how limited my earlier understanding had been. Not because I was incapable, but because I had never been exposed to enough perspectives to challenge it. That moment was not dramatic, but it was powerful in a quiet and lasting way."
        }
        p {
            "I began to realize that protecting an idea from challenge is not the same as proving it true. In fact, the need to protect something too carefully can sometimes indicate that it is weaker than it appears. This idea stayed with me because it made logical sense. If something is true, it should not need to be hidden from examination. It should be able to stand in the open and remain consistent. That became a standard I started applying to everything I believed. And once you apply that standard honestly, many things begin to look different. Because truth does not fear light, only uncertainty does."
        }
        p {
            "So I started reading more seriously, not to confirm what I already believed, but to understand what others believed and why. I read about Christianity, philosophy, and the history of religious thought. I watched documentaries in internet cafes when I had access, and listened to long discussions on a cheap phone that barely worked. These were not easy conditions, but they were enough to keep me searching. Because when you truly want to understand something, you use whatever tools you have. And the more I learned, the more I realized how much there was to explore. That realization did not discourage me, it motivated me even more."
        }
        p {
            "One idea became central to my thinking during this time, and it was simple but powerful. If something is true, then examining it closely should make it clearer, not more confusing. Truth should become stronger under pressure, not weaker. This idea acted like a filter for everything I encountered. Whenever I found something that became unclear or inconsistent under careful thought, I questioned it. And whenever something remained stable and meaningful even after deep examination, I paid closer attention to it. This process was not fast, but it was honest. And honesty, even when slow, leads somewhere real."
        }
        p {
            "Looking back, I can see that this period was not just about learning new ideas, but about changing how I approached truth itself. I moved from passive acceptance to active searching. I stopped relying only on what I had been told, and started building my understanding through experience and reasoning. That shift was not easy, but it was necessary. Because truth is not something you simply receive once and keep forever. It is something you must be willing to examine continuously. And once you begin that process sincerely, it reshapes the way you see everything."
        }
        h1 { id: "why-christianity-began-to-make-more-sense-to-me",
            a {
                href: "#why-christianity-began-to-make-more-sense-to-me",
                class: "header",
                "Why Christianity began to make more sense to me"
            }
        }
        p {
            "As I studied Christianity more seriously, I did not approach it with the intention of accepting it, but with the intention of testing it. I wanted to see if it could stand under the same level of examination I was applying to everything else, to see if it was just another layer of language or something more. At first, I expected to find weaknesses quickly, because that is what I had always assumed. But instead, I found something different, something that felt structured, coherent, and deeply connected to real human experience. This surprised me because I was not looking for it. I was simply following the same method I had applied everywhere else. And what I found made me pause and think more carefully."
        }
        p {
            "One of the first things that stood out to me was how Christianity deals with suffering. It does not deny it, minimize it, or explain it away quickly. Instead, it places suffering at the center of its story. It says that God entered into human life and experienced suffering directly. As I wrote in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/it-is-always-the-russians.md",
                "It is always the Russians"
            }
            ", the Soviets killed God by cutting the signal, by making the world cold and godless to replace Him with machines. But Christianity claims something even more radical: that the signal was not just cut by men, but that God allowed the signal to be \"killed\" in the death of Jesus to be with us in the silence. This is a very different claim from a distant God who observes from above."
        }
        p {
            "I had lived through difficult conditions, moments where explaining how I felt seemed impossible. In those moments, the idea of a distant and untouched God felt disconnected from reality. But the idea of a God who enters suffering changes that completely. It suggests that pain is not ignored, but understood from within. That does not remove suffering, but it gives it context. And context matters, because it changes how we interpret our experiences. This was one of the first times I felt that a belief system was truly engaging with reality instead of avoiding it. And that made me take it seriously."
        }
        p {
            "Another important aspect was that Christianity is built on public claims, not just private experiences. It is based on witnesses, events, and historical assertions that can be examined. This gave it a different kind of weight compared to systems based only on internal feelings or abstract patterns. Because feelings are important, but they are not always reliable on their own. A belief that connects both inner experience and external evidence is stronger than one that depends on only one side. This balance between the internal and external made Christianity feel more grounded. And that grounding made it easier to engage with logically."
        }
        p {
            "I also noticed that Christianity does not present life as simple or easy. It acknowledges failure, weakness, and brokenness in a very direct way. This honesty stood out to me because it matched what I had seen in real life. People struggle, fail, and carry burdens they cannot always fix, something I know all too well from my own exhaustion. A system that ignores that reality feels incomplete. But Christianity begins with that reality and builds from it. That made it feel more realistic and less idealized. And realism is essential when searching for truth."
        }
        p {
            "The more I examined it, the more I found that its ideas were interconnected in a way that made sense. The concepts of suffering, redemption, love, and justice were not separate, but part of a larger structure. Each part supported the others, creating a system that felt complete. This kind of coherence is difficult to ignore. Because random ideas do not usually form a consistent whole without deeper structure. And when you find that structure, it suggests something intentional rather than accidental. That realization made me continue exploring instead of dismissing it."
        }
        p {
            "At this point, I was no longer just observing Christianity from a distance. I was engaging with it seriously, testing it, questioning it, and trying to understand it fully. I was not ready to accept it yet, but I could no longer ignore it either. It had passed the first level of examination, and now it required deeper attention. This marked a new phase in my search. Because once something proves itself worth serious consideration, you owe it an honest evaluation. And that is exactly what I continued to do."
        }
        h2 { id: "the-trinity-and-the-idea-of-a-relational-god",
            a {
                href: "#the-trinity-and-the-idea-of-a-relational-god",
                class: "header",
                "The Trinity and the idea of a relational God"
            }
        }
        p {
            "At first, the concept of the Trinity was confusing and difficult to understand. It seemed to contradict basic logic, because it described God as one and yet also as Father, Son, and Holy Spirit. My initial reaction was to reject it as something unclear or unnecessary. But instead of dismissing it immediately, I decided to examine it more carefully. As I've argued in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/language-is-limited-asi-is-impossible.md",
                "Language is Limited. ASI is Impossible."
            }
            ", symbols and words often fail to capture the full size of a thought, so perhaps my logical boxes were just too small for the reality."
        }
        p {
            "As I thought about it more, I realized that the Trinity is not trying to break logic, but to describe something beyond simple categories. It is not saying three separate gods, but one unified reality expressed in relationship. This idea began to shift how I understood it. Because it was not about numbers, but about nature. It was describing a form of existence where relationship is built into the very structure of being. And that is a very different kind of claim. It moves the discussion from mathematics, the cold logic the Soviets tried to use to skin God, to meaning."
        }
        p {
            "This led me to a deeper realization about love. If God exists as a relational being, then love is not something created later, but something eternal. That means love is not dependent on creation, but part of reality itself. This idea felt powerful because it gave love a foundation. It was no longer just a human feeling, but something rooted in the nature of existence. And that changes how you see everything. Because it suggests that connection and relationship are not accidental, but essential."
        }
        p {
            "This idea also connected with my personal experience. I had often felt isolated and disconnected, like I was observing life rather than fully part of it. The idea that reality itself is relational challenged that feeling. It suggested that connection is not something optional, but something fundamental. And that gave a different meaning to human relationships. They were not just temporary interactions, but reflections of something deeper. That made the concept feel more real and less abstract."
        }
        p {
            "The idea of incarnation also became more meaningful in this context. If God is relational, then entering human life is not a contradiction, but an expression of that nature. It means God does not remain distant, but engages directly with creation, the Word becoming flesh, moving beyond the limitation of language into the reality of a body. This idea answered a question I had carried for a long time: Does God truly understand human experience? The concept of incarnation answers that with a clear yes. And that clarity made it easier to accept."
        }
        p {
            "The more I reflected on it, the more I saw that the Trinity was not an obstacle, but a key part of the overall structure. It explained how love, relationship, and unity could exist together. It provided a framework that connected many other ideas. And that made it more than just a difficult concept. It made it meaningful. Because meaning often lies behind complexity. And once you understand that complexity, it becomes something valuable rather than confusing."
        }
        p {
            "By this point, I was no longer rejecting the idea, but trying to understand it fully. I realized that some truths are not simple, but that does not make them false. In fact, deeper truths often require deeper thinking. The Trinity was one of those ideas. It challenged me, but it also expanded my understanding. And that made it worth exploring. Because anything that expands your understanding of reality is worth serious attention."
        }
        h2 { id: "why-forgiveness-grace-and-redemption-became-real-to-me",
            a {
                href: "#why-forgiveness-grace-and-redemption-became-real-to-me",
                class: "header",
                "Why forgiveness, grace, and redemption became real to me"
            }
        }
        p {
            "One of the most powerful things I discovered in Christianity was how it speaks about human failure in a way that is both honest and complete. It does not pretend that people are only slightly flawed or that they can fix themselves easily with enough effort. Instead, it says clearly that human beings are deeply broken in ways that reach beyond simple behavior. This matched the pattern I saw when the Soviets tried to replace faith with systems of control, they ignored the soul and focused only on the output. Christianity, however, addresses the brokenness directly. At first, this idea felt heavy, but the more I thought about it, the more it matched what I had already seen in life."
        }
        p {
            "In my own life, I had experienced moments where I knew I needed change but could not fully achieve it on my own. There were thoughts and struggles that did not simply disappear with effort or intention. This made me realize that human weakness is not just a matter of discipline, but something more complex. If the problem is deeper, then the solution must also be deeper. Christianity does not ignore this, but addresses it directly. It says that people need more than guidance; they need transformation. That distinction is important because it shifts the focus from self-repair to something greater. And that idea began to make sense to me."
        }
        p {
            "The concept of grace was something I had not fully understood before. Grace means receiving something you did not earn and could not earn on your own. At first, this felt unfair, because it goes against the idea that everything must be deserved, the cold, meritocratic logic of the world. But then I realized that life itself is full of things we do not earn, including existence, opportunities, and even moments of kindness. Not everything operates on strict fairness. Some things operate on generosity. And when I understood grace in that way, it no longer felt strange, but deeply meaningful."
        }
        p {
            "Forgiveness also became more real to me through this process. Not as a simple word, but as something that carries real weight. Because true forgiveness is not just ignoring wrong actions, but dealing with them in a meaningful way. Christianity presents forgiveness as something costly, not something cheap. It says that the cost is not ignored, but absorbed, just as God absorbed the weight of a broken world on the cross. That idea is powerful because it respects both justice and mercy at the same time. And finding that balance is something many systems struggle to do."
        }
        p {
            "I thought about times in my life when I needed forgiveness but had no way to earn it. Moments where the gap between who I was and who I wanted to be felt too wide to cross. In those moments, effort alone was not enough. I needed something beyond myself. The idea that forgiveness could come from outside, not as a reward but as a gift, changed how I saw those situations. It offered a way forward instead of leaving me stuck in failure. And that made it more than just an idea, it made it something practical and real."
        }
        p {
            "Redemption, in this context, became more than just a religious term. It became a way of understanding how broken things can be restored. Not by pretending the damage never happened, but by transforming it into something meaningful. This idea is deeply logical because it does not deny reality, it works through it. It acknowledges the damage and then provides a path beyond it. That is very different from the Soviet method of ignoring the soul to build a perfect machine. It is about facing the damage and overcoming it. And that approach felt more complete to me."
        }
        p {
            "All of these ideas together created a framework that felt both honest and hopeful. It did not deny human weakness, but it also did not leave it as the final answer. It provided a way to understand failure without being defined by it. And that is something I had been searching for without fully realizing it. Because everyone fails in some way, but not everyone finds a way beyond that failure. Christianity offered that path in a way that felt grounded and meaningful. And that made it something I could not ignore."
        }
        h2 { id: "why-i-ended-up-here-and-why-i-still-keep-going",
            a {
                href: "#why-i-ended-up-here-and-why-i-still-keep-going",
                class: "header",
                "Why I ended up here, and why I still keep going"
            }
        }
        p {
            "The idea of the resurrection was one of the most difficult parts for me to accept at first. It seemed unlikely, almost impossible, and easy to dismiss as a story created to bring comfort. But instead of rejecting it immediately, I decided to examine it carefully. If the Soviets really \"killed\" God as I claimed, if the connection to the divine signal was broken, then the resurrection is the ultimate proof that the signal cannot be permanently silenced. It is the moment where the \"cut\" thread is tied back together from the other side. So I took the time to look at the arguments, the context, and the consequences of the claim."
        }
        p {
            "One of the things that stood out to me was the transformation of the early followers. These were not powerful people with influence to protect. They were ordinary individuals who had experienced fear, loss, and uncertainty, the same shadows that loom over my own life. Yet something changed them in a way that made them willing to stand publicly for what they believed. That kind of transformation requires explanation. It is not enough to say they believed something strongly. The question is why they believed it so strongly after what they had experienced. And that question stayed with me."
        }
        p {
            "I began to understand that people may die for beliefs they think are true, but it is much harder to explain why they would suffer for something they know is false. Especially when denying it could save them. This does not automatically prove the resurrection, but it makes simple explanations less convincing. It forces you to consider that something significant must have happened, a return of the light even after the Soviets of that age thought they had extinguished it forever. And once you accept that, you have to explore what that something could be. This made the resurrection harder to dismiss casually."
        }
        p {
            "Beyond the historical side, the resurrection also had a personal meaning for me. It represents the idea that death and suffering are not the final outcome. That what appears to be the end is not necessarily the end. This idea connected with something deep in my own experience. Because during difficult times, I often felt that things would never improve. The idea that there is more beyond the worst moment gives a different perspective. It does not remove pain, but it changes how you see it, the death of Jesus was not a failure of God, but His deepest entrance into our world."
        }
        p {
            "Over time, I realized that my search had taken me through many different perspectives. I had considered atheism, philosophy, and other systems of thought. Each one offered something valuable, but none of them fully addressed everything I had experienced. Some explained parts of reality well, but left other parts unanswered. I needed something that could hold everything together, not just pieces of it. That was the standard I had set from the beginning. And I continued to apply it honestly."
        }
        p {
            "What I found was that Christianity was the only framework that could hold both the logical and the personal aspects of life together. It addressed suffering, justice, meaning, and hope in a way that felt complete. It did not require me to ignore reality, but to engage with it more deeply. It did not remove questions, but gave them a place within a larger structure, a signal that remains loud despite every attempt to drown it out with machines and noise. And that made it sustainable. Because a belief system must be able to handle real life, not just ideal situations."
        }
        p {
            "I did not arrive here because it was easy, expected, or comfortable. I arrived here because I followed the search honestly, even when it was difficult and uncertain. I still have questions, and I still experience moments of doubt. But those no longer feel like threats; they feel like part of the process. Because truth does not depend on perfect certainty, but on continued honesty. And what I have found is something that continues to make sense of my life as I live it. That is why I keep going, not because everything is solved, but because what I have found is strong enough to keep exploring."
        }
        p { "Till next time 👋!" }
    }
}
#[component(no_case_check)]
pub fn LlmsAreUsefullLmmsWillBreakReality() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey everyone 👋," }
        p {
            "In my previous post, "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/language-is-limited-asi-is-impossible.md",
                "Language is Limited. ASI is Impossible."
            }
            ", I spent a long time explaining why language is not the same thing as thought, why words are not the same thing as understanding, and why a machine built on text alone will never cross the wall into true superintelligence. I still believe all of that, and I will not take any of it back, because the argument was honest and the logic was solid. But today I want to go further. I want to talk about something that has been sitting in my head for years, growing louder every day, and I need to get it out before it eats me alive. I want to talk about why large language models are still genuinely useful, despite their limits, and why large mathematical models, as introduced in this whitepaper draft, are something far more serious, something that could actually begin to crack the surface of reality itself. I know that sounds extreme, and I know some people will read that sentence and roll their eyes, but I am asking you to stay with me, because the argument I am about to make is not based on hype or fantasy. It is based on what I have seen, what I have built, and what I understand about the difference between describing the world and actually modeling the world. That difference is the whole point of this post, and once you see it clearly, everything else falls into place."
        }
        p {
            "I have been thinking about this ever since I wrote "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/an-empty-life-filled-with-constant-suffering.md",
                "An Empty Life Filled With Constant Suffering"
            }
            ", where I talked about how words cannot fully capture my thoughts, and how language always falls short of the real thing inside our heads. That frustration is what led me here, because if language is limited, then we need to ask what comes after language, and the answer is not more language. The answer is structure, equations, simulation, and direct modeling of the physical world. That is what large mathematical models point toward, not because they are perfect today, but because they represent a direction that goes beyond text and into something much deeper. I wrote in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/it-is-always-the-russians.md",
                "It is always the Russians"
            }
            " that the Russians took God's skin and wrapped it around a machine, and I stand by that image, because that is exactly what happens when you take human knowledge, strip it of its soul, and feed it into a system that only knows patterns. But what if the next generation of models does not stop at patterns in text? What if it starts learning patterns in the physical world itself? That is the question I cannot stop thinking about, and that is the question this post is built around."
        }
        h2 { id: "llms-are-useful-and-i-am-not-going-to-pretend-otherwise",
            a {
                href: "#llms-are-useful-and-i-am-not-going-to-pretend-otherwise",
                class: "header",
                "LLMs Are Useful And I Am Not Going to Pretend Otherwise"
            }
        }
        p {
            "Let me start with something that might surprise people who read my earlier posts. I think large language models are useful. I am not going to pretend otherwise, because pretending otherwise would be dishonest, and I have spent too much of my life being honest about hard things to start lying about easy ones. I wrote in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/as-engineers-llms-should-pay-us-for-tokens-usage.md",
                "As Engineers, LLMs should pay us for tokens usage"
            }
            " about how the system exploits engineers, and I still believe that, but that does not mean the tool itself is worthless. A hammer can be used to build a house or to crush a hand, and the fact that someone charges you too much for the hammer does not mean the hammer cannot drive a nail. LLMs can summarize documents faster than I can read them, they can generate boilerplate code faster than I can type it, they can help me think through problems by acting as a sounding board, and they can translate between languages in ways that used to require expensive human translators. These are real capabilities, not illusions, and anyone who denies them is not being serious. I have used them myself, and they have saved me hours of work on tasks that would have otherwise drained me completely. The tool is real, the output is real, and the time saved is real."
        }
        p {
            "But here is the thing that people keep getting wrong, and this is where the trouble starts. People confuse usefulness with intelligence, and those are not the same thing at all. A calculator is useful, but nobody calls a calculator intelligent, because we understand that it is following rules, not thinking. A search engine is useful, but nobody calls a search engine wise, because we understand that it is retrieving information, not understanding it. An LLM is useful in the same way, except the output looks so much like human speech that people forget they are watching a very advanced pattern machine, not a mind. The words come out smooth, the grammar is clean, the tone is confident, and that smoothness tricks people into believing there is understanding behind the words, when really there is only statistical prediction shaped by billions of examples. That confusion is dangerous, because it leads people to trust the output more than they should, and trust without verification is how mistakes become disasters. I have seen engineers deploy LLM-generated code without checking it, and I have seen the bugs that resulted, and those bugs were not small. They were the kind of bugs that come from blind faith in a system that sounds right but does not actually know what it is doing."
        }
        p {
            "The usefulness of LLMs also has a ceiling, and that ceiling is made of language itself. I explained this in detail in my previous post, but I want to repeat the core idea here because it matters. Language is a compression of thought, not the thought itself. When I type a prompt, I am taking something rich and multidimensional from inside my head and squeezing it into a flat sequence of words. The model receives those words, runs them through its weights, and produces another flat sequence of words. At no point does anyone touch the actual structure of the problem. At no point does anyone engage with the real mechanism behind the question. The whole exchange happens inside a symbolic layer that sits on top of reality, never inside reality itself. That is fine for many tasks, because many tasks only need the symbolic layer. If I want a summary, I need text. If I want a translation, I need text. If I want a draft email, I need text. But if I want to understand why a bridge is failing, or how a drug interacts with a protein, or what the weather will look like in three days, I need something much deeper than text. I need structure, I need equations, I need simulation, and I need a model that can engage with the physical world directly, not just talk about it."
        }
        p {
            "This is also why hallucination is such a serious problem and not just a minor bug that will be fixed with more data or better training. Hallucination happens because the model does not know anything in the way a human knows things. It does not have a grounded understanding of truth or falsehood, because it was never trained on truth or falsehood. It was trained on text, and text contains truth and lies in equal measure, all mixed together in a soup that the model cannot separate. When the model generates something false, it is not making a mistake in the way a human makes a mistake. It is doing exactly what it was designed to do, which is produce the most statistically likely next token given the context. Sometimes that produces truth, and sometimes it produces garbage, and the model cannot tell the difference because it has no access to the ground truth of reality. It only has access to the patterns in the text it was trained on, and those patterns include errors, biases, contradictions, and outright fabrications. This is not a fixable flaw in a fundamentally sound system. It is a structural limitation of any system that learns from text alone, because text alone is not enough to anchor a model to the truth of the physical world."
        }
        p {
            "I also want to be clear about something that bothers me deeply. The companies that build these models know exactly what the limitations are, and they market around them instead of being honest about them, like what "
            a { href: "https://www.youtube.com/watch?v=5VRgk7_X7oc",
                "recently Sam Altman lied about here"
            }
            " instead of being honest about calling it hallucination. They show you the best outputs and hide the worst ones. They put guardrails on the edges and call it safety. They charge you per token while extracting your knowledge and your judgment for free. I wrote about all of this in my post about "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/as-engineers-llms-should-pay-us-for-tokens-usage.md",
                "how LLMs should pay engineers"
            }
            ", and the core argument has not changed. The tool is useful, but the business model is exploitative, and the marketing is actively misleading people about what the tool can and cannot do. Every time a company says their model can reason, they are stretching the word reason beyond its breaking point, because what the model does is not reasoning in any serious philosophical or scientific sense. It is pattern completion, dressed up in confident language, and sold to a world that is too tired and too busy to look beneath the surface. That is not progress. That is a sales pitch."
        }
        p {
            "Despite all of this, I want to be fair. LLMs have changed how I work, and some of those changes have been genuinely positive. They help me draft ideas faster, they help me explore different framings of a problem, they help me catch errors in my own writing, and they sometimes suggest connections I had not considered. These are real benefits, and I would be lying if I said otherwise. But benefits and limits can exist in the same system at the same time, and pretending the limits do not exist because the benefits are impressive is exactly the kind of sloppy thinking that I have been arguing against since my first post. The truth is simple. LLMs are useful tools trapped inside a symbolic cage, and no amount of scaling will break them out of that cage, because the cage itself is language, and language is not the world. That is not a small problem. That is the whole problem."
        }
        p {
            "I also think people need to understand that the usefulness of LLMs does not prove that the path to artificial general intelligence runs through text. Usefulness and progress toward AGI are completely separate questions, and confusing them has led to some of the worst thinking in modern technology. A typewriter is useful for writing, but no one would say that building a faster typewriter is the path to creating a mind. A microscope is useful for seeing small things, but no one would say that building a bigger microscope is the path to understanding consciousness. LLMs are useful for generating text, but that does not mean that generating better text is the path to building a system that truly understands the world. The path to real understanding goes through structure, through mathematics, through simulation, and through direct engagement with physical reality. Text is a waypoint, not the destination. And the sooner people accept that, the sooner we can start building systems that actually matter."
        }
        h2 { id: "language-is-still-a-cage-and-i-already-proved-it",
            a {
                href: "#language-is-still-a-cage-and-i-already-proved-it",
                class: "header",
                "Language Is Still A Cage And I Already Proved It"
            }
        }
        p {
            "I wrote an entire post about this, "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/language-is-limited-asi-is-impossible.md",
                "Language is Limited. ASI is Impossible."
            }
            ", and I am not going to repeat every argument here, because you can read it yourself and it still stands. But I want to draw out the parts that connect directly to what I am about to say about mathematical models, because the connection is the whole point of this post. Language is a compression layer, and compression always loses information. When I describe a sunset in words, I lose the colors, the temperature, the wind, the smell of the air, the feeling in my chest, and the thousand tiny details that make the moment what it is. The words get close, but they never arrive, and the gap between the words and the experience is where the real meaning lives. That gap is permanent, because language was never designed to carry the full weight of reality. It was designed to help humans communicate quickly, not to represent the world completely. And any system built on top of language inherits that same permanent gap."
        }
        p {
            "This matters for AI because the whole dream of artificial superintelligence through text is built on the assumption that the gap can be closed with more data, more parameters, and more compute. I do not believe that, and I explained why in detail. More text gives you more text, not more understanding. You can pile up a billion books and you will have a very large pile of books, but you will not have a single atom, a single photon, or a single heartbeat. The pile of words is always about the world, never part of the world, and that distinction is not a technicality. It is the deepest structural limitation in the entire field. A model trained purely on text can become very good at producing text, and that is exactly what happened, and that is exactly why people are impressed. But being good at producing text is not the same as understanding the structure of the world, and it never will be, no matter how many GPUs you throw at the problem."
        }
        p {
            "I also want to remind people of something I said before, which is that the brain is not a text machine. Your brain does things that have nothing to do with language all day long. You catch a ball without narrating the physics to yourself. You recognize a face without describing it in a sentence first. You feel danger before you can name it. You navigate a crowded street without writing an essay about pedestrian dynamics. These are all forms of intelligence, real and powerful forms, and they happen entirely outside the world of language. If intelligence were really just text, then mute people would be unintelligent, deaf people would be unintelligent, babies would be unintelligent, and animals would be unintelligent. But they are not. They are all deeply intelligent in ways that language cannot reach, because intelligence is broader than words, deeper than grammar, and older than writing itself. Any model that stays inside language stays inside only one narrow channel of intelligence, and that channel, while useful, is not the whole river."
        }
        p {
            "The cage of language also creates a false sense of confidence that I find genuinely frightening. When a model produces a fluent, confident, well-structured answer, people tend to believe it, because humans are wired to trust confident speech. This is a survival mechanism that evolved in a world where confident speakers were usually knowledgeable leaders, but it breaks down completely when the confident speaker is a machine that cannot verify its own output. I have watched people argue with each other using LLM-generated text as evidence, without either person checking whether the text was actually true. I have watched students submit LLM-generated essays and believe they understood the material, when really they understood nothing, because the model did the thinking for them and they only did the copying. I have watched engineers trust LLM-generated code in production systems, where a single hallucinated function call can bring down a service that thousands of people depend on. This is not intelligence. This is a confidence trick performed at industrial scale, and the victims are the people who trusted the output without questioning the source."
        }
        p {
            "This is also why I said induction has a ceiling. LLMs learn by looking at what already exists, they study the past, and they predict the next likely token. That means they are always downstream from human work, always recycling what has already been said, always remixing old patterns into new arrangements. They can generalize, they can surprise, and they can sometimes create things that look entirely fresh. But the freshness is still born inside a machine that feeds on prior expression, and that is not the same as genuine novelty. Genuine novelty comes from contact with the world, from experiment, from failure, from observation, from the moment when reality pushes back against your assumptions and forces you to see something you did not expect. A text model never has that moment, because it never touches reality. It only touches the shadows of reality that humans have already cast into words, and shadows, no matter how detailed, are still just shadows."
        }
        p {
            "Let me also say this as plainly as I can, because I think people need to hear it without any padding. The limit of language is not a future problem that might be solved tomorrow. It is a present fact that shapes everything LLMs can and cannot do right now. Every time a model hallucinates, that is the limit showing itself. Every time a model produces a confident wrong answer, that is the limit showing itself. Every time a model fails to generalize to a new domain, that is the limit showing itself. Every time a model cannot verify its own output against the real world, that is the limit showing itself. These are not bugs that will be patched in the next version. They are symptoms of a structural constraint that cannot be removed by making the model bigger, faster, or more expensive. The constraint is language itself, and language is the medium the model lives in, and you cannot fix the medium by adding more of the same medium. You need a different medium. And that is where mathematical models come in."
        }
        p {
            "I want to close this section by saying that recognizing the limits of language is not the same as dismissing language. Language is beautiful, powerful, and irreplaceable for human communication. I would not be writing this post without it, and you would not be reading it without it, and neither of us would be thinking about these ideas without the bridge that language builds between our minds. But a bridge and a destination are different things, and confusing them has led the entire AI industry down a path that is useful but ultimately narrow. The destination is not better text. The destination is better understanding of the world. And better understanding requires tools that go beyond text, into images, into sound, into motion, into structure, into equations, and into the physical fabric of reality itself. That is what the next section is about, and that is where the real story begins."
        }
        h2 { id: "the-world-is-not-made-of-words",
            a { href: "#the-world-is-not-made-of-words", class: "header",
                "The World Is Not Made Of Words"
            }
        }
        p {
            "The world is not made of words. The world is made of atoms, forces, fields, waves, particles, energy, matter, time, and space. None of these things are linguistic. None of them are made of grammar. None of them care about syntax, vocabulary, or sentence structure. An earthquake does not consult a dictionary before it shakes the ground. A virus does not read a paper before it mutates. A star does not check Wikipedia before it explodes. The world operates on physics, chemistry, biology, and mathematics, and all of these run independently of human language, because they existed for billions of years before the first human ever spoke a word. This is not a philosophical opinion. This is a plain fact that can be verified by looking at anything in the natural world and noticing that it works perfectly well without being described."
        }
        p {
            "And yet, we have built an entire industry of intelligence on the assumption that language is the right starting point for understanding the world. We have poured billions of dollars into systems that learn from text, generate text, and are evaluated by text, as if the universe were a giant essay waiting to be read. This assumption is not crazy, because language does encode a huge amount of human knowledge, and that knowledge is valuable. But the assumption is also deeply incomplete, because the knowledge encoded in language is always a filtered, compressed, lossy version of the real thing. When a physics textbook describes the Navier-Stokes equations, it is pointing at a truth, but the truth itself lives in the behavior of fluids, not in the ink on the page. When a medical textbook describes how a drug interacts with a receptor, it is pointing at a mechanism, but the mechanism itself lives in the chemistry of molecules, not in the sentences that describe them. The map is useful, but the map is not the territory, and building a system that only reads maps will never be the same as building a system that can navigate the actual territory."
        }
        p {
            "This is why I care about modeling the physical world directly, not through language, but through structure. The world has regularities that can be captured in mathematical form, and those forms are far more powerful than any verbal description. A single equation can encode an entire class of physical behavior in a few symbols, while a verbal description of the same behavior might take pages and still miss important details. The equation F equals ma describes the relationship between force, mass, and acceleration in a way that is precise, testable, universal, and compact. No sentence can match that level of compression, because sentences are built for communication between humans, not for exact specification of physical law. Mathematics is the language of the universe, and if we want machines that understand the universe, we need to teach them mathematics, not just English."
        }
        p {
            "This is also where the idea of physics-informed machine learning becomes important. Researchers have shown that neural networks can be trained not just on data but also on physical constraints, so that the model learns to respect known laws while fitting observed behavior. This is a huge deal, because it means the model is not just memorizing patterns in a dataset. It is learning within a framework that reflects the actual structure of the world. A physics-informed model cannot easily violate conservation laws, because those laws are built into the training process itself. This makes the model more reliable, more generalizable, and more scientifically meaningful than a model that learns from data alone. The difference is like the difference between a student who memorizes answers for an exam and a student who actually understands the subject. Both might get the same score on the test, but only one of them can handle a question they have never seen before, because understanding transfers and memorization does not."
        }
        p {
            "I also want to talk about neural operators, because they represent a step even further in this direction. A neural operator does not learn a single function from inputs to outputs. It learns a mapping between function spaces, which means it can take an entire field, like a temperature distribution or a pressure profile, and predict how that field evolves over time under given conditions. This is exactly the kind of computation that physics simulators do, except the neural operator learns to do it from data instead of from hand-coded equations. That matters because there are many real-world systems where the equations are too complex to solve by hand, or where the exact equations are not even known. In those cases, a neural operator can learn the dynamics directly from observation, which opens up a completely new way of doing science. You do not need to derive the equation first. You can let the machine learn it for you, and then check whether the learned equation matches reality."
        }
        p {
            "This is the direction that excites me, and it is the direction that most people in the AI conversation are completely ignoring. Everyone is talking about chatbots, about code generators, about writing assistants, and about the next version of the model that can hold a longer conversation. Nobody is talking about the fact that a different kind of model, trained on a different kind of data, using a different kind of objective, could learn the actual structure of the physical world. Not a verbal description of the world. Not a summary of the world. Not a persuasive essay about the world. The actual governing relations that make the world work. That is not a chatbot. That is a revolution in how humanity relates to reality itself, and the fact that it is happening quietly, in small research labs, while the whole world screams about ChatGPT, is one of the great ironies of our time."
        }
        p {
            "Let me put this as simply as I can, because I think simplicity is the best weapon against confusion. The world is not made of text, so text models will always be limited to describing the world from the outside. But the world is made of structure, and structure models can learn the world from the inside. A text model can tell you about gravity. A structure model can simulate gravity. A text model can describe a hurricane. A structure model can predict a hurricane. A text model can explain a chemical reaction. A structure model can run a chemical reaction in silico. The difference between telling and doing is the difference between a language model and a world model, and that difference is not small. It is the difference between reading about reality and actually modeling reality, and once you give a machine the ability to model reality, you give it the power to predict, to test, to explore, and eventually to intervene. That is where things get serious, and that is where the phrase \"break reality\" stops being a metaphor and starts being a description."
        }
        h2 { id: "why-mathematical-changes-everything",
            a { href: "#why-mathematical-changes-everything", class: "header",
                "Why Mathematical Changes Everything"
            }
        }
        p {
            "This is where I want to shift the conversation from what LLMs cannot do to what LMMs can do, because the transition from language-only models to large mathematical models is not a small upgrade. It is a category shift. A language model lives inside text. A mathematical model lives inside multiple streams of information at once, including text, images, video, audio, sensor data, and potentially any other type of structured input. That means a mathematical model is not limited to the symbolic layer of language. It can see a picture, hear a sound, watch a video, and process numerical data, all at the same time. That changes the entire game, because it opens the door to learning from the world directly, not just from descriptions of the world. And learning from the world directly is how humans learn, how animals learn, and how science works. It is the most natural and most powerful form of learning that exists."
        }
        p {
            "Think about what it means for a model to be able to process video. Video is not text. Video is a dense, continuous stream of visual information that encodes motion, depth, lighting, texture, occlusion, interaction, and physical dynamics. When a model watches a ball fall, it does not read a sentence about gravity. It sees gravity happening. It sees the acceleration, the trajectory, the impact, and the bounce. Those visual patterns contain physical information that no text description can fully capture, because text can only approximate continuous motion with discrete words. A model that learns from video can potentially learn the dynamics of the physical world without anyone telling it the equations, because the equations are already implicit in the motion it observes. Researchers have already shown that dynamical equations can be recovered from video data, where the system first learns latent state variables and then constructs a dynamical representation directly from observed motion. That is not science fiction. That is published science, and it is happening right now."
        }
        p {
            "This is also why I think the move from LLMs to LMMs is not just an engineering improvement but a philosophical shift in what we are trying to build. An LLM is trying to master language. An LMM is trying to perceive the world. Those are fundamentally different goals, and they lead to fundamentally different kinds of systems. A system that masters language can write, translate, summarize, and generate text with impressive fluency. A system that perceives the world can observe, model, predict, and interact with physical processes in ways that text alone can never support. Language mastery is a cognitive skill. World perception is the foundation of intelligence itself. Every intelligent creature on Earth perceives the world before it uses language, because perception is what gives language its meaning. Without perception, language is just empty symbols moving through a void. With perception, language becomes anchored to reality, and anchoring is exactly what current language models lack."
        }
        p {
            "The practical implications of this are enormous, and I do not think most people have thought them through. A mathematical model that can process images and text together can look at a medical scan and generate a diagnosis. A mathematical model that can process video and numerical data together can watch a manufacturing process and detect defects in real time. A mathematical model that can process satellite imagery and weather data together can predict natural disasters with greater accuracy than any text-based system ever could. These are not hypothetical applications. They are applications that are already being developed and tested, and they work because the model has access to information that text alone cannot provide. The image carries spatial structure. The video carries temporal dynamics. The sensor data carries physical measurements. All of these are forms of information that text can only approximate, and the approximation is always lossy, always incomplete, and always downstream from the real data. A mathematical model removes the middleman of language and lets the machine engage with the data directly, and that directness is the source of its power."
        }
        p {
            "I also want to connect this to something I wrote in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/technology-has-destroyed-my-livelihood.md",
                "Technology Has Destroyed My Livelihood"
            }
            ", where I talked about how the tech industry extracts value from people while giving back less and less. The arrival of mathematical models does not change that dynamic, and it might make it worse, because a system that can see, hear, and read is a system that can replace more kinds of human labor. But I am not going to lie about what the technology can do just because the economic system is unfair. The technology is real, the capability is real, and the direction is real. What we need is not to deny the technology but to demand that its benefits are shared fairly, which they currently are not. The same companies that scrape our code, our images, our voices, and our data to train these models are the same companies that charge us for the output and pay us nothing for the input. That was wrong with LLMs and it is even more wrong with LMMs, because mathematical models consume more kinds of human creation and consolidate more kinds of human skill into a single system that one company controls."
        }
        p {
            "Let me also address something that I think is underappreciated. The combination of multiple modalities does not just add capability. It creates capability that none of the individual modalities could produce alone. A model that can see an image and read text about that image can understand things that neither the image nor the text could communicate separately. The image provides spatial information that text cannot capture. The text provides contextual information that the image cannot express. Together, they create a richer representation than either one could create alone, and that richer representation is the foundation for deeper understanding. This is exactly how human cognition works. We do not understand the world through one channel. We understand it through the combination of vision, hearing, touch, memory, emotion, and language, all integrated into a single coherent experience. A mathematical model is the first step toward building a system that does something similar, and that step is far more important than anything that happened in the world of pure text models."
        }
        p {
            "I want to be careful here, because I do not want to oversell the current state of mathematical AI. The models that exist today are still limited, still make mistakes, still hallucinate, and still cannot truly understand the world in the way a human understands it. But the direction matters more than the current state, because the direction is pointing toward something genuinely new. The direction is pointing toward machines that can perceive, model, and interact with the physical world, not just talk about it. And once that direction produces systems that are reliable enough to trust, the world will change in ways that most people are not prepared for. That is not a threat. It is a prediction based on the trajectory I see, and I would rather prepare for it honestly than pretend it is not coming."
        }
        h2 { id: "equations-are-more-powerful-than-sentences",
            a {
                href: "#equations-are-more-powerful-than-sentences",
                class: "header",
                "Equations Are More Powerful Than Sentences"
            }
        }
        p {
            "This section is about something that most people in the AI conversation never talk about, because most people in the AI conversation are not scientists, not engineers, and not mathematicians. They are content creators, investors, and people who use ChatGPT to write emails. There is nothing wrong with that, but it means the conversation is missing the most important piece, which is that the real power of intelligence lies in equations, not in sentences. An equation can encode an entire class of physical behavior in a single line. A sentence can only describe one instance of that behavior in one particular context. The equation is general. The sentence is specific. The equation can be tested, inverted, differentiated, integrated, and composed with other equations. The sentence can only be read. That difference is not academic. It is the difference between tools that describe the world and tools that can compute the world, and computing the world is infinitely more powerful than describing it."
        }
        p {
            "Let me give you a concrete example, because concrete examples are the best way to make abstract ideas real. The equation for the trajectory of a projectile tells you exactly where the object will be at every moment in time, given its initial position, velocity, and the force of gravity. That single equation replaces an infinite number of sentences, because every possible trajectory is already contained within its structure. You do not need to describe each trajectory individually, because the equation generates all of them automatically. That is the power of mathematical compression. One equation contains more information than a library of sentences, because it captures the mechanism, not just the output. A sentence can tell you where the ball landed this one time. The equation can tell you where any ball will land under any conditions. That is the difference between reporting and understanding, and understanding is what intelligence is supposed to be about."
        }
        p {
            "This is why symbolic regression is such an important area of research, and why I think it deserves far more attention than it currently gets. Symbolic regression is a method for learning compact mathematical expressions directly from data, without being told what form the equation should take. The system looks at the data, searches through the space of possible mathematical expressions, and returns the simplest expression that fits the observations. This is not curve fitting in the traditional sense, because curve fitting assumes a fixed functional form and only tunes the parameters. Symbolic regression searches over the form itself, which means it can discover entirely new equations that no human has ever written down. That is a form of machine discovery, and it is happening now, not in some distant future, but in real labs with real data and real results."
        }
        p {
            "The reason this matters so much is that equations are the most efficient form of knowledge compression that humans have ever found. A single equation can replace volumes of empirical data, because it captures the underlying pattern that generates the data. If a machine can discover equations from observations, then it can compress an entire field of experimental results into a compact formal statement, and that statement can then be used to predict new observations, design new experiments, and generate new hypotheses. This is not just data analysis. This is scientific reasoning automated at its most fundamental level, and it happens entirely outside the world of language. The machine does not need to describe the law in English. It can express the law in mathematics, which is the native language of the universe, and that expression is more powerful, more precise, and more useful than any verbal description could ever be."
        }
        p {
            "I also want to connect this to the broader theme of my posts, which is that the world is being reshaped by technology in ways that most people do not understand and cannot control. In my post "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/technology-has-destroyed-my-livelihood.md",
                "Technology Has Destroyed My Livelihood"
            }
            ", I talked about how the tech industry sold us a lie about equal opportunity while building systems that extract value from the bottom and send it to the top. The same thing is happening with equation discovery, but in a different domain. The institutions that have access to these tools, the large labs, the well-funded universities, the big tech companies, will use them to accelerate their scientific output while the rest of the world falls further behind. A tool that can discover equations from data is a tool that can replace teams of PhDs, and the economic consequences of that are staggering. The people at the top will move faster, learn faster, and innovate faster, while the people at the bottom will be told that their skills are no longer needed. That pattern has repeated itself throughout the history of technology, and there is no reason to believe it will stop now."
        }
        p {
            "But the power is still real, and I refuse to hide from it just because the distribution is unfair. An equation is more powerful than a sentence, because an equation can be executed and a sentence can only be read. An equation can be tested against new data and a sentence can only be compared to other sentences. An equation can be composed with other equations to build larger systems and a sentence can only be placed next to other sentences to build larger texts. The computational power of mathematics is the foundation of modern civilization, and any system that can learn mathematics from observation is a system that can participate in the most fundamental form of human intellectual achievement. That is not hype. That is a sober description of what equation discovery means, and it means something enormous."
        }
        p {
            "I also think people need to understand that the move from text to equations is not just a technical upgrade. It is a change in what we mean by intelligence itself. If intelligence means the ability to produce fluent text, then LLMs are intelligent. But if intelligence means the ability to discover the hidden structure of the world and express it in precise, testable, computable form, then LLMs are not intelligent at all. They are eloquent, but they are not insightful. They are fluent, but they are not deep. They can write about physics, but they cannot do physics. They can describe an equation, but they cannot discover an equation. That gap between writing about and doing is the same gap I identified in my post about language being limited, and it is the gap that mathematical models and equation discovery tools are beginning to close. Not perfectly, not yet, but the direction is clear, and the direction matters more than the current state."
        }
        h2 { id: "simulation-is-the-real-intelligence",
            a { href: "#simulation-is-the-real-intelligence", class: "header",
                "Simulation Is The Real Intelligence"
            }
        }
        p {
            "This is the section where everything comes together, and this is the section where I think the argument becomes truly strong. If a model can learn the equations that govern a system, and if a model can simulate those equations forward in time, then the model can predict the future. Not in a mystical sense, not in a fortune-telling sense, but in the precise, scientific sense of computing what will happen next given the current state and the rules of evolution. That is what simulation is. It is the execution of understanding. It is the moment when knowledge stops being passive description and becomes active prediction. And active prediction is the foundation of all useful intelligence, because intelligence that cannot predict is intelligence that cannot plan, cannot prepare, and cannot control."
        }
        p {
            "Think about what a weather forecast is. It is a simulation. Scientists take the current state of the atmosphere, feed it into a system of equations that describe fluid dynamics, thermodynamics, and radiation transfer, and then run those equations forward in time to predict what the weather will look like tomorrow, next week, or next month. The better the model, the better the prediction. The better the prediction, the better the preparation. The better the preparation, the fewer people die in storms, floods, and droughts. Simulation is not an abstract academic exercise. It is a life-saving, civilization-enabling capability, and it depends entirely on having the right equations and the ability to solve them efficiently. A language model cannot do this, because a language model does not have equations. It has text. And text cannot be executed, only read."
        }
        p {
            "World models take this idea even further, because they do not just simulate one physical system. They learn to simulate entire environments, including the effects of actions within those environments. In the research on Dreamer and related systems, world models are described as systems that learn to predict future states and rewards from sensory inputs, and then use those predictions to plan actions without needing to interact with the real world at all. That is imagination. That is rehearsal. That is the ability to test a thousand possible futures in the safety of an internal model and then choose the best one. Humans do this all the time, we rehearse conversations in our heads, we imagine what will happen if we turn left instead of right, we plan our days by mentally simulating the consequences of different schedules. World models give machines the same capability, and once that capability is reliable, the machine can operate in the world with a level of foresight that pure language models can never achieve."
        }
        p {
            "This is also why I say that simulation is the real interface between intelligence and reality. Language is an interface between humans. Equations are an interface between the model and the mechanisms. But simulation is the interface between the model and the future. A model that can simulate does not need to wait for reality to happen. It can precompute reality. It can test alternatives. It can reject bad options before they cause harm. It can optimize outcomes before resources are committed. That is a fundamentally different relationship with truth than anything a language model offers, because a language model can only discuss the future in words, while a simulation model can compute the future in structure. Discussing is passive. Computing is active. And the difference between passive and active intelligence is the difference between watching the world and shaping the world."
        }
        p {
            "I want to connect this back to my personal experience, because everything I write comes from experience, not from theory. In "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/an-empty-life-filled-with-constant-suffering.md",
                "An Empty Life Filled With Constant Suffering"
            }
            ", I wrote about how I keep fighting and still see very little change, and how suffering becomes part of your identity when it lasts long enough. What frustrated me most during those years was not the pain itself, but the inability to predict or simulate better outcomes. I kept moving forward without any model of the future that could tell me which direction was likely to lead somewhere real. I was navigating by words, by hope, by advice, and by guesswork, and none of those were precise enough to cut through the fog of uncertainty. That experience is why I believe so strongly in simulation. If you have a model of the world that can predict consequences, you do not need to guess. You can compute. And computation, unlike hope, gives you actual answers that can be tested against actual results."
        }
        p {
            "Let me also say that the combination of equation discovery and simulation is the most powerful intellectual technology humans have ever developed, and this is not an exaggeration. Every bridge, every airplane, every drug, every satellite, every power plant, and every electronic device exists because someone discovered the relevant equations and then simulated them to verify the design before building it. Simulation is how we test ideas without breaking things. It is how we explore possibilities without wasting resources. It is how we learn from mistakes without paying the full price. And if a machine can learn to discover equations and simulate them autonomously, then the machine can participate in the design process at a level that was previously reserved for teams of highly trained engineers and scientists. That is not a small business opportunity. That is a transformation of the entire relationship between intelligence and physical reality, and it is the reason I chose the title of this post."
        }
        p {
            "I want to close this section by saying something that I think needs to be said plainly. Simulation is not just another feature of AI. It is the whole point. Language is a nice interface for humans, but simulation is the real work of intelligence. A mind that can only talk is a commentator. A mind that can simulate is a scientist. A commentator can describe the game, but only a scientist can change the rules. And when the rules start changing, reality itself starts bending, and that is what I mean when I say LMMs will break reality. Not destroy it. Not ruin it. But break our current understanding of what is possible, in the same way that every major scientific revolution broke the previous understanding. That is the promise, and it is a promise built on math, on physics, on structure, and on the hard-won knowledge of centuries of human science."
        }
        h2 { id: "the-danger-is-real-and-i-will-not-ignore-it",
            a {
                href: "#the-danger-is-real-and-i-will-not-ignore-it",
                class: "header",
                "The Danger Is Real And I Will Not Ignore It"
            }
        }
        p {
            "I have spent most of this post talking about the power and potential of mathematical models, equation discovery, and simulation, and I believe every word I have written. But I would be dishonest if I did not also talk about the danger, because the danger is real, and pretending it does not exist is exactly the kind of comfortable lie that I have been fighting against in every post I have written. Technology is not neutral. It has never been neutral. It carries the intentions, the biases, and the economic interests of the people who build it and the people who fund it. I wrote about this in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/it-is-always-the-russians.md",
                "It is always the Russians"
            }
            ", where I described how the Soviets killed God and the people who came from that tradition built the most powerful AI systems in the world. That pattern has not changed. The same people who built the current language models will build the next mathematical models, and they will carry the same assumptions, the same blindness, and the same hunger for control into the new era."
        }
        p {
            "The first danger is misuse. A system that can simulate the physical world can also simulate weapons, surveillance systems, environmental manipulation, and biological agents. This is not speculation. It is a direct consequence of the capability I have been describing. If a machine can discover equations from observation, then it can discover the equations of harmful systems just as easily as the equations of helpful ones. If a machine can simulate the future, then it can simulate the future of a weapon just as easily as the future of a bridge. The same tool that enables scientific breakthrough also enables scientific catastrophe, and the difference between the two depends entirely on who controls the tool and what they intend to do with it. Given the current state of the world, where power is concentrated in the hands of a few companies and governments, I am not optimistic about who will control these tools. The same companies that exploited LLMs for profit will exploit LMMs for profit, and the same governments that used surveillance for control will use simulation for control, and the people at the bottom will pay the price, as they always have."
        }
        p {
            "The second danger is displacement, and this one is personal because I have lived through it. In "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/technology-has-destroyed-my-livelihood.md",
                "Technology Has Destroyed My Livelihood"
            }
            ", I described how the tech industry destroyed my career while I was doing everything right. I coded, I built, I competed, I published, and I applied, and the system still threw me away. That was with text-based tools. Imagine what happens when mathematical models can do everything text-based models can do plus see, hear, and simulate. The number of human tasks that can be automated does not just increase. It explodes. A model that can read your code, see your screen, hear your voice, and simulate the outcomes of different engineering decisions can replace not just coders but entire engineering teams. That is not a future scenario. That is the stated goal of the companies building these systems, and they are not shy about it. They want to replace human labor with machine labor, because machine labor is cheaper, faster, and does not complain."
        }
        p {
            "The third danger is epistemic, meaning it affects how we know what we know. If a model can simulate reality convincingly, then it can also fabricate reality convincingly. A text-based hallucination is already dangerous, but a mathematical hallucination, one that includes fake images, fake video, fake audio, and fake data, is orders of magnitude more dangerous, because it attacks every channel of human perception at once. Right now, you can fact-check a text claim by looking at an image or watching a video. But if the image and the video are also generated by the model, then your fact-checking loop is broken, and you have no independent source of ground truth. That is an epistemic catastrophe, a situation where truth becomes indistinguishable from fabrication across all modalities, and it is not a distant hypothetical. It is a near-term consequence of the technology I have been describing, and very few people are taking it seriously enough."
        }
        p {
            "The fourth danger is concentration of power, and this one connects to everything I have written about in all my previous posts. A company or government that controls a system capable of equation discovery and reality simulation has a fundamental advantage over everyone else, because they can predict the future while everyone else is still guessing. They can optimize their strategies while everyone else is still debating. They can design physical systems while everyone else is still sketching on paper. That kind of advantage does not lead to shared prosperity. It leads to domination, because the people with the tool have no incentive to share it, and every incentive to protect it. History has shown this pattern over and over again. The people who control the most powerful technology always use it to extend their control, not to liberate others. There is no reason to believe this time will be different, and many reasons to believe it will be worse, because the technology is more powerful than anything that has come before, and the concentration of ownership is tighter than anything we have seen."
        }
        p {
            "I also want to say that the danger is not just to individuals or to jobs. The danger is to the entire structure of human knowledge and human society. If machines can discover equations from data, then the human scientists who currently do that work become less valuable. If machines can simulate futures, then the human planners and strategists who currently do that work become less valuable. If machines can perceive, model, and predict the physical world, then the entire human enterprise of trying to understand reality becomes something that machines do better, faster, and more cheaply. That sounds abstract, but the consequences are concrete. Fewer jobs for scientists. Fewer jobs for engineers. Fewer jobs for anyone whose work involves understanding the physical world. And the people who lose those jobs will not have anywhere to go, because the machine does not just automate one task. It automates the entire cognitive chain from observation to understanding to prediction to action. That is a displacement so comprehensive that no retraining program can fix it."
        }
        p {
            "Let me be clear about one more thing. I am not saying we should stop building these systems. I do not believe that is possible, and I do not believe it is desirable. The potential for good is too great. A system that can simulate the physical world can help us cure diseases, design better infrastructure, predict natural disasters, discover new materials, and solve problems that have resisted human effort for centuries. But the potential for harm is equally great, and ignoring that potential because we are excited about the benefits is exactly the kind of blindness that leads to catastrophe. The right response is not to stop building but to demand transparency, to demand fair distribution of benefits, to demand regulation, and to demand that the people who are most affected by these systems have a voice in how they are deployed. That is not happening right now, and it needs to happen before the technology outpaces our ability to control it."
        }
        p {
            "I want to end this section with a direct message to other engineers, especially those who have been through what I have been through. Do not let them pretend this is only about progress and opportunity. It is also about power and control, and the people who build these systems need to be honest about both sides. I have been honest about the limits of language models, I have been honest about how the industry exploits us, and I will be honest about mathematical models too. They are powerful, they are important, and they are dangerous. All three of those things are true at the same time, and accepting all three is the only honest position."
        }
        h2 { id: "what-comes-after-language",
            a { href: "#what-comes-after-language", class: "header", "What Comes After Language" }
        }
        p {
            "This is the final section, and it is the one where I want to zoom out and look at the big picture, because everything I have written so far leads to a single conclusion that I think is both obvious and terrifying. What comes after language is structure. What comes after description is computation. What comes after talking about the world is modeling the world. And once machines can model the world directly, through equations, through simulation, through mathematical perception, the entire relationship between humanity and reality changes. That is what I mean by breaking reality, and I do not use that phrase lightly."
        }
        p {
            "Let me explain what I mean in concrete terms. For most of human history, our access to reality was limited by our senses and our tools. We could see what was in front of us. We could measure what we could reach. We could predict only what our theories could calculate, and our theories were limited by our mathematical ability and our physical intuition. Every significant advance in science extended our reach, from telescopes to microscopes to particle accelerators to satellite networks, each tool gave us access to a new layer of reality that was previously hidden. But all of those tools required human interpretation. A human had to look through the telescope. A human had to read the instrument. A human had to analyze the data and discover the pattern. The bottleneck was always human cognition, and human cognition is slow, biased, limited by attention, and constrained by the number of hours in a day."
        }
        p {
            "What changes with mathematical AI and automated equation discovery is that the bottleneck shifts. The machine can observe, model, discover, and simulate without a human in the loop at every step. It can process more data than any human can read. It can search over more equations than any human can write. It can simulate more futures than any human can imagine. That does not make it smarter than a human in every way, and I am not claiming that. But it does make it faster at certain forms of scientific discovery, and speed in discovery translates directly into acceleration of knowledge. When knowledge accelerates, the world changes. When knowledge accelerates fast enough, the world changes faster than people can adapt, and that is exactly the situation we are heading into."
        }
        p {
            "This is also why I think the conversation about AI needs to move beyond language models entirely. The real action is happening in the intersection of machine learning and physical science, where models are learning to perceive the world mathematically, discover its governing equations symbolically, and simulate its dynamics numerically. That intersection is small right now, but it is growing, and the people working in it are some of the most talented scientists and engineers alive. They are not building chatbots. They are building tools that can participate in scientific reasoning at a fundamental level. They are building tools that can learn the laws of physics from video, discover compact mathematical expressions from data, and simulate the behavior of complex systems without being told the equations in advance. That is a different kind of AI than what the public sees, and it is far more important."
        }
        p {
            "I want to connect this to something I wrote in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/christianity-makes-perfect-sense.md",
                "Christianity Makes Perfect Sense!"
            }
            ", where I talked about how some truths are not simple but that does not make them false, and how deeper truths often require deeper thinking. The truth about where AI is heading is exactly like that. The surface conversation is about chatbots and productivity and who lost their job to automation. The deeper conversation is about whether machines can learn the structure of reality itself, and what happens to human civilization when they can. That deeper conversation is the one that matters, and it is the one that almost nobody is having, because it requires understanding physics, mathematics, machine learning, and philosophy all at once, and most people specialize in one and ignore the rest."
        }
        p {
            "Here is my honest assessment of where we are. LLMs are useful tools that operate entirely within the cage of language. They are powerful, impressive, and genuinely helpful for many tasks, but they will never understand the world because they never touch the world. LMMs are a bridge from language to perception, and that bridge opens the door to learning from the physical world directly. Equation discovery and symbolic regression allow machines to extract compact mathematical structure from data, which is the most fundamental form of understanding. Neural operators and physics-informed models allow machines to learn and simulate the dynamics of physical systems, which is the most fundamental form of prediction. And world models allow machines to simulate entire environments, including the effects of actions, which is the most fundamental form of planning. Put all of these together, and you have a system that can perceive, understand, predict, and plan, and that system is no longer a chatbot. It is a scientific partner. It is a simulation engine. It is a tool for breaking through the limits of human cognition and accessing layer of reality that were previously out of reach."
        }
        p {
            "That is why I chose the title of this post. LLMs are useful, and I will not pretend otherwise. But LMMs, combined with equation discovery and simulation, will break reality in the sense that they will shatter our current understanding of what machines can know, what machines can do, and how fast human knowledge can grow. That is not a threat or a promise. It is a trajectory, and trajectories are harder to deny than predictions, because they are based on what is already happening, not on what might happen. The research exists. The tools exist. The direction is clear. The only question is how fast we get there, and who controls the outcome when we do."
        }
        p {
            "I want to end this post the way I have ended every post since I started writing. I am not trying to be dramatic. I am not trying to scare anyone. I am trying to tell the truth as honestly as I can, from the perspective of someone who has built software, studied systems, lived through technological displacement, and spent years thinking about what intelligence actually is. The truth is that language models are useful but limited. The truth is that mathematical models are a step toward something much deeper. The truth is that equations are more powerful than sentences. The truth is that simulation is the real intelligence. And the truth is that all of this is coming faster than most people think, and the people who understand it first will have the most power, for better or for worse."
        }
        p { "Till next time 👋!" }
        h2 { id: "references",
            a { href: "#references", class: "header", "References" }
        }
        ul {
            li {
                "Physics-informed neural networks and data-driven PDE discovery: "
                a { href: "https://arxiv.org/abs/1711.10561", "arXiv:1711.10561" }
            }
            li {
                "LLM hallucination and reliability limits: "
                a { href: "https://arxiv.org/abs/2311.05232", "arXiv:2311.05232" }
            }
            li {
                "Physics-Informed Deep Neural Operator Networks: "
                a { href: "https://arxiv.org/abs/2207.05748", "arXiv:2207.05748" }
            }
            li {
                "Interpretable Scientific Discovery with Symbolic Regression: "
                a { href: "https://arxiv.org/abs/2211.10873", "arXiv:2211.10873" }
            }
            li {
                "Automated Discovery of Operable Dynamics from Videos: "
                a { href: "https://arxiv.org/abs/2410.11894", "arXiv:2410.11894" }
            }
            li {
                "Mastering diverse control tasks through world models: "
                a { href: "https://www.nature.com/articles/s41586-025-08744-2", "Nature" }
            }
        }
    }
}
#[component(no_case_check)]
pub fn MathematicalEquationsAreMultimodalByDefault() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey everyone 👋," }
        p {
            "In my previous post, "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/llms-are-usefull-lmms-will-break-reality.md",
                "LLMs are Useful. LMMs will Break Reality"
            }
            ", I made a case that language models are genuinely useful tools trapped inside a symbolic cage, and that multimodal models represent the first real step toward machines that can perceive and simulate the physical world. I talked about how equations are more powerful than sentences, how simulation is the real intelligence, and how the transition from text to structure is the most important shift happening in AI right now. I meant every word of that, and I am not walking any of it back, but I realized after publishing it that I left something important on the table, something that has been sitting in my head for years and that I need to say clearly before I can move on. The thing I left out is the reason why mathematical equations are special in a way that goes far beyond what most people in the AI conversation understand. Most people think of equations as formulas you memorize in school, abstract things that live on chalkboards and have no connection to real life. That is completely wrong, and the fact that so many people believe it is one of the biggest intellectual failures of modern education. Mathematical equations are not abstract decorations. They are the most compressed, most general, most powerful representations of reality that humans have ever discovered, and they are multimodal by default, meaning they can generate text, images, motion, sound, and physical predictions all from the same compact structure. That is the argument I am going to make in this post, and I am going to make it so thoroughly that by the end, you will either agree with me or you will have to explain exactly where my reasoning breaks down."
        }
        p {
            "I have been building toward this idea across several posts now, and I want to connect the threads so that people who have been following along can see how everything fits together. In "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/language-is-limited-asi-is-impossible.md",
                "Language is Limited. ASI is Impossible."
            }
            ", I argued that words are not thoughts, that the brain is not a text machine, and that any system built purely on language inherits the permanent gap between symbols and reality. In "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/an-empty-life-filled-with-constant-suffering.md",
                "An Empty Life Filled With Constant Suffering"
            }
            ", I talked about how words cannot fully capture what I feel, how language always falls short of the real thing inside my head, and how that frustration has shaped everything I think about intelligence. In "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/as-engineers-llms-should-pay-us-for-tokens-usage.md",
                "As Engineers, LLMs should pay us for tokens usage"
            }
            ", I argued that the current system exploits the very people who make it work, and that the value flows upward while the cost flows downward. And in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/technology-has-destroyed-my-livelihood.md",
                "Technology Has Destroyed My Livelihood"
            }
            ", I described how the tech industry destroyed my career while I was doing everything right, and how the promise of equal opportunity turned out to be a lie wrapped in marketing. All of those posts are pieces of the same puzzle, and this post is the piece that ties them together, because the central claim here is that if we want machines that actually understand reality instead of just talking about it, then mathematical equations are the right foundation, and they have been sitting right in front of us the whole time."
        }
        h2 { id: "why-i-think-about-equations-differently-than-most-people",
            a {
                href: "#why-i-think-about-equations-differently-than-most-people",
                class: "header",
                "Why I Think About Equations Differently Than Most People"
            }
        }
        p {
            "I did not grow up thinking about equations. I grew up in a poor village with no internet, no computer, and no cell phone, as I described in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/who-am-i.md",
                "my first post"
            }
            ", and the idea that mathematics could be beautiful or powerful was not something anyone around me ever talked about. Mathematics in school was memorization, repetition, and punishment for getting the wrong answer, and by the time I was a teenager, I had learned to associate equations with stress rather than with wonder. That changed when I started studying electrical engineering in college, because for the first time I saw equations not as abstract exercises but as descriptions of real things that I could build, test, and verify with my own hands. A circuit equation was not just symbols on a page. It was a prediction about what would happen when I connected wires and applied voltage, and when the prediction matched the measurement, something clicked inside me that has never unclicked. That moment, the moment when I realized that a few symbols on paper could predict the behavior of the physical world with terrifying accuracy, is the moment that changed how I think about intelligence, and it is the foundation of everything I am about to argue."
        }
        p {
            "The reason most people misunderstand equations is that they were taught equations as things to solve rather than things to see. When you learn F = ma, Newton's Second Law of Motion, in high school, the teacher tells you to plug in numbers and get an answer, and the whole exercise feels mechanical, pointless, and disconnected from anything real. But F = ma is not a homework problem. It is a statement about the structure of the universe. It says that force, mass, and acceleration are related in a specific, deterministic way, and that this relationship holds everywhere, at all times, for all objects, from a falling apple to a spinning galaxy. That is a law of nature written in the only language precise enough to express it, and that language is mathematics. No sentence in any human language can express the same content with the same precision, the same generality, and the same testability. The equation is not a description of gravity. It is gravity, compressed into a form that a finite mind can carry, transmit, and use. Once I understood that, I could never go back to thinking of equations as mere school exercises, and I could never take seriously any approach to intelligence that ignores this fundamental power."
        }
        p {
            "What makes equations truly special, and what most AI researchers seem to miss entirely, is that a single equation can generate outputs in multiple modalities simultaneously. Take the equation for a damped harmonic oscillator, which describes everything from a vibrating guitar string to the shock absorbers in your car. That one equation can be rendered as a mathematical formula on a page, which is a textual output. It can be plotted as a graph showing displacement over time, which is a visual output. It can be used to generate the actual sound wave produced by the oscillation, which is an audio output. It can be animated to show the physical motion of the oscillating object, which is a video output. And it can be integrated into a larger simulation to predict how the oscillation interacts with other forces, which is a predictive output. All of these outputs come from the same compact mathematical structure, and they are all exact, all consistent, and all grounded in the same underlying reality. That is what I mean when I say mathematical equations are multimodal by default. They do not need to be converted between modalities. They already contain all modalities within their structure, because they encode the mechanism rather than the surface appearance."
        }
        p {
            "This is fundamentally different from how language models handle multimodality. A language model that is extended to handle images does so by learning statistical associations between text tokens and image patches, which means it learns that certain words tend to appear near certain visual patterns, but it does not learn why those patterns exist or what generates them. It learns correlation, not causation. It learns the surface, not the mechanism. An equation-based system, by contrast, does not need to learn the association between a graph and its description, because both the graph and the description are generated from the same equation, and the equation is the source of truth for both. This is a profound difference, and it is the difference between a system that can mimic multimodal understanding and a system that actually has multimodal understanding. The first system can produce outputs that look right. The second system produces outputs that are right, because they are derived from a structure that is grounded in the actual laws of the world. That distinction matters enormously, and it is the distinction that the entire AI industry is currently ignoring in its rush to build bigger language models."
        }
        p {
            "I want to be honest about something here, because honesty is the only thing I have left, and I am not going to start compromising on it now. I am not a mathematician. I am a software engineer who struggled through years of electrical engineering courses and came out the other side with a deep respect for mathematics but no illusions about my own limitations. I cannot derive the Navier-Stokes equations from first principles, and I cannot prove the Riemann hypothesis, and I am not pretending otherwise. But I can recognize when a tool is more powerful than the one everyone is using, and I can articulate why, and that is exactly what I am doing in this post. You do not need to be a professional chef to know that a sharp knife cuts better than a dull one, and you do not need to be a Fields Medal winner to see that equations encode reality in a way that sentences never can. The argument I am making is not about my personal mathematical ability. It is about the inherent power of mathematical representation as a foundation for intelligent systems, and that argument stands regardless of whether I can solve a differential equation in my head."
        }
        p {
            "I also want to address the people who will say that I am romanticizing mathematics, that I am treating it like a religion the same way I accused the tech industry of being a religion in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/technology-has-destroyed-my-livelihood.md",
                "Technology Has Destroyed My Livelihood"
            }
            ". That is a fair challenge, and I want to answer it directly. I am not romanticizing mathematics. I am making a specific, testable claim, which is that mathematical equations compress more information into less structure than any other representational system humans have ever created, and that this compression is inherently multimodal because it encodes mechanisms rather than appearances. That claim can be verified by looking at the history of science, where every major breakthrough has been associated with the discovery of a compact mathematical law that unified previously disconnected observations. Newton unified falling apples and orbiting planets with a single equation. Maxwell unified electricity and magnetism with four equations. Einstein unified space and time with a single equation. Schrödinger unified particle behavior with a single equation. In every case, the power came from the compression, from the ability to say more with less, and from the fact that the compressed representation could generate predictions across multiple domains and multiple modalities. That is not romance. That is the historical record, and the historical record is the strongest form of evidence I know."
        }
        p {
            "Let me also say this, because I think it needs to be said and nobody in the AI conversation is saying it. The reason we even know that equations work is because we tested them against reality, and reality confirmed them. Newton did not just write F equals ma and declare victory. He used it to predict the motion of planets, and the predictions matched the observations, and that match is what gave the equation its power. This is the critical difference between mathematical knowledge and linguistic knowledge. Mathematical knowledge is testable, falsifiable, and grounded in physical verification. Linguistic knowledge is not. When a language model produces a fluent paragraph about gravity, you cannot test that paragraph against reality in any rigorous way. But when a mathematical model produces an equation for gravity, you can test that equation against every falling object on Earth and every orbit in the solar system, and if it matches, you know you have captured something real. That testability is not a minor detail. It is the entire foundation of scientific knowledge, and any system that claims to understand the world must be able to produce outputs that can be tested against the world. Language models cannot do this. Equation-based models can. And that difference is the whole argument of this post, compressed into two sentences."
        }
        h2 { id: "the-compression-power-that-nobody-talks-about",
            a {
                href: "#the-compression-power-that-nobody-talks-about",
                class: "header",
                "The Compression Power That Nobody Talks About"
            }
        }
        p {
            "I keep coming back to the idea of compression because I think it is the most important concept in all of intelligence, both human and artificial, and yet almost nobody in the AI conversation talks about it seriously. When I say compression, I do not mean zip files or data reduction algorithms. I mean the ability to take a vast, complex, high-dimensional reality and represent it with a compact structure that preserves the essential patterns while discarding the noise. That is what intelligence does. A human child who learns that heavy things fall is compressing billions of individual observations into a single rule, and that rule allows the child to predict the behavior of objects they have never seen before. A scientist who discovers Newton's law of gravitation is compressing every falling object, every orbiting planet, every tidal force, and every projectile trajectory into a single equation, and that equation allows the scientist to predict phenomena that have not yet been observed. Compression is not just a convenience. It is the mechanism of understanding itself, and any system that cannot compress is a system that cannot understand."
        }
        p {
            "Language models compress, but they compress in the wrong dimension. They compress statistical patterns in text, which means they learn that certain words tend to follow certain other words in certain contexts, and they can reproduce those patterns with impressive fidelity. But the patterns they learn are patterns in language, not patterns in reality. They learn how humans talk about the world, not how the world actually works. That is a crucial distinction, and it is the same distinction I made in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/language-is-limited-asi-is-impossible.md",
                "Language is Limited. ASI is Impossible."
            }
            " when I argued that more text gives you more text, not more understanding. A language model that has read every physics textbook ever written has not learned physics. It has learned how physicists write, which is a completely different thing. It can produce sentences that sound like physics, but it cannot produce predictions that are physics, because predictions require computation, and computation requires structure, and structure requires mathematics."
        }
        p {
            "Mathematical equations compress in the right dimension because they compress the mechanisms of reality rather than the descriptions of reality. Consider the wave equation, which describes how disturbances propagate through a medium. That single equation encodes the behavior of sound waves, water waves, light waves, seismic waves, and any other phenomenon that involves propagation through space and time. It does not describe any particular wave. It describes the universal pattern that all waves share, and from that universal pattern, you can derive the specific behavior of any particular wave by specifying the boundary conditions and the properties of the medium. That is an extraordinary level of compression, and it is compression that language cannot match. A textbook chapter on waves might take fifty pages to describe what the wave equation says in one line, and even then, the textbook will miss details that the equation captures exactly. The equation is not a summary of the textbook. The equation is the source from which the textbook is generated, and the source is always more powerful than the derivative."
        }
        p {
            "This is why I believe that the next generation of intelligent systems should be built around mathematical representations rather than linguistic ones. Not because mathematics is inherently superior in some abstract philosophical sense, but because mathematical representations compress more effectively, generalize more broadly, and generate more modalities than linguistic representations. A system that can discover the wave equation from data can then use that equation to generate predictions about sound, predictions about light, predictions about seismic activity, simulations of wave behavior, visualizations of wave propagation, and audio renderings of wave patterns, all from the same compact structure. A system that learns from text can describe waves in English, and that is useful, but it is not the same thing. The text-based system has learned to talk about waves. The equation-based system has learned waves themselves, and the difference between talking about something and knowing something is the difference between a commentator and a scientist."
        }
        p {
            "I know that some people will object that mathematical compression is not always possible, that not every phenomenon can be captured in a neat closed-form equation, and that the real world is too messy, too complex, and too nonlinear for mathematical elegance to always apply. That is a fair objection, and I want to address it honestly because I said I would not hide from hard questions. Yes, there are phenomena that resist clean mathematical compression. Turbulence in fluid dynamics is a famous example, where the governing equations are known (the Navier-Stokes equations) but the solutions are so complex that we still cannot predict turbulent flow analytically in many cases. Biological systems are another example, where the interactions between genes, proteins, cells, and organisms are so numerous and so nonlinear that no single equation can capture the whole picture. But here is the thing that the objectors miss. Even in these cases, mathematical structure is still the best tool we have. We may not be able to write a single closed-form equation for turbulence, but we can use mathematical frameworks like statistical mechanics, chaos theory, and computational fluid dynamics to understand and predict turbulent behavior far better than any verbal description could. The fact that the equations are complicated does not mean that mathematics has failed. It means that the phenomenon is complicated, and mathematics is the only tool precise enough to handle that complexity honestly."
        }
        p {
            "I also want to push back on the idea that because some phenomena are too complex for closed-form equations, we should give up on mathematical representation and stick with statistical learning from data. That argument confuses difficulty with impossibility. Yes, discovering mathematical structure from complex data is hard. Yes, it requires new methods, new computational tools, and new theoretical insights. But the fact that it is hard does not mean it is wrong. Symbolic regression, which I discussed in my previous post, has already shown that compact mathematical expressions can be recovered from data in many scientific domains (^1). Neural operators have shown that mappings between function spaces can be learned from data, which means that even when closed-form solutions are not available, mathematical structure can still be captured in learned representations that respect physical constraints (^2). Physics-informed neural networks have shown that known physical laws can be incorporated directly into the training process, producing models that are more accurate, more generalizable, and more scientifically meaningful than pure data-driven approaches (^3). These are not hypothetical future technologies. They are real methods, published in real journals, with real results, and they all point in the same direction: mathematical structure is the key to building intelligent systems that understand reality rather than just describing it."
        }
        p {
            "Let me also connect this to something deeply personal, because every idea I have is rooted in personal experience, and I refuse to pretend otherwise. When I was studying electrical engineering, the moments that changed my life were the moments when I saw a mathematical prediction confirmed by physical measurement. When I calculated the resonant frequency of a circuit and then measured it with an oscilloscope and the numbers matched, I felt something that no text could ever produce. It was not intellectual satisfaction. It was something closer to awe, the feeling that I had touched the actual structure of reality through a few symbols on paper. That feeling is what drives me, and it is what makes me believe that mathematical equations are not just useful tools but the deepest form of understanding we have access to. I have never felt that feeling from reading a language model's output, no matter how fluent or impressive. The output can be beautiful, it can be helpful, it can even be moving, but it does not carry the weight of ground truth, because it was never tested against the world. The equation carries that weight, because it was tested, and it survived the test, and that survival is what makes it real."
        }
        h2 { id: "why-every-equation-is-already-multimodal",
            a {
                href: "#why-every-equation-is-already-multimodal",
                class: "header",
                "Why Every Equation Is Already Multimodal"
            }
        }
        p {
            "This is the core claim of this post, and I want to make it as clearly and as carefully as I can, because if this claim is right, then it changes how we should think about the entire project of artificial intelligence. The claim is this: every well-formed mathematical equation is already multimodal, meaning it contains within its structure the ability to generate outputs in multiple sensory modalities, including text, images, sound, motion, and numerical predictions. This is a literal, verifiable fact about the nature of mathematical representation, and it follows directly from the way equations encode mechanisms rather than appearances."
        }
        p {
            "Let me start with the simplest example I can think of, which is the equation for a sine wave: y equals A times sin of omega t plus phi. That equation has five elements: amplitude A, angular frequency omega, time t, phase phi, and the output y. From those five elements, you can generate a written description of the wave in any human language, which is a text output. You can plot the wave as a graph of y versus t, which is a visual output. You can convert the wave into an audio signal by interpreting y as air pressure variation, which is an audio output. You can animate the wave by rendering successive frames of the plot over time, which is a video output. You can compute specific numerical values of y at any time t, which is a quantitative prediction. And you can compose this equation with other equations to model interference, resonance, harmony, and every other wave phenomenon, which is a structural output. All of these outputs come from the same equation, and they are all exactly consistent with each other, because they all derive from the same mathematical source. There is no hallucination possible here, because there is no statistical guessing. There is only computation from structure, and computation from structure is always faithful to the structure."
        }
        p {
            "Now compare this to how a language model handles the same information. If you ask a language model to describe a sine wave, it will produce a paragraph that is probably correct in its general content but that cannot be directly rendered as a graph, cannot be directly converted to audio, cannot be directly animated, and cannot be directly used for quantitative prediction. The paragraph is a separate representation that points at the same underlying reality but is not computationally connected to it. If you want a graph, you need a separate tool. If you want audio, you need a separate tool. If you want animation, you need a separate tool. If you want numerical predictions, you need a separate tool. The language model's output is a dead end in every modality except text, because text is the only modality it natively produces. The equation, by contrast, is a living root from which every modality can be grown, because the equation encodes the mechanism that generates all of them."
        }
        p {
            "This difference becomes even more dramatic when you consider complex systems. Take the Lorenz system, which is a set of three coupled differential equations that model atmospheric convection and produce the famous butterfly attractor (^4). From those three equations, you can generate the three-dimensional trajectory of the attractor, which is one of the most beautiful and recognizable visualizations in all of science. You can animate the trajectory to show how the system evolves over time, revealing the sensitive dependence on initial conditions that defines chaos. You can project the trajectory onto different planes to show different aspects of its geometry. You can compute the Lyapunov exponents to quantify the rate of divergence of nearby trajectories. You can use the equations to generate time series data that can be analyzed statistically. And you can embed the Lorenz system within a larger model to study how chaotic subsystems interact with other dynamics. All of these outputs, visual, temporal, statistical, structural, come from three equations, and they are all exact, all consistent, and all grounded in the same mathematical source. No language model can produce any of this from a text description, because text descriptions do not contain the computational structure needed to generate these outputs. The equations do."
        }
        p {
            "I want to be very precise about why this matters for AI, because I think the implication is enormous and underappreciated. If equations are inherently multimodal, then a system that can discover equations from data has effectively discovered a multimodal representation of the underlying phenomenon. It does not need to be separately trained on text, images, audio, and video of the phenomenon. It only needs to discover the equation, and from the equation, all modalities can be generated. This is a fundamentally more efficient approach than the current paradigm of training separate models on each modality and then trying to align them through cross-modal contrastive learning or other association-based methods. The current approach learns associations between modalities. The equation-based approach learns the source that generates all modalities. Learning associations is fragile, because associations can be spurious, can break down outside the training distribution, and cannot be verified against ground truth. Learning the source is robust, because the source deterministically generates the correct output in every modality, and that correctness can be verified by testing the equation against new observations."
        }
        p {
            "I should also point out that this multimodal property of equations is not limited to physics. Any phenomenon that has a mathematical description, and that includes phenomena in biology, economics, ecology, epidemiology, climate science, neuroscience, and dozens of other fields, can be represented by equations that are inherently multimodal. The SIR model in epidemiology (^5), which describes how infectious diseases spread through a population, can be rendered as equations, as time-series plots, as phase portraits, as spatial simulations, and as animated visualizations of disease spread across a network. The Lotka-Volterra equations in ecology, which describe predator-prey dynamics, can be rendered as equations, as oscillating population curves, as animated ecosystems, and as phase-plane diagrams that reveal the cyclic nature of the interaction. In every case, the equation is the seed from which all modalities grow, and the seed contains everything the modalities need, because the seed encodes the mechanism that produces them all. This is not a property that needs to be engineered or trained into the system. It is a property that exists by virtue of what mathematical equations are, and we just need to build systems that recognize and exploit it."
        }
        p {
            "Let me also address the obvious question of what happens when the equation is not known. In many real-world situations, we observe phenomena without knowing the governing equations, and in those cases, the multimodal power of equations seems irrelevant because we do not have the equations to start with. But this is exactly where equation discovery and symbolic regression become transformative. If a machine can observe data from a phenomenon, in any modality, and discover the compact mathematical structure that underlies it, then the machine has simultaneously discovered a representation that can generate outputs in all other modalities. That is a one-shot multimodal representation, learned from data, verified against observation, and computationally exact. It is the polar opposite of how current multimodal models work, where each modality requires its own training data, its own alignment loss, and its own set of learned associations. The equation-based approach is simpler, more principled, more verifiable, and more powerful, and the only reason it is not more widely used is that discovering equations from data is genuinely difficult. But difficult and impossible are very different things, and the recent progress in symbolic regression and neural equation discovery suggests that the difficulty barrier is falling faster than most people realize."
        }
        p {
            "I want to end this section with a thought that I think is important and that I have not seen anyone else articulate. The fact that equations are multimodal by default means that the multimodal problem in AI is not fundamentally a problem of alignment or association. It is a problem of discovery. If you can discover the right equation, you get multimodality for free. You do not need to train a model on millions of image-text pairs. You do not need to build a complex architecture that fuses vision and language. You do not need to worry about modality-specific hallucinations, because the equation generates all modalities from the same source, and the source is either right or wrong, and you can tell which by testing it. The entire multimodal alignment problem dissolves once you have the right equation, and that should tell us something profound about where the field should be heading. Instead of building bigger and bigger multimodal language models, we should be building systems that discover equations, because equation discovery is the shortest path to genuine multimodal understanding."
        }
        h2 { id: "the-difference-between-correlation-and-causation-is-not-academic",
            a {
                href: "#the-difference-between-correlation-and-causation-is-not-academic",
                class: "header",
                "The Difference Between Correlation And Causation Is Not Academic"
            }
        }
        p {
            "I wrote in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/language-is-limited-asi-is-impossible.md",
                "Language is Limited. ASI is Impossible."
            }
            " that language models learn patterns in text, not truths about the world, and I stand by that. But I want to push that idea further here, because the distinction between correlation and causation is not just a philosophical talking point. It is the difference between a system that can predict and a system that can only guess, and that difference has life-or-death consequences in fields like medicine, engineering, climate science, and policy. A system that learns correlations can tell you that A and B tend to occur together. A system that learns causation can tell you that A causes B, which means that if you change A, B will change in a predictable way. Correlation is observation. Causation is understanding. And the gap between them is exactly the gap between language models and equation-based models."
        }
        p {
            "Language models are correlation machines, and I say that without any disrespect, because correlation is genuinely useful. If I want to know what words typically follow \"the patient has a fever and a cough,\" a language model can give me a very good guess, because it has seen millions of similar sequences and learned the statistical patterns. But if I want to know what caused the fever and the cough, the language model cannot tell me, because it never learned causation. It never observed the biological mechanisms that produce symptoms. It only observed the text that humans wrote about those mechanisms, and text descriptions of causation are not the same as causal knowledge. A textbook can say \"the virus infects the cells and triggers an immune response,\" and a language model can reproduce that sentence perfectly, but reproducing the sentence does not mean the model understands the causal chain from viral entry to cellular infection to immune activation to fever. The model learned the words. It did not learn the mechanism. And without the mechanism, it cannot predict what will happen if you intervene, which is exactly what medicine requires."
        }
        p {
            "Mathematical equations encode causation because they specify the mechanism by which inputs produce outputs. The SIR model in epidemiology does not just say that susceptible people become infected. It specifies the rate at which they become infected as a function of the contact rate, the number of infected individuals, and the total population. That specification is a causal model, because it tells you exactly what will happen if you change the contact rate, if you quarantine infected individuals, or if you vaccinate a portion of the population. You can intervene on the model, change a variable, and see the downstream effects, because the model encodes the mechanism, not just the correlation. A language model cannot do this, because it does not have a mechanism to intervene on. It only has patterns in text, and you cannot intervene on a pattern in text, because text patterns are not computationally connected to real-world variables. This is why Pearl's work on causal inference (^6) is so important for the future of AI, because it provides a mathematical framework for reasoning about interventions, counterfactuals, and causal relationships, and it shows that these capabilities require structural models, not just statistical associations."
        }
        p {
            "The practical consequences of this distinction are staggering, and I do not think most people in the AI conversation appreciate how high the stakes are. Consider drug development. A pharmaceutical company wants to know whether a new compound will reduce blood pressure. A language model can summarize existing research, generate hypotheses, and draft research proposals, and those are genuinely useful tasks. But the language model cannot tell you whether the drug will work in a new patient population, because it does not have a causal model of how the drug interacts with the cardiovascular system. A mathematical model, built from differential equations that describe the pharmacokinetics and pharmacodynamics of the compound, can do exactly that, because it encodes the causal chain from drug administration to receptor binding to physiological response. That model can be tested against clinical data, refined based on new observations, and used to predict outcomes in populations that have never been studied. The language model's output is a guess based on what has been written before. The mathematical model's output is a prediction based on what the equations say will happen. And in medicine, the difference between a guess and a prediction is the difference between life and death."
        }
        p {
            "I want to connect this to my own experience, because abstract arguments always feel stronger when they are grounded in something real. When I was building automated trading systems for cryptocurrency markets, which I described in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/who-am-i.md",
                "my first post"
            }
            ", I learned very quickly that correlation-based strategies are fragile. A correlation that holds for months can break overnight, because correlations are surface patterns that do not encode the underlying mechanism driving the market. The strategies that survived were the ones built on structural models, on mathematical representations of market microstructure, order flow dynamics, and volatility clustering. Those models were not perfect, but they were robust in a way that pure correlation-based approaches never could be, because they captured something about how the market actually works rather than just what it happened to look like recently. That experience taught me a lesson that I carry with me into every argument I make about AI: correlation is cheap, causation is expensive, and the expensive thing is always more valuable."
        }
        p {
            "I also want to address the growing body of research on causal representation learning, because it directly supports the argument I am making. Researchers have shown that it is possible to learn causal variables and their relationships from observational data, even without explicit intervention data, by using structural assumptions and mathematical constraints (^7). This is exactly the kind of equation discovery I have been talking about, except applied to the causal structure of the world rather than just to its dynamics. If a machine can learn the causal graph that generates the observed data, then it has learned something far deeper than statistical associations. It has learned how the world is put together, and that knowledge can be used to predict the effects of actions, to design interventions, and to answer counterfactual questions like \"what would have happened if I had done X instead of Y.\" Language models cannot answer counterfactual questions, because they have no causal model to reason over. They can only produce the most statistically likely response given the prompt, which is not the same as reasoning about what would have happened in a different world."
        }
        p {
            "Let me close this section by saying something that I feel strongly about and that I think the entire AI community needs to hear. The obsession with language models is an obsession with the surface of intelligence, with the part that is visible and impressive and easy to market. But the real substance of intelligence lies beneath the surface, in the causal models, the mathematical structures, and the compressed representations that make prediction, planning, and intervention possible. No amount of scaling will turn a correlation machine into a causal machine, because correlations and causes are fundamentally different kinds of knowledge, and they require fundamentally different kinds of representation. If we want machines that truly understand the world, we need to build machines that discover and reason over mathematical structures, not machines that produce fluent text about those structures. The text is the surface. The equation is the substance. And the substance is where the real intelligence lives."
        }
        h2 { id: "simulation-as-the-ultimate-test-of-understanding",
            a {
                href: "#simulation-as-the-ultimate-test-of-understanding",
                class: "header",
                "Simulation As The Ultimate Test Of Understanding"
            }
        }
        p {
            "I talked about simulation in my previous post, and I want to go deeper here, because simulation is not just one application of mathematical knowledge. It is the ultimate test of whether you actually understand something, and it is the capability that separates systems that describe the world from systems that can engage with the world. If you truly understand a physical system, you can simulate it forward in time and predict what will happen next. If you cannot simulate it, you do not understand it, no matter how eloquently you can talk about it. That is the hardest test any model can face, and it is the test that language models will always fail, because simulation requires computation over structure, and language models only compute over tokens."
        }
        p {
            "Think about what happens when an engineer designs a bridge. The engineer does not just describe the bridge in words and hope for the best. The engineer builds a mathematical model of the bridge, specifying the geometry, the materials, the loads, the boundary conditions, and the governing equations of structural mechanics. Then the engineer simulates the model, applying the expected loads and checking whether the structure holds. If the simulation says the bridge will fail under a certain load, the engineer redesigns it before a single piece of steel is cut. That simulation is not a suggestion or a guess. It is a rigorous computation based on equations that have been tested against reality for centuries, and it is the reason that bridges do not fall down every day. Now imagine asking a language model to perform the same task. The language model can describe a bridge. It can list the materials. It can even write a plausible-sounding analysis of the structural loads. But it cannot simulate the bridge, because it does not have the mathematical model, and without the mathematical model, it cannot predict whether the bridge will actually stand or fall. The difference between describing and simulating is the difference between talking about safety and ensuring safety, and in engineering, that difference is measured in human lives."
        }
        p {
            "This is also why world models are so important, and why I believe they represent the next frontier of AI research. A world model is a learned simulator of an environment, a system that takes the current state and an action as input and predicts the next state as output. Researchers at DeepMind and elsewhere have shown that world models can learn to simulate complex environments from sensory data, and then use those simulations to plan actions without ever interacting with the real environment (^8). That is an extraordinary capability, because it means the model can rehearse thousands of possible futures in its head before committing to a single action, just like a human chess player imagines possible moves before touching a piece. But the quality of the rehearsal depends entirely on the quality of the simulation, and the quality of the simulation depends on the quality of the mathematical structure that the model has learned. A world model that learning accurate equations can simulate accurately. A world model that learns noisy approximations will simulate poorly. And there is no way to check the quality of the simulation without testing it against reality, which brings us back to the fundamental importance of mathematical structure and empirical verification."
        }
        p {
            "I want to connect this to something I said in "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/it-is-always-the-russians.md",
                "It is always the Russians"
            }
            ", where I talked about how the Soviets took God's skin and wrapped it around a machine. The image I had in mind was a machine that looks alive on the outside but is dead on the inside, a system that mimics understanding without possessing it. Language models are exactly that kind of machine. They wear the skin of intelligence, the fluent speech, the confident tone, the structured reasoning, but underneath the skin there is no simulation, no mechanism, no causal model, and no ground truth. They are eloquent corpses, and the fact that they move so convincingly is what makes them dangerous, because people trust them the way they would trust a living mind, without realizing that the thing they are trusting has no inner life, no model of the world, and no ability to check its own outputs against reality. A simulation-capable system is fundamentally different, because it has an inner model that can be tested, refined, and verified. It is alive in the way that matters for intelligence, not biologically alive, but computationally alive, able to run internal experiments, test hypotheses, and learn from the results of its own predictions."
        }
        p {
            "The combination of equation discovery and simulation is what I think will eventually produce systems worthy of being called intelligent in the full sense of the word. Not because they can talk. Not because they can write beautiful essays. Not because they can pass standardized tests. But because they can observe the world, discover its mathematical structure, simulate its dynamics, predict its future, and verify their predictions against new observations. That loop, from observation to discovery to simulation to prediction to verification, is the scientific method, and the scientific method is the most powerful engine of understanding that humans have ever created. If a machine can execute that loop autonomously, then the machine is not just a tool. It is a scientist. And a machine scientist that can run millions of experiments per second, searching over spaces of mathematical structures that no human could explore in a lifetime, would accelerate our understanding of reality in ways that are genuinely difficult to imagine. That is not hype. That is a sober extrapolation of capabilities that already exist in rudimentary form, and it is the reason I believe that mathematical models, not language models, are the path to genuine artificial intelligence."
        }
        p {
            "Let me also acknowledge that simulation is not magic and that simulated systems can be wrong. Models can be misspecified. Equations can be approximate. Numerical methods can introduce errors. Boundary conditions can be incorrect. All of these are real problems, and I am not pretending they do not exist. But the crucial difference is that simulation errors are testable. You can compare the simulation output to reality and measure the discrepancy. You can then use that discrepancy to improve the model, to refine the equations, to better specify the boundary conditions. The error becomes a signal for improvement, not a hidden trap. Language model errors, by contrast, are not testable in the same way, because there is no ground truth against which to compare the output. A language model's hallucination can only be detected by a human who already knows the answer, which defeats the purpose of using the model in the first place. A simulation's error can be detected by comparing it to new measurements, which is exactly the kind of self-correcting feedback loop that makes science work. The ability to detect and correct your own errors is the essence of learning, and simulation has it while language generation does not."
        }
        p {
            "I want to end this section with something that comes from my gut, not from a paper. I have spent years watching the AI industry celebrate models that can talk while ignoring models that can simulate. I have watched billions of dollars flow into chatbots while the researchers building neural operators, physics-informed networks, and symbolic regression tools struggle for funding. I have watched the media hype every new language model release while ignoring every paper that advances our ability to learn equations from data. And every time I see this, I feel the same frustration I felt when I was sending out applications and getting nothing back, the frustration of knowing that the thing that matters most is being ignored in favor of the thing that looks most impressive. The thing that looks impressive is fluent speech. The thing that matters is accurate simulation. And until the world understands that distinction, we will keep building increasingly eloquent machines that cannot tell you whether the bridge will stand or fall."
        }
        h2 { id: "what-this-means-for-the-future-of-intelligence",
            a {
                href: "#what-this-means-for-the-future-of-intelligence",
                class: "header",
                "What This Means For The Future Of Intelligence"
            }
        }
        p {
            "Everything I have written so far points toward a single conclusion, and I want to state it as plainly as I can. The future of artificial intelligence is not in language. It is in mathematics. Not because mathematics is prettier or more elegant or more intellectually sophisticated, although I think it is all of those things. But because mathematics is the only tool that connects us to the actual structure of reality, and connecting to reality is what intelligence is supposed to do. Language connects us to each other. Mathematics connects us to the world. Both connections are important, but only one of them is the foundation of understanding, and understanding is the foundation of everything else."
        }
        p {
            "I think the next decade will be defined by systems that combine equation discovery, multimodal perception, and simulation into a single loop. Imagine a system that can watch a video of a physical process, discover the equations that govern it, simulate those equations forward to predict what will happen next, and then compare its predictions to new observations to refine its model. That system would be learning science the way scientists learn science, through observation, hypothesis, prediction, and verification, except it would be doing it thousands of times faster over thousands of different phenomena simultaneously. It would not need to be told the equations. It would not need to be given a textbook. It would not need any language at all. It would learn the structure of the world directly from perception, and it would express that structure in the only language precise enough to capture it, which is mathematics. That is not a chatbot. That is a revolution, and it is coming whether or not the AI influencers on YouTube are ready for it."
        }
        p {
            "I also think this future carries enormous risks, and I refuse to pretend otherwise, because I have always been honest in these posts and I am not going to stop now. A system that can discover equations from observations can discover dangerous equations just as easily as beneficial ones. A system that can simulate the world can simulate weapons, surveillance systems, biological agents, and environmental manipulation. I wrote about these dangers in my previous post, and every word still applies. The technology I am describing in this post is not inherently good or inherently evil. It is inherently powerful, and power without accountability is how civilizations get crushed. The same companies that exploited language models for profit will exploit equation-based models for profit, and the same governments that used AI for surveillance will use simulation-capable AI for control, and the people at the bottom of the pyramid will bear the cost, as they always have. That is not cynicism. That is history, and anyone who thinks this time will be different has not been paying attention."
        }
        p {
            "But I want to say something else too, something more hopeful, because I do not want to end this post in despair. The fact that equations are multimodal by default means that the path to genuine artificial understanding is more accessible than people think. You do not need to train a model on billions of image-text pairs to get multimodal capability. You need to discover the right equations, and the right equations give you multimodality for free. That is an enormously simplifying insight, and it means that small teams with deep mathematical knowledge could potentially build systems that rival or surpass the capabilities of massive multimodal language models, at a fraction of the cost and with far greater reliability. The big companies want you to believe that AGI requires billions of dollars in compute and data. I believe it requires the right mathematical structure, and mathematical structure can be discovered by anyone with the right tools, the right data, and enough stubbornness to keep looking. I have always been stubborn, and that stubbornness is the only thing that has kept me alive through everything I have described in my previous posts."
        }
        p {
            "And that stubbornness is exactly why I started building "
            a { href: "https://github.com/kevin-rs/lmm", "lmm" }
            ", a proof of concept for everything I have been arguing in this post and in my previous one. I called it \"a language agnostic framework to reality,\" and I meant that literally. It is a tool written in Rust that has two core commands: "
            code { "discover" }
            " and "
            code { "simulate" }
            ". The "
            code { "discover" }
            " command takes raw data and attempts to find the compact mathematical equation that generated it, using the principles of symbolic regression I have been describing throughout this post. The "
            code { "simulate" }
            " command takes a discovered equation and runs it forward in time, producing predictions that can be tested against new observations. That is the whole loop I keep talking about, observation to equation to simulation to verification. It is still a work in progress. But it exists. It runs. It discovers equations from data and simulates them forward, and that alone puts it in a fundamentally different category from any language model, because its outputs are not statistically generated text. They are mathematically derived predictions that can be checked against reality. I built it because I needed to prove to myself that the ideas I have been writing about are not just philosophy. They are engineering. They are buildable."
        }
        p {
            "I also want to connect this to the broader question of what kind of world we want to live in, because technology is never just about technology. It is always about power, about who benefits and who pays and who gets left behind. In "
            a { href: "https://github.com/kevin-rs/blog/blob/main/docs/blog/src/technology-has-destroyed-my-livelihood.md",
                "Technology Has Destroyed My Livelihood"
            }
            ", I described a world where technology promises equality but delivers extraction, where the tools that are supposed to empower us end up controlling us, and where the only winners are the people who own the machines. That description is still accurate, and the transition from language models to equation-based models does not automatically change it. But equation-based models have one property that language models do not: their outputs are verifiable. An equation either matches the data or it does not. A simulation either predicts the right outcome or it does not. That verifiability is a form of accountability, because it means you can test whether the system is actually working, not just whether it sounds like it is working. And accountability is the single most important thing missing from the current AI landscape."
        }
        p {
            "The world does not need another chatbot. The world does not need another text generator. The world does not need another system that sounds confident while producing hallucinated garbage. What the world needs is a system that can observe reality, discover its structure, simulate its dynamics, predict its future, and verify its own predictions. That system exists in embryonic form in the research on symbolic regression, neural operators, physics-informed machine learning, and world models. It is scattered across dozens of papers, hundreds of experiments, and thousands of hours of work by researchers who will never be famous because they are not building products that consumers can use in a browser. But their work is the most important work happening in AI right now, and it is the work that will ultimately determine whether artificial intelligence becomes a genuine tool for understanding reality or just another way to generate plausible-sounding text that nobody can trust."
        }
        p {
            "I want to end with something I believe deeply. The universe is not made of words. It is made of structure, of pattern, of law, of mechanism, of the mathematical relationships that govern how every particle, every wave, every field, and every force behaves. If we want to build machines that understand the universe, we need to build machines that speak the language of the universe, and that language is mathematics. It has always been mathematics. Galileo said it four hundred years ago: the book of nature is written in the language of mathematics (^9). Nothing has changed since then, except that now we have the tools to let machines read that book themselves. The equations are already multimodal. The equations are already compressed. The equations are already grounded in reality. We just need to build systems that can discover them, and when we do, the age of text-based AI will look like what it always was: a necessary but ultimately narrow step on the path toward something much deeper, much truer, and much more powerful."
        }
        p {
            "I am not sure what comes next for me personally. I said in my first post that I do not know where I am going, and that has not changed. But I know what I believe. I believe that language is limited. I believe that equations are powerful. I believe that simulation is the real intelligence. I believe that mathematical structure is multimodal by default. And I believe that the future belongs to systems that can discover the hidden order beneath the chaos of the world, because that hidden order is the closest thing to God that I have ever found, and I say that as someone who has spent a very long time looking."
        }
        p { "Till next time 👋!" }
        h2 { id: "references",
            a { href: "#references", class: "header", "References" }
        }
        p {
            "(^1): Udrescu, S. M. & Tegmark, M., "
            em { "AI Feynman: A Physics-Inspired Method for Symbolic Regression" }
            ", "
            a { href: "https://arxiv.org/abs/1905.11481", "arXiv:1905.11481" }
        }
        p {
            "(^2): Lu, L. et al., "
            em { "Learning Nonlinear Operators via DeepONet" }
            ", "
            a { href: "https://arxiv.org/abs/1910.03193", "arXiv:1910.03193" }
        }
        p {
            "(^3): Raissi, M. et al., "
            em { "Physics Informed Deep Learning" }
            ", "
            a { href: "https://arxiv.org/abs/1711.10561", "arXiv:1711.10561" }
        }
        p {
            "(^4): Lorenz, E. N., "
            em { "Deterministic Nonperiodic Flow" }
            ", Journal of the Atmospheric Sciences, 1963"
        }
        p {
            "(^5): Kermack, W. O. & McKendrick, A. G., "
            em { "A Contribution to the Mathematical Theory of Epidemics" }
            ", Proceedings of the Royal Society, 1927"
        }
        p {
            "(^6): Pearl, J., "
            em { "Causality: Models, Reasoning, and Inference" }
            ", Cambridge University Press, 2009"
        }
        p {
            "(^7): Schölkopf, B. et al., "
            em { "Toward Causal Representation Learning" }
            ", "
            a { href: "https://arxiv.org/abs/2102.11107", "arXiv:2102.11107" }
        }
        p {
            "(^8): Hafner, D. et al., "
            em { "Mastering Diverse Domains through World Models" }
            ", "
            a { href: "https://www.nature.com/articles/s41586-025-08744-2", "Nature" }
        }
        p {
            "(^9): Galileo Galilei, "
            em { "Il Saggiatore" }
            " (The Assayer), 1623"
        }
    }
}

use dioxus::prelude::*;
use crate::components::blog::code::CodeBlock;
