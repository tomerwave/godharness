mod claude_code_hook;
mod static_files;

pub use claude_code_hook::{ClaudeCodeEvent, claude_code_hook_response};
pub use static_files::{
    AdapterError, FieldMapping, RenderedFile, render_shape_a, write_rendered_files,
};
