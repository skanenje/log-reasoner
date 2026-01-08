use crate::backends::EmbeddingBackend;
use crate::models::LogGroup;
use anyhow::Result;

pub struct EmbeddingGenerator<B: EmbeddingBackend> {
    backend: B,
}

impl<B: EmbeddingBackend> EmbeddingGenerator<B> {
    pub fn new(backend: B) -> Self {
        Self { backend }
    }

    /// Generate embeddings for log groups
    /// Returns: Vec of (group_index, embedding_vector)
    pub fn embed_groups(&self, groups: &[LogGroup]) -> Result<Vec<(usize, Vec<f32>)>> {
        println!("Generating embeddings for {} patterns...", groups.len());

        // Extract unique patterns
        let texts: Vec<String> = groups
            .iter()
            .map(|g| g.pattern.clone())
            .collect();

        // Generate embeddings
        let embeddings = self.backend.embed(&texts)?;

        // Pair with indices
        let result: Vec<(usize, Vec<f32>)> = embeddings
            .into_iter()
            .enumerate()
            .collect();

        println!("✓ Generated {} embeddings", result.len());

        Ok(result)
    }

    /// Calculate cosine similarity between two vectors
    #[allow(dead_code)]
    pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
        if a.len() != b.len() {
            return 0.0;
        }

        let dot_product: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
        let magnitude_a: f32 = a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let magnitude_b: f32 = b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if magnitude_a == 0.0 || magnitude_b == 0.0 {
            return 0.0;
        }

        dot_product / (magnitude_a * magnitude_b)
    }
}