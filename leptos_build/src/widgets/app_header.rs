use ratatui::{style::Stylize, symbols::border, text::Line, widgets::{Block, Widget}};

#[derive(Default, Debug)]
pub struct AppHeader;

impl Widget for &AppHeader{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let title = Line::from("Leptos Build Tool".bold());
        Block::bordered()
        .title(title.centered())
        .border_set(border::THICK).render(area, buf)

    }
}