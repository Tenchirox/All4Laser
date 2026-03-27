pub mod llm_provider;
pub mod prompt_parser;
pub mod scene_composer;
pub mod shape_library;
pub mod svg_path_parser;

pub use llm_provider::{AiBackend, AiConfig, call_llm};
pub use svg_path_parser::{extract_layered_paths, PathLayer};
