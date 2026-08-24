use crate::icons::source::{IconSource, Viewport};

/// Catalog of built-in stroke-based vector icons (24x24 grid viewport).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Icon {
    // Directional & Navigation
    ArrowRight,
    ArrowDown,
    ArrowLeft,
    ArrowUp,
    ChevronDown,
    ChevronUp,
    ChevronLeft,
    ChevronRight,
    CornerDownRight,
    CornerUpRight,

    // Actions & Toggles
    Check,
    X,
    Plus,
    Minus,
    PlusCircle,
    MinusCircle,
    CheckCircle,
    XCircle,

    // Media & Playback
    Play,
    Pause,
    Square,
    Volume2,
    VolumeX,

    // Weather & Themes
    Sun,
    Moon,
    Zap,

    // Security & Visibility
    Lock,
    Unlock,
    Eye,
    EyeOff,

    // UI & Layout
    Search,
    Menu,
    Grid,
    Sliders,
    Filter,
    Layers,
    Maximize2,
    Minimize2,

    // Communication & Objects
    Heart,
    Star,
    Bell,
    Bookmark,
    Settings,
    User,
    Home,
    Folder,
    Mail,
    Calendar,
    Clock,
    Terminal,
    Code,
    Cpu,
    Copy,
    Trash,
    Download,
    Upload,
    RefreshCw,
    Share2,
    ExternalLink,
    Edit3,
}

impl Icon {
    /// Returns the raw SVG path string `d="..."` for the icon.
    pub const fn path_data(&self) -> &'static str {
        match self {
            // Directional & Navigation
            Icon::ArrowRight => "M5 12h14M12 5l7 7-7 7",
            Icon::ArrowDown => "M12 5v14M5 12l7 7 7-7",
            Icon::ArrowLeft => "M19 12H5M12 19l-7-7 7-7",
            Icon::ArrowUp => "M12 19V5M19 12l-7-7-7 7",
            Icon::ChevronDown => "M6 9l6 6 6-6",
            Icon::ChevronUp => "M6 15l6-6 6 6",
            Icon::ChevronLeft => "M15 18l-6-6 6-6",
            Icon::ChevronRight => "M9 18l6-6-6-6",
            Icon::CornerDownRight => "M15 10l5 5-5 5M4 4v7a4 4 0 0 0 4 4h12",
            Icon::CornerUpRight => "M15 14l5-5-5-5M4 20v-7a4 4 0 0 1 4-4h12",

            // Actions & Toggles
            Icon::Check => "M20 6L9 17l-5-5",
            Icon::X => "M18 6L6 18M6 6l12 12",
            Icon::Plus => "M12 5v14M5 12h14",
            Icon::Minus => "M5 12h14",
            Icon::PlusCircle => "M12 8v8M8 12h8M22 12A10 10 0 1 1 2 12a10 10 0 0 1 20 0z",
            Icon::MinusCircle => "M8 12h8M22 12A10 10 0 1 1 2 12a10 10 0 0 1 20 0z",
            Icon::CheckCircle => "M22 11.08V12a10 10 0 1 1-5.93-9.14M22 4L12 14.01l-3-3",
            Icon::XCircle => "M15 9l-6 6M9 9l6 6M22 12A10 10 0 1 1 2 12a10 10 0 0 1 20 0z",

            // Media & Playback
            Icon::Play => "M6 4l14 8-14 8z",
            Icon::Pause => "M6 4v16M18 4v16",
            Icon::Square => "M5 5h14v14H5z",
            Icon::Volume2 => "M11 5L6 9H2v6h4l5 4V5zM19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07",
            Icon::VolumeX => "M11 5L6 9H2v6h4l5 4V5zM23 9l-6 6M17 9l6 6",

            // Weather & Themes
            Icon::Sun => "M12 8a4 4 0 1 0 0 8 4 4 0 1 0 0-8M12 2v2M12 20v2M4.93 4.93l1.41 1.41M17.66 17.66l1.41 1.41M2 12h2M20 12h2M4.93 19.07l1.41-1.41M17.66 6.34l1.41-1.41",
            Icon::Moon => "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z",
            Icon::Zap => "M13 2L3 14h9l-1 8 10-12h-9l1-8z",

            // Security & Visibility
            Icon::Lock => "M19 11H5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2zM7 11V7a5 5 0 0 1 10 0v4",
            Icon::Unlock => "M19 11H5a2 2 0 0 0-2 2v7a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7a2 2 0 0 0-2-2zM7 11V7a5 5 0 0 1 9.9-1",
            Icon::Eye => "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8zM12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6z",
            Icon::EyeOff => "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24M1 1l22 22",

            // UI & Layout
            Icon::Search => "M11 19a8 8 0 1 0 0-16 8 8 0 0 0 0 16zM21 21l-4.35-4.35",
            Icon::Menu => "M4 6h16M4 12h16M4 18h16",
            Icon::Grid => "M3 3h7v7H3zM14 3h7v7h-7zM14 14h7v7h-7zM3 14h7v7H3z",
            Icon::Sliders => "M4 21v-7M4 10V3M12 21v-9M12 8V3M20 21v-5M20 11V3M1 14h6M9 8h6M17 16h6",
            Icon::Filter => "M22 3H2l8 9.46V19l4 2v-8.54L22 3z",
            Icon::Layers => "M12 2L2 7l10 5 10-5-10-5zM2 17l10 5 10-5M2 12l10 5 10-5",
            Icon::Maximize2 => "M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7",
            Icon::Minimize2 => "M4 14h6v6M20 10h-6V4M14 10l7-7M3 21l7-7",

            // Communication & Objects
            Icon::Heart => "M19 14c1.49-1.46 3-3.21 3-5.5A4.5 4.5 0 0 0 14.5 4c-1.25 0-2.45.5-3.5 1.5C9.95 4.5 8.75 4 7.5 4A4.5 4.5 0 0 0 3 8.5c0 2.29 1.51 4.04 3 5.5l6 6z",
            Icon::Star => "M12 2l3.09 6.26L22 9.27l-5 4.87 1.18 6.88L12 17.77l-6.18 3.25L7 14.14 2 9.27l6.91-1.01L12 2z",
            Icon::Bell => "M6 8a6 6 0 0 1 12 0c0 7 3 9 3 9H3s3-2 3-9M10.3 21a1.94 1.94 0 0 0 3.4 0",
            Icon::Bookmark => "M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z",
            Icon::Settings => "M12 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z",
            Icon::User => "M19 21v-2a4 4 0 0 0-4-4H9a4 4 0 0 0-4 4v2M12 11a4 4 0 1 0 0-8 4 4 0 0 0 0 8z",
            Icon::Home => "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2zM9 22V12h6v10",
            Icon::Folder => "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z",
            Icon::Mail => "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2zM22 6l-10 7L2 6",
            Icon::Calendar => "M19 4H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2zM16 2v4M8 2v4M3 10h18",
            Icon::Clock => "M12 8v4l3 3M22 12A10 10 0 1 1 2 12a10 10 0 0 1 20 0z",
            Icon::Terminal => "M4 17l6-6-6-6M12 19h8",
            Icon::Code => "M16 18l6-6-6-6M8 6l-6 6 6 6",
            Icon::Cpu => "M4 4h16v16H4zM9 1v3M15 1v3M9 20v3M15 20v3M1 9h3M1 15h3M20 9h3M20 15h3",
            Icon::Copy => "M9 15a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h8a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2zM5 9H4a2 2 0 0 0-2 2v8a2 2 0 0 0 2 2h8a2 2 0 0 0 2-2v-1",
            Icon::Trash => "M3 6h18M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6M8 6V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2",
            Icon::Download => "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M7 10l5 5 5-5M12 15V3",
            Icon::Upload => "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4M17 8l-5-5-5 5M12 3v12",
            Icon::RefreshCw => "M21 12a9 9 0 0 0-9-9c-3.8 0-7.2 2.3-8.5 5.7M3 3v5h5M3 12a9 9 0 0 0 9 9c3.8 0 7.2-2.3 8.5-5.7M21 21v-5h-5",
            Icon::Share2 => "M18 8a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM6 15a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM18 22a3 3 0 1 0 0-6 3 3 0 0 0 0 6zM8.59 13.51l6.83 3.98M15.41 6.51l-6.82 3.98",
            Icon::ExternalLink => "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6M15 3h6v6M10 14L21 3",
            Icon::Edit3 => "M12 20h9M16.5 3.5a2.12 2.12 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z",
        }
    }

    /// Convenience method to create a `PathMorpher` between `self` and `target`.
    pub fn morph_to(&self, target: Icon) -> Result<crate::animation::PathMorpher, String> {
        crate::animation::PathMorpher::new(self.path_data(), target.path_data())
    }

    /// Convenience method to create a `MorphController` between `self` and `target`.
    pub fn controller_to(
        &self,
        target: Icon,
        config: crate::animation::SpringConfig,
    ) -> Result<crate::animation::MorphController, String> {
        crate::animation::MorphController::new(self.path_data(), target.path_data(), config)
    }

    /// Convenience method to create a `PathMorpher` from `self` to any `IconSource`.
    pub fn morph_to_source(
        &self,
        target: &dyn IconSource,
    ) -> Result<crate::animation::PathMorpher, String> {
        crate::animation::PathMorpher::new(self.path_data(), target.path_data())
    }

    /// Convenience method to create a `MorphController` from `self` to any `IconSource`.
    pub fn controller_to_source(
        &self,
        target: &dyn IconSource,
        config: crate::animation::SpringConfig,
    ) -> Result<crate::animation::MorphController, String> {
        crate::animation::MorphController::new(self.path_data(), target.path_data(), config)
    }
}

impl IconSource for Icon {
    fn path_data(&self) -> &str {
        self.path_data()
    }

    fn viewport(&self) -> Viewport {
        Viewport::STANDARD_24
    }
}
