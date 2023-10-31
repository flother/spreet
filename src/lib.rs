// Re-export resvg because its types are used in the spreet API. This removes the need for users of
// spreet to import resvg separately and manage version compatibility.
pub use resvg;

mod error;
pub use error::{SpreetError, SpreetResult};

mod fs;
pub use fs::*;

mod sprite;
pub use sprite::*;
