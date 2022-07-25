use regex::Regex;

#[derive(Debug)]
pub struct Word {
    pub content: String,
    pub morphemes: Vec<String>,
}

impl From<String> for Word {
    fn from(content: String) -> Self {
        let morphemes_pattern = Regex::new(r"([ptkbdgfscljwhmn]?[aiueo]n?)").unwrap();
        let morphemes_captures = morphemes_pattern.captures_iter(&content);
        let mut morphemes: Vec<String> = Vec::new();

        for capture in morphemes_captures {
            morphemes.push(capture.get(0).unwrap().as_str().to_string());
        }

        Word { content, morphemes }
    }
}

pub fn gloss_word(word: &Word) -> String {
    let json: ureq::serde_json::Value = ureq::get("https://go-kawaba.github.io/diomawe/data.json")
        .call()
        .expect("Could not retrieve JSON from diomawe")
        .into_json()
        .expect("Could not parse JSON from diomawe");

    if !word.morphemes.is_empty() {
        let mut string_vec = Vec::new();

        for morpheme in &word.morphemes {
            string_vec.push(json["baja"][morpheme]["Gloss"].to_string().replace("\"", ""));
        }

        let string = string_vec.join("-");

        string
    } else {
        String::from("invalid gloss")
    }
}
