use mydatamess_core::application::ports::inbound::resource_port::ResourcePort;

use crate::commands::registry::CommandRegistry;

pub struct AppState {
    pub resource_port: Box<dyn ResourcePort>,
    pub cmd_registry: CommandRegistry,
}
