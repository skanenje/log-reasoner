pub mod ollama;

use anyhow::Result;

/// Trait for AI backends (embedding + reasoning)
pub trait EmbeddingBackend {
    /// Generate embeddings for a batch of texts
    fn embed(&self, texts: &[String]) -> Result<Vec<Vec<f32>>>;
}