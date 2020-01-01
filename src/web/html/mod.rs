use horrorshow::{owned_html as html, Render, RenderOnce};

use super::Critical;

dirmod::all! {
    default pub(super);
    pub use args, lib
}

type Output = Result<String, Critical>;
