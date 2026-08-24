#[cfg(feature = "gpui")]
use gpui::*;
#[cfg(feature = "gpui")]
use morpheusicons::integrations::gpui::MorpheusAssetSource;
#[cfg(feature = "gpui")]
use morpheusicons::prelude::*;

#[cfg(feature = "gpui")]
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

#[cfg(feature = "gpui")]
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

#[cfg(feature = "gpui")]
struct MorphDemo {
    controller: MorphController,
    current_preset: PresetPair,
    current_spring: SpringConfig,
    is_forward: bool,
}

#[cfg(feature = "gpui")]
impl MorphDemo {
    fn new(_cx: &mut Context<Self>) -> Self {
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

    fn select_preset(&mut self, preset: PresetPair, window: &mut Window, cx: &mut Context<Self>) {
        self.current_preset = preset;
        let (from, to) = preset.icons();
        self.controller = MorphController::new(from.path_data(), to.path_data(), self.current_spring)
            .unwrap();
        self.is_forward = false;
        self.schedule_next_frame(window, cx);
    }

    fn select_spring(&mut self, spring: SpringConfig, window: &mut Window, cx: &mut Context<Self>) {
        self.current_spring = spring;
        let (from, to) = self.current_preset.icons();
        let progress = self.controller.progress();
        self.controller = MorphController::new(from.path_data(), to.path_data(), spring).unwrap();
        self.controller.set_progress(progress);
        self.schedule_next_frame(window, cx);
    }

    fn toggle_icon(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.is_forward = !self.is_forward;
        self.controller.toggle();
        self.schedule_next_frame(window, cx);
    }

    fn schedule_next_frame(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        cx.on_next_frame(window, move |this: &mut Self, _window: &mut Window, cx: &mut Context<Self>| {
            let active = this.controller.update(0.016);
            cx.notify();
            if active {
                this.schedule_next_frame(_window, cx);
            }
        });
    }
}

#[cfg(feature = "gpui")]
impl Render for MorphDemo {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let svg_element = MorpheusGpui::morph_svg(&self.controller)
            .size(px(96.0))
            .text_color(rgb(0x16a34a));

        let (icon_a, icon_b) = self.current_preset.icons();
        let progress = self.controller.progress();

        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_start()
            .size_full()
            .bg(rgb(0xffffff))
            .text_color(rgb(0x111317))
            .p_8()
            .gap_6()
            // Title Header
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .text_2xl()
                            .font_weight(FontWeight::BOLD)
                            .child("MorpheusIcons Showcase"),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(rgb(0x94a3b8))
                            .child("Morphing de Ícones Vetoriais em Tempo Real"),
                    ),
            )
            // Segmented Spring Selector
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(rgb(0x94a3b8))
                            .child("spring"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .h(px(36.0))
                            .p_1()
                            .rounded_lg()
                            .bg(rgb(0xf9fafb))
                            .border_1()
                            .border_color(rgb(0xe5e8ec))
                            .child(self.segment_btn("smooth", SpringConfig::SMOOTH, cx))
                            .child(self.segment_btn("snappy", SpringConfig::SNAPPY, cx))
                            .child(self.segment_btn("bouncy", SpringConfig::BOUNCY, cx)),
                    ),
            )
            // Main Stage Icon Card
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_4()
                    .p_6()
                    .w_full()
                    .max_w(px(520.0))
                    .rounded_2xl()
                    .bg(rgb(0xf9fafb))
                    .border_1()
                    .border_color(rgb(0xe5e8ec))
                    .shadow_lg()
                    // Icon Container
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .justify_center()
                            .size(px(140.0))
                            .rounded_xl()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe5e8ec))
                            .child(svg_element),
                    )
                    // Status Label
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .gap_1()
                            .child(
                                div()
                                    .text_base()
                                    .font_weight(FontWeight::BOLD)
                                    .child(format!("{:?} <-> {:?}", icon_a, icon_b)),
                            )
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(rgb(0x64748b))
                                    .child(format!("Progresso: {:.1}%", progress * 100.0)),
                            ),
                    )
                    // Main Toggle Button
                    .child(
                        div()
                            .id("toggle_btn")
                            .px_6()
                            .py_3()
                            .rounded_xl()
                            .bg(rgb(0x16a34a))
                            .text_color(rgb(0xffffff))
                            .font_weight(FontWeight::BOLD)
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event: &MouseDownEvent, window, cx| {
                                    this.toggle_icon(window, cx);
                                }),
                            )
                            .child(format!("Morph para {:?}", if self.is_forward { icon_a } else { icon_b })),
                    ),
            )
            // Wild Unrelated Section
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_3()
                    .w_full()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x38bdf8))
                            .child("🔥 Ícones Inusitados:"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(self.preset_button(PresetPair::HeartTerminal, cx))
                            .child(self.preset_button(PresetPair::CpuSun, cx))
                            .child(self.preset_button(PresetPair::BellCode, cx))
                            .child(self.preset_button(PresetPair::MailZap, cx)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(self.preset_button(PresetPair::TrashRefresh, cx))
                            .child(self.preset_button(PresetPair::FolderStar, cx))
                            .child(self.preset_button(PresetPair::UserLock, cx))
                            .child(self.preset_button(PresetPair::CalendarPlay, cx)),
                    ),
            )
            // Traditional UI Section
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_3()
                    .w_full()
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(rgb(0x94a3b8))
                            .child("Pares Tradicionais de UI:"),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(self.preset_button(PresetPair::SunMoon, cx))
                            .child(self.preset_button(PresetPair::PlayPause, cx))
                            .child(self.preset_button(PresetPair::MenuX, cx)),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_center()
                            .gap_2()
                            .child(self.preset_button(PresetPair::LockUnlock, cx))
                            .child(self.preset_button(PresetPair::EyeEyeOff, cx))
                            .child(self.preset_button(PresetPair::VolumeMute, cx)),
                    ),
            )
    }
}

#[cfg(feature = "gpui")]
impl MorphDemo {
    fn preset_button(&self, preset: PresetPair, cx: &mut Context<Self>) -> impl IntoElement {
        let is_selected = self.current_preset == preset;
        let bg_color = if is_selected {
            rgb(0x16a34a)
        } else {
            rgb(0xffffff)
        };
        let text_col = if is_selected {
            rgb(0xffffff)
        } else {
            rgb(0x565c66)
        };

        div()
            .id(SharedString::from(preset.name()))
            .px_3()
            .py_2()
            .rounded_lg()
            .bg(bg_color)
            .border_1()
            .border_color(if is_selected { rgb(0x16a34a) } else { rgb(0xe5e8ec) })
            .text_color(text_col)
            .text_xs()
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _event: &MouseDownEvent, window, cx| {
                    this.select_preset(preset, window, cx);
                }),
            )
            .child(preset.name())
    }

    fn segment_btn(
        &self,
        label: &'static str,
        config: SpringConfig,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let is_selected = self.current_spring == config;
        let bg_color = if is_selected {
            rgb(0x16a34a)
        } else {
            rgb(0xf9fafb)
        };
        let text_col = if is_selected {
            rgb(0xffffff)
        } else {
            rgb(0x565c66)
        };

        div()
            .id(SharedString::from(label))
            .flex()
            .items_center()
            .justify_center()
            .h_full()
            .px_4()
            .rounded_md()
            .bg(bg_color)
            .text_color(text_col)
            .text_xs()
            .font_weight(if is_selected { FontWeight::BOLD } else { FontWeight::MEDIUM })
            .cursor_pointer()
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(move |this, _event: &MouseDownEvent, window, cx| {
                    this.select_spring(config, window, cx);
                }),
            )
            .child(label)
    }
}

fn main() {
    #[cfg(feature = "gpui")]
    {
        Application::new()
            .with_assets(MorpheusAssetSource)
            .run(|cx: &mut App| {
                cx.open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(Bounds::centered(
                            None,
                            size(px(680.0), px(780.0)),
                            cx,
                        ))),
                        ..Default::default()
                    },
                    |_window, cx| cx.new(MorphDemo::new),
                )
                .unwrap();
            });
    }

    #[cfg(not(feature = "gpui"))]
    {
        println!("Please run with `--features gpui` to enable the GPUI demo.");
    }
}
