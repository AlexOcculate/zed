use gpui::rgba;

use crate::{
    Appearance, ThemeColorsRefinement, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn nord() -> UserThemeFamily {
    UserThemeFamily {
        name: "Nord".into(),
        author: "Sven Greb (svengreb)".into(),
        themes: vec![UserTheme {
            name: "Nord".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    border: Some(rgba(0x3b4252ff).into()),
                    border_variant: Some(rgba(0x3b4252ff).into()),
                    border_focused: Some(rgba(0x3b4252ff).into()),
                    border_selected: Some(rgba(0x3b4252ff).into()),
                    border_transparent: Some(rgba(0x3b4252ff).into()),
                    border_disabled: Some(rgba(0x3b4252ff).into()),
                    elevated_surface_background: Some(rgba(0x2e3440ff).into()),
                    surface_background: Some(rgba(0x2e3440ff).into()),
                    background: Some(rgba(0x2e3440ff).into()),
                    element_background: Some(rgba(0x88bfd0ee).into()),
                    text: Some(rgba(0xd8dee9ff).into()),
                    tab_inactive_background: Some(rgba(0x2e3440ff).into()),
                    tab_active_background: Some(rgba(0x3b4252ff).into()),
                    terminal_background: Some(rgba(0x2e3440ff).into()),
                    terminal_ansi_bright_black: Some(rgba(0x4c566aff).into()),
                    terminal_ansi_bright_red: Some(rgba(0xbf616aff).into()),
                    terminal_ansi_bright_green: Some(rgba(0xa3be8cff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0xebcb8bff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0x81a1c1ff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0xb48eacff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0x8fbcbbff).into()),
                    terminal_ansi_bright_white: Some(rgba(0xeceff4ff).into()),
                    terminal_ansi_black: Some(rgba(0x3b4252ff).into()),
                    terminal_ansi_red: Some(rgba(0xbf616aff).into()),
                    terminal_ansi_green: Some(rgba(0xa3be8cff).into()),
                    terminal_ansi_yellow: Some(rgba(0xebcb8bff).into()),
                    terminal_ansi_blue: Some(rgba(0x81a1c1ff).into()),
                    terminal_ansi_magenta: Some(rgba(0xb48eacff).into()),
                    terminal_ansi_cyan: Some(rgba(0x88bfd0ff).into()),
                    terminal_ansi_white: Some(rgba(0xe5e9f0ff).into()),
                    ..Default::default()
                },
            },
        }],
    }
}