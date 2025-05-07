use std::collections::HashMap;
use std::{fs, io};
use std::io::Write;
use std::path::Path;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Serialize, Debug)]
pub struct Template {
    questions: Vec<Question>,
    files: Vec<File>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Question {
    name: String,
    prompt: String,
    #[serde(rename = "type")]
    qtype: String, // type is a reserved word in Rust
    default: String,
}

#[derive(Deserialize, Serialize, Debug)]
struct File {
    file: String,
    placeholders: Vec<Placeholder>,
}

#[derive(Deserialize, Serialize, Debug)]
struct Placeholder {
    placeholder: String,
    replacement: String,
    description: String,
}

impl Template {
    pub fn parse_from_file<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let content = fs::read_to_string(path)?;
        let parsed: Template = serde_json::from_str(&content)?;
        Ok(parsed)
    }
    pub fn apply_replacements(
        &self,
        answers: &HashMap<String, String>,
        root: &Path,
    ) -> anyhow::Result<()> {
        for file_entry in &self.files {
            let file_path = root.join(&file_entry.file);
            let mut contents = fs::read_to_string(&file_path)?;

            for placeholder in &file_entry.placeholders {
                if let Some(value) = answers.get(&placeholder.replacement.trim_matches('{').trim_matches('}').to_string()) {
                    contents = contents.replace(&placeholder.placeholder, value);
                }
            }

            fs::write(&file_path, contents)?;
        }
        Ok(())
    }
    pub fn ask_questions(&self) -> HashMap<String, String> {
        let mut answers = HashMap::new();

        for q in &self.questions {
            print!("{} [{}]: ", q.prompt, q.default);
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();

            answers.insert(q.name.clone(), if input.is_empty() { q.default.clone() } else { input.to_string() });
        }

        answers
    }
}