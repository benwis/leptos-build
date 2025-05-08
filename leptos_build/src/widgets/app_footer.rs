use ratatui::{style::Stylize, symbols::border, text::Line, widgets::{Block, Widget}};

#[derive(Default, Debug)]
pub struct AppFooter;

impl Widget for &AppFooter{
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized {
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        Block::bordered()
        .title(instructions.centered())
        .border_set(border::THICK).render(area, buf)

    }
}