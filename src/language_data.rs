use fxhash::FxHashMap;
use indexmap::IndexMap;
use anyhow::Result;

use std::fs::File;
use std::io::prelude::*;
use serde::Deserialize;
use serde_json;

pub type CharacterData = smallmap::Map<char, f64>;
pub type BigramData = FxHashMap<[char; 2], f64>;
pub type TrigramData = Vec<([char; 3], f64)>;

#[derive(Deserialize)]
struct LanguageDataInter {
	pub language: String,
	pub characters: FxHashMap<char, f64>,
	pub bigrams: FxHashMap<String, f64>,
	pub skipgrams: FxHashMap<String, f64>,
	pub trigrams: IndexMap<String, f64>,
}

impl LanguageDataInter {
	fn get_char_data(&self, data: &FxHashMap<char, f64>) -> CharacterData {
		let mut res = CharacterData::new();
		for (c, f) in data.into_iter() {
			res.insert(*c, *f);
		}
		res
	}

	fn get_bigram_data(&self, data: &FxHashMap<String, f64>) -> BigramData {
		let mut res = BigramData::default();
		for (bigram, freq) in data {
			let bv = bigram.chars().collect::<Vec<char>>();

			let new_bigram = [bv[0], bv[1]];
			res.insert(new_bigram, *freq);
		}
		res
	}

	fn get_trigram_data(&self, data: &IndexMap<String, f64>) -> TrigramData {
		let mut res = TrigramData::new();
		for (trigram, freq) in data {
			let tv = trigram.chars().collect::<Vec<char>>();

			if tv[0] != tv[1] && tv[1] != tv[2] {
				let new_trigram = [tv[0], tv[1], tv[2]];
				res.push((new_trigram, *freq));
			}
		}
		res
	}
}

pub struct LanguageData {
	pub characters: CharacterData,
	pub bigrams: BigramData,
	pub skipgrams: BigramData,
	pub trigrams: TrigramData,
	pub language: String
}

impl From<LanguageDataInter> for LanguageData {
	fn from(inter: LanguageDataInter) -> Self {
		let characters = inter.get_char_data(&inter.characters);
		let bigrams = inter.get_bigram_data(&inter.bigrams);
		let skipgrams = inter.get_bigram_data(&inter.skipgrams);
		let trigrams = inter.get_trigram_data(&inter.trigrams);

		Self {
			characters, bigrams, skipgrams, trigrams, language: inter.language,
		}
	}
}

impl LanguageData {
	pub fn new(language: &str) -> Result<LanguageData> {
		let res = LanguageData::read_language_data(String::from(language))?;
		println!("language read: {}", res.language);
		Ok(res)
	}

	fn read_language_data(language: String) -> Result<LanguageData> {
		let file_path = format!("static/language_data/{}.json", language.to_lowercase());
		let mut file = File::open(file_path)?;
		let mut contents = String::new();

		file.read_to_string(&mut contents)?;
		let data: LanguageDataInter = serde_json::from_str(contents.as_str())?;
		Ok(LanguageData::from(data))
	}
}