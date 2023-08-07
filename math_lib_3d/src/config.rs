// Where the common globals and statics from C++ go.

use crate::renderer::Renderer;

pub struct Config {
    pub(crate) renderer: Renderer,
}
