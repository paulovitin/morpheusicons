use crate::animation::{MorphController, PathMorpher, SpringConfig};
use crate::icons::catalog::Icon;

/// Preset icon pairs commonly used for interactive UI toggles and wild morphing demos.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconPair {
    // Related UI pairs
    PlayPause,
    SunMoon,
    MenuX,
    LockUnlock,
    EyeEyeOff,
    VolumeMute,
    PlusMinus,
    MaximizeMinimize,
    ChevronUpDown,
    CheckX,

    // Completely Unrelated / Wild Pairs
    HeartTerminal,
    CpuSun,
    BellCode,
    MailZap,
    TrashRefresh,
    FolderStar,
    UserLock,
    CalendarPlay,
}

impl IconPair {
    /// Returns the starting `Icon` for this pair.
    pub const fn start_icon(&self) -> Icon {
        match self {
            IconPair::PlayPause => Icon::Play,
            IconPair::SunMoon => Icon::Sun,
            IconPair::MenuX => Icon::Menu,
            IconPair::LockUnlock => Icon::Lock,
            IconPair::EyeEyeOff => Icon::Eye,
            IconPair::VolumeMute => Icon::Volume2,
            IconPair::PlusMinus => Icon::Plus,
            IconPair::MaximizeMinimize => Icon::Maximize2,
            IconPair::ChevronUpDown => Icon::ChevronDown,
            IconPair::CheckX => Icon::Check,

            IconPair::HeartTerminal => Icon::Heart,
            IconPair::CpuSun => Icon::Cpu,
            IconPair::BellCode => Icon::Bell,
            IconPair::MailZap => Icon::Mail,
            IconPair::TrashRefresh => Icon::Trash,
            IconPair::FolderStar => Icon::Folder,
            IconPair::UserLock => Icon::User,
            IconPair::CalendarPlay => Icon::Calendar,
        }
    }

    /// Returns the ending `Icon` for this pair.
    pub const fn end_icon(&self) -> Icon {
        match self {
            IconPair::PlayPause => Icon::Pause,
            IconPair::SunMoon => Icon::Moon,
            IconPair::MenuX => Icon::X,
            IconPair::LockUnlock => Icon::Unlock,
            IconPair::EyeEyeOff => Icon::EyeOff,
            IconPair::VolumeMute => Icon::VolumeX,
            IconPair::PlusMinus => Icon::Minus,
            IconPair::MaximizeMinimize => Icon::Minimize2,
            IconPair::ChevronUpDown => Icon::ChevronUp,
            IconPair::CheckX => Icon::X,

            IconPair::HeartTerminal => Icon::Terminal,
            IconPair::CpuSun => Icon::Sun,
            IconPair::BellCode => Icon::Code,
            IconPair::MailZap => Icon::Zap,
            IconPair::TrashRefresh => Icon::RefreshCw,
            IconPair::FolderStar => Icon::Star,
            IconPair::UserLock => Icon::Lock,
            IconPair::CalendarPlay => Icon::Play,
        }
    }

    /// Creates a pre-configured `PathMorpher` for this icon pair.
    pub fn create_morpher(&self) -> Result<PathMorpher, String> {
        self.start_icon().morph_to(self.end_icon())
    }

    /// Creates a pre-configured `MorphController` for this icon pair.
    pub fn create_controller(&self, config: SpringConfig) -> Result<MorphController, String> {
        self.start_icon().controller_to(self.end_icon(), config)
    }
}
