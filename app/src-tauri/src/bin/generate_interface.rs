use std::{
    fs,
    path::{self, Path},
};

use mydatamess_app_lib::{
    build_cmd_registry,
    commands::{registry::CommandRegistry, resources::get_root_resource::GetRootResourceCmd},
};

fn main() {
    let file_path = Path::new("output_ts");

    fs::create_dir_all(file_path).unwrap();

    let cmd_registry = build_cmd_registry();

    cmd_registry.get_commands().for_each(|cmd| {
        let models = cmd.generate_ts_models();
        let request_file = file_path.join(format!("{}.ts", cmd.operation_id()));

        fs::write(
            request_file,
            format!(
                "{}\n{}\n{}\n",
                models.request, models.response, models.error
            ),
        )
        .expect("Unable to write file");
    })
}
