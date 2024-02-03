use std::rc::Rc;
use iced::{Background, BorderRadius, Color};
use iced::theme::Container as ThemeContainer;
use iced::theme::Scrollable as ThemeScrollable;
use iced::theme::Button as ThemeButton;
use iced::theme::TextInput as ThemeTextInput;
use iced::theme::PickList as ThemePickList;
use iced::theme::Checkbox as ThemeCheckbox;
use iced::theme::Menu as ThemeMenu;
use iced_style::menu::{Appearance as MenuAppearance, StyleSheet as MenuStyleSheet};
use iced::widget::scrollable::StyleSheet as ThemeScrollableStyleSheet;

use iced::widget::container::{Appearance as ContainerAppearance, StyleSheet};
use iced::widget::scrollable::{Scrollbar, Scroller};
use iced::widget::button::{Appearance as ButtonAppearance, StyleSheet as ButtonStyleSheet};
use iced::widget::text_input::{Appearance as TextInputAppearance, StyleSheet as TextInputStyleSheet};
use iced::widget::pick_list::{Appearance as PickListAppearance, StyleSheet as PickListStyleSheet};
use iced::widget::checkbox::{Appearance as CheckboxAppearance, StyleSheet as CheckboxStyleSheet};

#[derive(Debug, Clone)]
enum ThemeType {
    Custom,
}

pub fn container_theme() -> ThemeContainer {
    ThemeContainer::Custom(Box::new(ContainerTheme) as Box<dyn StyleSheet<Style = iced::Theme>>)
}

#[derive(Debug, Clone, Copy)]
struct ContainerTheme;

pub fn scrollable_theme() -> ThemeScrollable {
    ThemeScrollable::Custom(
        Box::new(ScrollableTheme) as Box<dyn ThemeScrollableStyleSheet<Style = iced::Theme>>
    )
}
#[derive(Debug, Clone, Copy)]
struct ScrollableTheme;

impl ThemeScrollableStyleSheet for ScrollableTheme {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> Scrollbar {
        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;
        Scrollbar {
            background: Some(Background::Color(Color::from_rgb(0.0, 0.0, 0.0))), // Customize the background color of the scrollbar
            border_radius: BorderRadius::from(5.0), // Customize the border radiux s of the scrollbar
            border_width: 0.0,                      // Customize the border width of the scrollbar
            border_color: Color::from_rgb(0.5, 0.5, 0.5), // Customize the border color of the scrollbar
            scroller: Scroller {
                color: Color::from_rgb(r, g, b),
                border_radius: BorderRadius::from(5.0), // Customize the border radius of the scroller
                border_width: 0.0, // Customize the border width of the scroller
                border_color: Color::from_rgb(0.5, 0.5, 0.5), // Customize the border color of the scroller
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_mouse_over_scrollbar: bool) -> Scrollbar {
        self.active(style)
    }
}

impl StyleSheet for ContainerTheme {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> ContainerAppearance {
        let mut appearance = ContainerAppearance {
            border_radius: BorderRadius::from(5.0),
            ..ContainerAppearance::default()
        };

        let red = 6.7 / 255.0;
        let green = 6.7 / 255.0;
        let blue = 6.7 / 255.0;

        appearance.background = Some(Background::Color(Color::from_rgb(
            red + 0.1,
            green + 0.1,
            blue + 0.1,
        )));
        appearance
    }
}

#[derive(Debug, Clone, Copy)]
struct ButtonTheme;

pub fn button_theme() -> ThemeButton {
    ThemeButton::Custom(Box::new(ButtonTheme) as Box<dyn ButtonStyleSheet<Style = iced::Theme>>)
}

impl ButtonStyleSheet for ButtonTheme {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> ButtonAppearance {

        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;

        let mut appearance = ButtonAppearance {
            border_radius: BorderRadius::from(2.0),
            background: Some(Background::Color(Color::from_rgb(r, g, b))),
            text_color: Color::from_rgb(0.8, 0.8, 0.8),
            ..ButtonAppearance::default()
        };

        appearance
    }

    fn hovered(&self, style: &Self::Style) -> ButtonAppearance {
        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;
        let mut appearance = ButtonAppearance {
            border_radius: BorderRadius::from(2.0),
            background: Some(Background::Color(Color::from_rgb(r + 0.2, g + 0.2, b + 0.2))),
            text_color: Color::from_rgb(0.8, 0.8, 0.8),
            ..ButtonAppearance::default()
        };

        appearance
    }

    fn pressed(&self, style: &Self::Style) -> ButtonAppearance {

        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;

        let mut appearance = ButtonAppearance {
            border_radius: BorderRadius::from(2.0),
            background: Some(Background::Color(Color::from_rgb(r + 0.3, g + 0.3, b + 0.3))),
            ..ButtonAppearance::default()
        };

        appearance
    }
}

#[derive(Debug, Clone, Copy)]
struct InputTheme;

pub fn text_input_theme() -> ThemeTextInput {
    ThemeTextInput::Custom(Box::new(InputTheme) as Box<dyn TextInputStyleSheet<Style = iced::Theme>>)
}

impl TextInputStyleSheet for InputTheme {
    type Style = iced::Theme;

    fn active(&self, style: &Self::Style) -> TextInputAppearance {
        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;
        let mut appearance = TextInputAppearance {
            background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)),
            border_radius: BorderRadius::from(2.0),
            border_color: Color::from_rgb(r, g, b),
            border_width: 0.4,
            icon_color: Color::from_rgb(0.5, 0.5, 0.5),
        };
        appearance
    }

    fn focused(&self, style: &Self::Style) -> TextInputAppearance {
        self.active(style)
    }

    fn hovered(&self, style: &Self::Style) -> TextInputAppearance {
        self.active(style)
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        Color::from_rgb(0.5, 0.5, 0.5)
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        Color::from_rgb(0.8, 0.8, 0.8)
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;
        Color::from_rgb(r, g, b)
    }

    fn disabled(&self, style: &Self::Style) -> TextInputAppearance {
        self.active(style)
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        Color::from_rgb(1.0, 1.0, 1.0)
    }
}

#[derive(Debug, Clone, Copy)]
struct CheckboxTheme;

pub fn checkbox_theme() -> ThemeCheckbox {
    ThemeCheckbox::Custom(Box::new(CheckboxTheme) as Box<dyn CheckboxStyleSheet<Style = iced::Theme>>)
}

impl CheckboxStyleSheet for CheckboxTheme {
    type Style = iced::Theme;
    fn active(&self, style: &Self::Style, is_checked: bool) -> CheckboxAppearance {
        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;
        let mut appearance = CheckboxAppearance {
            background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)),
            border_radius: BorderRadius::from(2.0),
            border_width: 0.4,
            border_color: Color::from_rgb(r, g, b),
            icon_color: Color::from_rgb(r, g, b),
            text_color: Some(Color::from_rgb(0.8, 0.8, 0.8)),
        };
        appearance
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> CheckboxAppearance {
        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;
        let mut appearance = CheckboxAppearance {
            background: Background::Color(Color::from_rgb(r - 0.2, g - 0.2, b - 0.2)),
            border_radius: BorderRadius::from(2.0),
            border_width: 0.4,
            border_color: Color::from_rgb(r, g, b),
            icon_color: Color::from_rgb(r + 0.1, g + 0.1, b + 0.1),
            text_color: Some(Color::from_rgb(0.8, 0.8, 0.8)),
        };
        appearance
    }
}

#[derive(Debug, Clone, Copy)]
struct PickListTheme;

#[derive(Debug, Clone, Copy)]
struct MenuTheme;

impl MenuStyleSheet for MenuTheme {
    type Style = iced::Theme;

    fn appearance(&self, style: &Self::Style) -> MenuAppearance {
        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;
        MenuAppearance {
            background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)),
            border_radius: BorderRadius::from(2.0),
            border_width: 0.4,
            border_color: Color::from_rgb(0.5, 0.5, 0.5),
            text_color: Color::from_rgb(0.8, 0.8, 0.8),
            selected_text_color: Color::from_rgb(0.8, 0.8, 0.8),
            selected_background: Background::Color(Color::from_rgb(r,g,b)),
        }
    }

}

pub fn pick_list_theme() -> ThemePickList {
    ThemePickList::Custom(
        Rc::new(PickListTheme) as Rc<dyn PickListStyleSheet<Style = iced::Theme>>,
        Rc::new(MenuTheme) as Rc<dyn MenuStyleSheet<Style = iced::Theme>>,
    )
}
impl PickListStyleSheet for PickListTheme {
    type Style = iced::Theme;

    fn active(&self, _style: &Self::Style) -> PickListAppearance {
        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;
        PickListAppearance {
            background: Background::Color(Color::from_rgb(0.2, 0.2, 0.2)),
            border_radius: BorderRadius::from(2.0),
            border_width: 0.4,
            border_color: Color::from_rgb(r, g, b),
            text_color: Color::from_rgb(0.8, 0.8, 0.8),
            placeholder_color: Color::from_rgb(0.8, 0.8, 0.8),
            handle_color: Color::from_rgb(r, g, b),
        }
    }

    fn hovered(&self, style: &Self::Style) -> PickListAppearance {
        let r = 73.3 / 100.0;
        let g = 15.7 / 100.0;
        let b = 68.6 / 100.0;
        PickListAppearance{
            background: Background::Color(Color::from_rgb(0.4, 0.4, 0.4)),
            border_radius: BorderRadius::from(2.0),
            border_width: 0.4,
            border_color: Color::from_rgb(r, g, b),
            text_color: Color::from_rgb(0.8, 0.8, 0.8),
            placeholder_color: Color::from_rgb(0.8, 0.8, 0.8),
            handle_color: Color::from_rgb(r, g, b),
        }
    }
}

// use iced_aw::card::StyleSheet as CardStyleSheet;
// use iced_aw::card::Appearance as CardAppearance;
// use iced_aw::style::card as ThemeCard;
