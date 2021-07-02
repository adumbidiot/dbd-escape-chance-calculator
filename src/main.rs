#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod hook_escape_chance_settings;
mod perk_picker;

use self::assets::Assets;
use self::assets::PerkImages;
use self::hook_escape_chance_settings::HookEscapeChanceSettings;
use self::perk_picker::PerkPicker;
use self::perk_picker::PerkPickerState;
use anyhow::Context;
use iced::Align;
use iced::Column;
use iced::Container;
use iced::Scrollable;
use iced::Space;
use iced::Text;
use iced::TextInput;
use iced::{Application, Clipboard, Command, Element, Length, Settings};

#[derive(Debug, Clone)]
pub enum Message {
    IncreaseSlipperyMeat,
    DecreaseSlipperyMeat,

    IncreaseUpTheAnte(u8),
    DecreaseUpTheAnte(u8),

    NumSaltyLipsChange(String),
    NumAliveSurvivorsChange(String),
}

pub struct App {
    // assets: Assets,
    escape_chance_settings: HookEscapeChanceSettings,

    scrollable_state: iced::scrollable::State,

    slippery_meat_perk_picker_state: PerkPickerState,
    slippery_meat_perk_images: PerkImages,

    up_the_ante_player_1_perk_picker_state: PerkPickerState,
    up_the_ante_player_2_perk_picker_state: PerkPickerState,
    up_the_ante_player_3_perk_picker_state: PerkPickerState,
    up_the_ante_player_4_perk_picker_state: PerkPickerState,
    up_the_ante_perk_images: PerkImages,

    salty_lips_input_state: iced::text_input::State,
    num_salty_lips_str: String,

    suvivors_alive_input_state: iced::text_input::State,
    num_alive_survivors_str: String,
}

impl Application for App {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = Assets;

    fn new(assets: Assets) -> (Self, Command<Message>) {
        let slippery_meat_perk_images = assets.slippery_meat_perk_images;

        (
            App {
                // assets,
                escape_chance_settings: HookEscapeChanceSettings::new(),

                scrollable_state: iced::scrollable::State::new(),

                slippery_meat_perk_picker_state: PerkPickerState::new(),
                slippery_meat_perk_images,

                up_the_ante_player_1_perk_picker_state: PerkPickerState::new(),
                up_the_ante_player_2_perk_picker_state: PerkPickerState::new(),
                up_the_ante_player_3_perk_picker_state: PerkPickerState::new(),
                up_the_ante_player_4_perk_picker_state: PerkPickerState::new(),
                up_the_ante_perk_images: assets.up_the_ante_perk_images,

                salty_lips_input_state: iced::text_input::State::new(),
                num_salty_lips_str: String::new(),

                suvivors_alive_input_state: iced::text_input::State::new(),
                num_alive_survivors_str: String::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("DBD Escape Chance Calculator")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::IncreaseSlipperyMeat => {
                self.escape_chance_settings.increment_slippery_meat_tier();
                Command::none()
            }
            Message::DecreaseSlipperyMeat => {
                self.escape_chance_settings.decrement_slippery_meat_tier();
                Command::none()
            }
            Message::IncreaseUpTheAnte(index) => {
                self.escape_chance_settings
                    .increment_up_the_ante_tier(index);
                Command::none()
            }
            Message::DecreaseUpTheAnte(index) => {
                self.escape_chance_settings
                    .decrement_up_the_ante_tier(index);
                Command::none()
            }
            Message::NumSaltyLipsChange(text) => {
                if text.is_empty() {
                    self.num_salty_lips_str.clear();
                    self.escape_chance_settings.num_salty_lips = 0;
                } else if let Ok(parsed) = text.parse::<u8>() {
                    if (0..5).contains(&parsed) {
                        self.num_salty_lips_str = text;
                        self.escape_chance_settings.num_salty_lips = parsed;
                    }
                }

                Command::none()
            }
            Message::NumAliveSurvivorsChange(text) => {
                if text.is_empty() {
                    self.num_alive_survivors_str.clear();
                    self.escape_chance_settings.num_alive_survivors = 1;
                } else if let Ok(parsed) = text.parse::<u8>() {
                    if (1..5).contains(&parsed) {
                        self.num_alive_survivors_str = text;
                        self.escape_chance_settings.num_alive_survivors = parsed;
                    }
                }

                Command::none()
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let slippery_meat_column = Column::new()
            .push(Text::new("Slippery Meat").size(20))
            .push(PerkPicker::new(
                &mut self.slippery_meat_perk_picker_state,
                self.escape_chance_settings.slippery_meat,
                &self.slippery_meat_perk_images,
                Message::DecreaseSlipperyMeat,
                Message::IncreaseSlipperyMeat,
            ))
            .align_items(Align::Center)
            .spacing(10);

        let up_the_ante_row_1 = Column::new()
            .push(Text::new("Up the Ante (Player 1)").size(20))
            .push(PerkPicker::new(
                &mut self.up_the_ante_player_1_perk_picker_state,
                self.escape_chance_settings.up_the_ante[0],
                &self.up_the_ante_perk_images,
                Message::DecreaseUpTheAnte(0),
                Message::IncreaseUpTheAnte(0),
            ))
            .align_items(Align::Center)
            .spacing(10);

        let body = Column::new()
            .push(Text::new("Dead by Daylight").size(40))
            .push(Text::new("Hook Escape Calculator").size(30))
            .push(Space::new(Length::Shrink, Length::Units(10)))
            .push(slippery_meat_column)
            .push(up_the_ante_row_1)
            .push(
                Column::new()
                    .push(Text::new("Up the Ante (Player 2)").size(20))
                    .push(PerkPicker::new(
                        &mut self.up_the_ante_player_2_perk_picker_state,
                        self.escape_chance_settings.up_the_ante[1],
                        &self.up_the_ante_perk_images,
                        Message::DecreaseUpTheAnte(1),
                        Message::IncreaseUpTheAnte(1),
                    ))
                    .align_items(Align::Center)
                    .spacing(10),
            )
            .push(
                Column::new()
                    .push(Text::new("Up the Ante (Player 3)").size(20))
                    .push(PerkPicker::new(
                        &mut self.up_the_ante_player_3_perk_picker_state,
                        self.escape_chance_settings.up_the_ante[2],
                        &self.up_the_ante_perk_images,
                        Message::DecreaseUpTheAnte(2),
                        Message::IncreaseUpTheAnte(2),
                    ))
                    .align_items(Align::Center)
                    .spacing(10),
            )
            .push(
                Column::new()
                    .push(Text::new("Up the Ante (Player 4)").size(20))
                    .push(PerkPicker::new(
                        &mut self.up_the_ante_player_4_perk_picker_state,
                        self.escape_chance_settings.up_the_ante[3],
                        &self.up_the_ante_perk_images,
                        Message::DecreaseUpTheAnte(3),
                        Message::IncreaseUpTheAnte(3),
                    ))
                    .align_items(Align::Center)
                    .spacing(10),
            )
            .push(
                Column::new()
                    .push(Text::new("Number of Salty Lips").size(20))
                    .push(
                        TextInput::new(
                            &mut self.salty_lips_input_state,
                            "# of salty lips",
                            &self.num_salty_lips_str,
                            Message::NumSaltyLipsChange,
                        )
                        .padding(10),
                    ),
            )
            .push(
                Column::new()
                    .push(Text::new("Number of Alive Survivors").size(20))
                    .push(
                        TextInput::new(
                            &mut self.suvivors_alive_input_state,
                            "# of alive survivors",
                            &self.num_alive_survivors_str,
                            Message::NumAliveSurvivorsChange,
                        )
                        .padding(10),
                    ),
            )
            .align_items(Align::Center)
            .width(Length::Fill)
            .spacing(20);

        Container::new(
            Column::new()
                .push(
                    Scrollable::new(&mut self.scrollable_state)
                        .push(body)
                        .padding(20)
                        .width(Length::Fill)
                        .height(Length::Fill),
                )
                .push(
                    Container::new(Text::new(format!(
                        "Total Escape Chance: {}%",
                        self.escape_chance_settings.calculate() * 100.0_f64
                    )))
                    .padding(20)
                    .style(ContainerForegroundStyle)
                    .width(Length::Fill),
                ),
        )
        .style(ContainerBackgroundStyle)
        .into()
    }
}

fn main() -> anyhow::Result<()> {
    let assets = Assets::new().context("failed to load assets")?;
    let mut settings = Settings::with_flags(assets);
    settings.window.size = (640, 480);
    App::run(settings).context("failed to run app")?;

    Ok(())
}

pub struct ContainerBackgroundStyle;

impl iced::container::StyleSheet for ContainerBackgroundStyle {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            background: iced::Color::BLACK.into(), // 0x3F, 0x3F, 0x3F
            text_color: iced::Color::WHITE.into(), // iced::Color::from_rgb8(0xFF, 0x00, 0x00).into(),
            ..iced::container::Style::default()
        }
    }
}

pub struct ContainerForegroundStyle;

impl iced::container::StyleSheet for ContainerForegroundStyle {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            background: iced::Color::from_rgb8(0x31, 0x36, 0x38).into(),
            text_color: iced::Color::WHITE.into(), // iced::Color::from_rgb8(0xFF, 0x00, 0x00).into(),
            ..iced::container::Style::default()
        }
    }
}
