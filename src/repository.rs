use std::path::Path;
use git2::Repository;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Repo {
    name: String,
    url: String,
}

pub fn search(template_name: &str) -> anyhow::Result<Repo> {
    let data = r#"
    [
        {
            "name": "gtk2",
            "url": "https://github.com/progzone122/kff-gtk2-template"
        }
    ]
    "#;

    // Парсим JSON в вектор
    let repos: Vec<Repo> = serde_json::from_str(data)?;

    // Ищем репозиторий с нужным именем
    for repo in repos {
        if repo.name == template_name {
            return Ok(repo);
        }
    }

    // Если репозиторий не найден, возвращаем ошибку
    Err(anyhow::anyhow!("Template {template_name} not found in repository"))
}

pub fn download(repo: &Repo) -> anyhow::Result<()> {
    let destination_path = Path::new(&repo.name);

    Repository::clone(&repo.url, destination_path)?;

    Ok(())
}