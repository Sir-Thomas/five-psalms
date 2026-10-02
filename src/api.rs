#![allow(unused)]
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
struct Verse {
    chapter: usize,
    verse: usize,
    name: String,
    text: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Psalm {
    // TODO: Create types for all these
    translation: String,
    abbreviation: String,
    lang: String,
    language: String,
    direction: String,
    encoding: String,
    book_nr: usize,
    book_name: String,
    chapter: usize,
    name: String,
    verses: Vec<Verse>,
}

impl Psalm {
    fn format(&self) -> String {
        self.verses.iter().map(|verse| verse.text.clone()).collect()
    }
}

pub struct GetBibleApi {
    url: &'static str,
    api_version: usize,
}

impl GetBibleApi {
    pub fn new() -> Self {
        Self {
            url: "https://api.getbible.net",
            api_version: 2,
        }
    }

    fn format_url(&self, version: &str, psalm: i8) -> String {
        format!(
            "{}/v{}/{}/19/{}.json",
            self.url, self.api_version, version, psalm
        )
    }

    pub async fn get_psalm(&self, version: &str, psalm: i8) -> String {
        let psalm = self.fetch_json(version, psalm).await;
        psalm.format()
    }

    async fn fetch_json(&self, version: &str, psalm: i8) -> Psalm {
        Request::get(&self.format_url(version, psalm))
            .send()
            .await
            .unwrap()
            .json::<Psalm>()
            .await
            .unwrap()
    }
}
