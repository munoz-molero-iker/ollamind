use ollama_rs::{models::LocalModel, Ollama};
use std::fmt;

//? [Custom wrapper for LocalModel]
struct LocalModelWrapper(LocalModel);
impl fmt::Display for LocalModelWrapper {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{}", self.0.name
            .to_string()
            .strip_suffix(":latest")
            .unwrap_or(self.0.name.as_str())
            .to_string()
        )

    }

}

//? [List all available models]
pub async fn list() {

    println!("Available models:");
    Ollama::default()
        .list_local_models()
        .await
        .unwrap()
        .into_iter()
        .map(LocalModelWrapper)
        .for_each(|model| println!("- {}", model));

}