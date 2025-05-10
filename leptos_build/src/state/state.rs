use std::sync::Arc;
use parking_lot::RwLock;
use crate::{cli::Cli, command::CommandCollection, config::Config};

#[derive(Default, Clone)]
pub struct State{
    pub cli: Arc<RwLock<Cli>>,
    pub config: Arc<RwLock<Config>>,
    pub commands: Arc<RwLock<CommandCollection>>
}

impl State{
    pub fn new(cli: Cli, config: Config, commands: CommandCollection)-> Self{
        Self{
            cli: Arc::new(RwLock::new(cli)),
            config: Arc::new(RwLock::new(config)),
            commands: Arc::new(RwLock::new(commands))
        }
    }
}