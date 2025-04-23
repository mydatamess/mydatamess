use schemars::JsonSchema;
use serde::Serialize;

#[derive(Debug, Serialize, JsonSchema)]
pub struct CommandError {
    pub message: String,
}

impl CommandError {
    pub fn new(message: &str) -> CommandError {
        CommandError {
            message: message.to_string(),
        }
    }
}

#[derive(Debug, Serialize, JsonSchema)]
pub struct CommandErrorWithData<T> {
    pub message: String,
    pub data: T,
}

impl<T> CommandErrorWithData<T> {
    pub fn new(message: &str, data: T) -> CommandErrorWithData<T> {
        CommandErrorWithData {
            message: message.to_string(),
            data,
        }
    }
}
