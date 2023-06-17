use std::{io::*, fs::File};
// note that bufread::MultiBzDecoder is _distinct_ from read::MultiBzDecoder
use bzip2::bufread::*;

use crate::wiktionary_api_path;

// i don't like that there are multiple result types
// that seems Bad
// also having to explicitly box dyn Error sucks, fine fuck you it's the rust way
type Lookup = std::result::Result<Option<String>, Box<dyn std::error::Error>>;

// WHY can you not implement traits on external types, like what??
// fortunately we needed to copy-paste the parse_wiki_text library to fix some bugs anyhow
pub fn lookup(word: &str) -> Lookup {
    if let Ok(file) = File::open(crate::index_path) {
        return lookup_local(word, file);
    } else {
        return lookup_online(word);
    }
}

fn lookup_local(word: &str, file: File) -> Lookup {
    let reader = BufReader::new(MultiBzDecoder::new(BufReader::new(file)));
    for line in reader.lines() {
        let line = line?;

        // format: file-offset:page-id:page-title
        let line = line.splitn(3, ":").collect::<Vec<&str>>();
        assert!(line.len() == 3, "Failed to parse line. Is your index file valid?");

        let offset = line.get(0).unwrap().parse::<u64>()?;
        let id = line.get(1).unwrap().parse::<u64>()?;
        let title = *line.get(2).unwrap(); // this dereference now makes sense

        if title == word {
            let file = File::open(crate::dictionary_path)?;
            let mut reader = BufReader::new(file);

            // note: our chunk contains multiple pages
            let offset = reader.seek(SeekFrom::Start(offset))?;
            let reader = BufReader::new(BzDecoder::new(reader));

            let mut buffer = String::new();
            let mut page = false;
            for line in reader.lines() {
                let line = line.unwrap();
                if line == format!("    <title>{}</title>", title) {
                    buffer.push_str("  <page>");
                    buffer.push_str("\n");
                    page = true;
                }
                if page {
                    buffer.push_str(&line);
                    buffer.push_str("\n");
                    if line == "  </page>" {
                        break;
                    }
                }
            }
            return Ok(Some(buffer));
        }
    }
    return Ok(None);
}

// holy shit this is compact
fn lookup_online(word: &str) -> Lookup {
    let response = reqwest::blocking::get(wiktionary_api_path.to_owned() + word)?.json::<serde_json::Value>()?;
    if let Some(serde_json::Value::String(wikitext)) = response.get("parse").and_then(|value| value.get("wikitext")) {
        return Ok(Some(String::from(wikitext)));
    } else {
        return Ok(None);
    }
}
