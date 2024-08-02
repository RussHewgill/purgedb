//

pub struct Keywords {
    // pub names: Vec<(u32, String)>,
    // pub colors: Vec<(u32, String)>,
}

impl Keywords {
    pub fn new(names: Vec<(u32, String)>, colors: Vec<(u32, String)>) -> Self {
        Self {
            // matcher: SkimMatcherV2::default(),
            // names,
            // colors,
        }
    }
}

#[cfg(feature = "nope")]
mod prev {
    use fuzzy_matcher::{skim::SkimMatcherV2, FuzzyMatcher};

    pub struct Keywords {
        matcher: SkimMatcherV2,
        pub names: Vec<(u32, String)>,
        pub colors: Vec<(u32, String)>,
    }

    impl Keywords {
        pub fn new(names: Vec<(u32, String)>, colors: Vec<(u32, String)>) -> Self {
            Self {
                matcher: SkimMatcherV2::default(),
                names,
                colors,
            }
        }

        pub fn search_names(&self, query: &str) -> Option<Vec<(i64, (u32, String))>> {
            let mut names = self.names.clone();

            // names.sort_by_cached_key(|&(_,n)| self.matcher.fuzzy_match(&n, query));

            let mut names: Vec<(i64, (u32, String))> = names
                .into_iter()
                .map(|(id, n)| (self.matcher.fuzzy_match(&n, query), (id, n)))
                .flat_map(|(s, n)| match s {
                    Some(s) => Some((s, n)),
                    None => None,
                })
                .collect();

            names.sort_by_key(|(s, _)| *s);

            if names.len() > 0 {
                Some(names)
            } else {
                None
            }
            // &self.names
        }
    }
}
