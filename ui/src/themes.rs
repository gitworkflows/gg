//! Defines common UI themes and styling.

use iced::{Color, Theme};

/// A simple theme struct for UI elements.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppTheme {
    Light,
    Dark,
}

impl AppTheme {
    pub fn background_color(&self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb(0.95, 0.95, 0.95),
            AppTheme::Dark => Color::from_rgb(0.15, 0.15, 0.15),
        }
    }

    pub fn foreground_color(&self) -> Color {
        match self {
            AppTheme::Light => Color::BLACK,
            AppTheme::Dark => Color::WHITE,
        }
    }

    pub fn accent_color(&self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb(0.0, 0.5, 1.0), // Blue
            AppTheme::Dark => Color::from_rgb(0.2, 0.7, 1.0), // Lighter blue
        }
    }

    pub fn text_input_background(&self) -> Color {
        match self {
            AppTheme::Light => Color::WHITE,
            AppTheme::Dark => Color::from_rgb(0.2, 0.2, 0.2),
        }
    }

    pub fn text_input_border(&self) -> Color {
        match self {
            AppTheme::Light => Color::from_rgb(0.8, 0.8, 0.8),
            AppTheme::Dark => Color::from_rgb(0.3, 0.3, 0.3),
        }
    }
}

impl From<AppTheme> for Theme {
    fn from(app_theme: AppTheme) -> Self {
        match app_theme {
            AppTheme::Light => Theme::Light,
            AppTheme::Dark => Theme::Dark,
        }
    }
}
