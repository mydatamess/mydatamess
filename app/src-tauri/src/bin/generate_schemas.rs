use std::env;
use std::fs;
use std::path::Path;

use mydatamess_app_lib::build_cmd_registry;
use schemars::schema::RootSchema;

fn write_schema(schema: &RootSchema, file_path: &Path) {
    let schema_json = serde_json::to_string_pretty(schema).unwrap();
    fs::write(file_path, schema_json).expect("Failed to write schema file");
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: generate_schemas <output_dir>");
        std::process::exit(1);
    }

    let output_dir = Path::new(&args[1]);
    fs::create_dir_all(output_dir).expect("Failed to create schemas directory");

    let registry = build_cmd_registry();

    for command in registry.get_commands() {
        let spec = command.command_spec();
        let cmd_dir = output_dir.join(command.operation_id());
        fs::create_dir_all(&cmd_dir).expect("Failed to create command directory");

        write_schema(&spec.request, &cmd_dir.join("request.json"));
        write_schema(&spec.response, &cmd_dir.join("response.json"));
        write_schema(&spec.error, &cmd_dir.join("error.json"));
    }
}
