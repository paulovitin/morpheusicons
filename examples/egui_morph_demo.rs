#[cfg(feature = "egui")]
use eframe::egui;
#[cfg(feature = "egui")]
use morpheusicons::integrations::egui::paint_morph_icon;
#[cfg(feature = "egui")]
use morpheusicons::prelude::*;

#[cfg(feature = "egui")]
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

#[cfg(feature = "egui")]
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

#[cfg(feature = "egui")]
struct EguiMorphApp {
    controller: MorphController,
    current_preset: PresetPair,
    current_spring: SpringConfig,
    is_forward: bool,
}

#[cfg(feature = "egui")]
impl EguiMorphApp {
    fn new(_cc: &eframe::CreationContext<'_>) -> Self {
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
            egui::Button::new(
                egui::RichText::new(text)
                    .size(12.0)
                    .color(egui::Color32::WHITE),
            )
            .fill(egui::Color32::from_rgb(22, 163, 74))
            .stroke(egui::Stroke::new(
                1.0_f32,
                egui::Color32::from_rgb(22, 163, 74),
            ))
            .rounding(egui::Rounding::same(8.0))
        } else {
            egui::Button::new(
                egui::RichText::new(text)
                    .size(12.0)
                    .color(egui::Color32::from_rgb(86, 92, 102)),
            )
            .fill(egui::Color32::from_rgb(255, 255, 255))
            .stroke(egui::Stroke::new(
                1.0_f32,
                egui::Color32::from_rgb(229, 232, 236),
            ))
            .rounding(egui::Rounding::same(8.0))
        };

        if ui.add(button).clicked() {
            self.select_preset(preset);
        }
    }

    fn segment_btn(&mut self, ui: &mut egui::Ui, label: &'static str, config: SpringConfig) {
        let is_selected = self.current_spring == config;
        let text_color = if is_selected {
            egui::Color32::WHITE
        } else {
            egui::Color32::from_rgb(86, 92, 102)
        };
        let bg_color = if is_selected {
            egui::Color32::from_rgb(22, 163, 74)
        } else {
            egui::Color32::from_rgb(249, 250, 251)
        };

        let button = egui::Button::new(egui::RichText::new(label).size(12.0).color(text_color))
            .fill(bg_color)
            .rounding(egui::Rounding::same(6.0));

        if ui.add(button).clicked() {
            self.select_spring(config);
        }
    }
}

#[cfg(feature = "egui")]
impl eframe::App for EguiMorphApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let active = self.controller.update(ctx.input(|i| i.stable_dt));
        if active {
            ctx.request_repaint();
        }

        let (icon_a, icon_b) = self.current_preset.icons();
        let progress = self.controller.progress();

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(255, 255, 255)))
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);

                    // Title Header
                    ui.heading(
                        egui::RichText::new("MorpheusIcons Showcase")
                            .size(24.0)
                            .strong()
                            .color(egui::Color32::from_rgb(17, 19, 23)),
                    );
                    ui.label(
                        egui::RichText::new("Morphing de Ícones Vetoriais em Tempo Real")
                            .size(12.0)
                            .color(egui::Color32::from_rgb(86, 92, 102)),
                    );

                    ui.add_space(16.0);

                    // Segmented Spring Selector
                    ui.horizontal(|ui| {
                        ui.add_space((ui.available_width() - 260.0).max(0.0) / 2.0);
                        ui.label(
                            egui::RichText::new("spring")
                                .size(12.0)
                                .color(egui::Color32::from_rgb(148, 163, 184)),
                        );
                        self.segment_btn(ui, "smooth", SpringConfig::SMOOTH);
                        self.segment_btn(ui, "snappy", SpringConfig::SNAPPY);
                        self.segment_btn(ui, "bouncy", SpringConfig::BOUNCY);
                    });

                    ui.add_space(16.0);

                    // Main Stage Icon Card
                    egui::Frame::none()
                        .fill(egui::Color32::from_rgb(249, 250, 251))
                        .stroke(egui::Stroke::new(
                            1.0_f32,
                            egui::Color32::from_rgb(229, 232, 236),
                        ))
                        .rounding(egui::Rounding::same(16.0))
                        .inner_margin(24.0)
                        .show(ui, |ui| {
                            ui.set_max_width(520.0);
                            ui.vertical_centered(|ui| {
                                // Icon Container Box
                                egui::Frame::none()
                                    .fill(egui::Color32::from_rgb(255, 255, 255))
                                    .stroke(egui::Stroke::new(
                                        1.0_f32,
                                        egui::Color32::from_rgb(229, 232, 236),
                                    ))
                                    .rounding(egui::Rounding::same(12.0))
                                    .inner_margin(22.0)
                                    .show(ui, |ui| {
                                        paint_morph_icon(
                                            ui,
                                            &self.controller,
                                            egui::vec2(96.0, 96.0),
                                            egui::Color32::from_rgb(22, 163, 74),
                                            3.0,
                                        );
                                    });

                                ui.add_space(12.0);

                                // Status Label
                                ui.label(
                                    egui::RichText::new(format!("{:?} <-> {:?}", icon_a, icon_b))
                                        .size(16.0)
                                        .strong()
                                        .color(egui::Color32::from_rgb(248, 250, 252)),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "Progresso: {:.1}%",
                                        progress * 100.0
                                    ))
                                    .size(12.0)
                                    .color(egui::Color32::from_rgb(100, 116, 139)),
                                );

                                ui.add_space(12.0);

                                // Main Toggle Button
                                let target_icon = if self.is_forward { icon_a } else { icon_b };
                                let toggle_btn = egui::Button::new(
                                    egui::RichText::new(format!("Morph para {:?}", target_icon))
                                        .size(14.0)
                                        .strong()
                                        .color(egui::Color32::WHITE),
                                )
                                .fill(egui::Color32::from_rgb(22, 163, 74))
                                .rounding(egui::Rounding::same(12.0));

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
                            .color(egui::Color32::from_rgb(22, 163, 74)),
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
                            .color(egui::Color32::from_rgb(148, 163, 184)),
                    );
                    ui.add_space(6.0);
                    ui.horizontal(|ui| {
                        ui.add_space((ui.available_width() - 360.0).max(0.0) / 2.0);
                        self.preset_button(ui, PresetPair::SunMoon);
                        self.preset_button(ui, PresetPair::PlayPause);
                        self.preset_button(ui, PresetPair::MenuX);
                    });
                    ui.add_space(4.0);
                    ui.horizontal(|ui| {
                        ui.add_space((ui.available_width() - 360.0).max(0.0) / 2.0);
                        self.preset_button(ui, PresetPair::LockUnlock);
                        self.preset_button(ui, PresetPair::EyeEyeOff);
                        self.preset_button(ui, PresetPair::VolumeMute);
                    });
                });
            });
    }
}

fn main() {
    #[cfg(feature = "egui")]
    {
        let options = eframe::NativeOptions {
            viewport: egui::ViewportBuilder::default()
                .with_inner_size([680.0, 780.0])
                .with_title("MorpheusIcons egui Demo"),
            ..Default::default()
        };
        let _ = eframe::run_native(
            "MorpheusIcons egui Demo",
            options,
            Box::new(|cc| Ok(Box::new(EguiMorphApp::new(cc)))),
        );
    }

    #[cfg(not(feature = "egui"))]
    {
        println!("Please run with `--features egui` to enable the egui demo.");
    }
}
