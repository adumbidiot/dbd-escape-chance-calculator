use crate::assets::PerkImages;
use crate::assets::PERK_ICON_SIZE;
use crate::hook_escape_chance_settings::Tier;
use iced::Align;
use iced::Button;
use iced::Container;
use iced::Image;
use iced::Length;
use iced::Row;
use iced::Text;

pub struct PerkPickerState {
    left_button_state: iced::button::State,
    right_button_state: iced::button::State,
}

impl PerkPickerState {
    pub fn new() -> Self {
        Self {
            left_button_state: iced::button::State::new(),
            right_button_state: iced::button::State::new(),
        }
    }
}

impl Default for PerkPickerState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PerkPicker<'a, Message> {
    state: &'a mut PerkPickerState,
    current: Option<Tier>,
    perk_images: &'a PerkImages,

    on_decrease: Message,
    on_increase: Message,
}

impl<'a, Message> PerkPicker<'a, Message> {
    pub fn new(
        state: &'a mut PerkPickerState,
        current: Option<Tier>,
        perk_images: &'a PerkImages,
        on_decrease: Message,
        on_increase: Message,
    ) -> Self {
        Self {
            state,
            current,
            perk_images,

            on_decrease,
            on_increase,
        }
    }
}

impl<'a, Message> PerkPicker<'a, Message>
where
    Message: Clone + 'a,
{
    pub fn into_element(self) -> iced::Element<'a, Message> {
        let lower_button = {
            let mut button = Button::new(&mut self.state.left_button_state, Text::new("<"));

            if self.current.is_some() {
                button = button.on_press(self.on_decrease);
            }

            button
        };

        let higher_button = {
            let mut button = Button::new(&mut self.state.right_button_state, Text::new(">"));

            if self.current != Some(Tier::III) {
                button = button.on_press(self.on_increase);
            }

            button
        };

        let image: iced::Element<_> = {
            match self.current {
                None => Container::new(Text::new("none"))
                    .width(Length::Units(PERK_ICON_SIZE))
                    .height(Length::Units(PERK_ICON_SIZE))
                    .center_x()
                    .center_y()
                    .into(),
                Some(Tier::I) => Image::new(self.perk_images.tier_i.clone()).into(),
                Some(Tier::II) => Image::new(self.perk_images.tier_ii.clone()).into(),
                Some(Tier::III) => Image::new(self.perk_images.tier_iii.clone()).into(),
            }
        };

        Row::new()
            .push(lower_button)
            .push(image)
            .push(higher_button)
            .spacing(10)
            .align_items(Align::Center)
            .into()
    }
}

impl<'a, Message> From<PerkPicker<'a, Message>> for iced::Element<'a, Message>
where
    Message: Clone + 'a,
{
    fn from(el: PerkPicker<'a, Message>) -> Self {
        el.into_element()
    }
}
