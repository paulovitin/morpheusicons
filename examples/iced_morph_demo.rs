//! Interactive Iced GUI Morphing Demo for MorpheusIcons
//!
//! Run with: `make run iced`

#[cfg(feature = "iced")]
use iced::widget::{button, column, container, row, text};
#[cfg(feature = "iced")]
use iced::{window, Alignment, Element, Length, Subscription, Task, Theme};
#[cfg(feature = "iced")]
use morpheusicons::integrations::iced::MorpheusIced;
#[cfg(feature = "iced")]
use morpheusicons::prelude::*;

#[cfg(feature = "iced")]
fn main() -> iced::Result {
    iced::application(
        "MorpheusIcons - Iced Showcase",
        IcedMorphDemo::update,
        IcedMorphDemo::view,
    )
    .subscription(IcedMorphDemo::subscription)
    .theme(IcedMorphDemo::theme)
    .run()
}

#[cfg(not(feature = "iced"))]
fn main() {
    println!("Run with `--features iced` to launch the Iced demo!");
}

#[cfg(feature = "iced")]
#[derive(Clone, Copy, PartialEq, Debug)]
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

#[cfg(feature = "iced")]
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

#[cfg(feature = "iced")]
struct IcedMorphDemo {
    controller: MorphController,
    current_preset: PresetPair,
    current_spring: SpringConfig,
    is_forward: bool,
}

#[cfg(feature = "iced")]
#[derive(Debug, Clone, Copy)]
enum Message {
    SelectPreset(PresetPair),
    SelectSpring(SpringConfig),
    Toggle,
    Tick,
}

#[cfg(feature = "iced")]
impl Default for IcedMorphDemo {
    fn default() -> Self {
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
}

#[cfg(feature = "iced")]
impl IcedMorphDemo {
    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SelectPreset(preset) => {
                self.current_preset = preset;
                let (from, to) = preset.icons();
                self.controller =
                    MorphController::new(from.path_data(), to.path_data(), self.current_spring)
                        .unwrap();
                self.is_forward = false;
            }
            Message::SelectSpring(spring) => {
                self.current_spring = spring;
                let (from, to) = self.current_preset.icons();
                let progress = self.controller.progress();
                self.controller =
                    MorphController::new(from.path_data(), to.path_data(), spring).unwrap();
                self.controller.set_progress(progress);
            }
            Message::Toggle => {
                self.is_forward = !self.is_forward;
                self.controller.toggle();
            }
            Message::Tick => {
                self.controller.update(0.016);
            }
        }
        Task::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        window::frames().map(|_| Message::Tick)
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }

    fn view(&self) -> Element<'_, Message> {
        let (icon_a, icon_b) = self.current_preset.icons();
        let progress = self.controller.progress();
        let target_icon = if self.is_forward { icon_a } else { icon_b };

        let icon_widget = MorpheusIced::morph_svg::<Theme>(&self.controller)
            .width(Length::Fixed(96.0))
            .height(Length::Fixed(96.0));

        let spring_selector = row![
            text("spring:").size(14),
            button(text("smooth")).on_press(Message::SelectSpring(SpringConfig::SMOOTH)),
            button(text("snappy")).on_press(Message::SelectSpring(SpringConfig::SNAPPY)),
            button(text("bouncy")).on_press(Message::SelectSpring(SpringConfig::BOUNCY)),
        ]
        .spacing(8)
        .align_y(Alignment::Center);

        let icon_card = container(
            column![
                container(icon_widget).padding(20),
                text(format!("{:?} <-> {:?}", icon_a, icon_b)).size(18),
                text(format!("Progresso: {:.1}%", progress * 100.0)).size(14),
                button(text(format!("Morph para {:?}", target_icon))).on_press(Message::Toggle),
            ]
            .spacing(12)
            .align_x(Alignment::Center),
        )
        .padding(24)
        .width(Length::Fixed(520.0));

        let wild_pairs = row![
            button(text(PresetPair::HeartTerminal.name()))
                .on_press(Message::SelectPreset(PresetPair::HeartTerminal)),
            button(text(PresetPair::CpuSun.name()))
                .on_press(Message::SelectPreset(PresetPair::CpuSun)),
            button(text(PresetPair::BellCode.name()))
                .on_press(Message::SelectPreset(PresetPair::BellCode)),
            button(text(PresetPair::MailZap.name()))
                .on_press(Message::SelectPreset(PresetPair::MailZap)),
        ]
        .spacing(8);

        let wild_pairs2 = row![
            button(text(PresetPair::TrashRefresh.name()))
                .on_press(Message::SelectPreset(PresetPair::TrashRefresh)),
            button(text(PresetPair::FolderStar.name()))
                .on_press(Message::SelectPreset(PresetPair::FolderStar)),
            button(text(PresetPair::UserLock.name()))
                .on_press(Message::SelectPreset(PresetPair::UserLock)),
            button(text(PresetPair::CalendarPlay.name()))
                .on_press(Message::SelectPreset(PresetPair::CalendarPlay)),
        ]
        .spacing(8);

        let ui_pairs = row![
            button(text(PresetPair::SunMoon.name()))
                .on_press(Message::SelectPreset(PresetPair::SunMoon)),
            button(text(PresetPair::PlayPause.name()))
                .on_press(Message::SelectPreset(PresetPair::PlayPause)),
            button(text(PresetPair::MenuX.name()))
                .on_press(Message::SelectPreset(PresetPair::MenuX)),
            button(text(PresetPair::LockUnlock.name()))
                .on_press(Message::SelectPreset(PresetPair::LockUnlock)),
        ]
        .spacing(8);

        let ui_pairs2 = row![
            button(text(PresetPair::EyeEyeOff.name()))
                .on_press(Message::SelectPreset(PresetPair::EyeEyeOff)),
            button(text(PresetPair::VolumeMute.name()))
                .on_press(Message::SelectPreset(PresetPair::VolumeMute)),
        ]
        .spacing(8);

        let content = column![
            text("MorpheusIcons Showcase").size(26),
            text("Morphing de Ícones Vetoriais em Tempo Real (Iced)").size(14),
            spring_selector,
            icon_card,
            text("🔥 Ícones Inusitados:").size(14),
            wild_pairs,
            wild_pairs2,
            text("Pares Tradicionais de UI:").size(14),
            ui_pairs,
            ui_pairs2,
        ]
        .spacing(14)
        .align_x(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
