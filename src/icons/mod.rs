pub mod catalog;
pub mod pairs;
pub mod source;
pub mod svg_extract;
pub mod validate;

pub use catalog::Icon;
pub use pairs::IconPair;
pub use source::{IconSource, RawIcon, ValidationError, ValidationResult, ValidationWarning, Viewport};
pub use svg_extract::{extract_path_from_svg, icon_from_svg, icon_from_svg_normalized, SvgExtraction};
pub use validate::{check_icon, check_morph_compatibility, check_path_data, MorphCompatibility, KnownIconLibrary};
