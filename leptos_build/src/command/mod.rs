use std::collections::HashMap;

mod build;
// mod end2end;
mod new;
// mod serve;
// mod test;
// pub mod watch;

pub use build::build_all;
use eyre::Result;
// pub use end2end::end2end_all;
pub use new::NewCommand;

use crate::plugin_host::host::WasiCommand;
// pub use serve::serve;
// pub use test::test_all;
// pub use watch::watch;

#[derive(Clone)]
pub struct BuiltinCommand{

}

//cmd: String, tx: mpsc::UnboundedSender<String>

/// The collection of installed commands
#[derive(Default)]
pub struct CommandCollection(
    HashMap<String, LeptosBuildCommand>
);


/// The two types of Command that leptos-build supports
#[derive(Clone)]
pub enum LeptosBuildCommand {
    Builtin (BuiltinCommand),
    Wasi (WasiCommand ),
}


/// Run the command from either the built in commands or the plugin commands, doing both the setup and cleanup
/// to display the output in the CommandOutput ratatui component
pub async fn run_command()-> Result<()>{

Ok(())
}
