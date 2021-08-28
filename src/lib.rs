pub mod shortcut;
pub mod shortcuts_parser;
pub mod shortcuts_writer;



// Re-exports
pub use shortcut::Shortcut;
pub use shortcuts_parser::parse_shortcuts;
pub use shortcuts_writer::shortcuts_to_bytes;