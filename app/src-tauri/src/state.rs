use mydatamess_core::application::ports::inbound::resource_port::ResourcePort;

pub struct AppState {
    pub resource_port: Box<dyn ResourcePort>,
}
