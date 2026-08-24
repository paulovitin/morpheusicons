//! WebAssembly bindings for MorpheusIcons.
//! Exposes the morph engine to JavaScript via wasm-bindgen.

use wasm_bindgen::prelude::*;

use crate::animation::{MorphController, SpringConfig};
use crate::icons::catalog::Icon;
use crate::icons::pairs::IconPair;

/// WASM-exposed morph controller that wraps the Rust engine.
#[wasm_bindgen]
pub struct WasmMorphController {
    inner: MorphController,
}

#[wasm_bindgen]
impl WasmMorphController {
    /// Create a controller from two SVG path `d` strings and a spring preset name.
    /// `spring_preset`: "bouncy", "gentle", "snappy", "smooth", "slomo"
    #[wasm_bindgen(constructor)]
    pub fn new(from_path: &str, to_path: &str, spring_preset: &str) -> Result<WasmMorphController, JsValue> {
        let config = match spring_preset {
            "bouncy" => SpringConfig::BOUNCY,
            "gentle" => SpringConfig::GENTLE,
            "snappy" => SpringConfig::SNAPPY,
            "smooth" => SpringConfig::SMOOTH,
            "slomo" => SpringConfig::SLO_MO,
            _ => SpringConfig::BOUNCY,
        };
        let inner = MorphController::new(from_path, to_path, config)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(WasmMorphController { inner })
    }

    /// Create a controller from an IconPair preset name.
    /// `pair_name`: "SunMoon", "PlayPause", "MenuX", "LockUnlock", etc.
    #[wasm_bindgen(js_name = "fromPair")]
    pub fn from_pair(pair_name: &str, spring_preset: &str) -> Result<WasmMorphController, JsValue> {
        let pair = parse_icon_pair(pair_name)
            .ok_or_else(|| JsValue::from_str(&format!("Unknown pair: {pair_name}")))?;
        let config = match spring_preset {
            "bouncy" => SpringConfig::BOUNCY,
            "gentle" => SpringConfig::GENTLE,
            "snappy" => SpringConfig::SNAPPY,
            "smooth" => SpringConfig::SMOOTH,
            "slomo" => SpringConfig::SLO_MO,
            _ => SpringConfig::BOUNCY,
        };
        let inner = pair.create_controller(config)
            .map_err(|e| JsValue::from_str(&e))?;
        Ok(WasmMorphController { inner })
    }

    /// Advance the spring physics by `dt` seconds. Returns true if still animating.
    pub fn update(&mut self, dt: f32) -> bool {
        self.inner.update(dt)
    }

    /// Toggle between start and end icons.
    pub fn toggle(&mut self) {
        self.inner.toggle();
    }

    /// Morph towards the end icon (t=1.0).
    #[wasm_bindgen(js_name = "morphToEnd")]
    pub fn morph_to_end(&mut self) {
        self.inner.morph_to_end();
    }

    /// Morph towards the start icon (t=0.0).
    #[wasm_bindgen(js_name = "morphToStart")]
    pub fn morph_to_start(&mut self) {
        self.inner.morph_to_start();
    }

    /// Get current progress (0.0 to 1.0).
    pub fn progress(&self) -> f32 {
        self.inner.progress()
    }

    /// Set explicit progress value (bypasses spring).
    #[wasm_bindgen(js_name = "setProgress")]
    pub fn set_progress(&mut self, t: f32) {
        self.inner.set_progress(t);
    }

    /// Get the current interpolated SVG path `d` string.
    #[wasm_bindgen(js_name = "currentSvgPath")]
    pub fn current_svg_path(&self) -> String {
        self.inner.current_svg_path()
    }

    /// Change the spring preset dynamically.
    #[wasm_bindgen(js_name = "setSpring")]
    pub fn set_spring(&mut self, spring_preset: &str) {
        let config = match spring_preset {
            "bouncy" => SpringConfig::BOUNCY,
            "gentle" => SpringConfig::GENTLE,
            "snappy" => SpringConfig::SNAPPY,
            "smooth" => SpringConfig::SMOOTH,
            "slomo" => SpringConfig::SLO_MO,
            _ => SpringConfig::BOUNCY,
        };
        self.inner = MorphController::from_morpher(
            self.inner.morpher().clone(),
            config,
        );
    }
}

/// Get the SVG path `d` string for a named icon from the catalog.
#[wasm_bindgen(js_name = "getIconPath")]
pub fn get_icon_path(name: &str) -> Result<String, JsValue> {
    let icon = parse_icon(name)
        .ok_or_else(|| JsValue::from_str(&format!("Unknown icon: {name}")))?;
    Ok(icon.path_data().to_string())
}

/// Get all available icon names as a JSON array string.
#[wasm_bindgen(js_name = "listIcons")]
pub fn list_icons() -> String {
    let names = [
        "ArrowRight", "ArrowDown", "ArrowLeft", "ArrowUp",
        "ChevronDown", "ChevronUp", "ChevronLeft", "ChevronRight",
        "CornerDownRight", "CornerUpRight",
        "Check", "X", "Plus", "Minus", "PlusCircle", "MinusCircle",
        "CheckCircle", "XCircle",
        "Play", "Pause", "Square", "Volume2", "VolumeX",
        "Sun", "Moon", "Zap",
        "Lock", "Unlock", "Eye", "EyeOff",
        "Search", "Menu", "Grid", "Sliders", "Filter", "Layers",
        "Maximize2", "Minimize2",
        "Heart", "Star", "Bell", "Bookmark", "Settings", "User",
        "Home", "Folder", "Mail", "Calendar", "Clock", "Terminal",
        "Code", "Cpu", "Copy", "Trash", "Download", "Upload",
        "RefreshCw", "Share2", "ExternalLink", "Edit3",
    ];
    format!("[{}]", names.iter().map(|n| format!("\"{}\"", n)).collect::<Vec<_>>().join(","))
}

/// Get all available icon pair names as a JSON array string.
#[wasm_bindgen(js_name = "listPairs")]
pub fn list_pairs() -> String {
    let pairs = [
        "PlayPause", "SunMoon", "MenuX", "LockUnlock", "EyeEyeOff",
        "VolumeMute", "PlusMinus", "MaximizeMinimize", "ChevronUpDown", "CheckX",
        "HeartTerminal", "CpuSun", "BellCode", "MailZap",
        "TrashRefresh", "FolderStar", "UserLock", "CalendarPlay",
    ];
    format!("[{}]", pairs.iter().map(|n| format!("\"{}\"", n)).collect::<Vec<_>>().join(","))
}

fn parse_icon(name: &str) -> Option<Icon> {
    match name {
        "ArrowRight" => Some(Icon::ArrowRight),
        "ArrowDown" => Some(Icon::ArrowDown),
        "ArrowLeft" => Some(Icon::ArrowLeft),
        "ArrowUp" => Some(Icon::ArrowUp),
        "ChevronDown" => Some(Icon::ChevronDown),
        "ChevronUp" => Some(Icon::ChevronUp),
        "ChevronLeft" => Some(Icon::ChevronLeft),
        "ChevronRight" => Some(Icon::ChevronRight),
        "CornerDownRight" => Some(Icon::CornerDownRight),
        "CornerUpRight" => Some(Icon::CornerUpRight),
        "Check" => Some(Icon::Check),
        "X" => Some(Icon::X),
        "Plus" => Some(Icon::Plus),
        "Minus" => Some(Icon::Minus),
        "PlusCircle" => Some(Icon::PlusCircle),
        "MinusCircle" => Some(Icon::MinusCircle),
        "CheckCircle" => Some(Icon::CheckCircle),
        "XCircle" => Some(Icon::XCircle),
        "Play" => Some(Icon::Play),
        "Pause" => Some(Icon::Pause),
        "Square" => Some(Icon::Square),
        "Volume2" => Some(Icon::Volume2),
        "VolumeX" => Some(Icon::VolumeX),
        "Sun" => Some(Icon::Sun),
        "Moon" => Some(Icon::Moon),
        "Zap" => Some(Icon::Zap),
        "Lock" => Some(Icon::Lock),
        "Unlock" => Some(Icon::Unlock),
        "Eye" => Some(Icon::Eye),
        "EyeOff" => Some(Icon::EyeOff),
        "Search" => Some(Icon::Search),
        "Menu" => Some(Icon::Menu),
        "Grid" => Some(Icon::Grid),
        "Sliders" => Some(Icon::Sliders),
        "Filter" => Some(Icon::Filter),
        "Layers" => Some(Icon::Layers),
        "Maximize2" => Some(Icon::Maximize2),
        "Minimize2" => Some(Icon::Minimize2),
        "Heart" => Some(Icon::Heart),
        "Star" => Some(Icon::Star),
        "Bell" => Some(Icon::Bell),
        "Bookmark" => Some(Icon::Bookmark),
        "Settings" => Some(Icon::Settings),
        "User" => Some(Icon::User),
        "Home" => Some(Icon::Home),
        "Folder" => Some(Icon::Folder),
        "Mail" => Some(Icon::Mail),
        "Calendar" => Some(Icon::Calendar),
        "Clock" => Some(Icon::Clock),
        "Terminal" => Some(Icon::Terminal),
        "Code" => Some(Icon::Code),
        "Cpu" => Some(Icon::Cpu),
        "Copy" => Some(Icon::Copy),
        "Trash" => Some(Icon::Trash),
        "Download" => Some(Icon::Download),
        "Upload" => Some(Icon::Upload),
        "RefreshCw" => Some(Icon::RefreshCw),
        "Share2" => Some(Icon::Share2),
        "ExternalLink" => Some(Icon::ExternalLink),
        "Edit3" => Some(Icon::Edit3),
        _ => None,
    }
}

fn parse_icon_pair(name: &str) -> Option<IconPair> {
    match name {
        "PlayPause" => Some(IconPair::PlayPause),
        "SunMoon" => Some(IconPair::SunMoon),
        "MenuX" => Some(IconPair::MenuX),
        "LockUnlock" => Some(IconPair::LockUnlock),
        "EyeEyeOff" => Some(IconPair::EyeEyeOff),
        "VolumeMute" => Some(IconPair::VolumeMute),
        "PlusMinus" => Some(IconPair::PlusMinus),
        "MaximizeMinimize" => Some(IconPair::MaximizeMinimize),
        "ChevronUpDown" => Some(IconPair::ChevronUpDown),
        "CheckX" => Some(IconPair::CheckX),
        "HeartTerminal" => Some(IconPair::HeartTerminal),
        "CpuSun" => Some(IconPair::CpuSun),
        "BellCode" => Some(IconPair::BellCode),
        "MailZap" => Some(IconPair::MailZap),
        "TrashRefresh" => Some(IconPair::TrashRefresh),
        "FolderStar" => Some(IconPair::FolderStar),
        "UserLock" => Some(IconPair::UserLock),
        "CalendarPlay" => Some(IconPair::CalendarPlay),
        _ => None,
    }
}
