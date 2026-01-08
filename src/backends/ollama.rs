use super::EmbeddingBackend;
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};

const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434";
const DEFAULT_MODEL: &str = "nomic-embed-text";

pub struct OllamaBackend {
    base_url: String,
    model: String,
    client: reqwest::blocking::Client,
}

impl OllamaBackend {
    pub fn new() -> Self {
        Self {
            base_url: DEFAULT_OLLAMA_URL.to_string(),
            model: DEFAULT_MODEL.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    #[allow(dead_code)]
    pub fn with_model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    /// Check if Ollama is running and model is available
    pub fn check_available(&self) -> Result<()> {
        let url = format!("{}/api/tags", self.base_url);
        
        let response = self.client
            .get(&url)
            .send()
            .context("Failed to connect to Ollama. Is it running? Start with: ollama serve")?;

        if !response.status().is_success() {
            return Err(anyhow!("Ollama is running but returned error status"));
        }

        let models: ModelsResponse = response.json()
            .context("Failed to parse Ollama models response")?;

        let model_exists = models.models.iter().any(|m| m.name.starts_with(&self.model));
        
        if !model_exists {
            return Err(anyhow!(
                "Model '{}' not found. Pull it with: ollama pull {}",
                self.model,
                self.model
            ));
        }

        Ok(())
    }
}

impl EmbeddingBackend for OllamaBackend {
    fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>> {
        let url = format!("{}/api/embeddings", self.base_url);
        
        let mut embeddings = Vec::new();

        for text in texts {
            let request = EmbedRequest {
                model: self.model.clone(),
                prompt: text.clone(),
            };

            let response = self.client
                .post(&url)
                .json(&request)
                .send()
                .context("Failed to send embedding request to Ollama")?;

            if !response.status().is_success() {
                return Err(anyhow!("Ollama embedding request failed: {}", response.status()));
            }

            let embed_response: EmbedResponse = response.json()
                .context("Failed to parse Ollama embedding response")?;

            embeddings.push(embed_response.embedding);
        }

        Ok(embeddings)
    }
}

// Request/Response structures for Ollama API
#[derive(Serialize)]
struct EmbedRequest {
    model: String,
    prompt: String,
}

#[derive(Deserialize)]
struct EmbedResponse {
    embedding: Vec<f32>,
}

#[derive(Deserialize)]
struct ModelsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
struct ModelInfo {
    name: String,
}