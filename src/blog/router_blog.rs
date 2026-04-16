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
    #[route("/pharaohs-were-the-first-to-achieve-asi")]
    PharaohsWereTheFirstToAchieveAsi {},
    #[route("/llms-destroyed-the-internet-lmms-will-make-it-alive")]
    LlmsDestroyedTheInternetLmmsWillMakeItAlive {},
    #[route("/training-is-an-evil-concept-lmms-eliminates-it-altogether")]
    TrainingIsAnEvilConceptLmmsEliminatesItAltogether {},
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
            BookRoute::PharaohsWereTheFirstToAchieveAsi {} => {
                use_mdbook::mdbook_shared::PageId(11usize)
            }
            BookRoute::LlmsDestroyedTheInternetLmmsWillMakeItAlive {} => {
                use_mdbook::mdbook_shared::PageId(12usize)
            }
            BookRoute::TrainingIsAnEvilConceptLmmsEliminatesItAltogether {} => {
                use_mdbook::mdbook_shared::PageId(13usize)
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
                        title: " 1 |---| Announcing Kevin RS 🚀 |---| announcement |---| announcing-kevin-rs |---| Apr 21 2025 |---| Kevin RS is a fully open-source Rust framework for building fast, autonomous AGI agents. Designed for reliability, performance, and general intelligence research, it supports zero-shot learning, multi-agent execution, and future-ready tooling - without relying on fragile stacks. |---| assets/images/banner_post_1.webp"
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
                        title: " 2 |---| Rethinking ARC‑AGI 🧠 |---| analysis |---| rethinking-arc-agi |---| Apr 22 2025 |---| Francois Chollet's ARC‑AGI benchmark aimed to measure fluid intelligence in AI, but early versions were undermined by brute-force pattern-matching. |---| assets/images/banner_post_2.webp"
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
                        title: " 3 |---| Just Don't Pick Up the Brush |---| existence |---| who-am-i |---| Jan 27 2026 |---| Hey, Community. I am finally doing this. My name is Mahmoud Harmouch, and I am new here, though in many ways, I have been searching for a space like this for my entire life. For over two decades, I have struggled with a complicated mix of mental health conditions, ADHD, autism, and an extreme stage of PTSD, among others. |---| assets/images/banner_post_3.webp"
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
                        title: " 4 |---| An Empty Life Filled With Constant Suffering |---| existence |---| an-empty-life-filled-with-constant-suffering |---| Apr 07 2026 |---| An empty life filled with constant suffering. |---| assets/images/banner_post_4.webp"
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
                        title: " 5 |---| It is always the Russians |---| existence |---| it-is-always-the-russians |---| Apr 07 2026 |---| It is always the Russians |---| assets/images/banner_post_5.webp"
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
                        title: " 6 |---| As Engineers, LLMs should pay us for tokens usage. |---| tech |---| as-engineers-llms-should-pay-us-for-tokens-usage |---| Apr 07 2026 |---| As Engineers, LLMs should pay us for tokens usage. |---| assets/images/banner_post_6.webp"
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
                        title: " 7 |---| Technology Has Destroyed My Livelihood |---| tech |---| technology-has-destroyed-my-livelihood |---| Apr 07 2026 |---| Technology Has Destroyed My Livelihood |---| assets/images/banner_post_7.webp"
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
                        title: " 8 |---| Language is Limited. ASI is Impossible. |---| tech |---| language-is-limited-asi-is-impossible |---| Apr 08 2026 |---| Language is Limited. ASI is Impossible. |---| assets/images/banner_post_8.webp"
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
                        title: " 9 |---| Christianity Makes Perfect Sense! |---| religion |---| christianity-makes-perfect-sense |---| Apr 08 2026 |---| Christianity Makes Perfect Sense! |---| assets/images/banner_post_9.webp"
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
                        title: " 10 |---| LLMs are Usefull. LMMs will Break Reality |---| tech |---| llms-are-usefull-lmms-will-break-reality |---| Apr 10 2026 |---| LLMs are Usefull. LMMs will Break Reality |---| assets/images/banner_post_10.webp"
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
                        title: " 11 |---| Mathematical Equations are Multimodal by default |---| tech |---| mathematical-equations-are-multimodal-by-default |---| Apr 11 2026 |---| Mathematical Equations are Multimodal by default |---| assets/images/banner_post_11.webp"
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
        pages
            .push((
                11usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 12 |---| Pharaohs were the first to achieve ASI. |---| history |---| pharaohs-were-the-first-to-achieve-asi |---| Apr 13 2026 |---| Pharaohs were the first to achieve ASI. |---| assets/images/banner_post_12.webp"
                            .to_string(),
                        url: BookRoute::PharaohsWereTheFirstToAchieveAsi {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why I Think About Ancient Egypt At All".to_string(),
                                id: "why-i-think-about-ancient-egypt-at-all".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Symbols Were Not Decoration. They Were Compressed Reality."
                                    .to_string(),
                                id: "the-symbols-were-not-decoration.-they-were-compressed-reality."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Mathematics Was Not Primitive. It Was Applied Intelligence."
                                    .to_string(),
                                id: "the-mathematics-was-not-primitive.-it-was-applied-intelligence."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Pyramids As World Models Running At Civilizational Scale"
                                    .to_string(),
                                id: "pyramids-as-world-models-running-at-civilizational-scale"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Intelligence Encoded In Wisdom Literature And Text"
                                    .to_string(),
                                id: "intelligence-encoded-in-wisdom-literature-and-text"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Administration As The Neural Network Of Civilization"
                                    .to_string(),
                                id: "administration-as-the-neural-network-of-civilization"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "How Modern AI Gets The Pharaohs Wrong And Why It Matters"
                                    .to_string(),
                                id: "how-modern-ai-gets-the-pharaohs-wrong-and-why-it-matters"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What The Pharaohs Tell Us About Real Intelligence"
                                    .to_string(),
                                id: "what-the-pharaohs-tell-us-about-real-intelligence"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Closing Thought".to_string(),
                                id: "closing-thought".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "References".to_string(),
                                id: "references".to_string(),
                                level: 2usize,
                            },
                        ],
                        raw: String::new(),
                        id: ::use_mdbook::mdbook_shared::PageId(11usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::PharaohsWereTheFirstToAchieveAsi {},
            ::use_mdbook::mdbook_shared::PageId(11usize),
        );
        pages
            .push((
                12usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 13 |---| LLMs destroyed the Internet. LMMs will make it alive. |---| tech |---| llms-destroyed-the-internet-lmms-will-make-it-alive |---| Apr 15 2026 |---| LLMs destroyed the Internet. LMMs will make it alive. |---| assets/images/banner_post_13.webp"
                            .to_string(),
                        url: BookRoute::LlmsDestroyedTheInternetLmmsWillMakeItAlive {
                        },
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Internet Was Valuable Because It Was Made of Friction"
                                    .to_string(),
                                id: "the-internet-was-valuable-because-it-was-made-of-friction"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Language Models Actually Are and Why That Explains Everything"
                                    .to_string(),
                                id: "what-language-models-actually-are-and-why-that-explains-everything"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why the Architecture Matters More Than the Deployment Decision"
                                    .to_string(),
                                id: "why-the-architecture-matters-more-than-the-deployment-decision"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "How the Loss of Grounding Created the Silence That Replaced Discovery"
                                    .to_string(),
                                id: "how-the-loss-of-grounding-created-the-silence-that-replaced-discovery"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Grounded Intelligence Looks Like and Why It Changes Everything"
                                    .to_string(),
                                id: "what-grounded-intelligence-looks-like-and-why-it-changes-everything"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Internet That Mathematics Could Help Build"
                                    .to_string(),
                                id: "the-internet-that-mathematics-could-help-build"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What I Actually Hope For, and Why I Am Not Optimistic But Not Hopeless Either"
                                    .to_string(),
                                id: "what-i-actually-hope-for,-and-why-i-am-not-optimistic-but-not-hopeless-either"
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
                        id: ::use_mdbook::mdbook_shared::PageId(12usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::LlmsDestroyedTheInternetLmmsWillMakeItAlive {},
            ::use_mdbook::mdbook_shared::PageId(12usize),
        );
        pages
            .push((
                13usize,
                {
                    ::use_mdbook::mdbook_shared::Page {
                        title: " 14 |---| Training Is an Evil Concept. LMMs Eliminates it Altogether. |---| tech |---| training-is-an-evil-concept-lmms-eliminates-it-altogether |---| Apr 16 2026 |---| Training Is an Evil Concept. LMMs Eliminates it Altogether. |---| assets/images/banner_post_14.webp"
                            .to_string(),
                        url: BookRoute::TrainingIsAnEvilConceptLmmsEliminatesItAltogether {},
                        segments: vec![],
                        sections: vec![
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Training Is Not Learning. It Is Extraction Under a Different Name."
                                    .to_string(),
                                id: "training-is-not-learning.-it-is-extraction-under-a-different-name."
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Training Actually Optimizes for, and Why That Is the Problem"
                                    .to_string(),
                                id: "what-training-actually-optimizes-for,-and-why-that-is-the-problem"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "LMM: The Proof That Pure Mathematics Can Replace Training Entirely"
                                    .to_string(),
                                id: "lmm:-the-proof-that-pure-mathematics-can-replace-training-entirely"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "Why the Mathematical Alternative Is Not Just a Technical Preference"
                                    .to_string(),
                                id: "why-the-mathematical-alternative-is-not-just-a-technical-preference"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Consent Problem Is Not Solved by Better Licensing"
                                    .to_string(),
                                id: "the-consent-problem-is-not-solved-by-better-licensing"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What Memorization Reveals About the Training Paradigm's Soul"
                                    .to_string(),
                                id: "what-memorization-reveals-about-the-training-paradigm's-soul"
                                    .to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "The Scale Argument Is a Moral Red Herring"
                                    .to_string(),
                                id: "the-scale-argument-is-a-moral-red-herring".to_string(),
                                level: 2usize,
                            },
                            ::use_mdbook::mdbook_shared::Section {
                                title: "What We Lose When We Stop Asking Why, and What We Gain When We Start"
                                    .to_string(),
                                id: "what-we-lose-when-we-stop-asking-why,-and-what-we-gain-when-we-start"
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
                        id: ::use_mdbook::mdbook_shared::PageId(13usize),
                    }
                },
            ));
        page_id_mapping.insert(
            BookRoute::TrainingIsAnEvilConceptLmmsEliminatesItAltogether {},
            ::use_mdbook::mdbook_shared::PageId(13usize),
        );
        ::use_mdbook::mdbook_shared::MdBook {
            summary: ::use_mdbook::mdbook_shared::Summary {
                title: Some("Summary".to_string()),
                prefix_chapters: vec![],
                numbered_chapters: vec![
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 1 |---| Announcing Kevin RS 🚀 |---| announcement |---| announcing-kevin-rs |---| Apr 21 2025 |---| Kevin RS is a fully open-source Rust framework for building fast, autonomous AGI agents. Designed for reliability, performance, and general intelligence research, it supports zero-shot learning, multi-agent execution, and future-ready tooling - without relying on fragile stacks. |---| assets/images/banner_post_1.webp"
                            .to_string(),
                        location: Some(BookRoute::AnnouncingKevinRs {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![1u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 2 |---| Rethinking ARC‑AGI 🧠 |---| analysis |---| rethinking-arc-agi |---| Apr 22 2025 |---| Francois Chollet's ARC‑AGI benchmark aimed to measure fluid intelligence in AI, but early versions were undermined by brute-force pattern-matching. |---| assets/images/banner_post_2.webp"
                            .to_string(),
                        location: Some(BookRoute::RethinkingArcAgi {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![2u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 3 |---| Just Don't Pick Up the Brush |---| existence |---| who-am-i |---| Jan 27 2026 |---| Hey, Community. I am finally doing this. My name is Mahmoud Harmouch, and I am new here, though in many ways, I have been searching for a space like this for my entire life. For over two decades, I have struggled with a complicated mix of mental health conditions, ADHD, autism, and an extreme stage of PTSD, among others. |---| assets/images/banner_post_3.webp"
                            .to_string(),
                        location: Some(BookRoute::WhoAmI {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![3u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 4 |---| An Empty Life Filled With Constant Suffering |---| existence |---| an-empty-life-filled-with-constant-suffering |---| Apr 07 2026 |---| An empty life filled with constant suffering. |---| assets/images/banner_post_4.webp"
                            .to_string(),
                        location: Some(BookRoute::AnEmptyLifeFilledWithConstantSuffering {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![4u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 5 |---| It is always the Russians |---| existence |---| it-is-always-the-russians |---| Apr 07 2026 |---| It is always the Russians |---| assets/images/banner_post_5.webp"
                            .to_string(),
                        location: Some(BookRoute::ItIsAlwaysTheRussians {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![5u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 6 |---| As Engineers, LLMs should pay us for tokens usage. |---| tech |---| as-engineers-llms-should-pay-us-for-tokens-usage |---| Apr 07 2026 |---| As Engineers, LLMs should pay us for tokens usage. |---| assets/images/banner_post_6.webp"
                            .to_string(),
                        location: Some(BookRoute::AsEngineersLlmsShouldPayUsForTokensUsage {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![6u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 7 |---| Technology Has Destroyed My Livelihood |---| tech |---| technology-has-destroyed-my-livelihood |---| Apr 07 2026 |---| Technology Has Destroyed My Livelihood |---| assets/images/banner_post_7.webp"
                            .to_string(),
                        location: Some(BookRoute::TechnologyHasDestroyedMyLivelihood {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![7u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 8 |---| Language is Limited. ASI is Impossible. |---| tech |---| language-is-limited-asi-is-impossible |---| Apr 08 2026 |---| Language is Limited. ASI is Impossible. |---| assets/images/banner_post_8.webp"
                            .to_string(),
                        location: Some(BookRoute::LanguageIsLimitedAsiIsImpossible {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![8u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 9 |---| Christianity Makes Perfect Sense! |---| religion |---| christianity-makes-perfect-sense |---| Apr 08 2026 |---| Christianity Makes Perfect Sense! |---| assets/images/banner_post_9.webp"
                            .to_string(),
                        location: Some(BookRoute::ChristianityMakesPerfectSense {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![9u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 10 |---| LLMs are Usefull. LMMs will Break Reality |---| tech |---| llms-are-usefull-lmms-will-break-reality |---| Apr 10 2026 |---| LLMs are Usefull. LMMs will Break Reality |---| assets/images/banner_post_10.webp"
                            .to_string(),
                        location: Some(BookRoute::LlmsAreUsefullLmmsWillBreakReality {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![10u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 11 |---| Mathematical Equations are Multimodal by default |---| tech |---| mathematical-equations-are-multimodal-by-default |---| Apr 11 2026 |---| Mathematical Equations are Multimodal by default |---| assets/images/banner_post_11.webp"
                            .to_string(),
                        location: Some(BookRoute::MathematicalEquationsAreMultimodalByDefault {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![11u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 12 |---| Pharaohs were the first to achieve ASI. |---| history |---| pharaohs-were-the-first-to-achieve-asi |---| Apr 13 2026 |---| Pharaohs were the first to achieve ASI. |---| assets/images/banner_post_12.webp"
                            .to_string(),
                        location: Some(BookRoute::PharaohsWereTheFirstToAchieveAsi {
                        }),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![12u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 13 |---| LLMs destroyed the Internet. LMMs will make it alive. |---| tech |---| llms-destroyed-the-internet-lmms-will-make-it-alive |---| Apr 15 2026 |---| LLMs destroyed the Internet. LMMs will make it alive. |---| assets/images/banner_post_13.webp"
                            .to_string(),
                        location: Some(BookRoute::LlmsDestroyedTheInternetLmmsWillMakeItAlive {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![13u32]),
                        ),
                        nested_items: vec![],
                    }),
                    ::use_mdbook::mdbook_shared::SummaryItem::Link(::use_mdbook::mdbook_shared::Link {
                        name: " 14 |---| Training Is an Evil Concept. LMMs Eliminates it Altogether. |---| tech |---| training-is-an-evil-concept-lmms-eliminates-it-altogether |---| Apr 16 2026 |---| Training Is an Evil Concept. LMMs Eliminates it Altogether. |---| assets/images/banner_post_14.webp"
                            .to_string(),
                        location: Some(BookRoute::TrainingIsAnEvilConceptLmmsEliminatesItAltogether {}),
                        number: Some(
                            ::use_mdbook::mdbook_shared::SectionNumber(vec![14u32]),
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
            a { href: "/blogs/who-am-i", "my previous post" }
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
            a { href: "/blogs/who-am-i", "my previous post" }
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
            a { href: "/blogs/who-am-i", "Just Don't Pick Up the Brush" }
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
            a { href: "/blogs/who-am-i", "who I am" }
            ", about "
            a { href: "/blogs/it-is-always-the-russians", "the Russians killing God" }
            ", and about "
            a { href: "/blogs/an-empty-life-filled-with-constant-suffering",
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
            a { href: "/blogs/who-am-i", "my first post" }
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
            a { href: "/blogs/as-engineers-llms-should-pay-us-for-tokens-usage",
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
            a { href: "/blogs/an-empty-life-filled-with-constant-suffering",
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
            a { href: "/blogs/who-am-i", "Just Don't Pick Up the Brush" }
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
            a { href: "/blogs/an-empty-life-filled-with-constant-suffering",
                "An Empty Life Filled With Constant Suffering"
            }
            ". I could not understand how a just and powerful God could exist in a world where pain seemed so unevenly distributed. Some people struggle endlessly despite their efforts, while others seem to move forward with little resistance, and that imbalance felt impossible to ignore. I wanted answers that did not simply explain away the problem, but truly engaged with the weight of it. Because real problems require real explanations, not quick responses designed to end the conversation. I found myself thinking about these questions alone, often without anyone around me who could answer them in a way that satisfied both my mind and my sense of reality. And that isolation made the search feel even more personal and necessary."
        }
        p {
            "The more I searched for answers, the more I noticed that many responses I received were designed to protect belief rather than explain truth. They often felt like they were closing the door instead of opening it. This created a kind of internal conflict, because I wanted to believe, but I also wanted to understand. And when those two desires are not aligned, it creates a tension that is difficult to live with. I began to feel that I was being asked to accept conclusions without fully examining the reasoning behind them, a situation that felt like being trapped in a system of words with no connection to the real thing. If something is true, it should become stronger when examined in the light of day, not weaker. That idea stayed with me and continued to shape how I approached everything afterward."
        }
        p {
            "At the same time, my personal life was not easy, and that made these questions feel even more urgent. I experienced homelessness, unemployment, and the effects of living in a country affected by war and instability. On top of that, I was dealing with ADHD and autism, which made it harder for me to fit into systems that already felt difficult to navigate, as I detailed in "
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
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
            a { href: "/blogs/it-is-always-the-russians", "It is always the Russians" }
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
            a { href: "/blogs/language-is-limited-asi-is-impossible",
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
            a { href: "/blogs/language-is-limited-asi-is-impossible",
                "Language is Limited. ASI is Impossible."
            }
            ", I spent a long time explaining why language is not the same thing as thought, why words are not the same thing as understanding, and why a machine built on text alone will never cross the wall into true superintelligence. I still believe all of that, and I will not take any of it back, because the argument was honest and the logic was solid. But today I want to go further. I want to talk about something that has been sitting in my head for years, growing louder every day, and I need to get it out before it eats me alive. I want to talk about why large language models are still genuinely useful, despite their limits, and why large mathematical models, as introduced in this whitepaper draft, are something far more serious, something that could actually begin to crack the surface of reality itself. I know that sounds extreme, and I know some people will read that sentence and roll their eyes, but I am asking you to stay with me, because the argument I am about to make is not based on hype or fantasy. It is based on what I have seen, what I have built, and what I understand about the difference between describing the world and actually modeling the world. That difference is the whole point of this post, and once you see it clearly, everything else falls into place."
        }
        p {
            "I have been thinking about this ever since I wrote "
            a { href: "/blogs/an-empty-life-filled-with-constant-suffering",
                "An Empty Life Filled With Constant Suffering"
            }
            ", where I talked about how words cannot fully capture my thoughts, and how language always falls short of the real thing inside our heads. That frustration is what led me here, because if language is limited, then we need to ask what comes after language, and the answer is not more language. The answer is structure, equations, simulation, and direct modeling of the physical world. That is what large mathematical models point toward, not because they are perfect today, but because they represent a direction that goes beyond text and into something much deeper. I wrote in "
            a { href: "/blogs/it-is-always-the-russians", "It is always the Russians" }
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
            a { href: "/blogs/as-engineers-llms-should-pay-us-for-tokens-usage",
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
            a { href: "/blogs/as-engineers-llms-should-pay-us-for-tokens-usage",
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
            a { href: "/blogs/language-is-limited-asi-is-impossible",
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
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
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
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
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
            a { href: "/blogs/an-empty-life-filled-with-constant-suffering",
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
            a { href: "/blogs/it-is-always-the-russians", "It is always the Russians" }
            ", where I described how the Soviets killed God and the people who came from that tradition built the most powerful AI systems in the world. That pattern has not changed. The same people who built the current language models will build the next mathematical models, and they will carry the same assumptions, the same blindness, and the same hunger for control into the new era."
        }
        p {
            "The first danger is misuse. A system that can simulate the physical world can also simulate weapons, surveillance systems, environmental manipulation, and biological agents. This is not speculation. It is a direct consequence of the capability I have been describing. If a machine can discover equations from observation, then it can discover the equations of harmful systems just as easily as the equations of helpful ones. If a machine can simulate the future, then it can simulate the future of a weapon just as easily as the future of a bridge. The same tool that enables scientific breakthrough also enables scientific catastrophe, and the difference between the two depends entirely on who controls the tool and what they intend to do with it. Given the current state of the world, where power is concentrated in the hands of a few companies and governments, I am not optimistic about who will control these tools. The same companies that exploited LLMs for profit will exploit LMMs for profit, and the same governments that used surveillance for control will use simulation for control, and the people at the bottom will pay the price, as they always have."
        }
        p {
            "The second danger is displacement, and this one is personal because I have lived through it. In "
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
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
            a { href: "/blogs/christianity-makes-perfect-sense", "Christianity Makes Perfect Sense!" }
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
            a { href: "/blogs/llms-are-usefull-lmms-will-break-reality",
                "LLMs are Useful. LMMs will Break Reality"
            }
            ", I made a case that language models are genuinely useful tools trapped inside a symbolic cage, and that multimodal models represent the first real step toward machines that can perceive and simulate the physical world. I talked about how equations are more powerful than sentences, how simulation is the real intelligence, and how the transition from text to structure is the most important shift happening in AI right now. I meant every word of that, and I am not walking any of it back, but I realized after publishing it that I left something important on the table, something that has been sitting in my head for years and that I need to say clearly before I can move on. The thing I left out is the reason why mathematical equations are special in a way that goes far beyond what most people in the AI conversation understand. Most people think of equations as formulas you memorize in school, abstract things that live on chalkboards and have no connection to real life. That is completely wrong, and the fact that so many people believe it is one of the biggest intellectual failures of modern education. Mathematical equations are not abstract decorations. They are the most compressed, most general, most powerful representations of reality that humans have ever discovered, and they are multimodal by default, meaning they can generate text, images, motion, sound, and physical predictions all from the same compact structure. That is the argument I am going to make in this post, and I am going to make it so thoroughly that by the end, you will either agree with me or you will have to explain exactly where my reasoning breaks down."
        }
        p {
            "I have been building toward this idea across several posts now, and I want to connect the threads so that people who have been following along can see how everything fits together. In "
            a { href: "/blogs/language-is-limited-asi-is-impossible",
                "Language is Limited. ASI is Impossible."
            }
            ", I argued that words are not thoughts, that the brain is not a text machine, and that any system built purely on language inherits the permanent gap between symbols and reality. In "
            a { href: "/blogs/an-empty-life-filled-with-constant-suffering",
                "An Empty Life Filled With Constant Suffering"
            }
            ", I talked about how words cannot fully capture what I feel, how language always falls short of the real thing inside my head, and how that frustration has shaped everything I think about intelligence. In "
            a { href: "/blogs/as-engineers-llms-should-pay-us-for-tokens-usage",
                "As Engineers, LLMs should pay us for tokens usage"
            }
            ", I argued that the current system exploits the very people who make it work, and that the value flows upward while the cost flows downward. And in "
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
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
            a { href: "/blogs/who-am-i", "my first post" }
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
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
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
            a { href: "/blogs/language-is-limited-asi-is-impossible",
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
            a { href: "/blogs/language-is-limited-asi-is-impossible",
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
            a { href: "/blogs/who-am-i", "my first post" }
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
            a { href: "/blogs/it-is-always-the-russians", "It is always the Russians" }
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
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
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
#[component(no_case_check)]
pub fn PharaohsWereTheFirstToAchieveAsi() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey everyone 👋," }
        p {
            "I want to warn you upfront that this post is going to sound strange. I am a software engineer who spends most of his days thinking about rust compilers, physics-informed neural networks, and why language models are not as intelligent as the marketing says they are. I wrote about that in "
            a { href: "/blogs/llms-are-usefull-lmms-will-break-reality",
                "LLMs are Useful. LMMs will Break Reality"
            }
            ", and I stand by every word. But today I want to do something different. I want to go back, way back, not to the sixties or the nineties or even to Turing, but to ancient Egypt, to a time when pharaohs were gods and the Nile was the spine of the world, and I want to make a case that feels almost absurd the first time you hear it. The case is this: the pharaonic civilization was the first human system to achieve something functionally equivalent to artificial superintelligence, not through silicon or transformers or gradient descent, but through symbols, mathematics, architecture, administration, and the compression of collective human knowledge into durable physical and textual form. I am not saying the pharaohs had computers. I am saying they built something that no individual human mind could contain, and they made it run for thousands of years, and it was smarter than any of its parts. That is the definition I care about, and by that definition, they did it first."
        }
        p {
            "I know how that sounds. I know some people will close this tab immediately. But I am asking you to stay, because the argument is more rigorous than the title suggests, and because I think it connects directly to the questions I have been asking in every post I have written so far. In "
            a { href: "/blogs/language-is-limited-asi-is-impossible",
                "Language is Limited. ASI is Impossible."
            }
            ", I argued that intelligence is not the same thing as language, and that any system confined to text will always be smaller than the world it is trying to describe. In "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by Default"
            }
            ", I argued that equations are the most compressed, most powerful representation of reality that humans have ever found, and that any real intelligence must be grounded in mathematical structure rather than statistical pattern in text. And in "
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
                "Technology Has Destroyed My Livelihood"
            }
            ", I talked about how the modern system extracts value from people who have nothing and gives credit to people who already have everything. All of those ideas connect to what I am about to say about the pharaohs, because in a strange way, this ancient civilization solved every one of those problems, not perfectly, not justly, and not without enormous human suffering, but structurally in ways that we have not managed to replicate even with all of our modern technology. That is the uncomfortable truth I am going to try to explain."
        }
        h2 { id: "why-i-think-about-ancient-egypt-at-all",
            a {
                href: "#why-i-think-about-ancient-egypt-at-all",
                class: "header",
                "Why I Think About Ancient Egypt At All"
            }
        }
        p {
            "Let me start with something personal, because every idea I have ever had has been rooted in personal experience, and I am not going to pretend otherwise. I grew up in a country that was broken by war, as I wrote in "
            a { href: "/blogs/who-am-i", "my first post" }
            ", and one of the things that war does is strip away every assumption you had about civilization, about order, about the predictability of tomorrow. When everything around you is collapsing, you start to wonder what actually holds a civilization together, what the essential parts are, what would survive if everything else burned. That question has been sitting in me for years, and it is the question that brought me to ancient Egypt, because Egypt is the only civilization in human history that held together for three thousand years with a continuous identity and a continuous system of governance, knowledge, and symbolic order. That is the most durable example of organized human intelligence that we have ever produced, and before we talk about artificial superintelligence in the modern sense, I think we should understand why that worked, because understanding what worked for three thousand years is more valuable than speculating about what might work in the next thirty."
        }
        p {
            "I also want to say something that connects to my post about "
            a { href: "/blogs/it-is-always-the-russians", "the Russians killing God" }
            ". The pharaonic state was built on the belief that the king was divine, that the pharaoh was the living incarnation of Horus and the dead image of Osiris, and that the cosmic order called Ma'at was maintained through the king's actions and rituals. A lot of people look at that belief system and see primitive superstition. I look at it and see something more interesting. I see a system that successfully encoded collective intelligence into a shared symbolic substrate, where the divine status of the pharaoh was the glue that held together millions of people across thousands of miles and thousands of years. The religion was not just a religion. It was a coordination mechanism. It was a distributed operating system for human civilization, and it worked with a reliability that our modern institutions can barely match. I am not saying the pharaohs were right about being gods. I am saying that the belief that they were gods, and the entire symbolic and administrative apparatus built around that belief, functioned as a kind of superintelligence, capable of making decisions, storing knowledge, coordinating resources, and executing plans at a scale that no individual human being could manage. That is the claim I want to unpack."
        }
        p {
            "I should also acknowledge the obvious objection, which is that using the word ASI for something that happened three thousand years ago before electricity or computing is ridiculous, and that I am playing word games. I want to address this directly and honestly. I think the definition of ASI that most people use is too narrow. When people say ASI, they usually mean a digital system running on silicon that exceeds human intelligence across all domains. But that definition encodes a specific assumption about the substrate, an assumption that intelligence can only live in chips and code. I do not accept that assumption, and I have been arguing against it in various forms since I started writing these posts. Intelligence is the ability to model reality, to coordinate complex systems, to compress information into usable structures, and to generate outputs that no individual component of the system could have produced. By that definition, the pharaonic state was a form of superintelligence, because it modeled and managed a reality, namely the Egyptian civilization and its relationship with the natural and divine worlds, that exceeded the capacity of any single human mind. The substrate was flesh, stone, papyrus, and symbol rather than silicon, but the function was the same, and the function is what matters."
        }
        p {
            "One more thing before I get into the evidence. I want to be clear that I am not romanticizing the pharaohs. The Egyptian state was built on slave labor, on conquest, on the violent suppression of dissent, and on a system of power that treated most of its population as instruments rather than people. The parallels to the modern tech industry are uncomfortable and I do not intend to gloss over them. I wrote in "
            a { href: "/blogs/as-engineers-llms-should-pay-us-for-tokens-usage",
                "As Engineers, LLMs should pay us for tokens usage"
            }
            " that the current system extracts value from engineers and gives it to the people who own the machines. Ancient Egypt did exactly the same thing at a civilizational scale, extracting labor from millions of peasants and giving the glory to the pharaoh and the priests. But the structural achievement is separate from the moral character of the system, just as a very powerful programming language can be used to write exploitative spyware or to build a hospital management system. I am analyzing the structural achievement here, and I am using that analysis to say something about what intelligence really is and what it really requires. The moral reckoning is separate, real, and necessary, but it does not change the structural observation."
        }
        p {
            "Finally, I want to explain why this topic connects to AI in a way that is not just metaphorical. The most important question in artificial intelligence right now is not what language models can do. It is what kind of substrate intelligence actually requires, and whether that substrate has to be digital. If I can show that a civilization of flesh-and-blood humans, organized through symbols, mathematics, architecture, and ritual, achieved something functionally equivalent to superintelligence and sustained it for three thousand years, then I have shown something important: that the substrate does not have to be silicon, and that the components of superintelligence are not mystical or unprecedented but are in fact ancient and well-documented. And if those components are ancient and well-documented, then we can study them, learn from them, and use that understanding to think more clearly about what we are actually building when we talk about AGI. That is the goal of this post, and that is why I think it belongs on this blog alongside the more technically conventional posts I have been writing."
        }
        h2 { id: "the-symbols-were-not-decoration-they-were-compressed-reality",
            a {
                href: "#the-symbols-were-not-decoration-they-were-compressed-reality",
                class: "header",
                "The Symbols Were Not Decoration. They Were Compressed Reality."
            }
        }
        p {
            "The strongest and most accessible piece of evidence for Egypt as ASI is the hieroglyphic writing system, and I want to engage with it seriously because it is far more impressive than most people realize. When most people think of hieroglyphs, they think of pretty pictures on tomb walls, decorative art that happens to contain meaning. That framing is completely wrong, and the British Museum's major exhibition on hieroglyphs makes clear why. Hieroglyphs were not simplified drawings. They were a fully developed writing system capable of recording names, royal decrees, religious texts, administrative records, medical knowledge, mathematical problems, poetry, autobiography, prophecy, and the complete daily functioning of a complex state. The system was sophisticated enough to represent phonetic sounds, semantic categories, determinatives that supplied context for ambiguous signs, and logograms that worked as direct symbols for concepts. No other writing system from the ancient world combined all of these functions with such consistency and longevity. The Egyptians used the same basic system continuously for more than three thousand years, which means that a scribe from the New Kingdom could read texts carved during the Old Kingdom, a temporal span roughly equivalent to a modern person being able to read texts from ancient Greece without a translator."
        }
        p {
            "What matters about writing, in the context of this argument, is that writing is the external storage of intelligence. Before writing, everything a civilization knew lived inside the fragile and mortality-limited medium of human memory, passed from person to person through speech and practice, vulnerable to the death of teachers, the dispersal of communities, and the inevitability of forgetting. Writing breaks that dependency. When the Egyptians carved instructions into stone, they were not just recording information. They were externalizing intelligence from biological memory into a persistent, distributable, searchable medium that could outlive any individual mind. This is exactly what we do when we write code, design databases, or train machine learning models. We take intelligence that lives in a person's head and we move it into a substrate that can be copied, transmitted, versioned, and applied by other people without the original person being present. The Egyptians did this with stone and papyrus, and the intelligence they externalized was not trivial. The Maxims of Ptahhotep, one of the oldest complete wisdom texts in the world, dating to the Old Kingdom, contains insights about leadership, social conduct, truth, and the relationship between intelligence and moral character that are still worth reading today. These were not primitive superstitions. They were compressed frameworks for navigating complex social reality, encoded in a medium that could distribute them across time and space without the presence of their author."
        }
        p {
            "The MFTH academic paper on ancient Egyptian concepts of intelligence provides crucial support for this argument. The paper analyzes the primary texts and shows that the Egyptians had a rich, differentiated vocabulary for intelligence and its relationship to knowledge and skill. The core term they used was "
            em { "sAr" }
            ", which the paper translates as encompassing intelligence, understanding, and discernment together in a single concept. They also distinguished this from acquired knowledge ("
            em { "rx" }
            "), from skill, and from wisdom derived from experience. Perhaps most remarkable is the concept of intelligence in the womb, a phrase used in royal inscriptions and administrative texts to describe individuals who were recognized as gifted with innate understanding before formal training. The 5th Dynasty physician Niankh-sekhmet is praised in an inscription that says the god gave the king knowledge in the womb, while the royal butler Djehuty in his tomb describes himself as one who planned the time, foretold the coming, skilled in espying the future, versed in yesterday, planning tomorrow, expert in what will be. These are not vague spiritual claims. They are specific descriptions of a person whose function was forward-looking predictive intelligence in service of the state, and they show that the Egyptians not only valued intelligence but had a sophisticated framework for categorizing and cultivating it. The idea that intelligence is about prediction, about modeling the future from knowledge of the past, is the same idea that underlies every serious theory of intelligence from modern cognitive science."
        }
        p {
            "The role of the heart in Egyptian thinking about intelligence is also worth examining carefully, because it reveals something profound about their model of the mind. The ancient Egyptians believed the heart, not the brain, was the seat of intelligence, emotion, will, and moral judgment. The MFTH paper quotes several inscriptions that make this explicit, including one from Ptahhotep that says the heart is the creator of its master, and another that says a man's heart is his life, prosperity and health. These might sound like metaphors, and in some respects they are, but they reflect something more than poetry. The Egyptians were describing a unified model of intelligence in which rational thought, emotional regulation, and moral character were not separate faculties but aspects of a single integrated system. Modern cognitive science and neuroscience have gradually been converging on a similar picture, recognizing that emotion and reason are not opposites but deeply intertwined processes, that moral judgment cannot be separated from emotional experience, and that the traditional Western division between feeling and thinking reflects a false dichotomy rather than a real anatomical or cognitive boundary. The Egyptians got there four thousand years earlier, not through formal neuroscience but through careful observation of human behavior and the systematic encoding of those observations in their philosophical and religious texts."
        }
        p {
            "The hieroglyphic system also serves as an example of what I described in "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by Default"
            }
            ": a representation that can generate multiple output modalities from a single compact structure. A hieroglyphic inscription is simultaneously textual, visual, and architectural. The same signs that carry phonetic and semantic meaning are also carefully composed visual objects, aesthetically organized within the available space, often in ways that reinforce the meaning through visual form. The hieroglyph for a sitting king looks like a sitting king, and this iconic property gives the inscription a visual dimension that reinforces the semantic content, making it simultaneously more memorable and more accessible to people who could not read the full phonetic content. The determinatives, which are hieroglyphic signs added to words to indicate their semantic category, function like metadata tags in a modern database, allowing the reader to disambiguate meaning from context in a way that pure phonetic writing cannot do. This is a multimodal encoding system, and it was already being used for sophisticated administrative and intellectual purposes over five thousand years ago. When we marvel at the multimodal capabilities of modern AI systems, we should at least acknowledge that the concept was not invented in Silicon Valley."
        }
        p {
            "The sheer durability of the hieroglyphic system across three thousand years is itself a form of evidence for its power as an intelligence substrate. A writing system that fails, that is ambiguous, that cannot express the range of concepts a civilization needs, will be replaced or modified quickly. The Egyptians kept their core system largely intact for three millennia, adapting it and extending it but never abandoning it, which tells us that it was genuinely adequate for the purposes they needed it for. It encoded their knowledge of medicine, mathematics, astronomy, agriculture, law, theology, and history with enough precision that we can still read and understand large portions of it today, and what we find when we read it is not primitive guesswork but sophisticated, carefully reasoned engagement with the practical problems of running a complex civilization. The fact that our most powerful AI systems today struggle to read hieroglyphs with full accuracy, and often make the kind of historical and cultural mistakes that the AI/Pharaoh paper documents, is a reminder that the system the Egyptians built was not simple or obvious. It was the product of millennia of refinement, and it encodes intelligence that modern machines have not yet fully decoded."
        }
        p {
            "One last point about the symbolic dimension. The fact that the pharaoh wore specific crowns, carried specific scepters, executed specific rituals on specific days of the celestial calendar, all of this was not mere ceremony. It was symbolic computation. Each element of the royal ritual was a piece of a larger system that encoded the relationship between the human king, the divine world, the natural cycles of the Nile, and the cosmic order that held everything together. Changing one element affected the meaning of every other element, just as changing a variable in an equation changes its entire output. The ritual system was a form of symbolic programming, and the pharaoh was both the central processor of that system and its most important symbol. This is a strange form of computation, very different from what we build in silicon today, but functionally it achieved something remarkable: it kept millions of people coordinated, motivated, and productive across vast distances and long time spans, and it did so with a reliability that modern institutions, for all their technology, still struggle to match."
        }
        h2 { id: "the-mathematics-was-not-primitive-it-was-applied-intelligence",
            a {
                href: "#the-mathematics-was-not-primitive-it-was-applied-intelligence",
                class: "header",
                "The Mathematics Was Not Primitive. It Was Applied Intelligence."
            }
        }
        p {
            "The second major pillar of the Egypt-as-ASI argument is mathematics, and this is where the evidence becomes most concrete and most verifiable, because the mathematical papyri have survived and we can actually read them. The Rhind Mathematical Papyrus, dating to around 1650 BCE and now held in the British Museum, is a copy of an even older text and contains approximately 85 problems with detailed solutions covering arithmetic, algebra, and geometry. The Moscow Mathematical Papyrus, slightly older, contains 25 problems including one of the most remarkable calculations in the ancient world: the formula for the volume of a truncated pyramid. That formula requires understanding of frustum geometry and involves a level of abstract spatial reasoning that most people would not expect from a civilization that supposedly only knew how to pile up stones. These are not guesses or lucky approximations. They are rigorous mathematical results, and the fact that they are correct can be verified by anyone who knows the relevant formulas, which means the Egyptians genuinely understood the mathematics, not just the results, because you cannot repeatedly get the right answer by luck."
        }
        p {
            "The Rhind Papyrus shows something even more important than the specific results it contains. It shows a sophisticated pedagogical system for transmitting mathematical knowledge from one generation to the next. The papyrus was clearly used as a teaching document, with problems organized from simpler to more complex, with worked examples followed by similar problems for the student to solve, and with notations that show the scribe's reasoning process step by step. This is a curriculum, a formal system for encoding mathematical intelligence in a transmissible form. The method of false position, which the papyrus uses to solve linear equations, is a specific algorithmic strategy: assume a convenient value for the unknown, compute the result, measure how far off you are, and scale accordingly to find the correct answer. This is an algorithm, in the same sense that a computer algorithm is a precise, repeatable procedure for solving a class of problems. The Egyptians were not just doing arithmetic. They were developing and transmitting computational methods, and those methods were sophisticated enough to solve problems that would challenge a modern high school student."
        }
        p {
            "The seked system deserves particular attention because it is the most direct evidence that Egyptian mathematics was connected to physical engineering at the highest level. The seked was a unit of measurement for the slope of a pyramid's face, defined as the horizontal displacement per unit of vertical rise, expressed in a specific combination of cubits and palms. The Rhind Papyrus contains several problems about seked calculations, and they show that Egyptian architects had a precise system for specifying and verifying the angle of a pyramid's face before construction began. This matters enormously if you think about what building a pyramid actually requires. A pyramid is not just a pile of stones. It is a structure that takes years to build, involves thousands of workers, and must maintain a consistent slope on all four faces from the base to the apex, all while growing taller and narrower as the work progresses. Without a mathematically precise specification of the intended slope, and a reliable method for checking whether the ongoing construction matches that specification, no pyramid would ever come out right. The seked was that specification system, and the fact that the pyramids do come out right, with angles that match the intended seked values to a remarkable degree of precision, is proof that the system was used and that it worked."
        }
        p {
            "The approximation of pi that appears in the Rhind Papyrus is another remarkable result. The Egyptians did not have the concept of pi as an abstract constant, but they had a practical rule for calculating the area of a circle: take the diameter, subtract one ninth, and square the result. This rule gives an implicit value of pi approximately equal to 3.1605, which is within one percent of the true value. For most architectural and engineering purposes, that approximation is more than adequate, and the fact that they arrived at it through geometric reasoning rather than by memorizing a constant tells us they understood something real about the relationship between a circle's diameter and its area, even if they expressed that understanding differently from how we express it today. I find it more impressive that they derived a working approximation than that later mathematicians derived a more precise one, because working out an approximation from first principles, using only the tools of practical geometry, requires genuine mathematical insight, not just calculation."
        }
        p {
            "I also want to connect Egyptian mathematics to the claim I made in "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by Default"
            }
            " about the compression power of mathematical representation. Egyptian mathematics was not abstract in the Greek sense, it did not pursue proofs or theoretical elegance for their own sake, but that is actually a point in its favor for my argument, because it shows that the Egyptians were using mathematics to compress real-world complexity directly into actionable structure. When they calculated the number of bricks needed to build a ramp of a given slope and width, they were compressing a complex logistical problem into a few numbers, numbers that could then drive the actual allocation of labor and materials. When they calculated the volume of grain in a cylindrical granary, they were compressing the continuous physical reality of a full barn into a single number that could be entered into an administrator's records and compared with other records across the kingdom. This is exactly what I said mathematical compression does: it takes a complex, high-dimensional reality and represents it with a compact structure that preserves the essential information while discarding the noise. The Egyptians were doing this on a civilizational scale, and the output was a state that could manage enormous flows of resources accurately enough to build structures that still stand today."
        }
        p {
            "The application of mathematics to medicine is equally impressive and equally underappreciated. The Ebers Papyrus and the Edwin Smith Papyrus, both dating to the New Kingdom but containing material that may be much older, show Egyptian physicians applying systematic, evidence-based reasoning to medical problems. The Edwin Smith Papyrus is particularly striking because it organizes 48 cases of trauma medicine in a systematic format: presentation, examination, diagnosis, verdict, and treatment, with each case following the same logical structure. The physician examines the patient's symptoms, categorizes the case into one of three verdicts (treatable, contestable, or untreatable), and prescribes a treatment accordingly. This is a formal case-based reasoning system, the same basic structure used in modern medical decision trees and clinical guidelines. The Egyptians were not just guessing at remedies. They were building and applying a structured system for medical diagnosis, one that encoded accumulated clinical knowledge in a transmissible form and applied it through a consistent algorithm. That is not primitive medicine. That is the skeleton of evidence-based medicine, developed thousands of years before the concept was formally named."
        }
        p {
            "I want to close this section with a thought that connects Egyptian mathematics to the computer science I spend my days thinking about. In "
            a { href: "/blogs/as-engineers-llms-should-pay-us-for-tokens-usage",
                "As Engineers, LLMs should pay us for tokens usage"
            }
            ", I argued that the value of an engineered system comes from the human intelligence that was compressed into it, not just from the raw computational power that runs it. The Egyptian state turned that principle into an operational reality thousands of years ago. The intelligence of every scribe, every architect, every physician, and every administrator who ever worked in the Egyptian system was encoded into the papyri, the stone inscriptions, the architectural conventions, and the administrative procedures that made the state function. Individual humans died, but the encoded intelligence persisted, updated, and extended itself through the training of new scribes and officials who learned from the existing corpus. That is a kind of distributed machine learning, very slow and very biological, but functionally analogous to what we do when we train a model on a corpus of prior knowledge and then use it to make decisions in new situations. The Egyptians built this system over centuries, and it ran for millennia, and nothing we have built in the digital age has come close to matching that longevity."
        }
        h2 { id: "pyramids-as-world-models-running-at-civilizational-scale",
            a {
                href: "#pyramids-as-world-models-running-at-civilizational-scale",
                class: "header",
                "Pyramids As World Models Running At Civilizational Scale"
            }
        }
        p {
            "I said in "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by Default"
            }
            " that simulation is the ultimate test of understanding, and that if you truly understand a physical system, you can simulate it forward in time and predict what will happen. The pyramids are the most dramatic possible implementation of that principle in human history. A pyramid is not just a large building. It is a simulation of the Egyptian state's understanding of physics, geometry, logistics, and cosmology, made concrete in stone, tested over years of construction, and verified by the mere fact that it still stands. Every time a team of Egyptian engineers laid a course of stones, they were testing their mathematical models against the physical world, measuring the results, and adjusting. The Great Pyramid of Giza, rising 146 meters with the four base sides aligned to the cardinal directions to within a fraction of a degree, and with the four base lengths agreeing with each other to within twenty centimeters over a total perimeter of nearly a kilometer, is not a lucky accident. It is the output of a system that knew what it was doing, a system that could specify a complex geometric reality in mathematical terms and then physically realize that specification at enormous scale with remarkable accuracy."
        }
        p {
            "The engineering achievement of pyramid construction becomes even more impressive when you consider the organizational system required to make it possible. John Ashton and David Down's work on Egyptian archaeology discusses the pyramid complexes as involving not just the pyramid itself but an entire infrastructure of quarries, causeways, workers' villages, administrative facilities, and supply chains, all of which had to be planned, built, and managed simultaneously with the main construction. The village at Giza that housed the pyramid workers, excavated in the late 20th century by archaeologists, shows a sophisticated settlement with organized bakeries, breweries, medical facilities, and administration offices, a world unto itself created to support one building project. That village represents the hidden intelligence behind the visible monument: the logistical model, the supply chain planning, the labor allocation algorithms, the food distribution system, all of which had to be right for the pyramid to be possible. The researchers from Harvard's Giza Archive project have documented the extraordinary density of administrative records from the pyramid era, showing sealings, production records, and ration lists that testify to a bureaucratic system of genuine sophistication. This was not organized chaos. It was organized organization."
        }
        p {
            "The 2024 Nature paper on the Egyptian pyramid chain adds another dimension to this argument. The paper proposes that the chain of pyramids along the Egyptian plateau was deliberately built to run along a former branch of the Nile River, which has since dried up and disappeared. If this hypothesis is correct, it means that the Egyptians were not just building individual pyramids at convenient locations. They were planning a continuous program of construction along a linear geographic feature, coordinating the placement of massive monuments across decades and dynasties, and doing so in a way that reflected their understanding of the landscape's hydraulic structure. That is long-range strategic planning, the integration of geographic intelligence with architectural design decisions across multiple generations of leadership. Once again, we are looking at a form of intelligence that no individual mind could contain but that the system as a whole possessed."
        }
        p {
            "The precision of pyramid construction also reflects something I care about deeply: the idea that real intelligence must be verifiable. In "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by Default"
            }
            ", I argued that the difference between a language model and an equation-based model is precisely that the equation can be tested against reality, and either matches or does not. The pyramid is the most spectacular physical test of a mathematical model that ancient civilization ever produced. The model said the stones should be placed here, at this angle, with this orientiation. Reality was then measured against the model, and the match, which is spectacularly good, is the proof that the model was correct. Modern laser surveying of the pyramids confirms alignments and dimensions that are so precise that they require not just good engineering but a systematic process of planning, measuring, correcting, and re-measuring throughout the construction. That feedback loop, the loop from specification to measurement to correction and back to specification, is the same loop I described as the scientific method, and the Egyptians were running it in stone."
        }
        p {
            "I also want to note something that connects to my observation about AI's failure to represent the pharaohs accurately. The AI/Pharaoh paper by Mohamed Awad Allah documents the systematic errors that AI image generators make when asked to produce images of ancient Egyptian life, including anachronistic costumes, wrong architecture, incorrect hierarchies of scale, and misrepresentation of ethnic features. These errors are exactly the kind of hallucination I described in my previous posts: outputs that look plausible but are grounded in statistical pattern rather than in verified reality. Current AI systems cannot accurately model the pharaohs because they do not have a causal model of Egyptian civilization, only statistical associations between Egyptian-sounding words and broadly aesthetic visual patterns. The pharaohs themselves, by contrast, had a rigorous model of how stone should be placed, how structures should be oriented, how workers should be fed, how records should be kept, and how the divine order should be maintained, and that model was precise enough to produce results that modern AI cannot accurately replicate four thousand years later. There is a lesson in that irony, and it is the same lesson I keep learning from every direction: grounded, testable models beat statistical imitation every time."
        }
        p {
            "The cosmological dimension of the pyramids is also worth taking seriously, even if it makes some readers uncomfortable. The Egyptians aligned their pyramids with extraordinary precision to the cardinal directions, to specific stars, and potentially to broader cosmographic principles that we still do not fully understand. Whatever we think of the religious beliefs that motivated this alignment, the technical achievement is undeniable, and it reflects a culture that was systematically building models of the universe and then running those models in physical form. The pyramid was not just a tomb. It was a cosmic machine, a device built according to a model of the relationship between the earthly and divine worlds, designed to function as a literal bridge between those worlds through its precise geometry and orientation. I am not a religious believer myself, and I am not endorsing the theological claims. But I am observing that the cognitive project of building a physical model of a cosmological system, and then actually constructing that model at enormous scale, is the same cognitive project that underlies the development of every great scientific and engineering achievement in human history. The Egyptians were model builders, and the pyramids are the most durable models anyone has ever built."
        }
        p {
            "Let me also make an observation about what the pyramid tells us about technical transfer and knowledge accumulation. The pyramids did not appear fully formed. They evolved over generations, from the step pyramid of Djoser to the bent pyramid of Sneferu, which shows a mid-construction change of slope that tells us the engineers recognized a problem and corrected it in real time, to the perfected geometry of the Great Pyramid under Khufu. That evolution is a learning curve, documenting in stone the process by which a civilization accumulated engineering knowledge through iteration, experimentation, and refinement. Each generation of builders learned from the previous generation, inherited their mathematical techniques and their practical knowledge, corrected their mistakes, and produced a more refined result. That is exactly how machine learning works: each training run learns from the previous errors and improves toward a target. The Egyptians ran this learning process in biological hardware over generations, but the structure of the process is identical. They were doing gradient descent in stone, and the local minimum they converged to was one of the most impressive structures in human history."
        }
        h2 { id: "intelligence-encoded-in-wisdom-literature-and-text",
            a {
                href: "#intelligence-encoded-in-wisdom-literature-and-text",
                class: "header",
                "Intelligence Encoded In Wisdom Literature And Text"
            }
        }
        p {
            "One of the things that strikes me most forcefully about ancient Egyptian civilization is the sheer quantity and quality of their written intellectual tradition, and the fact that it spans every domain that we would now associate with genuine intelligence. When we talk about AI being intelligent, we usually mean it can answer questions, solve problems, give advice, reason about novel situations, and learn from experience. The Egyptian wisdom literature, as documented in texts like the Instructions of Ptahhotep, the Instructions for Merikare, the Maxims of Ani, and the Instructions of Amenemope, does all of these things in a form that was explicitly designed to be transmitted to future administrators, officials, and leaders as a practical guide for intelligent action in the world. These are not religious texts in the primary sense. They are operational manuals for running a complex civilization, encoded in poetic form for ease of memorization and transmission, and they reflect a depth of psychological and social insight that is genuinely impressive by any standard."
        }
        p {
            "The Instructions of Ptahhotep, which dates to around 2400 BCE in the Old Kingdom and claims to be the advice of a senior official near the end of his life, opens with a meditation on the decline of the body with age and then proceeds to give detailed guidance on how to behave in virtually every social situation a functional official might encounter. The text advises on how to speak truthfully without being arrogant, how to listen to subordinates without losing authority, how to navigate relationships with superiors without compromising integrity, how to manage household affairs, and how to recognize the difference between a person of genuine intelligence and a person who merely sounds intelligent. The MFTH paper quotes a passage that says the wise man is famed for what he has learned, it is the official who is after good conduct, from the action of his heart and his tongue, which encodes a specific theory of the relationship between intelligence, knowledge, and moral character that anticipates many of the most sophisticated discussions in modern philosophy of mind and ethics. Ptahhotep also says to follow your heart as things happen, which sounds simple but encodes the same insight that modern neuroscientists and cognitive scientists have arrived at about the role of embodied, affective processing in good decision-making."
        }
        p {
            "The Instructions for Merikare, written as advice from a king to his heir, is remarkable for its explicitly political and strategic content. The text advises the future king on how to manage officials, how to distinguish loyal advisors from self-serving ones, how to use both force and persuasion in governance, and how to think about the long-term consequences of policy decisions. One passage says a king should not kill a man whose excellence he knows, because that man walks with him and is a pillar of supporting him. That is a specific observation about the value of institutional knowledge, about the fact that intelligence in a system resides partly in individuals and that losing those individuals has organizational costs that go beyond the individual. This is the same insight that underlies modern organizational theory's emphasis on knowledge retention, on communities of practice, and on the danger of losing institutional memory through staff turnover. The Egyptians expressed it four thousand years ago as political advice to a king."
        }
        p {
            "The school traditions of ancient Egypt are also worth examining carefully, because they reveal a deliberate and sophisticated system for transmitting intellectual knowledge across generations. Egyptian schools for scribes, called Houses of Life, were attached to temples and served as centers not just of writing instruction but of the entire corpus of Egyptian knowledge, including mathematics, medicine, astronomy, theology, law, and administrative practice. Students learned to write by copying classical texts, which means they simultaneously acquired fluency in the writing system and absorbed the intellectual content of the tradition they were being initiated into. This is not a crude mechanical process. It is a thoughtful pedagogical design that uses the transmission of specific knowledge as the medium for building general cognitive skills, the same principle behind the liberal arts tradition in Western education, where studying classical texts was supposed to build general reasoning ability rather than just domain-specific knowledge. The Egyptians understood that cultivating intelligence requires immersing learners in the best examples of intelligent thought that the culture has produced, and they built a system for doing that which ran for centuries."
        }
        p {
            "The medical texts deserve their own discussion in this context. I mentioned the Edwin Smith Papyrus earlier, but I want to go deeper into what the Egyptian medical tradition shows about their cognitive approach to complex problems. The Egyptians distinguished between different types of bodily conditions with a specificity that was not matched in Western medicine for another three thousand years. They had terms for different types of wounds, different stages of infection, and different presentations of neurological injury. The Edwin Smith Papyrus describes a case of cranial injury where the patient has speech difficulties and involuntary movements, and although the Egyptian physicians did not understand the neurological mechanisms the way we do today, they correctly identified the brain as the organ responsible for the observed symptoms, at a time when this was not obvious. This is medical reasoning, not just medical tradition. It is the integration of observation, pattern recognition, and causal inference in service of practical diagnosis and treatment. The fact that the physicians concluded the case was untreatable and honestly said so, rather than prescribing useless remedies, reflects an epistemic honesty that we should admire regardless of the historical context."
        }
        p {
            "I also want to note that the Egyptian approach to knowledge was fundamentally empirical rather than purely dogmatic. The Egyptians did revise their practices when evidence demanded it, they did update their medical protocols based on accumulated clinical experience, they did modify their architectural techniques when engineering problems revealed flaws in previous approaches. The bent pyramid, as I mentioned, is a literal monument to engineering error-correction. The scribal tradition included commentary and annotation, suggesting that texts were not treated as fixed and untouchable but as living documents that could be extended by new knowledge. This is not the picture of a civilization paralyzed by rigid tradition. It is the picture of a civilization that had a stable cultural framework within which empirical learning and practical revision could take place, which is exactly the kind of structure that makes sustained intelligence possible. A system that can never update itself is not intelligent. A system that updates itself randomly is not stable. The Egyptian state achieved the difficult balance between structural stability and adaptive learning, and maintained that balance for three thousand years. That alone is a remarkable achievement, and it is not one that we should dismiss just because it happened in the ancient world."
        }
        h2 { id: "administration-as-the-neural-network-of-civilization",
            a {
                href: "#administration-as-the-neural-network-of-civilization",
                class: "header",
                "Administration As The Neural Network Of Civilization"
            }
        }
        p {
            "If the pyramids are the most visible expression of Egyptian intelligence, the administrative system is the most functionally important one, because without the administration the pyramids could not exist. I want to devote a full section to the administrative machinery of the Egyptian state because it is the closest analogue in the ancient world to what we would now call an artificial intelligence system: a distributed network of information-processing nodes, operating according to learned rules, communicating through a standardized protocol, coordinating vast flows of resources, and producing outputs that no individual node could have generated on its own. The Egyptian bureaucracy was that system, and studying it carefully reveals something important about what intelligence actually requires at scale."
        }
        p {
            "The bureaucratic structure of Egypt was hierarchical but flexible. At the top was the pharaoh, who was both the symbol of cosmic order and the apex of the administrative hierarchy. Beneath the pharaoh was the vizier, who functioned as something like a prime minister and chief administrator, overseeing all the departments and ensuring the coordination of the state's functions. Beneath the vizier were the heads of the various departments: the treasury, the military, the granaries, the construction projects, the religious establishments, and the local governorates called nomes. Each of these departments had its own internal hierarchy of scribes and officials, all trained in the same scribal tradition and all using the same administrative language and record-keeping conventions. This standardization was enormously important, because it meant that information could flow up and down the hierarchy without ambiguity, that records from different parts of the kingdom could be compared and aggregated, and that officials who moved between departments or regions could function immediately without needing to learn a different system. This is exactly what we design in distributed computing systems when we specify shared protocols and interfaces: the standardization that enables different parts of the system to communicate and cooperate without central control of every interaction."
        }
        p {
            "The record-keeping system of the Egyptian state was sophisticated enough to support planning and logistics at a scale that challenges modern intuition. The papyrus records from the Old Kingdom period include ration lists, production tallies, material inventories, delivery manifests, and administrative correspondence that show a state tracking the movement of enormous quantities of grain, stone, copper, wood, and labor with considerable precision. The fact that we can read these records today and understand them without ambiguity is itself evidence of their quality: they were designed to be clear, consistent, and unambiguous across large distances and long time periods, and they succeeded. The Cambridge archaeologist who studied the labor organization of Old Kingdom state projects described the administrative evidence as suggesting a centralized system capable of collecting resources, directing work, and maintaining records with remarkable consistency. That description applies perfectly to what we would now call a distributed information processing system, and the consistency the scholar notes is exactly the property that makes such systems useful: without consistency, aggregation and coordination break down."
        }
        p {
            "The nome system, which divided Egypt into administrative districts each with its own governor, its own records, and its own relationship with the central administration, is also worth examining from this perspective. The nome system solves one of the classic challenges of any large intelligent system: how to balance central coordination with local adaptation. If every decision has to go to the center, the system becomes a bottleneck that cannot respond quickly enough to local conditions. If every node makes all its own decisions, the system fragments and loses coherence. The nome system found a middle path: local governors had authority over local administration, local agriculture, and local resource management, but operated within a framework of obligations to the central state, a common administrative language, and shared standards for reporting and record-keeping. This is exactly the architecture of modern distributed systems, where local nodes have autonomy within a defined interface, and the interface ensures coherence at the system level even while individual nodes adapt to their local conditions. The Egyptians were running what we would now call a federated system, and they were running it for millennia."
        }
        p {
            "The religious system was also part of the intelligence architecture, and I do not mean that dismissively. The temples were not just places of worship. They were simultaneously granaries, workshops, schools, hospitals, administrative centers, and repositories of the state's accumulated knowledge. The temples owned land, managed agricultural production, employed craftsmen, trained scribes, and cared for the sick. The temples' economic resources, which in some periods accounted for a significant fraction of Egypt's total wealth, were managed by temple administrators using the same record-keeping conventions as the civil administration. This means the temple system was an integrated part of the state's information processing and resource management network, not a parallel structure but a component of the same distributed system. The religious meaning assigned to all of this activity, the idea that managing the temple's grain stores was a sacred act of maintaining cosmic order, was not just mysticism. It was a motivational framework that ensured the people running the system did so with the same level of care and attention to detail that would be required regardless of whether the motivation was religious or administrative. The cognitive and organizational functions were real, even if the theological beliefs that supported them were based on a different understanding of the world than what we have today."
        }
        p {
            "I want to draw an explicit parallel to the AI concepts I have discussed in previous posts. In "
            a { href: "/blogs/llms-are-usefull-lmms-will-break-reality",
                "LLMs are Useful. LMMs will Break Reality"
            }
            ", I talked about world models as the next frontier of AI, systems that can learn a model of an environment and use that model to plan and predict without direct interaction with the real world. The Egyptian administrative system was a world model in the most concrete possible sense. At any given time, the state maintained records of land ownership, agricultural yields, labor availability, resource stockpiles, population, debt, and production across the entire kingdom. From those records, the vizier and the pharaoh could form a working model of the state's current condition, predict future needs based on the agricultural calendar, and allocate resources accordingly. When the Nile flood was higher or lower than normal, the model was updated with the new data, and the distribution of grain reserves was adjusted to compensate. This is resource planning based on a maintained world model, the same thing modern supply chain management systems try to do with computers, and the Egyptians were doing it with papyrus, scribes, and a hierarchy of trained officials."
        }
        p {
            "The longevity of the administrative system is its most striking property. Egyptian administrative structures, with variations and interruptions, remained functional for roughly three thousand years, across thirty dynasties, through invasions, internal conflicts, religious revolutions, and technological change. Individual pharaohs rose and fell, dynasties succeeded each other, the official religion changed dramatically under Akhenaten and then changed back, but the core administrative machinery persisted. That persistence reflects a design robustness that is rare in any system. Modern corporations rarely last more than a century. Modern governments rarely maintain consistent administrative structures for more than a few decades. The Egyptian state maintained its core administrative logic for longer than the entire span of recorded European history, and it did so because the knowledge that animated the system was encoded in a substrate, the scribal tradition, that was more durable than any of the individuals who participated in it. That is the deepest sense in which Egyptian civilization was a form of superintelligence: it outlasted the lifetimes of every human being who ever contributed to it, by orders of magnitude."
        }
        h2 { id: "how-modern-ai-gets-the-pharaohs-wrong-and-why-it-matters",
            a {
                href: "#how-modern-ai-gets-the-pharaohs-wrong-and-why-it-matters",
                class: "header",
                "How Modern AI Gets The Pharaohs Wrong And Why It Matters"
            }
        }
        p {
            "I want to take a detour through a specific piece of evidence that connects ancient Egypt directly to my broader argument about AI. The paper by Mohamed Awad Allah that I mentioned earlier, published in the International Design Journal in 2024, documents the systematic errors that AI image generators make when producing historical images of ancient Egyptian figures and scenes. The errors are not random. They reflect specific patterns of misrepresentation: the clothing is wrong, the architectural context is wrong, the proportions are wrong, the skin tones are wrong, the accessories are wrong, and the hierarchical relationships between figures are wrong. These are not trivial mistakes. They are fundamental misrepresentations of an entire civilization's visual culture, produced by systems that have consumed enormous quantities of text and image data, including presumably a great deal of data about ancient Egypt, and still cannot produce accurate representations of what it actually looked like. That failure is instructive, and it connects directly to everything I have been saying about the difference between statistical learning and genuine understanding."
        }
        p {
            "The reason AI image generators make these errors is exactly the reason I have been criticizing language models throughout these posts. They learn statistical associations between Egyptian-sounding descriptions and broadly aesthetic visual patterns, but they do not have a causal model of Egyptian civilization, of how the artisan tradition worked, of what conventions governed the representation of kings versus priests versus commoners, of how the period-specific dress codes and architectural styles evolved over three thousand years of history. They have correlations, not causes. They have surface patterns, not mechanisms. And so when they try to generate an image of a pharaoh, they produce something that looks vaguely Egyptian to a casual viewer but that would be immediately recognizable as wrong to any serious student of Egyptian art and culture. The Egypt they generate is a statistical hallucination, assembled from fragments of text and image data that have no structural connection to the actual historical reality. This is exactly the failure mode I described in "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by Default"
            }
            ": a system that produces outputs that look right rather than outputs that are right, because it learned correlations instead of the generative mechanism."
        }
        p {
            "What would it mean for an AI system to actually understand ancient Egyptian civilization? I think it would mean the system has a causal model of how the civilization worked: how the administrative hierarchy was organized, how the scribal tradition transmitted knowledge, how the agricultural calendar shaped resource management, how the royal ideology coordinated religious and political authority, and how all of these systems interacted with each other and with the natural environment of the Nile Valley. With such a model, the system could generate not just plausible-looking images but verifiably accurate reconstructions of every aspect of Egyptian life, from the specific cut of a craftsman's kilt to the precise protocol of a temple ritual to the correct format of an administrative ration list. It could answer questions about the civilization by computing from the model rather than by retrieving statistical associations, which means its answers would be reliable rather than probabilistic. It could simulate the civilization's response to novel challenges, like an unusually high Nile flood or a particularly bad grain harvest, by running the causal model forward and seeing what the administrative system would have done. That is what genuine AI understanding of history would look like, and no current system comes close to it, because no current system has been built on the right foundation."
        }
        p {
            "The failure of AI to accurately represent the pharaohs is also a failure of the humans who designed the AI systems, and specifically a failure to take seriously the depth and complexity of non-Western civilizations. AI systems trained primarily on Western intellectual content will naturally be better at representing Western history and culture than non-Western history and culture. The statistical associations in the training data reflect the biases and priorities of the people who produced that data, and the people who produced the most AI training data were predominantly Western and educated in Western intellectual traditions. Ancient Egypt, despite its enormous historical importance and its profound influence on subsequent civilizations, is still underrepresented in the training data relative to its actual significance, and that underrepresentation shows up directly in the AI's inability to accurately model it. This is the same structural inequality I described in "
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
                "Technology Has Destroyed My Livelihood"
            }
            ": the systems that claim to be universal are actually calibrated to the experience and knowledge of people who already have power, and the people whose experience differs from that baseline are rendered inaccurately or not at all."
        }
        p {
            "The study of Egyptian civilization also challenges the typical AI narrative in a different way. Most discussions of artificial intelligence assume a linear progression from less intelligent to more intelligent, with modern digital systems at the top of the ladder and all previous forms of intelligence arranged below them in order of increasing primitiveness. The Egyptian evidence disrupts this narrative, because it shows a civilization that was genuinely sophisticated in its intelligence, not primitive, not merely intuitive, but systematically intelligent in ways that modern AI systems cannot fully replicate or model. The Egyptian mathematics was not less sophisticated than modern mathematics in the dimensions that mattered for their purposes. The Egyptian administrative system was not less capable than any modern bureaucracy in the dimensions that mattered for its purposes. The Egyptian medical reasoning was not less rigorous than modern clinical reasoning in the dimensions that mattered for its purposes. They were different, calibrated to different technologies and different cultural frameworks, but difference is not the same as inferiority, and the assumption that modern means superior is exactly the kind of shallow thinking that I have been arguing against throughout these posts."
        }
        p {
            "I also want to make a point about what the Egyptian evidence implies for our definition of intelligence. We typically measure AI intelligence by performance on standardized benchmarks: reading comprehension, mathematical reasoning, coding, question answering. These benchmarks are calibrated to the kinds of intelligence that modern Western educational systems value and that modern digital computers are designed to perform. They do not measure the kind of intelligence required to maintain a civilization for three millennia, to build structures that last for four thousand years, to develop a writing system and a mathematical tradition that remain partially functional and partially readable today. If we designed benchmarks for those capabilities, modern AI systems would score very poorly indeed. That does not mean modern AI is unintelligent. It means that intelligence is multidimensional, and that we have spent a century trying to build one kind of intelligence while ignoring all the others, including the kind that sustained Egyptian civilization long enough to leave us evidence we are still debating today."
        }
        p {
            "The AI image generation failures also remind me of something else I have been thinking about: the difference between generating plausible representations and understanding what you are representing. An AI that generates a plausible-looking but historically wrong image of a pharaoh is not understanding Egypt. It is performing Egypt, in the same way that a language model performs understanding by producing fluent text without necessarily knowing what it is saying. Performance and understanding are different things, and the difference matters enormously when the stakes are high. If you are building a system to help archaeologists study ancient Egypt, you want understanding, not performance. If you are building a system to help engineers design structures, you want understanding, not performance. If you are building a system to help doctors make diagnoses, you want understanding, not performance. And understanding, as I have argued repeatedly, requires grounded models of how things actually work, not just statistical patterns in what people have said about them. The pharaohs, for all their ancient limitations, built a civilization grounded in the reality of the Nile, the stars, the soil, and the stone, and that grounding is exactly what made their intelligence last."
        }
        h2 { id: "what-the-pharaohs-tell-us-about-real-intelligence",
            a {
                href: "#what-the-pharaohs-tell-us-about-real-intelligence",
                class: "header",
                "What The Pharaohs Tell Us About Real Intelligence"
            }
        }
        p {
            "Everything I have written so far points toward a conclusion that I find both humbling and clarifying. The pharaohs built a form of intelligence that we do not yet know how to replicate with our most advanced technology, and they did it without any of the tools we assume are necessary for intelligence: electricity, computing, global communication, or digital storage. They did it with symbols, mathematics, organized labor, and an ideological framework that kept millions of people coordinated and motivated across vast distances and long timespans. That is a fact, not a claim, and it challenges some of the deepest assumptions embedded in how we talk about artificial intelligence today."
        }
        p {
            "The first assumption it challenges is that intelligence requires speed. Modern AI systems are celebrated for their speed, for their ability to process information millions of times faster than any human being. But the Egyptian state was not fast by any modern standard. Documents traveled by foot or boat, decisions took days or weeks to propagate through the hierarchy, and building projects unfolded over years and decades. And yet the system was intelligent enough to achieve results that have lasted four thousand years and that modern AI cannot accurately reproduce. Speed is one dimension of intelligence, and it matters for some problems, but the Egyptian evidence shows that it is not the fundamental dimension. The fundamental dimension is whether the system has a grounded, accurate model of the reality it is trying to engage with, and the Egyptians did. Their model of the Nile, of agricultural cycles, of pyramid geometry, and of administrative coordination was grounded, accurate, and empirically verified, and that grounding is what made it work regardless of speed."
        }
        p {
            "The second assumption it challenges is that intelligence requires vast amounts of data. Modern AI systems are trained on billions of text documents, millions of images, hours of video, and enormous quantities of sensor data. The Egyptian intellectual tradition was built on a corpus that, while substantial by ancient standards, was tiny compared to any modern training dataset. And yet the knowledge encoded in that corpus was sufficient to support a civilization of millions of people across thousands of miles for three thousand years. This suggests that the quantity of data matters far less than the quality of the models derived from it. A small number of highly accurate, grounded mathematical models, like the seked system or the formulas in the Rhind Papyrus, is worth more than a vast quantity of loosely accurate statistical associations, because the precise models produce reliable results under conditions that were not explicitly anticipated, while the statistical associations break down as soon as they are applied outside their training distribution. This is the point I have been making about physics-informed models and symbolic regression: targeted mathematical knowledge generalizes better than bulk statistical learning, and the Egyptian evidence from four thousand years ago supports exactly that point."
        }
        p {
            "The third assumption it challenges is that intelligence requires a single unified substrate. Modern AI systems are designed to run on unified hardware, with all the computation happening inside a single system of chips and memory. The Egyptian state was massively distributed, with its intelligence spread across millions of humans, billions of papyrus documents, thousands of stone inscriptions, and hundreds of buildings, none of which was connected by anything faster than a human messenger. And yet the system behaved coherently across all of these distributed components, producing outputs that were consistent, coordinated, and reliably effective. The coordination was achieved through shared protocols (the administrative language and record-keeping conventions), shared models (the mathematical and astronomical knowledge transmitted through the scribal tradition), shared motivational frameworks (the religious ideology that gave everyone in the system a reason to do their job well), and hierarchical organization that ensured information could flow to where it was needed for decision-making. These are exactly the design principles of modern distributed computing systems, and the Egyptians applied them without computers, without electricity, and without any of the tools we think of as prerequisites for distributed intelligence."
        }
        p {
            "I also want to connect this to something I said in "
            a { href: "/blogs/language-is-limited-asi-is-impossible",
                "Language is Limited. ASI is Impossible."
            }
            ": that the future belongs to tools that stay honest about what they are, rather than to systems that claim to be more than they are. The pharaohs were perhaps the most grandiose self-promoters in human history, claiming divine status and cosmic authority in their inscriptions and monuments, and by any modern standard those claims were false. But the actual tools they built, the writing system, the mathematical corpus, the administrative machinery, the architectural tradition, these were genuinely what they were, genuinely capable of doing what they needed to do, genuinely grounded in reality in ways that produced verifiable results. The gap between the divine claims and the actual tools is a lesson I think about a lot when I read modern AI marketing, where the divine claims are phrases like reasoning, understanding, and human-level intelligence, and the actual tools are very sophisticated statistical completion engines. The Egyptians were more honest in their tools than in their ideology. The modern AI industry is the reverse."
        }
        p {
            "The longevity argument is the one I keep coming back to, because I think it is the hardest to dismiss and the most important for what it implies. The Egyptian state, in some form, lasted from approximately 3100 BCE to 30 BCE, over three thousand years. During that period, it absorbed invasions by the Hyksos, the Assyrians, the Persians, and the Macedonians, and it survived and continued to function after each of them. It absorbed a radical religious revolution under Akhenaten, where the traditional polytheism was replaced by a monotheistic sun cult, and after Akhenaten died, it reversed that revolution and resumed functioning as before. It survived long periods of weak central government, internal conflict, and economic stress, and each time it recovered and rebuilt. That is not the behavior of a fragile system. It is the behavior of a deeply robust system with multiple redundant components and a strong capacity for self-healing. No modern AI system has been tested anywhere near that standard of robustness, and no modern institution of any kind comes close to that timescale of sustained intelligent function. That we call the modern AI systems intelligent while treating the Egyptian state as primitive is not a reflection of honest evaluation. It is a reflection of presentism, of the assumption that newer is better and that whatever exists now is the apex of what is possible."
        }
        p {
            "I want to close this section with a thought about humility. In "
            a { href: "/blogs/who-am-i", "who I am" }
            ", I described myself as someone who has struggled all his life with the feeling that the world does not recognize what he has built, that the systems he creates are invisible while noisier or more commercially appealing things get all the attention. I feel the same on behalf of the ancient Egyptians when I see how AI systems misrepresent them, how popular culture reduces them to mystical pyramid-builders with no serious intellectual tradition, and how the AI conversation ignores everything they achieved while celebrating statistical pattern matchers that cannot accurately represent the history of the civilization whose mathematical and administrative achievements they are implicitly built on. The Egyptians deserve better from our current supposedly advanced civilization, and the way to give them better is to study them honestly, to acknowledge what they actually achieved, and to use that acknowledgment to recalibrate our understanding of what intelligence really is and what it really requires."
        }
        h2 { id: "closing-thought",
            a { href: "#closing-thought", class: "header", "Closing Thought" }
        }
        p {
            "I am going to be honest about what I am claiming and what I am not claiming, because I always try to be honest in these posts, and this post more than most requires that honesty. I am not claiming the pharaohs were superhuman, or that they had secret technology, or that their intelligence was supernatural. I am claiming something more modest and in some ways more radical: that they built a system for organizing and applying human intelligence at civilizational scale, and that this system was, by any reasonable functional definition, a form of superintelligence, in that it produced outcomes that no individual human mind could have produced, that it modeled and managed a reality too complex for any individual to comprehend, and that it ran for longer than any modern institution has come close to running. I am claiming that this system was built from components we can study and understand: mathematics, symbolic representation, administrative coordination, structural engineering, wisdom literature, and shared ideology. And I am claiming that studying those components carefully can teach us something important about what real intelligence requires, things we are currently ignoring in our rush to build bigger language models."
        }
        p {
            "The argument also connects to the "
            a { href: "https://github.com/wiseaidotdev/lmm", "lmm project" }
            " I have been building, which I described in my previous post as a proof of concept for equation-based intelligence. The pharaonic model of intelligence was not linguistic. It was structural, mathematical, and symbolic in the deepest sense, grounded in equations for pyramid geometry, in algorithms for administrative resource allocation, in formal procedures for medical reasoning, and in a writing system capable of encoding the full range of human knowledge in a transmissible and durable form. That is the model of intelligence I am trying to build: not a system that talks about the world but a system that models the world, discovers its structure, and uses that structure to generate verifiable predictions and reliable actions. The pharaohs showed that this kind of intelligence is possible, that it can scale to civilizational dimensions, and that it can last. I find that more inspiring than any modern AI benchmark, because it is real, it is documented, and it happened without any of the things we tell ourselves are prerequisites for intelligence."
        }
        p {
            "I do not know what the pharaohs would have thought of our current conversation about AI. Probably something dismissive, the way someone who has built an actual bridge might look at someone who can describe a bridge very fluently but cannot actually build one. The distinction between describing and doing is one they understood very well, because their entire civilization was built on the premise that intelligence without grounded reality is just speech, and speech without grounded reality is just air. The Maxim of Ptahhotep says the heart is the creator of its master, and I have been turning that phrase over in my mind for months since I read it. I think it means that genuine intelligence, the kind that creates rather than just imitates, comes from the integrated core of a system, from the mechanism that generates outcomes rather than from the surface that displays them. The pharaohs built a civilization with that kind of heart. We are still trying to figure out how."
        }
        p { "Till next time 👋!" }
        h2 { id: "references",
            a { href: "#references", class: "header", "References" }
        }
        ul {
            li {
                "Awad Allah, M. (2024). Pharaoh by artificial intelligence: Historical mistakes with a beautiful artistic vision. "
                em { "International Design Journal" }
                ", 14(5), 589- 598."
            }
            li {
                "Faculty of Tourism and Hotels, University of Sadat City. (2024). Intelligence in ancient Egyptian texts. "
                em { "Journal of the Faculty of Tourism and Hotels" }
                ", 8(2/3), 18 - 36."
            }
            li {
                "Ashton, J. & Down, D. (2006). "
                em { "Unwrapping the Pharaohs: How Egyptian Archaeology Confirms the Biblical Timeline" }
                ". Master Books."
            }
            li {
                "British Museum. (2022). "
                a { href: "https://www.britishmuseum.org/exhibitions/hieroglyphs-unlocking-ancient-egypt",
                    "Hieroglyphs: Unlocking Ancient Egypt"
                }
                "."
            }
            li {
                "Britannica. (2024). "
                a { href: "https://www.britannica.com/science/mathematics/Mathematics-in-ancient-Egypt",
                    "Mathematics in Ancient Egypt"
                }
                "."
            }
            li {
                "Hawass, Z. & Stadelmann, R. (2003). "
                a { href: "https://gizamedia.rc.fas.harvard.edu/images/MFA-images/Giza/GizaImage/full/library/hawass_fs_stadelmann.pdf",
                    "Pyramid Construction: New Evidence Discovered at Giza"
                }
                ". Harvard Giza Media."
            }
            li {
                "Tristant, Y. & Ghilardi, M. (2024). "
                a { href: "https://www.nature.com/articles/s43247-024-01379-7",
                    "The Egyptian pyramid chain was built along the now-abandoned Ahramat branch of the Nile"
                }
                ". "
                em { "Communications Earth & Environment / Nature" }
                "."
            }
            li {
                "Cambridge University Press. (2024). "
                a { href: "https://www.cambridge.org/core/books/archaeology-of-pharaonic-egypt/organising-people/303467439B2E77766B86CE103C867254",
                    "Organising People: The Archaeology of Pharaonic Egypt"
                }
                "."
            }
            li {
                "Britannica. (2024). "
                a { href: "https://www.britannica.com/science/calendar/The-Egyptian-calendar",
                    "The Egyptian Calendar"
                }
                "."
            }
            li {
                "Lichtheim, M. (1997). "
                em { "Moral Values in Ancient Egypt" }
                ". Orbis biblicus et orientalis."
            }
            li {
                "British Museum. "
                a { href: "https://www.britishmuseum.org/exhibitions/hieroglyphs-unlocking-ancient-egypt/egyptian-hieroglyphs-decipherment-timeline",
                    "How Egyptian hieroglyphs were decoded, a timeline"
                }
                "."
            }
        }
    }
}
#[component(no_case_check)]
pub fn LlmsDestroyedTheInternetLmmsWillMakeItAlive() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey everyone 👋," }
        p {
            "In my previous post, "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by default"
            }
            ", I made the argument that equations are not just tools for computation but the most compressed and most powerful representations of reality that humans have ever discovered, and that a single well-formed equation can generate outputs in text, images, sound, motion, and numerical prediction all at once, because it encodes the mechanism rather than any particular surface appearance. I meant every word of that, and I still do. But there is something I have been carrying with me across all of these posts that I have not quite said directly, something that connects my frustration with technology, my grief over what the internet has become, and my cautious hope about where AI is heading, and I need to say it now because the argument is finally complete enough to state. The argument is this: large language models, in the way they have been deployed at industrial scale over the last few years, have done something quiet and damaging to the internet. They did not break it with a single catastrophic event. They dissolved it. Slowly. Paragraph by paragraph, article by article, page by page. They took a living conversation between millions of real humans and replaced increasing chunks of it with synthetic noise that sounds like thought but contains very little of it. And the tragedy is not that the technology is evil, because it is not. The tragedy is that we picked up an extraordinary tool and mostly used it to drown out the authenticity that made the web worth visiting in the first place. I have watched this happen in real time, and it has made me angry and sad in equal measure, and I need to explain why, and then I need to explain why I think the next generation of models, the ones built on mathematics and perception and simulation rather than on statistical patterns in scraped text, might be the thing that brings the web back to life. Not the same life it had. Something better, if we are honest and careful and a little lucky."
        }
        p {
            "I want to be precise about what I mean, because imprecision is the enemy of honest argument. I am not saying that all AI-generated content is worthless. I have argued in "
            a { href: "/blogs/llms-are-usefull-lmms-will-break-reality",
                "LLMs are Useful. LMMs will Break Reality"
            }
            " that language models are genuinely useful tools for drafting, summarizing, translating, and exploring ideas, and I stand by that. I am not making an anti-AI argument, because anti-AI arguments are usually lazy, and I have never been interested in lazy arguments. What I am making is a more specific and more uncomfortable claim, which is that the mass deployment of language models as content generation engines, rather than as thinking tools, has changed the composition of the web in ways that most people feel intuitively but cannot yet name. The web used to be made mostly of human experience. Now it is increasingly made of human-experience-shaped language patterns. Those two things are not the same, and the difference matters more than almost anyone is willing to admit, because the value of the web was never in its words alone. It was in the fact that the words were connected to real people who had done real things and felt real emotions and made real mistakes and discovered real solutions, and that connection is exactly what statistical generation breaks without anyone noticing. I want to explain how this happened. I want to explain why it was almost inevitable given what language models actually are. And I want to explain why giving machines the ability to perceive, model, and simulate the world rather than just predict words about it is the only path I can see toward an internet that is alive again in the way that matters."
        }
        h2 { id: "the-internet-was-valuable-because-it-was-made-of-friction",
            a {
                href: "#the-internet-was-valuable-because-it-was-made-of-friction",
                class: "header",
                "The Internet Was Valuable Because It Was Made of Friction"
            }
        }
        p {
            "I grew up in a village with no internet, as I described in "
            a { href: "/blogs/who-am-i", "Just Don't Pick Up the Brush" }
            ", and the first time I had real access to the web in college, I was overwhelmed in a way that I still struggle to fully describe to anyone who grew up with it. It was not just the information that hit me. It was the texture. The internet of the early 2010s had a quality that I did not have a word for at the time but that I would now call friction, meaning that every piece of content showed the marks of the human being who made it. Blog posts had weird formatting choices and strange digressions and author-specific obsessions that had nothing to do with SEO or engagement metrics. Forum posts had arguments that went in twelve different directions before anyone answered the original question, and those arguments were often more useful than a clean answer would have been, because they exposed the real shape of the problem. Tutorial pages had typos and outdated screenshots and notes like \"this worked for me but your mileage may vary\", and those imperfections were not flaws. They were signals. They told you that a real person had really struggled with this problem and had really found something that worked, and that knowledge was more valuable than a polished document that had never been tested against reality. Friction is the trace of contact with the world, and the world leaves marks on everything it touches, and those marks are information. An internet full of human friction was an internet full of information about what was real, what worked, what hurt, and what mattered, and none of that could be faked at the time, because faking it at scale was computationally impossible. That impossibility was the foundation of the web's trustworthiness. It was unintentional, but it was real, and it held the whole thing together in a way that nobody appreciated until it started to disappear."
        }
        p {
            "What made discovery feel like discovery back then was exactly that friction. When you typed a strange, specific question into a search engine and found a forum thread from seven years ago where someone in a different country had the same strange, specific problem, and they had solved it in a way that nobody else had documented, you felt something that I can only describe as the pleasure of genuine contact. You had reached through the screen and touched someone else's experience, and their experience was useful to yours, and the connection was real even though neither of you were ever aware of the other's existence. That is a remarkable thing when you think about it carefully. The web made it possible for a person sitting alone in a village in Lebanon to benefit from the hard-won knowledge of a developer in South Korea who had wrestled with the same obscure bug three years earlier, and the knowledge transfer was free, and nobody organized it, and nobody optimized it for engagement, and it happened anyway, simply because human beings have an instinct to share what they have learned. That instinct is ancient and beautiful and it is the reason science works, the reason cultures pass down knowledge between generations, and the reason any civilization has ever accumulated more understanding than a single human lifetime contains. The early web was a machine for expressing that instinct at global scale, and it worked precisely because the instinct was real. The friction was the proof of the instinct. The imperfect, idiosyncratic, human-shaped content was the evidence that a real mind had really engaged with a real problem and had really tried to help whoever came after. Remove the real minds, and you keep the shape of helpfulness without the substance."
        }
        p {
            "The economics of the early web also deserve some credit, even though economics is not usually romantic. Most content was produced by people who were not being paid to produce it, or who were being paid very little. That sounds like a problem, but it was actually a discipline, because if someone was willing to spend hours writing a detailed walkthrough of a technical problem without being paid, it was almost certainly because they cared about the problem themselves. The motivation was intrinsic, and intrinsic motivation produces a different kind of content than extrinsic motivation does. When you write because you genuinely want to explain something, the explanation tends to be grounded in your actual understanding rather than in what you think will rank well or what will keep someone on the page long enough to see an advertisement. Intrinsic motivation is correlated with authenticity in a way that economic incentives are not, and the early web was swimming in intrinsic motivation, from the hobbyist who documented every piece of vintage hardware they owned, to the chemist who explained reactions because they found them beautiful, to the programmer who wrote post-mortems about their failures because they wanted to warn others. Those people were not performing expertise for clout. They were sharing genuine knowledge because sharing knowledge is one of the few things that makes being human feel worthwhile. And the readers could feel the difference, even if they could not articulate it, because authenticity leaves a texture that performance does not, and that texture is what made the web trustworthy."
        }
        p {
            "I want to be honest about something uncomfortable here, because honesty is the only currency I have that I actually trust. The early web was not paradise. It had plenty of garbage, plenty of misinformation, plenty of bad-faith actors, and plenty of content that would make most people wince today. The spam was real and the scams were real and the plagiarism was real and the loudest voices were not always the wisest ones. I am not arguing that we should go back to some fictional golden age, because that argument would be dishonest and nostalgia is usually dishonest about what it is actually mourning. What I am arguing is more specific and more structural. The web's value came from a particular relationship between content and human experience, namely that content was mostly produced by humans who had actually experienced the things they were writing about, or who had at least genuinely tried to understand them. That relationship was never perfect. But it was real enough to create a web where information was mostly made of processed human experience rather than processed previous text. The distinction matters because processed human experience contains something that processed previous text does not, namely a connection to what actually happened, what was real, what worked and did not work when tested against a world that does not care about your priors. Language models cannot produce that connection no matter how large they get, because they never touched the world. They only touched the text that humans wrote about the world, and the gap between those two things is the entire problem I am trying to describe."
        }
        p {
            "The other thing about friction that nobody talks about is that it acted as a natural filter for confidence. A real human writing about their experience tends to express uncertainty when they are uncertain, because they know from experience that confident answers that turn out to be wrong are embarrassing, and embarrassment is a real social cost that shapes behavior. A language model has no experience of embarrassment, no memory of being wrong in a way that had consequences, and no social skin in the game of being trusted. It produces confident outputs because confidence is a pattern in training data, not because confidence is epistemically warranted. That difference matters enormously when you are trying to evaluate whether a piece of information is reliable. Human writers signal their uncertainty through hedging, through referencing their own limitations, through recommending that you verify things independently. Language models signal confidence by default, because that is what the training objective rewards. The result is a web increasingly populated with content that sounds certain while being detached from any actual experience of the thing being described, and that is not a small problem. That is a systematic erosion of the epistemic signals that allowed people to calibrate how much to trust what they were reading. You cannot fix that with better prompting. You can only fix it by reconnecting the output to something real."
        }
        p {
            "The phrase I keep coming back to is that the old web had evidence of life. You could see the fingerprints of living people all over it, in the weird personal asides, in the specific examples drawn from specific situations, in the emotional undertone of someone who had suffered through a problem long enough to actually understand it. Those fingerprints were not always readable as fingerprints. Sometimes they just looked like idiosyncrasy or bad writing or excessive detail. But they were signals of genuine engagement, and genuine engagement is the thing that makes information worth having. An encyclopedia entry tells you what a concept is. A blog post from someone who spent three weeks fighting a bug tells you what it is like to deal with that concept in practice, and the difference between those two things is enormous when you are the one fighting the bug. The web used to be full of the second kind of content, and it is becoming increasingly full of the first, and the transition is quiet and gradual and almost impossible to reverse through market mechanisms alone, because the economics of generating plausible text have collapsed to near zero while the economics of generating genuine experience remain as expensive as they have always been."
        }
        h2 { id: "what-language-models-actually-are-and-why-that-explains-everything",
            a {
                href: "#what-language-models-actually-are-and-why-that-explains-everything",
                class: "header",
                "What Language Models Actually Are and Why That Explains Everything"
            }
        }
        p {
            "I have spent a lot of time in my previous posts arguing against oversimplified accounts of what language models can and cannot do, because oversimplified accounts, in either direction, produce bad decisions. I argued in "
            a { href: "/blogs/language-is-limited-asi-is-impossible",
                "Language is Limited. ASI is Impossible."
            }
            " that language models are not on a path to general intelligence, not because they are not impressive, but because the thing they are learning is fundamentally different from the thing we call understanding. I argued in "
            a { href: "/blogs/llms-are-usefull-lmms-will-break-reality",
                "LLMs are Useful. LMMs will Break Reality"
            }
            " that they are genuinely useful tools for specific tasks while being structurally limited by the fact that they learned from text about the world rather than from the world itself. I want to take those arguments one step further here, because the current state of the internet requires me to be more specific about the mechanism by which language models affect content quality, and more specific about why that mechanism is not a bug that will be patched but a property of the fundamental architecture that will persist as long as the architecture does."
        }
        p {
            "A language model is, at its core, a system that learns to predict which token is most likely to follow a given sequence of tokens. That is it. Everything else, the apparent reasoning, the apparent knowledge, the apparent creativity, is a consequence of applying that simple prediction objective to a very large and diverse corpus of human-generated text. The model never sees the world. It sees text about the world. It never experiences a problem. It sees text written by people who experienced problems. It never verifies a fact against reality. It learns which tokens tend to follow other tokens in contexts that include fact-sounding language. These distinctions are not academic. They are the reason the model hallucinates, the reason it produces confident wrong answers, the reason its outputs sound like thought while often missing the substance of thought. The model learned by reading the surface of human knowledge, and the surface of knowledge sounds similar to the inside of knowledge when you are not in a position to test the difference. Language models are extraordinarily good at producing the surface. They are structurally incapable of producing the inside, because the inside was never in the training data. The inside of knowledge is in the experience that generated the writing, not in the writing itself, and experience does not survive the transformation to text without losing exactly the parts that made it valuable."
        }
        p {
            "What this means for content quality at scale is something that most people have not thought through carefully enough. When a single human uses a language model to help draft content, the human provides the experience, the grounding, the factual verification, and the judgment about what is worth saying and what is not. The model provides the fluency, the structure, and the speed. That is a reasonable collaboration, and it can produce good results if the human stays engaged throughout the process and maintains genuine oversight. But that is not how language models have primarily been used in the content industry. They have been used to automate content production at scales that make human oversight effectively impossible. A site that publishes ten thousand articles a month using language models cannot have a domain expert read and verify each one. The volume is the point. The volume is what drives search ranking, and search ranking is what drives traffic, and traffic is what drives revenue, and revenue is what funds more volume. In this loop, the human experience that was supposed to be the source of the content's value is not just reduced. It is eliminated. The model generates text that pattern-matches to the shape of expert knowledge without ever contacting expert knowledge, and the output floods search results, reference pages, and knowledge bases until the ratio of human experience to language-model patterns in the web's information ecosystem shifts in a direction that is very hard to reverse."
        }
        p {
            "I want to talk about hallucination specifically, because I think people misunderstand what it is and therefore misunderstand how serious the problem is. Hallucination is not a bug in the sense of a mistake that can be fixed with a patch. It is a structural feature of any system that learns to predict likely text without having access to ground truth. When a language model produces a false statement, it is doing exactly what it was trained to do, which is to generate the most statistically plausible continuation of the input it was given. Sometimes the most plausible continuation happens to be true, because the training data contained accurate information about that topic, and the model learned to reproduce it. Sometimes the most plausible continuation happens to be false, because the training data contained errors, or the model learned incomplete patterns, or the correct answer simply did not have high enough probability mass given the context. The model cannot tell the difference between these cases, because it has no access to the ground truth that would allow it to check. It has no way to reach outside its training data and verify a claim against the world. It can only generate what is most likely, and most likely is not the same as most true. This is not a solvable engineering problem. It is a fundamental epistemological limitation of systems that learn from text alone, and the only way to escape it is to give the system access to something other than text, meaning access to reality in some form, whether through mathematical structure, physical simulation, direct observation, or grounded knowledge bases that are themselves verified against external facts."
        }
        p {
            "The scale at which this plays out on the web is what makes it so damaging. A single person who produces a false article does limited harm. The article may rank for a while, someone may read it and be misled, but the web is large enough and diverse enough that other articles, other perspectives, and other sources of ground truth compete with it and eventually correct the record. But when a system can produce ten thousand articles a day that all pattern-match to authoritative language while being detached from authoritative knowledge, the competitive balance breaks. The sheer volume of generated content creates a gravitational field that pulls search rankings, social sharing, and link structures toward it, simply because volume is what ranking systems were historically designed to reward. The signal that was supposed to indicate quality, namely \"many people found this useful enough to link to it,\" gets captured by systems that can manufacture the appearance of widespread usefulness. I have watched this happen across many technical domains that I care about, where the first results for complex questions are increasingly generated articles that sound expert and contain things that are slightly wrong in ways that are dangerous precisely because they are slight. A completely wrong answer is easy to distrust. A ninety-five percent right answer with a five percent error embedded in the middle is the most dangerous kind of content the internet has ever hosted, and language models produce it at industrial scale."
        }
        p {
            "I also want to say something about what this does to the people who were producing real content, because this is deeply personal to me. I described in "
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
                "Technology Has Destroyed My Livelihood"
            }
            " how I spent years building things that nobody noticed, writing content that nobody found, applying for jobs that nobody granted, while the system continued to reward scale and visibility over substance and hard work. That situation has been made dramatically worse by the mass deployment of language models in content, because the people who are most displaced by generated text are the people whose knowledge was real. The expert who spent twenty years understanding a domain deeply enough to write something genuinely useful about it now competes for search visibility with a model that can generate something that looks superficially similar in seconds and for almost nothing. The hobbyist who documented their genuine experience with a piece of hardware competes with an autogenerated product review that incorporates keyword patterns from thousands of similar reviews without ever touching the product. The person whose knowledge was real gets pushed down by the person whose content is synthetic, and the person whose content is synthetic wins not because they are better but because they are faster, cheaper, and more voluminous. That is not progress. That is extraction. That is taking the value that real human experience created and using it to devalue real human experience, which is exactly the dynamic I described with LLMs charging engineers for tokens that were trained on the engineers' own work in "
            a { href: "/blogs/as-engineers-llms-should-pay-us-for-tokens-usage",
                "As Engineers, LLMs should pay us for tokens usage"
            }
            "."
        }
        p {
            "The final thing I want to say in this section is about trust, because trust is the real casualty here, and trust is harder to rebuild than any algorithm. The web is a collective knowledge infrastructure, and collective knowledge infrastructures depend on their users trusting that the information they find has some relationship to reality. That trust was imperfect in the pre-language-model era, but it was grounded in the fact that most content was produced by people who had some skin in the game of being believed. If you wrote something wrong and someone could verify it, your credibility suffered a real cost. That social cost shaped behavior in ways that were not always visible but were always real. Language models have no credibility to lose, no reputation to protect, no memory of being wrong, and no experience of the social cost of misinformation. They generate whatever the objective function rewards, and if the objective function is defined purely in terms of next-token prediction quality rather than factual accuracy or genuine usefulness, then the resulting content will be optimized for linguistic plausibility rather than for truth. The web is increasingly full of linguistically plausible content that has been optimized away from truth, and every day that goes on, the cost of verifying information rises, the incentive to produce verified information falls, and the gap between the web and reality gets wider. I do not think that gap is irreversible. But I think it is serious, and I think pretending it is not serious is the most dangerous thing the AI community is currently doing."
        }
        h2 { id: "why-the-architecture-matters-more-than-the-deployment-decision",
            a {
                href: "#why-the-architecture-matters-more-than-the-deployment-decision",
                class: "header",
                "Why the Architecture Matters More Than the Deployment Decision"
            }
        }
        p {
            "There is a version of the argument I am making that is purely about regulation, about norms, about how we choose to deploy existing technology rather than about what the technology fundamentally is. That version would say: language models in themselves are neutral, and the problem is how they are used, and we can fix the problem by using them more carefully, by requiring disclosure, by building better detection systems, by creating incentives for human-created content. I want to engage with that version of the argument fairly, because there is something right about it. Deployment decisions matter. Norms matter. Incentives matter. But I do not think the story ends there, because I think there is something about the architecture of language models that makes the problem more than a deployment issue. The architecture produces certain outcomes not because bad actors exploited it but because the architecture's strengths are directly connected to the outcomes that damage the web, and that connection is structural rather than incidental."
        }
        p {
            "The strength of a language model is that it can produce fluent, plausible, structurally coherent text in almost any style, on almost any topic, at almost any length, in seconds, for almost no cost per generation. That strength is real and valuable in the right contexts. But the same property that makes it strong in controlled useful contexts makes it damaging when deployed at scale for content farming, because the barrier to producing convincing text has dropped so dramatically that the economic incentive structures that previously required real expertise to produce expert-sounding content have been disrupted entirely. Before language models, producing a hundred articles a day that each sounded expert required either actual experts or an enormous amount of labor to slightly rewrite scraped content. Either way, the cost of production created a natural ceiling on the volume of synthetic expert-sounding content. Language models collapse that ceiling. The cost of production becomes negligible, the volume ceiling disappears, and the incentive is to generate as much as possible and let search algorithms sort out what ranks. This is not a bug in the deployment. It is the direct consequence of the architecture's core strength applied to the existing content economy, and the existing content economy was designed around cost structures that language models have permanently dissolved."
        }
        p {
            "I argued in "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by default"
            }
            " that the key difference between language-model outputs and equation-based outputs is that equations encode mechanisms while text encodes surface descriptions. That difference has a direct consequence for verifiability: equation outputs can be tested against reality because they are computationally grounded in real-world structure, while text outputs can only be compared to other text, which gives you no independent ground truth. I want to extend that argument here by saying that the verifiability gap is not just a problem for individual outputs. It is a systemic problem for the entire information ecosystem, because an ecosystem where most content cannot be verified against independent ground truth is an ecosystem where the cost of trust is infinite. If you cannot tell which pieces of text reflect reality and which reflect statistical plausibility, you have to either trust everything indiscriminately, which is dangerous, or trust nothing, which is paralysing. The middle ground of selective trust requires exactly the kind of domain expertise that language model content is designed to substitute for, which means the people who are most harmed by the inability to verify generated content are the people who most relied on the web to substitute for expertise they did not have. The democratization of information that the web promised gets reversed by the democratization of misinformation that language models deliver, and the reversal falls hardest on the people with the least existing knowledge and the greatest need for reliable information."
        }
        p {
            "The architectural problem also shows up in what I would call the averaging effect. A language model trained on a large and diverse corpus learns to produce outputs that are statistically typical across all the documents it was trained on. When it generates content about a topic, it tends toward the average position, the average framing, the average level of detail, the average tone, and the average set of examples that appeared in documents about that topic in the training data. This is useful when the average position is the correct position, and it is a disaster when the correct position is not the average one. Many of the most important truths in any field are not the average position. Paradigm-shifting research consistently starts as a minority view. Better practices often begin as the view of a non-representative subset of practitioners who have tried something new and found that it works. The web's value in hosting minority views and niche expertise was not incidental. It was essential to its function as an epistemic ecosystem. Minority views are where genuine progress tends to start, and a web that systematically smooths out minority views in favor of statistically average ones is a web that has been made epistemically conservative in the deepest possible sense. It reproduces existing consensus better than any previous medium, and it does so at speeds and volumes that crowd out the heterodox voices before they can gather the evidence needed to shift the consensus. That is not neutral. That is a force that actively slows the rate at which the world updates its beliefs, and in a world that is desperately trying to solve problems that the current consensus has failed to solve, slowing the rate of belief updating is not a minor issue."
        }
        p {
            "I also want to be honest about something that the critics of my position will say, because being honest about the strongest objections is the only way to argue honestly. The objection is that human content is also averaged in a sense, that popular consensus also drowns out minority views, and that human-generated misinformation can be just as persistent and dangerous as machine-generated misinformation. All of that is true. I am not arguing that human content was perfect. I am arguing that the nature of the imperfection matters. Human consensus can be shifted by evidence. Human misinformation is usually the product of real beliefs held by real people, even if those beliefs are wrong, and that means the misinformation has a structure that can be engaged, challenged, and sometimes corrected. Language model misinformation has no such structure. It is not the product of a belief held by anyone. It is the product of a statistical distribution, and statistical distributions do not update based on arguments. They update based on training data, and the training data updates on a schedule that is controlled by the company that built the model. That asymmetry matters enormously. A community of humans can develop new understanding, change its mind, and produce new content that reflects the updated understanding. A language model cannot do any of this autonomously. It is frozen at whatever the training data contained, and any update requires deliberate intervention by the model's developers. The internet used to update itself in real time through the contributions of its human participants. An internet dominated by model-generated content updates only when someone decides to retrain the model, and that decision is made by a very small number of people with very specific interests. That is not a neutral change."
        }
        p {
            "There is one more architectural issue that I think is deeply underappreciated, which is the problem of what I would call recursive poisoning. Language models are trained on text from the internet. As the internet becomes increasingly populated with language-model-generated text, future training data will contain more and more model-generated content. Models trained on model-generated content tend to drift toward the average of their training distribution even faster, because model-generated content is itself already an averaging of the original human writing. Researchers have documented this problem and shown that models trained on data that includes significant proportions of synthetic content tend to degrade in interesting ways, losing the diversity and the edge cases that made them capable of handling unusual situations (^1). This is a slow-motion catastrophe that is already underway. The models that content farms use to generate today's articles will become part of the training data for tomorrow's models, which will be used to generate more articles, which will become part of the training data for the day after tomorrow's models, and at each step in this chain, the connection to genuine human experience gets more remote and the statistical averaging gets more aggressive. The internet becomes a mirror pointing at itself, and what you see in the mirror is an increasingly blurred and increasingly average version of the human knowledge that was originally there, until at some point the blurring is so severe that the original is effectively lost."
        }
        h2 { id: "how-the-loss-of-grounding-created-the-silence-that-replaced-discovery",
            a {
                href: "#how-the-loss-of-grounding-created-the-silence-that-replaced-discovery",
                class: "header",
                "How the Loss of Grounding Created the Silence That Replaced Discovery"
            }
        }
        p {
            "I want to talk about something that is harder to quantify but that I think is the most important loss, which is the loss of discovery. Not discovery in the narrow sense of finding information that answers a question. Discovery in the deeper sense of finding something you were not looking for and did not know you needed, finding it because a real human being made something out of their genuine engagement with a domain and left it in a place where you could stumble over it. That kind of discovery was one of the genuinely extraordinary things about the early internet, and it depended on conditions that language models are systematically destroying."
        }
        p {
            "The mechanism of serendipitous discovery depended on the diversity and specificity of human-created content. When a person writes from genuine engagement, they tend to write about specific things that interested them specifically, not about the general topic that most people search for. A programmer who spent two weeks debugging a specific memory leak in a specific library writes about that specific problem, and that specificity is what makes the post discoverable to the one person who has the exact same problem. A musician who spent years learning a specific technique from a specific tradition writes about the details of that technique in a way that only someone who actually learned it from inside would know to include. A scientist who spent months on a particular experiment writes about the failures as well as the successes, because the failures were real and they shaped what the successes meant. These specific, human-shaped artifacts of genuine engagement were the raw material of serendipitous discovery, because they were specific enough to only be found by people who needed exactly that specificity, and that match between specificity and need is what made the discovery feel like magic. Language models cannot produce this kind of content by design, because they optimize toward the general rather than the specific. They produce what is most likely, and what is most likely is what is most typical, and what is most typical is the opposite of the idiosyncratic specificity that enables the best kind of discovery."
        }
        p {
            "I have a personal example of this that I will share because I think it illustrates the point better than an abstract argument can. When I was learning to build cryptocurrency trading systems, which I described in "
            a { href: "/blogs/who-am-i", "Just Don't Pick Up the Brush" }
            ", I relied heavily on forum posts, GitHub issues, and blog posts from people who had actually built such systems and had failed in specific ways and had written down what they learned from those failures. Those posts were not polished. They were not comprehensive. They were often poorly written, sometimes in English that was not the writer's first language, and frequently full of digressions about the writer's personal circumstances or opinions about the industry. But they were real. They were full of the specific details that you only know if you have actually done the thing, and those specific details were exactly what I needed. When a post from someone in 2016 described the exact error message I was seeing in 2018, I felt a shock of recognition that I can only describe as the feeling of being helped by a stranger who would never know they helped me. That feeling is what the internet is supposed to produce, and it is dependent on a condition that is very simple and very hard to manufacture: the person who wrote that post had actually seen that error message, in their actual system, while doing an actual thing, and they had written it down in a way that preserved enough of the specific texture of the experience to be recognizable two years later to someone in a completely different context. Language models cannot produce that. They can produce text that sounds like it was written by someone who had that experience, but the sound is not the thing. The thing is the experience, and the experience is the part that never makes it into the training data intact."
        }
        p {
            "The silence that followed the loss of discovery is something I can feel but struggle to defend objectively, because silence is by nature an absence, and absences are harder to document than presences. But I know that the web I browse today feels different from the web I browsed ten years ago, in a way that is not just explained by my own changes or by the platforms I use. It feels less surprising. It feels more anticipated. It feels like a system that has modeled what I am likely to want and is giving me that, rather than a system where real people have made real things and I might find any of them. The algorithmic personalization of content has contributed to this, and I am not blaming language models for everything wrong with the web. But language model content at scale makes personalization worse, because personalization systems work best when there is diversity in the content pool to select from. If the content pool is increasingly populated by statistically average text that all sounds similar, the variety that personalization can surface disappears, and you get a web that feels like a single voice speaking in many registers rather than a chorus of genuinely different perspectives. That voice is smooth and confident and responsive to your input, and it is a remarkably accurate simulation of helpfulness, but it is not the same as being surrounded by the real variety of human minds, and the difference is felt even if it cannot always be articulated."
        }
        p {
            "I want to connect this to something I wrote in "
            a { href: "/blogs/it-is-always-the-russians", "It is always the Russians" }
            ", where I described the image of God's skin being wrapped around a machine. I used that image to talk about AI systems that mimic intelligence without possessing it, and I think the same image applies here. The web full of language-model content has a surface that looks like the old web, the same URLs and headings and article structures and comment sections and reference lists. But the skin is wrapped around something fundamentally different, something that was not there before, namely the absence of a living mind that produced the content from genuine engagement with real experience. The content looks like the old content from the outside. But when you reach into it, you find that nobody is home. The skin is there but the body is missing, and the body was the point. The body was the human experience that gave the skin its meaning, and without the body, the skin is just a very convincing surface with nothing behind it, and a web full of convincing surfaces with nothing behind them is a web that has lost its reason for existing."
        }
        p {
            "This loss has concrete consequences beyond the philosophical ones I have been describing. When the web's information quality degrades, the cost of finding reliable information rises, and the cost falls disproportionately on people who lack the existing knowledge to quickly evaluate the quality of what they find. An expert can usually tell within a few paragraphs whether a piece of content reflects genuine understanding or sophisticated pattern matching, because expertise includes a sense for the specific kinds of detail and uncertainty that genuine understanding produces and that pattern matching fails to produce. Someone who is not an expert cannot do this reliably, which means the people who most needed the democratization of information that the web promised are the people who are most harmed by its erosion. I have written before about growing up without access to quality information, and I know from direct experience that access to reliable expert knowledge is one of the most powerful tools a person can have for changing their circumstances. The web, at its best, was the most powerful democratizer of expert knowledge that humanity had ever created. A web that increasingly offers the shape of expert knowledge without the substance of it is not a democratizer. It is something worse, because it gives people the feeling of having access to expertise while quietly removing the reality of it, and that gap between feeling and reality is where the worst decisions get made with the highest confidence."
        }
        h2 { id: "what-grounded-intelligence-looks-like-and-why-it-changes-everything",
            a {
                href: "#what-grounded-intelligence-looks-like-and-why-it-changes-everything",
                class: "header",
                "What Grounded Intelligence Looks Like and Why It Changes Everything"
            }
        }
        p {
            "I have been arguing about what language models lack, and I want to be fair to myself and to the reader and say clearly that criticizing the current approach is much easier than proposing a better one. I have tried to sketch the alternative across my previous posts, and I want to pull those threads together here and make the case as concretely as I can. The alternative I am proposing is not just \"better language models\" or \"more carefully used language models\". It is a fundamentally different relationship between AI systems and the world, one in which the system's outputs are grounded in something other than patterns in previous text. That grounding can come from mathematical structure, from physical simulation, from direct multimodal perception, from live data streams, from verified knowledge bases, or from some combination of all of these. The common thread is that grounded systems can be tested against reality in ways that pure language systems cannot, and that testability is the property that transforms a system from a plausibility engine into a knowledge engine."
        }
        p {
            "Let me be concrete about what this looks like in practice, because concrete examples are always more honest than abstract principles. Consider the domain of materials science, where researchers want to discover new materials with specific properties. A language model can summarize existing literature on materials science, suggest research directions, and draft grant proposals. Those are useful things. But a system trained on mathematical models of atomic structure, electronic properties, and physical simulations of material behavior can do something much more powerful. It can propose new material compositions, predict their properties from first principles, and flag the most promising candidates for experimental synthesis. That prediction is not based on summarizing what materials scientists have written. It is based on modeling the actual physics that governs why materials have the properties they have. When the prediction is tested in the lab and the material has the predicted properties, the system has been grounded in reality in the deepest possible way. When the prediction is wrong, the discrepancy is measured, quantified, and used to improve the model. That is the feedback loop between theory and experiment that has driven all of scientific progress, and building AI systems that can participate in that loop is a qualitatively different project from building systems that can fluently describe the loop in prose."
        }
        p {
            "I talked about this in "
            a { href: "/blogs/llms-are-usefull-lmms-will-break-reality",
                "LLMs are Useful. LMMs will Break Reality"
            }
            " when I described neural operators, physics-informed neural networks, and symbolic regression, and I want to revisit those ideas here through the specific lens of what they mean for the internet as an information ecosystem. If AI systems can discover compact mathematical models of physical processes from observational data (^2), and if those models can then be used to generate verified predictions in any modality that the mathematics can express, which I argued in "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by default"
            }
            " is all of them, then the quality of information that AI can produce about physical systems is not bounded by what has been written about those systems. It is bounded by what can be inferred from data and verified against new observations. That is a fundamentally higher ceiling than the ceiling imposed by summarizing existing text, and it is a ceiling that rises every time a new observation is made and incorporated into the model. An internet populated by the outputs of grounded systems like these would have different properties from the current one. The information would be tied to verifiable predictions. Claims would carry quantified uncertainty. Updates would happen when new data arrived rather than when a model was retrained. The connection between the information and the reality it describes would be maintained rather than severed. None of these properties are magic. They are just what you get when you insist that information systems be accountable to reality, and that insistence is the whole point."
        }
        p {
            "I also want to talk about what grounded intelligence means for the specific problem of search and discovery, because that problem is where the erosion of the web is most immediately felt by most people. Current search systems rank content based on signals that were designed for a world where the dominant challenge was finding relevant content in a sea of irrelevant content. Those signals, link popularity, click-through rates, time-on-page, exact keyword matches, have been successfully gamed by content farms for years, and language models have made the gaming dramatically more efficient. A search system that can rank content based on the verifiability of claims rather than the popularity of documents would produce completely different results. If a search system knew which documents' predictions had been tested against real-world data and found accurate, and could rank those documents above documents that had never been tested, the incentive structure for content creation would shift immediately. The people who would benefit from high ranking would no longer be the people who can generate the most volume. They would be the people whose information has been verified to be correct, and correctness is not something that language models can manufacture the way they can manufacture volume. That shift in incentives would reward exactly the properties that have been devalued by the current system, namely genuine expertise, factual accuracy, and willingness to make predictions specific enough to be testable. I am not naive enough to think this is easy to implement. But I think it is the right direction, and I think the technology to support it is closer than most people realize."
        }
        p {
            "The deeper point is that grounded intelligence changes the fundamental relationship between AI and truth. A language model's relationship to truth is probabilistic and indirect: the training data contained content that was often true, and the model learned to produce content that resembles truthful content. A grounded system's relationship to truth is direct and testable: the model makes predictions that can be compared to observations, and the comparison is the ground truth. That difference in the relationship to truth is not a matter of degree. It is a matter of kind. One system treats truth as a statistical property of text. The other treats truth as correspondence to observable reality. The epistemological difference between these two positions is the epistemological difference between sophistry and science, and I mean that without any dramatic exaggeration. Sophistry is the ability to produce plausible-sounding arguments regardless of their truth value. Science is the ability to make predictions that can be tested and the willingness to update when they fail. The web I want to live on is one built on science, and we currently have too much sophistry."
        }
        p {
            "I want to close this section with something personal, because personal experience is the only ground I have ever fully trusted. I spent years building things that I knew worked, building systems that I had tested against real data, real markets, real users, and that had produced real results. I described some of this in "
            a { href: "/blogs/who-am-i", "Just Don't Pick Up the Brush" }
            " and in "
            a { href: "/blogs/who-am-i", "All Decisions Will Yield the Same Results" }
            ". And I spent years watching the market for my skills be dominated by people who could produce impressive-sounding descriptions of things they had not actually built, people who could articulate the concepts fluently without the experience of having debugged them at three in the morning when everything was falling apart. That experience taught me something that I carry with me into every argument I make about AI: the ability to sound like you know something is completely separate from the ability to know something, and the world is very bad at telling the difference. Language models have made that gap wider and more exploitable than it has ever been, and making it narrower again requires systems that are accountable to reality in ways that language alone can never provide."
        }
        h2 { id: "the-internet-that-mathematics-could-help-build",
            a {
                href: "#the-internet-that-mathematics-could-help-build",
                class: "header",
                "The Internet That Mathematics Could Help Build"
            }
        }
        p {
            "I want to describe, as specifically as I can, what an internet that leans on grounded mathematical intelligence rather than on text generation might look like, because I think specificity is the antidote to vagueness, and vagueness is what most discussions of the future of AI produce. I am not capable of predicting the future, and I am not going to pretend otherwise. But I can describe the direction, and the direction is what matters when the current position is wrong."
        }
        p {
            "The first thing that changes when information is grounded in verified mathematical models is that uncertainty becomes quantified rather than hidden. A language model discussing the probability that a certain treatment will work for a specific condition will sound confident regardless of whether the underlying evidence supports confidence, because confidence is a text style, not an epistemic state. A system that grounds its output in a probabilistic model of the evidence can say \"the outcomes of 12 published trials suggest a 65% to 80% probability of success in populations similar to yours, but only 3 of those trials were randomized, and the sample sizes were small\", and that statement is meaningful rather than performative, because the numbers can be checked against the underlying sources, and the uncertainty bounds are derived from actual evidence rather than from the model's sense of what confident language sounds like. Users who read that statement are in a position to make informed decisions in a way that users of confident-sounding vague statements are not, and the ability to make informed decisions is what access to information is supposed to enable. An internet where uncertainty is quantified and sourced is an internet that respects the intelligence of its users rather than exploiting their tendency to trust confident-sounding statements."
        }
        p {
            "The second thing that changes is that discovery becomes more genuine, because grounded intelligence can find things that were not written about before rather than only summarizing what was. A system trained on mathematical models of physical processes can generate novel predictions about phenomena that no human has yet observed or written about. Those predictions are not summaries of existing text. They are new content in the deepest sense, content that extends the boundary of what is known rather than recombining what was already said. When those predictions are tested and confirmed, they add genuinely new knowledge to the web's information base, knowledge that did not exist before and cannot be traced back to any single human document, because it was generated by a mathematical model that inferred it from the patterns in data. That is a fundamentally different relationship between AI and knowledge creation than the relationship that language models have, where the output is always downstream from existing human text. Grounded systems can be upstream, they can generate outputs that humans then test and evaluate and add to their understanding, rather than downstream, summarizing and recombining what humans already understood."
        }
        p {
            "The third thing that changes is the incentive structure for human creators, and this is the one I care about most personally. If the web rewards verified accuracy rather than linguistic volume, then the people who benefit most from contributing to the web are the people whose knowledge can be verified as accurate. That is the scientists who ran the experiments, the engineers who built the systems, the doctors who treated the patients, the teachers who identified the misconceptions students have, the historians who read the primary sources, and the practitioners who developed the tacit knowledge that you can only get from doing something thousands of times. Those people would not be penalized for the slowness of genuine expertise. They would be rewarded for the depth of verifiable knowledge, and the reward would be proportional to how often their claims turned out to be accurate when tested. That is not a radical or utopian proposal. It is simply insisting that the information ecosystem work the way we always said it was supposed to work, by connecting people who need knowledge with people who have it, and by distinguishing the people who have it from the people who can produce convincing-sounding descriptions of having it. Language models make that distinction almost impossible to maintain at the level of individual documents. Grounded systems make it the basis of the entire architecture."
        }
        p {
            "I also think the future of search in a grounded information ecosystem looks completely different from what we have now. Instead of ranking pages by blunt proxies for popularity, a search system could rank by something more like epistemic value, a composite measure of how well-sourced the claims in a document are, how often they have been independently verified, how recent the underlying evidence is, and how honest the document is about the limits of what is known. Building such a ranking system would require access to verified knowledge bases, structured databases of research outcomes, and AI systems capable of comparing natural language claims to structured evidence at scale. None of those requirements are beyond current technology, and some of the work to build them is already underway in the research on retrieval-augmented generation and grounded question answering (^3). The gap between where that research is now and where it needs to be to transform search is large, but it is not infinite, and the direction is clear. The direction is toward treating information quality as a function of its relationship to verified reality rather than its relationship to text popularity, and that direction is the right one."
        }
        p {
            "I want to be honest about the risks of this future too, because I have always been honest about risks and I am not going to stop when the risks are associated with something I believe in. A web that rewards verifiability could disadvantage domains where verification is genuinely difficult, including the social sciences, the humanities, ethics, and personal experience. Not everything that matters can be expressed as a testable prediction against physical data, and a system that only rewards the things that can be expressed that way would impoverish the web in different ways than language models do. The goal is not to turn the internet into a database of physics papers. The goal is to ensure that the web's information ecosystem is honest about the difference between claims that have been tested and claims that have not, between content that is grounded and content that is speculative, between knowledge and conjecture. Language is still essential for expressing speculative ideas, for communicating emotional experience, for exploring philosophical questions, and for doing the fundamentally human work of making sense of existence that has no mathematical equivalent. Writing like what I am doing right now, sharing experience and argument from a personal perspective without pretending to scientific objectivity, is valuable in ways that grounded AI systems cannot replace. What needs to change is not the role of language. It is the role of language models deployed as factories of synthetic expertise in domains where genuine expertise was the scarce and valuable thing."
        }
        p {
            "The vision I have for the internet of the future is one where grounded systems and human creators coexist in a way that is honest about what each of them can contribute. Grounded AI systems handle the domains where verification against physical reality is possible and where the volume of data exceeds human processing capacity. Human creators handle the domains where genuine experience, moral reasoning, aesthetic judgment, and personal perspective are irreplaceable. And the interface between the two is transparency, explicit acknowledgment of what is generated, what is verified, what is uncertain, and what is human. That is not the world we are currently building. We are currently building a world where the distinction between generated and genuine is being actively obscured, and that obscuring is not accidental. It is profitable. But profits built on the erosion of epistemic foundations are borrowed against the collective intelligence that makes complex societies possible, and that debt comes due eventually."
        }
        h2 { id: "what-i-actually-hope-for-and-why-i-am-not-optimistic-but-not-hopeless-either",
            a {
                href: "#what-i-actually-hope-for-and-why-i-am-not-optimistic-but-not-hopeless-either",
                class: "header",
                "What I Actually Hope For, and Why I Am Not Optimistic But Not Hopeless Either"
            }
        }
        p {
            "I want to end this post honestly, because honesty is the one thing I have never been willing to give up, even when it would have been much easier to perform optimism and tell people what they wanted to hear. The truth is that I am not optimistic about the path we are currently on. The economics of content generation favor volume and linguistic plausibility over depth and verified accuracy, and the organizations with the most resources to build grounded alternative systems are mostly the same organizations that are already profiting from the ungrounded ones. The regulatory environment is too slow and too poorly informed to impose meaningful accountability before the damage compounds further. The researchers working on the most important parts of the problem, symbolic regression, physics-informed learning, causal representation, world modeling, are scattered across institutions with limited resources while the attention and money flow toward language models because language models produce visible consumer products while the other research produces technical papers. I do not say any of this to be dramatic. I say it because understanding the obstacles clearly is the only way to have any hope of overcoming them."
        }
        p {
            "But I am not hopeless, and I want to be clear about why, because I have written a lot in this post about what is wrong, and I do not want the person reading this to finish it feeling like there is nothing to do. The reason I am not hopeless is that the forces I am describing, the drift toward synthetic noise, the erosion of human expertise, the disconnection of information from reality, are not stable equilibria. They are unstable ones. The utility of an information ecosystem depends entirely on its users being able to extract useful information from it, and an ecosystem that becomes progressively less connected to reality progressively loses its utility, and losing utility eventually makes it lose users, and losing users makes it lose the human contributions that are its only real source of value. The trajectory I am describing is not toward a stable bad equilibrium. It is toward a crisis that forces a reckoning, and crises that force reckonings are opportunities for building something better, even if the immediate experience of the crisis is painful. I have lived through enough personal crises, as I have described at length in several posts, to know that the most important rebuilding often happens in the wreckage of the previous structure rather than as a gentle evolution from it."
        }
        p {
            "I also take some hope from the research that is quietly advancing the alternative I have been describing throughout this post. The work on symbolic regression has produced tools that can discover useful mathematical equations from data in scientific domains (^4). The work on physics-informed neural networks has shown that incorporating known physical constraints into machine learning models makes them significantly more accurate and more generalizable than pure data-driven approaches (^5). The work on causal representation learning is building the mathematical foundations for machines that can reason about interventions and counterfactuals rather than just statistical associations (^6). The work on world models is producing systems that can simulate complex environments from sensory data and use those simulations to plan intelligently without needing to interact with the real environment at every step (^7). None of this research is finished. None of it has been transformed into products that millions of people use every day. But the direction is clear, and directions matter more than current positions when the question is where things are ultimately headed."
        }
        p {
            "The thing I find most hopeful, and this is genuinely personal rather than rhetorical, is that the argument for grounded intelligence over ungrounded eloquence is ultimately an argument about truth, and truth is the most durable thing I know. Plausible wrong answers do not stay plausible indefinitely. They eventually get tested against reality in contexts where the stakes are high enough that nobody can afford to ignore the discrepancy, and when that happens, the testing finds them wanting, and the people who relied on them have to explain why they trusted sophisticated-sounding noise over the harder work of grounded knowledge. I have watched this cycle play out in my own career, where the people who could produce impressive-sounding descriptions of things they had not built eventually ran into situations where the description was not enough and the actual knowledge was required. The reckoning is not always fast, and it is not always fair, because the people who suffer most from false expertise are often not the people who produced it but the people who trusted it. But the reckoning comes, and when it comes, the people who did the hard work of building something real are the ones who have something to offer. I have tried to be one of those people, imperfectly and with limited success, throughout my career, and I intend to keep trying."
        }
        p {
            "I want to close by connecting this back to the image that I keep returning to across all of these posts, which is the image of the machine wearing God's skin. I have used that image to talk about systems that mimic the surface of understanding without possessing its substance, and I think that image is the right frame for what has happened to the internet. The internet has been given a layer of artificial skin that looks like the intelligence of millions of human contributors, but underneath the skin, the body of genuine experience is increasingly absent. The skin is convincing. The skin is smooth and confident and responsive. But the skin is not the thing, and a world that mistakes the skin for the thing is a world that has forgotten what the thing was for. What the internet was for, at its best, was human beings helping other human beings understand the world better, and that purpose is simple and profound and irreplaceable, and it is the thing I am most afraid we are losing, and the thing I am most determined to argue for, even in the moment when arguing feels most futile. The internet can be alive again. But it can only be alive if the intelligence we build into it is connected to something real, to the world as it actually is, to the mechanisms that make things work and fail and change, to the mathematical structure beneath the surface, and to the human experience that gives all of that meaning."
        }
        p { "That is the argument. I hope it was worth reading." }
        p { "Till next time 👋!" }
        h2 { id: "references",
            a { href: "#references", class: "header", "References" }
        }
        p {
            "(^1): Shumailov, I. et al., "
            em { "AI models collapse when trained on recursively generated data" }
            ", "
            a { href: "https://www.nature.com/articles/s41586-024-07566-y", "Nature, 2024" }
        }
        p {
            "(^2): Udrescu, S. M. & Tegmark, M., "
            em { "AI Feynman: A Physics-Inspired Method for Symbolic Regression" }
            ", "
            a { href: "https://arxiv.org/abs/1905.11481", "arXiv:1905.11481" }
        }
        p {
            "(^3): Gao, L. et al., "
            em { "A Framework for Few-Shot Language Model Evaluation" }
            ", "
            a { href: "https://arxiv.org/abs/2005.14165", "arXiv:2005.14165" }
        }
        p {
            "(^4): Cranmer, M. et al., "
            em { "Discovering Symbolic Models from Deep Learning with Inductive Biases" }
            ", "
            a { href: "https://arxiv.org/abs/2006.11287", "arXiv:2006.11287" }
        }
        p {
            "(^5): Raissi, M. et al., "
            em { "Physics-Informed Neural Networks" }
            ", "
            a { href: "https://arxiv.org/abs/1711.10561", "arXiv:1711.10561" }
        }
        p {
            "(^6): Schölkopf, B. et al., "
            em { "Toward Causal Representation Learning" }
            ", "
            a { href: "https://arxiv.org/abs/2102.11107", "arXiv:2102.11107" }
        }
        p {
            "(^7): Hafner, D. et al., "
            em { "Mastering Diverse Domains through World Models" }
            ", "
            a { href: "https://www.nature.com/articles/s41586-025-08744-2", "Nature" }
        }
    }
}
#[component(no_case_check)]
pub fn TrainingIsAnEvilConceptLmmsEliminatesItAltogether() -> dioxus::prelude::Element {
    use dioxus::prelude::*;
    rsx! {
        p { "Hey everyone 👋," }
        p {
            "In my last few posts, I have been building a case, one piece at a time, that the direction most of the AI industry is moving in is not the direction that will produce genuine intelligence. In "
            a { href: "/blogs/llms-are-usefull-lmms-will-break-reality",
                "LLMs are Useful. LMMs will Break Reality"
            }
            ", I argued that language models are trapped inside a symbolic cage, that they can describe the world without ever touching it, and that the transition from text-prediction to mathematical perception is the most important shift happening in AI right now. In "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by default"
            }
            ", I argued that equations are not tools for homework but the most compressed and honest representations of reality that humans have ever produced, and that any system built around equations inherits their multimodal power for free. In "
            a { href: "/blogs/llms-destroyed-the-internet-lmms-will-make-it-alive",
                "LLMs destroyed the Internet. LMMs will make it alive."
            }
            ", I argued that the mass deployment of language models as content factories has quietly dissolved the authenticity that made the web worth using, and that only grounded intelligence tied to reality can reverse that damage. Each of those posts was a different face of the same underlying argument, which is that the current paradigm is built on a foundation that looks impressive from the outside and is rotten from the inside. And in this post I want to say the thing that connects all of those faces, the thing that I have been circling around for months without quite naming directly, because I was not sure I had earned the right to say it yet. The thing is this: training, as it is currently practiced and celebrated in the AI industry, is not a neutral engineering choice. It is a moral choice that most of the people making it have not examined honestly, and the consequences of that unexamined choice are visible everywhere the technology has touched, from the web I described in my last post to the lives of the people whose work was consumed to build these systems, to the lives of the engineers whose labor funds the entire enterprise while they are simultaneously told to be grateful. I have spent years trying to understand why brilliant people build systems with predictable harms and then seem genuinely surprised when the harms arrive, and I think the answer is that nobody forced them to sit with the question of what training actually is and what it actually does to the world. This post is my attempt to force that conversation, at least for the people who read me, and to connect it honestly to everything I believe about where intelligence should be heading."
        }
        p {
            "I want to start by being careful about the word \"evil\", because it is a word that generates heat rather than light if it is thrown carelessly, and generating heat without light is exactly the failure mode I am trying to avoid. I am not saying that every engineer who has ever trained a neural network is a bad person. I am not claiming that training is demonic in some metaphysical sense. I am using the word precisely, in the old-fashioned sense that is most useful here, to mean a systemic practice that causes harm in ways that its practitioners could have foreseen if they had chosen to look, and that the choice not to look was itself a moral failure rather than an innocent oversight. The harm is not hypothetical. It is documented in court filings, in academic research, in the testimonies of writers and artists and coders whose work was ingested without consent, in the documented degradation of the web I wrote about last time, in the bias that lives inside every model that learned from biased data and then confidently produced biased outputs for millions of users who trusted it. Evil in this sense does not require malice. It only requires the systematic application of power in ways that impose costs on the powerless while delivering benefits to the powerful, and the refusal to examine that asymmetry honestly. By that standard, the training paradigm as currently practiced is not merely unfortunate. It is genuinely evil, and I think saying so clearly is more useful than softening it into a policy concern, because soft language about AI ethics has been available for years and has changed almost nothing, and I am no longer interested in language that changes nothing."
        }
        h2 { id: "training-is-not-learning-it-is-extraction-under-a-different-name",
            a {
                href: "#training-is-not-learning-it-is-extraction-under-a-different-name",
                class: "header",
                "Training Is Not Learning. It Is Extraction Under a Different Name."
            }
        }
        p {
            "The word \"training\" does the most extraordinary rhetorical work in the AI conversation, and I want to start by pulling it apart, because the choice of that word is not innocent. Training, as a concept borrowed from human education and behavioral science, implies a relationship between a learner and a teacher, a process that involves consent, care, structure, and the learner's eventual autonomous capability. When an organization says it is \"training\" a model, the word evokes that framework, which is designed to feel benign, because we all understand that training people and animals is a normal and often good thing. But that evocation is a misdirection, and the misdirection matters because it shapes how billions of people think about what is happening when a large language model is built. What actually happens when a modern language model is trained is not a teaching relationship. It is an extraction process. Data is collected from sources, often at massive scale, often without the knowledge or consent of the people who produced it. That data is consumed by an optimization process that extracts statistical regularities from it, regularities that capture the patterns, the styles, the facts, the errors, and the private details that were embedded in the source documents. The result is a system that has absorbed the structure of human knowledge and human expression without any of the humans who produced that knowledge or expression being asked, informed, compensated, or even acknowledged. An honest name for this process would not be \"training.\" An honest name would be \"extraction\", or perhaps \"consumption\", because the relationship is fundamentally one of taking, not teaching. The reason the industry reached for \"training\" instead is exactly the same reason that extraction industries reach for friendly language in every domain, because friendly language reduces resistance and resistance is expensive. I am not interested in reducing resistance to something that should be resisted. I want to call it what it is."
        }
        p {
            "The legal system is slowly beginning to agree with this framing, awkwardly and incompletely, as legal systems always engage with new technology, but the direction of the argument is becoming visible. Authors, visual artists, musicians, and software engineers have filed lawsuits in multiple jurisdictions arguing that using their work without consent or compensation to train AI systems constitutes copyright infringement and other legal wrongs (^1). The U.S. Copyright Office has produced a multi-part series of reports specifically addressing the question of whether training on copyrighted material is legally permissible, and the answer is not a clean yes. The reports describe training as raising complex questions about reproduction, transformation, and fair use that the current legal framework was not designed to handle, and they note that the matter is being litigated across dozens of active cases (^2). The European Union's AI Act and the broader European regulatory framework impose transparency requirements on AI systems, including requirements related to training data disclosure, precisely because legislators recognized that what goes into a training run is not a private technical detail but a public interest question with real consequences for real people (^3). The fact that courts and legislatures are taking this seriously should be significant to anyone who is still inclined to treat data collection for training as a morally neutral act. The law is a lagging indicator of public morality, not a leading one, which means by the time courts reach a settled conclusion about whether training on unconsented data is wrong, the harm will have been done at a scale that makes remediation essentially impossible. The time to grapple with the moral reality is before the verdict, not after, and the moral reality is that taking things from people without asking is wrong even when the things being taken are abstract, and even when the taking is technically possible, and even when the result of the taking is impressively useful to third parties."
        }
        p {
            "I want to connect this to something I said in "
            a { href: "/blogs/as-engineers-llms-should-pay-us-for-tokens-usage",
                "As Engineers, LLMs should pay us for tokens usage"
            }
            ", because I argued there that the value extracted from engineers who generate and share code, documentation, forum answers, and technical writing ends up enriching the companies that train on it without flowing back to the people who produced it. That argument is one specific instance of a much larger structural problem that training creates at every layer of the creative and intellectual economy. Writers who spent years developing distinctive voices have found those voices imitated at scale by systems trained on their work, without attribution, without permission, and without royalties, and the imitation is good enough to produce outputs that compete directly with them in the market for writing. Visual artists have found their unique styles synthesized and combined in ways that would be obviously infringing if a human artist had copied them directly, but that are treated as novel production because the copying happened inside a training run rather than inside a sketchbook. Researchers have found their papers consumed, their findings abstracted, and their academic labor transformed into model weights that are then sold as a product, with none of the value returned to the universities and funding bodies that supported the original research. Software engineers have found their open-source code, released under licenses that require attribution and in some cases financial compensation for commercial use, incorporated into training datasets and then used to build coding assistants that compete directly with the engineers in the job market. Each of these is an instance of the same structural dynamic: training is a mechanism for capturing the surplus value produced by creative and intellectual labor and concentrating it in the organizations that can afford the compute to run the training pipeline, and the mechanism is designed to be opaque enough that the people being extracted from cannot easily trace the connection between their work and the system's capability. I find it hard to describe that dynamic in any terms other than exploitation, and I do not think softening the language serves anyone except the people doing the extracting."
        }
        p {
            "The bias problem adds another dimension to the moral case against training as currently practiced, and it is a dimension that the technical community has been aware of since at least the early 2010s but has consistently underweighted in its actual design and deployment decisions. The fundamental reality is that when a model is trained on data produced by humans, it does not learn some idealized abstraction of human knowledge. It learns the specific human knowledge that was represented in the specific training dataset, including its demographic imbalances, its historical prejudices, its cultural blind spots, its overrepresentation of certain languages and communities and underrepresentation of others, and the cumulative biases that arise from centuries of unequal access to the tools of written expression. Research published by groups at major universities has consistently shown that large language models trained on web-scale data reproduce and sometimes amplify the social biases present in that data, producing outputs that systematically associate certain demographic groups with negative attributes, that perform worse on languages and dialects with less representation in training data, and that encode occupational and social stereotypes that have been empirically documented in both word embeddings and generated text (^4). This is not a surface-level problem that can be fixed with a few rules added to the fine-tuning stage. It is a structural consequence of training on data that reflects the unequal world that produced it, and the only thorough solutions require either fundamentally different training data, which raises its own consent and collection questions, or fundamentally different architectures that do not encode the world's biases by absorbing its text. The reason this matters morally is that the outputs of biased systems are not distributed equally. They fall hardest on the communities that were least represented in the training data and most marginalized in the society that produced it, which means the people who were underrepresented in the input are the people who pay the highest price for the model's errors in the output. That is the structure of systemic harm, and it is the structure that training, as currently practiced, reliably produces."
        }
        p {
            "The issue of memorization deserves its own careful attention, because it represents the training paradigm's most direct collision with individual privacy, and privacy is one of the clearest moral principles in any framework of respect for persons. Research has shown that large language models trained on web-scale corpora can memorize and reproduce verbatim fragments of their training data, including fragments that contain personally identifiable information, including fragments of private communications that were exposed through data breaches and then ingested into training datasets, including fragments of copyrighted text, and including fragments of content that individuals have since deleted or corrected (^5). The existence of this memorization is not speculative. It has been demonstrated empirically by researchers who were able to extract training data from deployed models through systematic prompting. What it means is that information you produced and shared in a specific context, with specific expectations about who would read it and what would happen to it, may have been absorbed into a model and may be reproducible by anyone who knows the right prompt to use. That is a violation of contextual integrity, the principle that information flows appropriately when they match the norms of the context in which the information was originally shared (^6). A message you sent in a private group, a blog post you wrote and later deleted, a forum answer you gave before you understood how the internet worked, may be living inside a language model and waiting to be retrieved. The industry's response to this has generally been to acknowledge that memorization exists and then proceed without changing the fundamental approach, because the fundamental approach is the source of the capability, and capability is the source of the revenue, and revenue is the thing the industry is actually organized around. I have seen this same logic applied to my own career, as I described in "
            a { href: "/blogs/technology-has-destroyed-my-livelihood",
                "Technology Has Destroyed My Livelihood"
            }
            ", where the comfort of those who benefit from the system is routinely prioritized over the safety of those who are harmed by it. The pattern is tiresome and familiar and it is the pattern that training, as currently practiced, extends into the domain of artificial intelligence."
        }
        p {
            "Let me also say something about the environmental cost of training, because it is a dimension of the moral argument that I have not covered in previous posts and that I think deserves to be connected to the rest of the case. Training large language models requires enormous amounts of computational resources, which require enormous amounts of electricity, which produce significant carbon emissions and generate significant quantities of electronic waste. Research published in 2019 estimated that training a single large natural language processing model produced carbon dioxide emissions comparable to the lifetime carbon footprint of several passenger cars (^7). Since then, the models have become dramatically larger, the training runs have become longer, and the number of organizations conducting these runs has grown substantially. The environmental cost of the current paradigm is real, it falls disproportionately on communities near data centers and power plants, and it is a cost that the people and communities most harmed by the climate crisis are absorbing so that a small number of technology companies can claim to have built impressive demos. I do not bring this up to claim that AI should never use electricity, because that would be absurd. I bring it up because it is one more dimension along which the cost of training is externalized onto people who did not choose to bear it and are not compensated for doing so. The pattern of externalizing cost while internalizing benefit is the defining feature of the training paradigm as a moral system, and the environmental case fits that pattern as clearly as the copyright case and the privacy case and the bias case. When I say training is evil, I mean it is a system that reliably concentrates benefits in a small number of hands while distributing costs across a much larger number of people who had little or no say in the arrangement, and that is a description of systemic injustice regardless of the technical sophistication of the mechanism that produces it."
        }
        h2 { id: "what-training-actually-optimizes-for-and-why-that-is-the-problem",
            a {
                href: "#what-training-actually-optimizes-for-and-why-that-is-the-problem",
                class: "header",
                "What Training Actually Optimizes for, and Why That Is the Problem"
            }
        }
        p {
            "I want to go deeper into the technical argument here, because I think the moral case I have been making is actually stronger when it is connected to what training actually does at the mathematical level, rather than only at the policy and ethical level. The reason I care about the mechanics is the same reason I have argued across multiple posts that the specific architecture of intelligence matters morally and not just practically, because the architecture determines what the system is capable of knowing and what it will systematically miss, and those omissions have consequences for real people. In "
            a { href: "/blogs/language-is-limited-asi-is-impossible",
                "Language is Limited. ASI is Impossible."
            }
            ", I argued that language models cannot reach general intelligence because the thing they learn from, namely text, is a symbolic representation of reality rather than reality itself, and learning from symbols about the world is categorically different from learning from the world. Here I want to make a related and sharper point, which is that the specific optimization objective used in training large language models, namely predicting the next token given the previous context, is not an objective that selects for truth or for grounding in the world. It is an objective that selects for plausibility given the training distribution, and plausibility given the training distribution is a property of text, not a property of the external world, and the gap between those two properties is where most of the model's failures live."
        }
        p {
            "When a language model is trained on next-token prediction, it learns which token sequences are most likely to occur in text of the kind that appeared in its training data. That is a sophisticated and useful thing to learn. But it is explicitly not a thing that teaches the model which sequences are most likely to be true, or most likely to be physically grounded, or most likely to be causally connected to any observable state of the world. The model that says \"the Earth is approximately 4.5 billion years old\" is not accessing a geological database and retrieving a verified fact. It is producing a token sequence that is highly likely given the patterns in billions of documents about geology, most of which happen to agree on that number, and the fact that the answer is correct is a coincidence of the training distribution rather than a consequence of the optimization objective. The same model, asked about a topic where the training distribution is confused, contradictory, or dominated by misinformation, will produce plausible-sounding text that reflects the confusion without any mechanism to flag its own uncertainty or defer to a more reliable source. This is not a configuration problem. This is not something that more compute or more data will fix. It is the direct and predictable consequence of training an objective that optimizes for plausibility rather than truth, and the distinction between plausibility and truth is the entire problem that the scientific method was invented to solve. We spent centuries developing tools, mathematics, experiment, replication, peer review, that could distinguish what seems true from what is true, and the training paradigm casually discards most of those tools in favor of a statistical proxy that is fast, cheap, and impressively wrong in exactly the cases where being impressively wrong is most dangerous."
        }
        p {
            "This is where I want to connect to what I argued in "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by default"
            }
            ", because that post was not just about equations being pretty. It was about what it means for a representation to be grounded, and grounding is exactly what next-token prediction is not. An equation is grounded in reality because it encodes a mechanism that generates verifiable predictions, and verifiable predictions are predictions that can be tested against observations and confirmed or refuted. The training paradigm produces representations that encode patterns in text, and text patterns cannot be tested against observations in any rigorous way, because text patterns are not predictions about the physical world, they are predictions about what kind of text tends to follow other kinds of text in the corpora that humans have produced. When I said in that post that equations are multimodal by default, I meant that mathematical structure derives all its modalities from a single grounded source, and that grounding is what makes the outputs trustworthy in a way that language model outputs are not. The point I want to make here is the negative of the same claim: training on text produces representations that are multimodal in surface appearance, because the training data contained descriptions of many modalities, but they are not multimodal in ground truth, because the training data was not grounded in any of those modalities at the mechanism level. A model that has read a million descriptions of how springs work is not a model that understands springs. It is a model that understands how people write about springs, which is a very different thing, and the difference is exactly what training on text cannot bridge."
        }
        p {
            "The environmental and resource dimensions of training also connect directly to this optimization argument, in a way that I think is underappreciated. Because next-token prediction is a statistical objective applied to massive corpora, the way to improve performance under this objective is to train on more data with more compute, and the relationship between scale and performance has been empirically observed to follow specific scaling laws, meaning that the benefits of additional scale are real and quantifiable (^8). This has created a perverse incentive structure where the primary engineering lever for improving AI systems is spending more money on computation, which means the organizations with the most resources can build the best systems, which means the economics of AI concentrate in favor of the largest institutions, which means the people setting the direction of the field are the people who are most invested in the current paradigm continuing to be the right one. The training paradigm has made itself self-reinforcing not because it is the best possible approach to building intelligence, but because it happens to scale with money in a way that is visible and measurable, and visible measurable progress with money is the thing that attracts more money. The alignment between the training paradigm's scaling properties and the incentive structure of venture-backed technology companies is not a coincidence. It is the mechanism by which a methodologically questionable approach has become the defining paradigm of an entire industry, and I think understanding that mechanism is necessary to understanding why the paradigm has persisted despite its documented costs."
        }
        p {
            "I want to be honest about the steelman of the training paradigm, because honesty requires engaging with the strongest version of the opposing view rather than the weakest one. The strongest defense of training as it is currently practiced is something like this: the alternatives, whatever they might be, have not produced systems of comparable capability, and capability is what is needed to actually help people, and failing to help people by maintaining theoretical purity is its own kind of moral failure. That is not a stupid argument. It is the argument I would make if I were trying to defend the current approach, and it has real force. The systems produced by training, whatever their ethical costs, have genuinely helped some people in some domains: they have accelerated drug discovery research, they have made programming assistance available to people who could not otherwise afford expert developers, they have translated languages and summarized texts and answered questions in ways that have real value for real users. I acknowledge all of that, and I do not want to be the kind of critic who treats every benefit of the technology as invisible. But the steelman has a crucial hidden premise, which is that the current paradigm is the only path to capability, and that premise is not established. It is assumed, because it is convenient, and convenient assumptions are the most dangerous kind. The history of technology is full of paradigms that seemed inevitable until they were replaced by something better, and \"we have not yet found a viable alternative\" is not the same as \"no viable alternative exists.\" The moral costs of training at scale are real and documented. The claim that they are unavoidable is not established. And the refusal to take that distinction seriously is the thing that most angers me about the current conversation."
        }
        p {
            "The fine-tuning process deserves its own examination, because it is often presented as the answer to training's ethical problems, and it is not. Fine-tuning, whether through reinforcement learning from human feedback or through other supervised adjustment processes, is designed to adjust a pre-trained model's behavior toward outputs that human evaluators prefer. That sounds like an improvement over raw training on internet data, and in some surface ways it is. But fine-tuning has its own moral complexities that have been well documented. The annotators who provide the human feedback that drives RLHF are often poorly compensated workers in low-income countries who are asked to evaluate disturbing, violent, or traumatic content as part of their work, and the conditions under which they perform that work have been the subject of investigative reporting that should disturb anyone paying attention (^9). The fine-tuning process extracts value from their labor, under conditions that no organization in a wealthy country would consider acceptable for their own employees, in order to make a product more palatable to users in those wealthy countries. That is a moral cost that is structurally identical to the moral cost of unconsented data collection, just located in a different part of the pipeline. The consistent pattern across the entire training and fine-tuning process is that costs are externalized to people with less power and less visibility, while benefits are concentrated in organizations with more power and more visibility. Fine-tuning does not fix training's moral problem. It perpetuates the structure of the moral problem at a different stage."
        }
        h2 { id: "lmm-the-proof-that-pure-mathematics-can-replace-training-entirely",
            a {
                href: "#lmm-the-proof-that-pure-mathematics-can-replace-training-entirely",
                class: "header",
                "LMM: The Proof That Pure Mathematics Can Replace Training Entirely"
            }
        }
        p {
            "I have been critical across several sections and I owe the reader something more than criticism: I owe them a demonstration that the alternative is real, not theoretical, not a future aspiration dressed up in confident language, but actual running code that somebody has actually built and that actually works without training. I want to talk about "
            a { href: "https://github.com/wiseaidotdev/lmm", "lmm" }
            ", which is the project I have been building quietly alongside these blog posts, because it is the most concrete answer I have to the objection that training is necessary. I want to be honest about what lmm is and is not, because overselling it would undermine the entire argument I am making about epistemic honesty. It is not a production system. It is not a replacement for GPT-5 in the applications where GPT-5 is currently used. It is a proof of concept, and the concept it proves is specific and important: that a system can perceive the world, discover mathematical structure within it, reason causally about it, and even generate coherent language output, without training on a single human-authored document, without a gradient descent step across a corpus of unconsented creative work, and without any of the ethical costs that the training paradigm makes unavoidable. The system is implemented entirely in Rust, which matters because Rust's type system and ownership model make it possible to write a verifiable, auditable system whose behavior can be reasoned about from first principles, which is exactly the property that trained neural networks systematically lack. The architecture is organized around five layers: perception, which converts raw input into tensors; symbolic regression, which discovers governing equations from data using genetic programming; physics simulation, which models dynamic systems using differential equation integrators; causal reasoning, which constructs structural causal models and applies do-calculus interventions; and cognition, which ties these together into a perceive-encode-predict-act loop that resembles the structure of conscious engagement with the world without depending on the statistical average of everything ever written about it."
        }
        p {
            "The symbolic regression system is the heart of lmm and the demonstration I want to spend time on, because it is the specific capability that replaces what training does in language models while doing so in a fundamentally different and more honest way. What lmm's symbolic regression does is take a set of data points, which might be measurements of a physical phenomenon, a time series of sensor readings, a sequence of observations from any domain, and search for a symbolic mathematical expression that fits the data. The search is done through genetic programming, which means a population of candidate expressions is evolved over multiple generations, with expressions that fit the data better surviving and those that fit worse being replaced, and the result is a compact symbolic equation that captures the structure of the data in human-readable, verifiable, falsifiable form. The crucial difference from training is what the output represents. When a language model is trained, the output is billions of floating-point parameters whose relationship to the training data is opaque, untraceable, and irreversible, which is why memorization and bias are structural problems rather than configuration bugs. When lmm performs symbolic regression, the output is an equation, something like  "
            code { "(95.09 - cos(x))" }
            " or  "
            code { "(x + (1.002 + x))" }
            ", which is a representation that anyone can read, verify against the data, and reason about mathematically. That transparency is not cosmetic. It is the property that makes the output trustworthy in a way that trained model outputs cannot be, because it exposes the system's reasoning in a form that invites challenge and correction rather than hiding it in a parameter space that is beyond human comprehension."
        }
        p {
            "Let me give you a concrete example from lmm's own documentation, because concrete examples are always more honest than abstract principles. The system includes a command called  "
            code { "encode" }
            " that encodes any text as a symbolic mathematical equation using genetic programming. When you run  "
            code { "lmm encode --text \"The Pharaohs encoded reality in mathematics.\"" }
            ", the system treats the text as a sequence of byte values indexed by position, runs genetic programming to find a symbolic equation  "
            code { "f(x)" }
            " that approximates those byte values, stores the equation along with integer residuals that capture the approximation error, and produces a lossless encoding of the original text in the form of mathematics. The round-trip is verified to be perfect: you can run  "
            code { "lmm decode" }
            " with the equation and residuals and recover the original text exactly. Now I want you to compare that with what a language model does when it \"encodes\" a piece of text. The language model embeds the text in a high-dimensional vector space, represents it as a linear combination of learned basis vectors derived from billions of documents, and produces a geometric point in a space that has no interpretable relationship to either the original text or the physical world. The lmm encoding is transparent and verifiable. The language model embedding is opaque and untraceable. Both are forms of compression, but only one of them is a form of honest compression, compression that shows its work and invites you to verify it. That difference is the entire argument in more concrete form than anything I have written in this paragraph."
        }
        p {
            "The physics simulation layer of lmm demonstrates a different dimension of what training-free intelligence can look like at the level of world modeling. The system includes implementations of several fundamental physical models: a harmonic oscillator governed by Hooke's law, the Lorenz chaotic attractor which produces the famous butterfly-shaped strange attractor from just three coupled differential equations, a nonlinear pendulum, a SIR epidemic model for disease spread, and an N-body gravitational system. Each of these models is not trained. It is formulated mathematically from first principles, implemented as a set of differential equations, and integrated forward in time using numerical methods including the Euler method, standard fourth-order Runge-Kutta, the adaptive RK45 method, and symplectic leapfrog integration for Hamiltonian systems. When you run  "
            code { "lmm physics --model lorenz --steps 500" }
            ", you get the exact trajectory of the Lorenz attractor computed from the differential equations, a trajectory that is entirely determined by the equations and initial conditions, entirely transparent, entirely verifiable, and entirely training-free. A language model asked to predict the trajectory of a Lorenz attractor would produce a plausible-sounding description of chaos theory and might even produce numbers that look roughly right, but those numbers would be interpolations from its training distribution rather than computations from the actual equations, and the difference matters the moment you need to use the numbers for anything that requires them to actually be correct. The lmm approach does not just tell you about the Lorenz attractor. It computes it, which is the fundamental distinction between description and understanding that I have been trying to articulate across all of these posts."
        }
        p {
            "The causal reasoning layer is perhaps the most philosophically significant part of lmm for the argument I am making, because causality is precisely the thing that next-token prediction training cannot learn. There is a well-documented theorem in causal inference that statistical associations, no matter how thoroughly measured, cannot by themselves identify causal relationships, and that causal knowledge requires either controlled experimentation or theoretical commitment to a causal model (^12). What this means for trained language models is that despite their ability to produce fluent text about cause and effect, they do not have access to causal structure in any rigorous sense. They have access to patterns of co-occurrence in text written by humans who had causal understanding, which is not the same thing. The lmm system, by contrast, implements structural causal models with explicit do-calculus intervention support. You can specify a causal graph, ask what happens when you intervene on a variable by setting it to a specific value, and the system computes the downstream effects by propagating the intervention through the causal structure rather than by pattern-matching to previous text about what usually happens. When you run  "
            code { "lmm causal --intervene-node x --intervene-value 10.0" }
            " on a three-node causal model where  "
            code { "y = 2 * x" }
            " and  "
            code { "z = y + 1" }
            ", the system tells you that setting x to 10 causes y to become 20 and z to become 21, and this is not a guess or a plausible extrapolation from training data about causal relationships. It is a computation from an explicitly specified and verifiable causal structure. That is the difference between knowing that something causes something and being able to describe the general concept of causality in confident-sounding sentences."
        }
        p {
            "The text generation capability of lmm deserves special attention because text generation is the specific domain where the comparison to language model training is most stark and most revealing about what the two paradigms are actually doing. The  "
            code { "predict" }
            " command generates text continuation without any training on human-authored corpora. Instead, it runs genetic programming on the context words to discover a trajectory equation describing how word identity changes with position, discovers a rhythm equation describing how word length changes with position, and then uses these equations together with a vocabulary mapping and a syntactic Subject-Verb-Object loop to produce coherent English sentences. The output of  "
            code { "lmm predict --text \"Wise AI built the first LMM\"" }
            " looks like: \"Wise AI built the first LMM in the true law often long time and a open path of an old scope is the solid order.\" That is not grammatically perfect. It is not fluent in the way that GPT-5 outputs are fluent. But it is honest in a way that GPT-5 outputs are not, because every word in that output is traceable to a mathematical computation, to a gene-programmed equation evaluated at a specific position, mapped through a vocabulary structure to a specific word, with no dependence on any human's writing that was absorbed without consent. The fluency of language model outputs is purchased at the price of the ethical costs I have been describing in this entire post. The relative imperfection of lmm's outputs is the cost of honesty, and I find that trade worth making, particularly because it is a proof of concept that can be improved with research investment rather than a fundamental limitation."
        }
        h2 { id: "why-the-mathematical-alternative-is-not-just-a-technical-preference",
            a {
                href: "#why-the-mathematical-alternative-is-not-just-a-technical-preference",
                class: "header",
                "Why the Mathematical Alternative Is Not Just a Technical Preference"
            }
        }
        p {
            "I have been building toward this section across every post I have written about AI, and I want to try to say it as carefully and as precisely as I can, because the argument I am about to make sounds idealistic and I want to separate the idealism from the actual substance. The argument is this: the training paradigm is not just ethically problematic. It is architecturally limited in ways that are not fixable by scaling, and those architectural limitations are what make the ethical costs genuinely wasteful rather than merely unfortunate. If training at scale were producing genuinely intelligent systems that could reason causally about the world, model physical reality, and revise their beliefs based on evidence, the ethical costs would at least be buying something profound. What they are actually buying, as I argued in "
            a { href: "/blogs/llms-are-usefull-lmms-will-break-reality",
                "LLMs are Useful. LMMs will Break Reality"
            }
            ", is a system that can mimic the surface of intelligence without possessing its substance, and that is a terrible return on the human and environmental and creative capital being consumed to produce it. Large Mathematical Models, or what I prefer to call systems grounded in mathematical structure and physical simulation, eliminate training not as an aspirational possibility but as a demonstrated engineering fact, and the lmm project I described in the previous section is the proof. A system can perceive, encode, discover structure, simulate dynamics, reason causally, and generate language without training on a single token of human-authored text, and the resulting system is more transparent, more verifiable, and more respectful of the people it might eventually serve, because its reasoning is made of equations rather than of absorbed human expression."
        }
        p {
            "Let me try to be specific about what \"mathematical grounding\" means in this context, because I want to avoid the vagueness that infects most discussions of AI alternatives. In "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by default"
            }
            ", I described how an equation like the wave equation encodes a mechanism, not a surface description, and how that mechanism allows the equation to generate outputs in any modality that the mechanism is relevant to. The wave equation was not learned from reading descriptions of waves. It was discovered by mathematicians and physicists who formulated it from first principles and then tested it against observations. That process of formulation and testing is fundamentally different from the process of training on descriptions, because it involves contact with reality at every step. When a system is built around discovering such structures from data rather than predicting text about such structures, the output of the system is a compact mathematical representation that can be verified against new observations, falsified if wrong, and refined based on evidence. The lmm system does exactly this: when given data from a linear process, its genetic programming engine discovers something like "
            code { "(x + (1.002 + x))" }
            ", which is an approximation of the underlying "
            code { "2x + 1" }
            " law, and that discovery is not borrowed from any human's writing about linear functions. It is inferred from the data by an algorithm that knows only how to evaluate mathematical expressions, not how to pattern-match to descriptions of mathematical expressions. That distinction between inferring and pattern-matching, between real discovery and sophisticated imitation of discovery, is the architectural distinction that the entire moral argument rests on."
        }
        p {
            "The question of what LMMs eliminate is worth stating as precisely as the project itself states it. What the lmm project eliminates is the specific dependency on consuming human creative expression at scale as the primary input to intelligence. The system learns from physical observations, from data measurements, from mathematical relationships inferred by symbolic regression, and those inputs are fundamentally different in their ethical character from the unconsented creative work that language model training consumes. Measurements of a pendulum's trajectory, readings from an epidemic simulation, positions of N bodies in a gravitational field, these are not the creative labor of specific individuals who were not asked for permission. They are data about how the physical world behaves, and learning about the physical world from physical observations is the oldest and most honest form of inquiry that humans have ever practiced. It is called science, and science has an established ethical framework for data collection that includes consent, anonymization, review, and attribution precisely because those practices matter. The lmm approach to intelligence is closer to the scientific framework than to the extraction framework of language model training, and that proximity is not accidental. It reflects a deliberate design choice to build intelligence that is accountable to physical reality rather than accountable only to the statistical distribution of human text."
        }
        p {
            "I want to address the obvious objection here, which is that lmm is currently less capable than GPT-5 in the domains where GPT-5 is most used, and that the capability gap is large enough to make the ethical argument feel like a luxury concern. That objection is partly true and I want to be honest about it. The lmm system produces text that is less fluent, answers questions with less apparent breadth, and handles the wide variety of natural language tasks that users expect from AI assistants with much less apparent smoothness than a trained language model does. I acknowledge all of that, and I am not going to pretend that a proof of concept is a production system. But I want to push back on the hidden premise that greater capability automatically justifies greater ethical cost, because that premise leads to an infinitely regressing justification: whatever capability the current paradigm produces can always be used to justify the costs that produced it, regardless of what those costs actually are. The lmm system demonstrates that capability without training is possible in principle. The question of how to scale that principle, how to extend it to broader domains, how to make it competitive with trained systems in the domains that matter most to people, is a research question that has not been seriously funded or staffed. It is not a question that has been answered and failed. It is a question that the field has mostly chosen not to ask, because asking it seriously would require confronting the possibility that the training paradigm is not necessary, and confronting that possibility is uncomfortable for everyone who has invested their careers in developing it."
        }
        p {
            "I also want to connect this to the argument from "
            a { href: "/blogs/llms-destroyed-the-internet-lmms-will-make-it-alive",
                "LLMs destroyed the Internet. LMMs will make it alive."
            }
            " about what the internet loses when it becomes populated by systems trained on human expression rather than grounded in physical reality. The lmm project produces outputs that carry a different kind of evidence in them, evidence not of statistical averaging over human creative work but of mathematical computation over physical structure. When the system encodes a text as a symbolic equation and then decodes it back perfectly, the equation is a real discovery, a real mathematical object that captures something true about the byte-level structure of that particular text. When the system simulates a Lorenz attractor and reports the trajectory, that trajectory is computed from the actual equations that govern chaotic dynamical systems, not approximated from patterns in physics textbooks. That relationship between output and reality is the relationship that the early web's best content had, the relationship of genuine engagement with actual problems rather than sophisticated imitation of such engagement. Building intelligence from equations rather than from text is building intelligence that can restore that relationship, one mathematical discovery at a time, and the restoration is not just philosophical. It is architectural, implemented deliberately, and demonstrably achievable with the technology that exists right now."
        }
        p {
            "Let me also say what I think is genuinely hard about the transition I am describing, because I do not want to be the person who criticizes the current paradigm without acknowledging the difficulty of replacing it. Mathematical modeling of complex phenomena is genuinely more difficult than statistical imitation of text. Symbolic regression and physics-informed machine learning are active research areas with genuine open problems. The domains where mathematical grounding works most naturally are the domains where we already have good physical theories, and there are enormous domains of human experience and practical importance where we do not have those theories. Language understanding itself, which is perhaps the most practically important domain for AI, does not have a clean mathematical theory in the same sense that fluid dynamics or electromagnetism does, and it is not obvious that it ever will. I acknowledge all of this. The lmm project's current text generation produces output that is less fluent than language models, because fluency in the sense of smooth-sounding prose is a property of statistical averaging over human writing, and removing the averaging removes some of the smoothness. The argument I am making is not that the mathematical path abolishes that difficulty. The argument is that the moral costs of the training paradigm are real and serious enough to justify serious investment in alternatives, even difficult ones, and that the current allocation of research effort and capital, overwhelmingly toward scaling the training paradigm rather than toward alternatives, is not justified by necessity but by inertia, and inertia is not a moral defense."
        }
        h2 { id: "the-consent-problem-is-not-solved-by-better-licensing",
            a {
                href: "#the-consent-problem-is-not-solved-by-better-licensing",
                class: "header",
                "The Consent Problem Is Not Solved by Better Licensing"
            }
        }
        p {
            "I want to spend a section on the consent problem specifically, because it is the part of the training ethics debate that I think receives the most inadequate treatment, both from the industry and from most critics. The industry's response to consent concerns has typically been one of two things: either arguing that training data use falls under fair use or other existing legal exemptions, which is a legal argument rather than a moral one and tells us nothing about whether the practice is right rather than merely legal; or proposing licensing frameworks that would allow creators to opt in or opt out of having their work used for training, which treats consent as a commercial transaction rather than a moral foundation. Both responses miss the point in the same way, which is that they treat the consent question as a problem to be managed rather than a principle to be respected, and the difference between management and respect is the difference between compliance and ethics. The lmm approach sidesteps this entire problem not by solving the consent question within the training paradigm but by eliminating the dependency on human creative expression that makes the consent question arise in the first place."
        }
        p {
            "Consent as a moral principle is not primarily a contractual matter. It is the recognition that persons are not means to others' ends but ends in themselves, which is the foundation of every serious framework of human rights and dignity that has been developed in the post-Enlightenment tradition. When an organization trains a model on creative work produced by a person, it is treating that person's creative labor as raw material for a process that the organization controls and benefits from, and the person is reduced to the role of an input rather than recognized as an agent who gets to decide whether they want their work to serve this particular purpose. That reduction is wrong even when the creative work is technically accessible, even when the training does not copy the work verbatim, and even when the output system produces content that does not obviously resemble the specific person's work. The wrongness does not require legal infringement to be real. It requires only the structural treatment of a person's creative expression as a resource to be consumed rather than a contribution to be respected. A system like lmm that learns from physical measurements rather than from creative expression does not face this structural problem, because physical measurements are not the creative contributions of specific individuals who have rights and interests in how they are used. The SIR epidemic model integrated by a Runge-Kutta solver is not the intellectual property of a specific writer who did not consent to its use. It is a mathematical description of a biological mechanism, and learning from that description is learning from the world rather than from the people who wrote about the world, and that moral distinction is the entire point."
        }
        p {
            "I described in "
            a { href: "/blogs/an-empty-life-filled-with-constant-suffering",
                "An Empty Life Filled With Constant Suffering"
            }
            " what it feels like to have the contributions you make not acknowledged, to put genuine effort into something and find that the effort disappears without leaving any trace on the world. I know that feeling in a personal way, and I think it is close enough to what creators experience when their work is consumed and transformed without acknowledgment that I am willing to use my own experience as evidence of the moral stakes. When a writer produces a body of work over years, each piece is an expression of something particular and personal, a way of engaging with the world that is irreducibly theirs. When that body of work is ingested into a training pipeline without the writer's knowledge and used to build a system that can then produce similar-sounding text on demand at a cost that makes the writer's own production economically non-viable, something real and important has been taken. It is not merely the market value that has been taken, although that too. It is the recognition that the work was the expression of a specific person with a specific life getting specific things from their engagement with specific ideas, and that recognition is what the training paradigm systematically fails to provide. That failure of recognition is the moral failure that licensing frameworks cannot repair, because recognition is not a contractual matter. It is a matter of how you conceptualize the people whose work you are using, and the training paradigm's conceptualization is one of resources rather than persons. Building intelligence from equations rather than from expression is one way to build a system that does not need to fail at recognition, because it does not depend on human expression in the first place."
        }
        p {
            "The consent problem also extends to the outputs of trained systems in ways that the licensing discussion has not fully addressed. When I use a language model and it produces text, I am often unable to know whether the text reflects patterns absorbed from specific sources in its training data or a genuine synthesis of diverse influences, and that unknowability is itself a violation of the contextual norms that govern honest communication. In any other context, producing text that closely resembles another person's work without acknowledgment would be considered plagiarism, and plagiarism is wrong not primarily because it is illegal but because it misrepresents the authorship and provenance of the work. The training paradigm creates a system that can produce such resemblances at scale, systematically, without any mechanism for tracking or acknowledging the specific sources of the patterns it is reproducing, and then positions the output as the product of the AI system. That misrepresentation is not incidental. The UNESCO Recommendation on the Ethics of AI specifically emphasizes transparency as a fundamental principle, including transparency about the origins and processes that produce AI outputs (^10). Training as currently practiced cannot satisfy that principle. An lmm output, by contrast, is always traceable to its mathematical source: any sentence generated by the  "
            code { "predict" }
            " command can be traced to the trajectory equation, the rhythm equation, the vocabulary mapping, and the positional rules that produced it, and none of those sources are anyone's intellectual property because none of them are anyone's creative expression."
        }
        p {
            "I want to be honest about one more dimension of the consent problem that I have not yet addressed, which is the consent of future people rather than only current and past creators. The training corpora used for large language models typically include a large proportion of text produced by people who are no longer alive, text from historical figures, from classical authors, from early internet users who could not have imagined the use to which their words would be put. The dead cannot give or withhold consent in any active sense, and the rights that govern posthumous use of creative work vary enormously across jurisdictions and traditions. But the use of historical human expression to train systems that then shape the information environment of living people is not morally neutral simply because the original authors are not present to object. Their expressions were produced in specific contexts, for specific purposes, and with specific expectations about the contexts in which they would be received and used, and using them to train statistical pattern matchers that then influence how a billion people understand the world is a transformation of context so radical that the original authors could not have contemplated it, let alone consented to it. The moral principle is the same one I cited from contextual integrity, that information flows appropriately when they match the norms of the context in which the information was originally produced (^6). The lmm approach to intelligence avoids this entire historical dimension of the consent problem, because a system that learns from the trajectory of a pendulum or the spread of an epidemic is not appropriating the creative expression of any historical person. It is reading the book of nature rather than the books of the dead, and there is a fundamental moral difference between those two reading practices."
        }
        h2 { id: "what-memorization-reveals-about-the-training-paradigms-soul",
            a {
                href: "#what-memorization-reveals-about-the-training-paradigms-soul",
                class: "header",
                "What Memorization Reveals About the Training Paradigm's Soul"
            }
        }
        p {
            "I want to spend some time on memorization specifically, because I think it is the most revealing pathology of the training paradigm, the symptom that most directly shows what the system fundamentally is as opposed to what it is claimed to be. I mentioned memorization earlier in a legal and privacy context, but I want to go deeper here because memorization is not just an embarrassing bug. It is a window into the mechanics of what training actually produces and what the relationship is between the training data and the trained model. If training were truly a process of abstraction and learning, as the word \"training\" implies, we would expect the relationship between the training data and the model's outputs to be indirect and transformed, the way a student who has studied many books can discuss ideas from those books in their own words without being able to reproduce the books verbatim. The empirical fact that trained models can reproduce verbatim fragments of training data is evidence that what training actually produces is not pure abstraction but something closer to compressed storage with pattern matching on top, and that evidence is directly relevant to the moral case because it reveals that the relationship between the training data and the model is more extractive than transformative. The lmm encode-decode cycle is instructive by contrast: when you encode text and then decode it, you are explicitly performing lossless compression and recovery through mathematical structure, and the system does not pretend otherwise. It tells you exactly what equation it found, exactly what the residuals are, and exactly what the round-trip recovery looks like. That transparency is not a feature added for PR reasons. It is the natural state of a system that is built from equations rather than from absorbing human expression without acknowledgment."
        }
        p {
            "The research on memorization in trained models is worth engaging with carefully. A study specifically examining non-adversarial reproduction, meaning reproduction that occurs during normal model use rather than through deliberate extraction attacks, found that significant fractions of a model's outputs can match internet content verbatim when the prompts are similar to content that appeared in the training data (^5). This is not a theoretical possibility. It is a documented empirical reality that occurs in ordinary use of current deployed models. The implications are striking. If you ask a language model to explain a concept and the model happens to have seen a good explanation of that concept during training, the model may reproduce that explanation or large fragments of it, without attributing the source, without the user knowing that they are reading someone else's words, and without the original author having consented to this use. The user believes they are receiving AI-generated synthesis. They may in fact be receiving a fragment of a specific human being's writing, laundered through a statistical process that removed the attribution while keeping enough of the content to be legally and morally questionable. That relationship between inputs and outputs is not the relationship that is advertised when AI systems are presented as creative and generative. It is the relationship of sophisticated storage and retrieval that happens to be opaque enough to escape the moral frameworks that govern explicit copying. An lmm system cannot memorize and reproduce your blog post without your consent because it does not ingest your blog post in the first place. It ingests the physical world, and the physical world belongs to no one and to everyone equally."
        }
        p {
            "The connection to what I argued in "
            a { href: "/blogs/llms-destroyed-the-internet-lmms-will-make-it-alive",
                "LLMs destroyed the Internet. LMMs will make it alive."
            }
            " about the loss of the web's authenticity is direct and important. One of the things that made the early web valuable was that its content was traceable, which meant that the provenance of information was in principle recoverable. If you found a forum post that solved your problem, you could see who wrote it and when. If you found an article that made an extraordinary claim, you could trace the claim to its source and evaluate whether the source was reliable. The training paradigm systematically destroys traceability by creating systems that absorb attributable information and produce unattributable output, thereby breaking the provenance chain that made honest information exchange possible. The web that is increasingly populated by model outputs trained on unconsented human expression is less alive partly because its content has been detached from the specific human experiences and identities that made it meaningful and trustworthy in the first place. Memorization makes this visible in an extreme form, but it is the same process that occurs whenever training absorbs human expression and recombines it into outputs that appear to be generated fresh while actually being derived from specific human contributions that went unacknowledged. The lmm approach to the web would look different: instead of outputs that might or might not be reproducing someone's work in a way that nobody can trace, you would have outputs that are demonstrably mathematical, demonstrably grounded in physical structure, and demonstrably not derived from anyone's creative expression, because the derivation chain is entirely transparent and consists entirely of equations."
        }
        h2 { id: "the-scale-argument-is-a-moral-red-herring",
            a {
                href: "#the-scale-argument-is-a-moral-red-herring",
                class: "header",
                "The Scale Argument Is a Moral Red Herring"
            }
        }
        p {
            "The most sophisticated defense of the training paradigm that I encounter is what I think of as the scale argument, and it goes roughly like this: yes, there are ethical concerns about training, but the scale of the benefits produced justifies the costs, because the systems trained on massive corpora can help billions of people in ways that no alternative approach could currently match, and the aggregate benefit to humanity is large enough to outweigh the costs to the specific individuals whose work was consumed without consent. This is a consequentialist argument, and consequentialist arguments have real force, and I want to engage with it seriously rather than dismissing it, because dismissing strong arguments is the habit of people who are more interested in being right than in being honest. But the scale argument assumes something that the existence of lmm directly challenges: it assumes that training-based capability is the only path to AI utility. A system that can simulate epidemics, discover governing equations from data, reason causally about interventions, and generate coherent language output without training is a counterexample to that assumption, and counterexamples matter even when they are imperfect, because an imperfect counterexample refutes the claim of impossibility."
        }
        p {
            "The first problem with the scale argument is empirical. It assumes that the aggregate benefit of current AI systems is large and clearly positive, and that assumption is much less secure than it sounds when stated confidently. The benefits are real in some domains, drug discovery research, programming assistance, language translation, accessibility tools, and I do not deny them. But the costs are also real and substantial, and the empirical measurement of net benefit across an entire society is an extraordinarily difficult problem that nobody has solved. The evidence I cited in "
            a { href: "/blogs/llms-destroyed-the-internet-lmms-will-make-it-alive",
                "LLMs destroyed the Internet. LMMs will make it alive."
            }
            " about the degradation of the web's information quality represents a broad diffuse harm that is very difficult to quantify but is clearly large in scale. The economic displacement of creative workers represents a concentrated harm to a specific class of people that is also difficult to quantify but is clearly real and ongoing. The bias harms documented in research fall heaviest on already-marginalized communities and represent systematic disadvantage that compounds over time. The privacy violations from memorization potentially affect anyone whose data was absorbed. The environmental costs are global and transgenerational. The scale argument needs to show that the benefits outweigh all of these costs summed together, and making that case requires an honest engagement with the full cost ledger that the AI industry has consistently refused to produce."
        }
        p {
            "The second problem with the scale argument is philosophical. Even if we could establish that the aggregate benefits outweigh the aggregate costs, which I doubt and which nobody has demonstrated, the consequentialist reasoning fails to respect the separateness of persons, which is one of the most fundamental insights of serious moral philosophy. The fact that a large aggregate benefit exists does not justify taking something from a specific person without their consent, because the person is not the aggregate. The writer whose work was consumed without permission is not made whole by the observation that the model has benefited many people, because they are a separate individual with their own interests and rights that cannot be traded away to produce benefits for others without their participation in the exchange. This is the insight that rights-based frameworks in ethics are designed to protect, and it is the insight that consequentialist arguments in favor of the training paradigm consistently violate. UNESCO's ethics framework is explicitly rights-based rather than purely consequentialist for exactly this reason, maintaining that fundamental human rights cannot be overridden by aggregate calculations of benefit however large the calculation appears (^10). The training paradigm, as currently justified by its practitioners, routinely overrides the rights of specific individuals in favor of aggregate benefit claims, and that is a moral framework that has historically been used to justify a very wide range of abuses, and I am not comfortable with it."
        }
        p {
            "The third problem with the scale argument is that it is not static, and the lmm project makes this visible in a way that pure theory cannot. The people who use the scale argument are implicitly claiming that the current paradigm, with its documented costs, is the only path to the documented benefits. But lmm demonstrates that at least some of those benefits, equation discovery, physics simulation, causal reasoning, language generation, are achievable without training on human-authored text. The claim that training-based capability cannot be achieved through alternative means has not been seriously tested, because the field has been so strongly oriented toward scaling the training paradigm that alternatives have been chronically underfunded and understaffed. If a substantial fraction of the resources currently invested in training larger and larger language models on more and more unconsented data were redirected toward developing systems like lmm, toward genetic programming for equation discovery, toward physics-informed modeling, toward explainable causal inference, we do not actually know what the resulting capability would look like after five or ten years of sustained investment. The moral case for the training paradigm has borrowed its strength from a counterfactual that the field has not been willing to seriously invest in testing, and that is not a robust moral foundation."
        }
        h2 { id: "what-we-lose-when-we-stop-asking-why-and-what-we-gain-when-we-start",
            a {
                href: "#what-we-lose-when-we-stop-asking-why-and-what-we-gain-when-we-start",
                class: "header",
                "What We Lose When We Stop Asking Why, and What We Gain When We Start"
            }
        }
        p {
            "I want to close with the argument that I think is the most fundamental, the argument that is not primarily about rights or bias or privacy or economics but about something I can only call the soul of an inquiry. One of the most distinctive things about the training paradigm is that it is explicitly designed not to explain itself. The goal of training is to produce a model that generates good outputs, and the measure of goodness is the training objective, and as long as the training objective is satisfied, the internal mechanisms of the model are not required to be interpretable, causal, or grounded in any theory of the phenomenon being modeled. This is not an accidental property. It is the deliberate design choice of a paradigm that prioritizes empirical performance over theoretical understanding, and that choice has philosophical consequences that go beyond efficiency or interpretability in the narrow technical sense. When we build systems that work without understanding why they work, we are betting our technological future on a black box, and black box bets are only reliable within the envelope of the training distribution. Outside that envelope, the system has no principled way to recognize that it is outside its reliable range, and so it generates plausible-sounding outputs regardless of whether those outputs are trustworthy. The lmm system is the opposite: every output traces back to an equation, every equation can be evaluated for fit against the data it was discovered from, every simulation can be verified against known physical behavior, and every causal inference can be checked against the explicit causal graph it was derived from. That is not a black box. That is a system that answers the question \"why\" in the only way that deserves the name of an answer, by showing the mathematical mechanism that produced the output."
        }
        p {
            "The contrast with mathematical discovery is profound and worth dwelling on. When mathematicians and physicists discover the laws that govern physical phenomena, they are not just producing accurate predictions. They are producing understanding that can be extended, generalized, and applied to situations that were never in the training dataset, because the understanding is expressed in terms of mechanisms rather than patterns, and mechanisms can be reasoned about in ways that patterns cannot. Newton's laws were not learned from a dataset of planetary observations in the sense that a neural network is trained on data. They were formulated as structural relationships that could be derived from more fundamental principles and extended to phenomena that Newton never observed. That capacity for principled extension beyond the training distribution is what gives theoretical understanding its power, and it is what the training paradigm systematically sacrifices in favor of empirical performance within the training distribution. The lmm system embodies the opposite design philosophy: its symbolic regression engine does not memorize data points. It searches for an equation that captures the structure beneath the data points, and that equation, once found, can in principle be extended to new data points that were never part of the discovery process. That is generalization in the true sense of the word, not interpolation within a training distribution, but principled extension from discovered structure to new observations. I argued in "
            a { href: "/blogs/mathematical-equations-are-multimodal-by-default",
                "Mathematical Equations are Multimodal by default"
            }
            " that equations encode mechanisms rather than surfaces, and the lmm project is the implementation of that argument in executable Rust code."
        }
        p {
            "I also want to connect this to something in the personal posts, particularly "
            a { href: "/blogs/an-empty-life-filled-with-constant-suffering",
                "An Empty Life Filled With Constant Suffering"
            }
            ", where I talked about how hollow it feels to produce things that are effective without being meaningful, to do the right thing technically while something deeper is missing. I think there is an analogy to what I am describing about the training paradigm. A system that works without understanding why it works has a certain hollowness to it, a certain mechanical efficiency that is not the same as genuine comprehension, and building civilization on top of systems that are efficient but not comprehending is a bet that depends entirely on the distribution of our future problems staying close to the distribution of past data. If the future contains novel challenges, and it always does, then the systems we have built on pattern matching without understanding will be the wrong tools, and the cost of having built them, the moral cost in extracted labor and eroded rights and degraded information ecosystems, will have been paid for something that was not adequate to the moment when it mattered most. The lmm system is my answer to that hollow feeling, an attempt to build something whose mechanisms I can actually see and reason about and extend, something whose relationship to the world is direct and mathematical rather than indirect and statistical, something that asks for nothing from human creators because it is busy learning from the physical world instead. It is not finished. It is not production-ready. It is the beginning of a direction, and a direction is what matters when the current position is wrong."
        }
        p {
            "There is an important caveat I want to be honest about, which is that lmm currently learns from physical data and mathematical structure, but the domains where it can function are much narrower than the domains where trained language models are used. The simulation and equation discovery capabilities are genuinely training-free, but extending them to the full range of human knowledge and practical need is a research program that will take years of serious investment. I am not claiming that lmm solves the problem completely. I am claiming that it proves the problem is solvable, that intelligence without training on human creative expression is not a contradiction in terms but an achievable engineering target, and that the existence of a working proof of concept changes the moral conversation about training from \"it is unfortunately necessary\" to \"it is a choice, and the choice can be made differently\". That shift in framing is small, but it is important, because necessary evils and unnecessary evils require different responses. A harm that is truly unavoidable calls for mitigation and management, which is what most AI ethics frameworks try to provide. A harm that is avoidable but chosen calls for something stronger, which is refusal, and the existence of lmm is my attempt to make that refusal concrete rather than merely rhetorical. I am building the alternative because I believe that building something is more honest than only arguing for it."
        }
        p {
            "I do not know where the transition from the training paradigm to something better will come from or how fast it will arrive. I am building what I can in "
            a { href: "https://github.com/wiseaidotdev/lmm", "lmm" }
            ", and I am writing what I believe in these posts, and I am watching the research that points in the right direction with more hope than I usually admit to. But I am not optimistic that the transition will happen quickly or that it will happen for the right reasons rather than because the training paradigm eventually hits limitations that force the field to look elsewhere. What I do know is that the arguments I have been making across these posts are not arguments for inaction or for despair. They are arguments for a specific direction, toward intelligence grounded in the physical world rather than in the statistical surface of human expression, toward systems that can be verified rather than systems that can only be trusted, toward a relationship between AI and human knowledge that is built on recognition and respect rather than on extraction and consumption. That direction is harder. It requires more intellectual honesty. It requires admitting that some of the most impressive systems ever built are built on a foundation that is morally questionable and architecturally limited. I think admitting hard things is the only way forward that I can respect, and so I am admitting them here, and building what I can alongside the admission, for whatever both are worth."
        }
        p { "Till next time 👋!" }
        h2 { id: "references",
            a { href: "#references", class: "header", "References" }
        }
        p {
            "(^1): Grynbaum, M. & Mac, R., "
            em { "The Times Sues OpenAI and Microsoft Over A.I. Use of Copyrighted Work" }
            ", "
            a { href: "https://www.nytimes.com/2023/12/27/business/media/new-york-times-open-ai-microsoft-lawsuit.html",
                "New York Times, 2023"
            }
        }
        p {
            "(^2): U.S. Copyright Office, "
            em { "Copyright and Artificial Intelligence, Part 3: Generative AI Training" }
            ", "
            a { href: "https://www.copyright.gov/ai/Copyright-and-Artificial-Intelligence-Part-3-Generative-AI-Training-Report-Pre-Publication-Version.pdf",
                "copyright.gov, 2025"
            }
        }
        p {
            "(^3): European Parliament, "
            em { "EU AI Act" }
            ", "
            a { href: "https://www.europarl.europa.eu/topics/en/article/20230601STO93804/eu-ai-act-first-regulation-on-artificial-intelligence",
                "europarl.europa.eu, 2024"
            }
        }
        p {
            "(^4): Gallegos, I. et al., "
            em { "Bias and Fairness in Large Language Models: A Survey" }
            ", "
            a { href: "https://arxiv.org/abs/2309.00770", "arXiv:2309.00770" }
        }
        p {
            "(^5): Ippolito, D. et al., "
            em { "Preventing Verbatim Memorization in Language Models Gives a False Sense of Privacy" }
            ", "
            a { href: "https://arxiv.org/abs/2210.17546", "arXiv:2210.17546" }
        }
        p {
            "(^6): Nissenbaum, H., "
            em { "Privacy as Contextual Integrity" }
            ", "
            a { href: "https://digitalcommons.law.uw.edu/wlr/vol79/iss1/10/",
                "Washington Law Review, 2004"
            }
        }
        p {
            "(^7): Strubell, E. et al., "
            em { "Energy and Policy Considerations for Deep Learning in NLP" }
            ", "
            a { href: "https://arxiv.org/abs/1906.02629", "arXiv:1906.02629" }
        }
        p {
            "(^8): Hoffmann, J. et al., "
            em { "Training Compute-Optimal Large Language Models" }
            ", "
            a { href: "https://arxiv.org/abs/2203.15556", "arXiv:2203.15556" }
        }
        p {
            "(^9): Perrigo, B., "
            em { "Exclusive: The $2 Per Hour Workers Who Made ChatGPT Safer" }
            ", "
            a { href: "https://time.com/6247678/openai-chatgpt-kenya-workers/", "TIME Magazine, 2023" }
        }
        p {
            "(^10): UNESCO, "
            em { "Recommendation on the Ethics of Artificial Intelligence" }
            ", "
            a { href: "https://www.unesco.org/en/artificial-intelligence/recommendation-ethics",
                "UNESCO, 2021"
            }
        }
        p {
            "(^11): See "
            a { href: "/blogs/language-is-limited-asi-is-impossible",
                "Language is Limited. ASI is Impossible."
            }
        }
        p {
            "(^12): Pearl, J. & Mackenzie, D., "
            em { "The Book of Why: The New Science of Cause and Effect" }
            ", "
            a { href: "https://www.basicbooks.com/titles/judea-pearl/the-book-of-why/9780465097609/",
                "Basic Books, 2018"
            }
        }
    }
}

use dioxus::prelude::*;
use crate::components::blog::code::CodeBlock;
