use color_eyre::Result;
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;

use super::Component;
use crate::{action::Action, state::State};

#[derive(Default)]
pub struct Header {
    command_tx: Option<UnboundedSender<Action>>,
    state: State,
}

impl Header {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Header {
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
            let title = Line::from("Leptos Build Tool".bold());
            let block = Block::new()
            .title(title.centered());
        frame.render_widget(block, area);
        Ok(())
    }
}
