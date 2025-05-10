use color_eyre::Result;
use ratatui::prelude::*;
use tokio::sync::mpsc::UnboundedSender;

use super::{command_output::CommandOutput, footer::Footer, header::Header, Component};
use crate::{action::Action, state::State};

#[derive(Default)]
pub struct Home {
    command_tx: Option<UnboundedSender<Action>>,
    state: State,
}

impl Home {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Component for Home {
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
        let layout: [Rect; 3] = Layout::default().direction(Direction::Vertical).constraints([Constraint::Ratio(1,10), Constraint::Ratio(8, 10), Constraint::Ratio(1,10)]).areas(area);
        Header::new().draw(frame, layout[0])?;
        CommandOutput::new().draw(frame, layout[1])?;
        Footer::new().draw(frame, layout[2])?;


        Ok(())
    }
}
