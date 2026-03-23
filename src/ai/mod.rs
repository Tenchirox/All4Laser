pub mod prompt_parser;
pub mod shape_library;
pub mod scene_composer;
pub mod llm_provider;
pub mod svg_path_parser;

pub use scene_composer::generate_from_prompt;
pub use llm_provider::{AiBackend, AiConfig, call_llm};
pub use svg_path_parser::{extract_paths_from_svg, extract_layered_paths, scale_paths, PathLayer, LayeredPath};
