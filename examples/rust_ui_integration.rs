//! Interactive `rust-ui` / Shadcn Component Showcase Demo
//!
//! Run with: `make run rust-ui`

use eframe::egui::{self, Color32, Stroke};
use morpheusicons::integrations::egui::paint_morph_icon;
use morpheusicons::prelude::*;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 720.0])
            .with_min_inner_size([720.0, 600.0])
            .with_title("MorpheusIcons - rust-ui Showcase"),
        ..Default::default()
    };

    eframe::run_native(
        "MorpheusIcons - rust-ui Showcase",
        options,
        Box::new(|_cc| Ok(Box::new(RustUiShowcaseApp::new()))),
    )
}

#[derive(Clone, Copy, PartialEq)]
enum PresetPair {
    // UI Pairs
    SunMoon,
    PlayPause,
    MenuX,
    LockUnlock,
    EyeEyeOff,
    VolumeMute,

    // Wild / Unrelated Pairs
    HeartTerminal,
    CpuSun,
    BellCode,
    MailZap,
    TrashRefresh,
    FolderStar,
    UserLock,
    CalendarPlay,
}

impl PresetPair {
    fn name(&self) -> &'static str {
        match self {
            PresetPair::SunMoon => "Sun <-> Moon",
            PresetPair::PlayPause => "Play <-> Pause",
            PresetPair::MenuX => "Menu <-> Close",
            PresetPair::LockUnlock => "Lock <-> Unlock",
            PresetPair::EyeEyeOff => "Eye <-> EyeOff",
            PresetPair::VolumeMute => "Volume <-> Mute",

            PresetPair::HeartTerminal => "Heart <-> Terminal",
            PresetPair::CpuSun => "CPU <-> Sun",
            PresetPair::BellCode => "Bell <-> Code",
            PresetPair::MailZap => "Mail <-> Zap",
            PresetPair::TrashRefresh => "Trash <-> Refresh",
            PresetPair::FolderStar => "Folder <-> Star",
            PresetPair::UserLock => "User <-> Lock",
            PresetPair::CalendarPlay => "Calendar <-> Play",
        }
    }

    fn icons(&self) -> (Icon, Icon) {
        match self {
            PresetPair::SunMoon => (Icon::Sun, Icon::Moon),
            PresetPair::PlayPause => (Icon::Play, Icon::Pause),
            PresetPair::MenuX => (Icon::Menu, Icon::X),
            PresetPair::LockUnlock => (Icon::Lock, Icon::Unlock),
            PresetPair::EyeEyeOff => (Icon::Eye, Icon::EyeOff),
            PresetPair::VolumeMute => (Icon::Volume2, Icon::VolumeX),

            PresetPair::HeartTerminal => (Icon::Heart, Icon::Terminal),
            PresetPair::CpuSun => (Icon::Cpu, Icon::Sun),
            PresetPair::BellCode => (Icon::Bell, Icon::Code),
            PresetPair::MailZap => (Icon::Mail, Icon::Zap),
            PresetPair::TrashRefresh => (Icon::Trash, Icon::RefreshCw),
            PresetPair::FolderStar => (Icon::Folder, Icon::Star),
            PresetPair::UserLock => (Icon::User, Icon::Lock),
            PresetPair::CalendarPlay => (Icon::Calendar, Icon::Play),
        }
    }
}

struct RustUiShowcaseApp {
    controller: MorphController,
    current_preset: PresetPair,
    current_spring: SpringConfig,
    is_forward: bool,
}

impl RustUiShowcaseApp {
    fn new() -> Self {
        let preset = PresetPair::SunMoon;
        let spring = SpringConfig::SMOOTH;
        let (from, to) = preset.icons();

        let controller = MorphController::new(from.path_data(), to.path_data(), spring)
            .expect("Failed to create morph controller");

        Self {
            controller,
            current_preset: preset,
            current_spring: spring,
            is_forward: false,
        }
    }

    fn select_preset(&mut self, preset: PresetPair) {
        self.current_preset = preset;
        let (from, to) = preset.icons();
        self.controller =
            MorphController::new(from.path_data(), to.path_data(), self.current_spring).unwrap();
        self.is_forward = false;
    }

    fn select_spring(&mut self, spring: SpringConfig) {
        self.current_spring = spring;
        let (from, to) = self.current_preset.icons();
        let progress = self.controller.progress();
        self.controller = MorphController::new(from.path_data(), to.path_data(), spring).unwrap();
        self.controller.set_progress(progress);
    }

    fn toggle_icon(&mut self) {
        self.is_forward = !self.is_forward;
        self.controller.toggle();
    }

    fn preset_button(&mut self, ui: &mut egui::Ui, preset: PresetPair) {
        let is_selected = self.current_preset == preset;
        let text = preset.name();
        let button = if is_selected {
            egui::Button::new(egui::RichText::new(text).size(12.0).color(Color32::WHITE))
                .fill(Color32::from_rgb(22, 163, 74))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(22, 163, 74)))
                .corner_radius(8.0)
        } else {
            egui::Button::new(
                egui::RichText::new(text)
                    .size(12.0)
                    .color(Color32::from_rgb(86, 92, 102)),
            )
            .fill(Color32::from_rgb(255, 255, 255))
            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(229, 232, 236)))
            .corner_radius(8.0)
        };

        if ui.add(button).clicked() {
            self.select_preset(preset);
        }
    }

    fn segment_btn(&mut self, ui: &mut egui::Ui, label: &'static str, config: SpringConfig) {
        let is_selected = self.current_spring == config;
        let text_color = if is_selected {
            Color32::WHITE
        } else {
            Color32::from_rgb(86, 92, 102)
        };
        let bg_color = if is_selected {
            Color32::from_rgb(22, 163, 74)
        } else {
            Color32::from_rgb(249, 250, 251)
        };

        let button = egui::Button::new(egui::RichText::new(label).size(12.0).color(text_color))
            .fill(bg_color)
            .corner_radius(6.0);

        if ui.add(button).clicked() {
            self.select_spring(config);
        }
    }
}

impl eframe::App for RustUiShowcaseApp {
    fn logic(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let active = self.controller.update(ctx.input(|i| i.stable_dt));
        if active {
            ctx.request_repaint();
        }
    }

    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let (icon_a, icon_b) = self.current_preset.icons();
        let progress = self.controller.progress();

        ui.vertical_centered(|ui| {
            ui.add_space(20.0);

            // Title Header
            ui.heading(
                egui::RichText::new("rust-ui Component Showcase")
                    .size(24.0)
                    .strong()
                    .color(Color32::from_rgb(17, 19, 23)),
            );
            ui.label(
                egui::RichText::new(
                    "Morphing de Ícones Vetoriais em Tempo Real (Leptos & Tailwind)",
                )
                .size(12.0)
                .color(Color32::from_rgb(86, 92, 102)),
            );

            ui.add_space(16.0);

            // Segmented Spring Selector
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 260.0).max(0.0) / 2.0);
                ui.label(
                    egui::RichText::new("spring")
                        .size(12.0)
                        .color(Color32::from_rgb(148, 163, 184)),
                );
                self.segment_btn(ui, "smooth", SpringConfig::SMOOTH);
                self.segment_btn(ui, "snappy", SpringConfig::SNAPPY);
                self.segment_btn(ui, "bouncy", SpringConfig::BOUNCY);
            });

            ui.add_space(16.0);

            // Main Stage Icon Card
            egui::Frame::new()
                .fill(Color32::from_rgb(249, 250, 251))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(229, 232, 236)))
                .corner_radius(16.0)
                .inner_margin(24.0)
                .show(ui, |ui| {
                    ui.set_max_width(520.0);
                    ui.vertical_centered(|ui| {
                        // Icon Display Box
                        egui::Frame::new()
                            .fill(Color32::from_rgb(255, 255, 255))
                            .stroke(Stroke::new(1.0_f32, Color32::from_rgb(229, 232, 236)))
                            .corner_radius(12.0)
                            .inner_margin(22.0)
                            .show(ui, |ui| {
                                paint_morph_icon(
                                    ui,
                                    &self.controller,
                                    egui::vec2(96.0, 96.0),
                                    Color32::from_rgb(22, 163, 74),
                                    3.0,
                                );
                            });

                        ui.add_space(12.0);

                        // Status Label
                        ui.label(
                            egui::RichText::new(format!("{:?} <-> {:?}", icon_a, icon_b))
                                .size(16.0)
                                .strong()
                                .color(Color32::from_rgb(248, 250, 252)),
                        );
                        ui.label(
                            egui::RichText::new(format!(
                                "Progresso: {:.1}%",
                                progress * 100.0
                            ))
                            .size(12.0)
                            .color(Color32::from_rgb(100, 116, 139)),
                        );

                        ui.add_space(12.0);

                        // Main Toggle Button
                        let target_icon = if self.is_forward { icon_a } else { icon_b };
                        let toggle_btn = egui::Button::new(
                            egui::RichText::new(format!("Morph para {:?}", target_icon))
                                .size(14.0)
                                .strong()
                                .color(Color32::WHITE),
                        )
                        .fill(Color32::from_rgb(22, 163, 74))
                        .corner_radius(12.0);

                        if ui.add(toggle_btn).clicked() {
                            self.toggle_icon();
                        }
                    });
                });

            ui.add_space(20.0);

            // Wild Unrelated Section
            ui.label(
                egui::RichText::new("🔥 Ícones Inusitados:")
                    .size(12.0)
                    .strong()
                    .color(Color32::from_rgb(56, 189, 248)),
            );
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 480.0).max(0.0) / 2.0);
                self.preset_button(ui, PresetPair::HeartTerminal);
                self.preset_button(ui, PresetPair::CpuSun);
                self.preset_button(ui, PresetPair::BellCode);
                self.preset_button(ui, PresetPair::MailZap);
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 480.0).max(0.0) / 2.0);
                self.preset_button(ui, PresetPair::TrashRefresh);
                self.preset_button(ui, PresetPair::FolderStar);
                self.preset_button(ui, PresetPair::UserLock);
                self.preset_button(ui, PresetPair::CalendarPlay);
            });

            ui.add_space(16.0);

            // Traditional UI Section
            ui.label(
                egui::RichText::new("Pares Tradicionais de UI:")
                    .size(12.0)
                    .strong()
                    .color(Color32::from_rgb(148, 163, 184)),
            );
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 480.0).max(0.0) / 2.0);
                self.preset_button(ui, PresetPair::SunMoon);
                self.preset_button(ui, PresetPair::PlayPause);
                self.preset_button(ui, PresetPair::MenuX);
                self.preset_button(ui, PresetPair::LockUnlock);
            });
            ui.add_space(4.0);
            ui.horizontal(|ui| {
                ui.add_space((ui.available_width() - 480.0).max(0.0) / 2.0);
                self.preset_button(ui, PresetPair::EyeEyeOff);
                self.preset_button(ui, PresetPair::VolumeMute);
            });

            ui.add_space(20.0);

            // Leptos / rust-ui Code Snippet Box
            egui::Frame::new()
                .fill(Color32::from_rgb(249, 250, 251))
                .stroke(Stroke::new(1.0_f32, Color32::from_rgb(229, 232, 236)))
                .corner_radius(12.0)
                .inner_margin(12.0)
                .show(ui, |ui| {
                    ui.set_max_width(540.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("📝 Código Leptos / rust-ui:")
                                .size(11.0)
                                .strong()
                                .color(Color32::from_rgb(56, 189, 248)),
                        );
                    });
                    ui.add_space(4.0);
                    let snippet = "<button class=\"inline-flex items-center justify-center p-2 rounded-md bg-background hover:bg-accent\">\n  <MorphIcon d=path_d class=\"w-6 h-6 text-foreground\" />\n</button>".to_string();
                    ui.label(
                        egui::RichText::new(snippet)
                            .size(11.0)
                            .color(Color32::from_rgb(226, 232, 240))
                            .monospace(),
                    );
                });
        });
    }
}
