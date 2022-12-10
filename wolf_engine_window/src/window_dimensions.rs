#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub mod prelude {
    pub use super::*;
}

/// Represents the size of a window, in pixels.
#[derive(Debug, Eq, PartialEq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WindowDimensions {

    /// The width of a window, in pixels.
    pub width: usize,

    /// The height of a window, in pixels.
    pub height: usize,
}

impl WindowDimensions {

    /// Create a new set of dimensions from the provide pixel values.
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

impl From<(usize, usize)> for WindowDimensions {
    fn from(dimensions: (usize, usize)) -> Self {
        Self::new(dimensions.0, dimensions.1)
    }
}

#[cfg(test)]
mod window_dimensions_tests {
    use super::*;

    #[test]
    pub fn should_have_width_and_height() {
        let dimensions = WindowDimensions::new(800, 600);
        assert_eq!(dimensions.width, 800);
        assert_eq!(dimensions.height, 600);
    }

    #[test]
    fn should_convert_from_tuple() {
        let dimensions: WindowDimensions = (800, 600).into();
        assert_eq!(dimensions.width, 800);
        assert_eq!(dimensions.height, 600);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn should_implement_deserialize() {
        let toml_str = r#"
        width = 800
        height = 600
        "#;
        let dimensions: WindowDimensions = toml::from_str(toml_str).unwrap();
        assert_eq!(dimensions.width, 800);
        assert_eq!(dimensions.height, 600);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn should_implement_serialize() {
        let dimensions = WindowDimensions::new(1280, 720);
        let toml_str = toml::to_string(&dimensions).unwrap();

        assert_eq!(
            toml_str,
            "width = 1280\n".to_owned() + &"height = 720\n".to_owned()
        );
    }
}
