use color_eyre::Result;
use ratatui::{prelude::*, symbols::border, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, state::State};

#[derive(Default)]
pub struct Footer {
    command_tx: Option<UnboundedSender<Action>>,
    state: State
}

impl Footer {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Footer {
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) -> Result<()> {
        self.command_tx = Some(tx);
        Ok(())
    }

    fn register_state_handler(&mut self, state: State) -> Result<()> {
        self.state = state;
        Ok(())
    }

    fn update(&mut self, action: Action) -> Result<Option<Action>> {
        match action {
            Action::Tick => {
                // add any logic here that should run on every tick
            }
            Action::Render => {
                // add any logic here that should run on every render
            }
            _ => {}
        }
        Ok(None)
    }

    fn draw(&mut self, frame: &mut Frame, area: Rect) -> Result<()> {
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered()
        .title(instructions.centered())
        .border_set(border::THICK);
        frame.render_widget(block, area);
        Ok(())
    }
}
