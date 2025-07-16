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
    default: serde_json::Value,
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
            let value = Self::validate(&q.prompt, &q.qtype, q.default.clone());
            answers.insert(q.name.clone(), value);
        }

        answers
    }

    fn validate(prompt: &str, qtype: &str, default: serde_json::Value) -> String {
        fn default_to_string(default: &serde_json::Value) -> String {
            match default {
                serde_json::Value::String(s) => s.clone(),
                _ => default.to_string(),
            }
        }

        loop {
            print!("{} [{}]: ", prompt, default_to_string(&default));
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();
            let trimmed = input.trim();

            // If input is empty, use default
            if trimmed.is_empty() {
                return default_to_string(&default);
            }

            match qtype {
                "string" => return trimmed.to_string(),

                "number" => {
                    if trimmed.parse::<i64>().is_ok() {
                        return trimmed.to_string();
                    } else {
                        println!("Invalid number, please enter a valid integer.");
                    }
                }

                "bool" => {
                    match trimmed.to_lowercase().as_str() {
                        "true" | "1" | "yes" | "y" => return "true".to_string(),
                        "false" | "0" | "no" | "n" => return "false".to_string(),
                        _ => println!("Invalid boolean, please enter yes/no, true/false, 1/0."),
                    }
                }

                _ => {
                    println!("Unknown type '{}', treating input as string.", qtype);
                    return trimmed.to_string();
                }
            }
        }
    }
}