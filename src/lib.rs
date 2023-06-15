pub mod error;
pub mod fs;
pub mod sprite;

// Re-export resvg because its types are used in the spreet API. This removes the need for users of
// spreet to import resvg separately and manage version compatibility.
pub use resvg;
