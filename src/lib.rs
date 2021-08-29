pub mod shortcut;
pub mod shortcuts_parser;
pub mod shortcuts_writer;
pub mod app_id_generator;



// Re-exports
pub use shortcut::Shortcut;
pub use shortcuts_parser::parse_shortcuts;
pub use shortcuts_writer::shortcuts_to_bytes;
pub use app_id_generator::calculate_app_id_for_shortcut;