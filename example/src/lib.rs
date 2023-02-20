//! This is an example library demonstrating various attributes from the
//! stability crate.

/// This function does something really risky!
///
/// Don't use it yet!
#[stability::unstable(feature = "risky-function", issue = "#101")]
pub fn risky_function() {
    unimplemented!()
}
