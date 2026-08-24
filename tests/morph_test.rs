use morpheusicons::prelude::*;

#[test]
fn test_svg_path_parsing() {
    let path_str = "M 5 12 h 14 M 12 5 l 7 7 -7 7 Z";
    let icon_path = IconPath::parse(path_str).expect("Failed to parse valid SVG path");
    assert!(!icon_path.subpaths.is_empty());
}

#[test]
fn test_procrustes_morph_play_to_pause() {
    let play = Icon::Play;
    let pause = Icon::Pause;

    let morpher = play
        .morph_to(pause)
        .expect("Failed to create morpher between Play and Pause");

    // Progress 0.0 should be near start icon
    let svg_at_0 = morpher.to_svg_path(0.0);
    assert!(!svg_at_0.is_empty());

    // Progress 0.5 should be intermediate morphed state
    let svg_at_mid = morpher.to_svg_path(0.5);
    assert!(!svg_at_mid.is_empty());

    // Progress 1.0 should be target icon
    let svg_at_1 = morpher.to_svg_path(1.0);
    assert!(!svg_at_1.is_empty());
}

#[test]
fn test_icon_pair_presets() {
    let pairs = [
        IconPair::PlayPause,
        IconPair::SunMoon,
        IconPair::MenuX,
        IconPair::LockUnlock,
        IconPair::EyeEyeOff,
        IconPair::VolumeMute,
        IconPair::PlusMinus,
        IconPair::MaximizeMinimize,
        IconPair::ChevronUpDown,
        IconPair::CheckX,
    ];

    for pair in pairs {
        let mut controller = pair
            .create_controller(SpringConfig::DEFAULT)
            .expect("Failed to create preset controller");

        assert_eq!(controller.progress(), 0.0);
        controller.morph_to_end();
        assert_eq!(controller.morpher().samples_per_subpath(), 64);
    }
}

#[test]
fn test_spring_controller_animation() {
    let mut controller = Icon::Sun
        .controller_to(Icon::Moon, SpringConfig::DEFAULT)
        .expect("Failed to create controller");

    assert_eq!(controller.progress(), 0.0);

    // Trigger morph to end
    controller.morph_to_end();

    // Step animation for 1 second in 16ms increments
    for _ in 0..60 {
        controller.update(0.016);
    }

    // Should have converged near target 1.0
    assert!((controller.progress() - 1.0).abs() < 0.05);
}

#[test]
fn test_interruptible_target_switch() {
    let mut controller = IconPair::PlayPause
        .create_controller(SpringConfig::SLO_MO)
        .unwrap();

    controller.morph_to_end();
    for _ in 0..5 {
        controller.update(0.016);
    }
    let mid_progress = controller.progress();
    assert!(mid_progress > 0.0 && mid_progress < 1.0);

    // Switch target mid-flight back to start
    controller.morph_to_start();
    for _ in 0..100 {
        controller.update(0.016);
    }
    assert!((controller.progress() - 0.0).abs() < 0.05);
}

#[test]
fn test_catalog_all_icons_valid() {
    let catalog = [
        Icon::ArrowRight,
        Icon::ArrowDown,
        Icon::ArrowLeft,
        Icon::ArrowUp,
        Icon::ChevronDown,
        Icon::ChevronUp,
        Icon::ChevronLeft,
        Icon::ChevronRight,
        Icon::CornerDownRight,
        Icon::CornerUpRight,
        Icon::Check,
        Icon::X,
        Icon::Plus,
        Icon::Minus,
        Icon::PlusCircle,
        Icon::MinusCircle,
        Icon::CheckCircle,
        Icon::XCircle,
        Icon::Play,
        Icon::Pause,
        Icon::Square,
        Icon::Volume2,
        Icon::VolumeX,
        Icon::Sun,
        Icon::Moon,
        Icon::Zap,
        Icon::Lock,
        Icon::Unlock,
        Icon::Eye,
        Icon::EyeOff,
        Icon::Search,
        Icon::Menu,
        Icon::Grid,
        Icon::Sliders,
        Icon::Filter,
        Icon::Layers,
        Icon::Maximize2,
        Icon::Minimize2,
        Icon::Heart,
        Icon::Star,
        Icon::Bell,
        Icon::Bookmark,
        Icon::Settings,
        Icon::User,
        Icon::Home,
        Icon::Folder,
        Icon::Mail,
        Icon::Calendar,
        Icon::Clock,
        Icon::Terminal,
        Icon::Code,
        Icon::Cpu,
        Icon::Copy,
        Icon::Trash,
        Icon::Download,
        Icon::Upload,
        Icon::RefreshCw,
        Icon::Share2,
        Icon::ExternalLink,
        Icon::Edit3,
    ];

    for icon in catalog {
        let parsed = IconPath::parse(icon.path_data());
        assert!(parsed.is_ok(), "Failed to parse icon {:?}", icon);
    }
}
