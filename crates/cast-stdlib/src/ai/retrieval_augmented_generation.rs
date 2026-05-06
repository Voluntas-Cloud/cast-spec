//! `retrieval_augmented_generation` — answer using retrieved context.

/// Sentinel for `retrieval_augmented_generation`.
pub struct RetrievalAugmentedGeneration;

cast::concept! {
    name: "retrieval_augmented_generation",
    summary: "Answer with the help of retrieved context rather than \
              relying on the model's parametric memory. The retrieval \
              step is where most of the quality lives; if the wrong \
              passages come back, the model just hallucinates more \
              fluently.",
    anchors: [cast_stdlib::ai::retrieval_augmented_generation::RetrievalAugmentedGeneration],
    tags: ["cast_stdlib", "ai"],
}
