//! Automation for coding a client application.
//!

/// An object which can be displayed. Returns JavaScript client source.
pub trait ClientDisplay {
    fn render_src() -> &'static str;
}
