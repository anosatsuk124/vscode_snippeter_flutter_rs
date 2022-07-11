use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let pwd = env::var("PWD").unwrap();

    let args: Vec<String> = std::env::args().collect();

    match args.get(1).expect("not enough arguments").as_str() {
        "gen" => {
            let mut codegen = Command::new("flutter_rust_bridge_codegen");
            let rust_output_dir = format!("{}/rust/src/bridge_generated", &pwd);
            let rust_outputs = [
                format!("{}/api.rs", &rust_output_dir),
                format!("{}/json.rs", &rust_output_dir),
            ];

            if !Path::new(&rust_output_dir).exists() {
                std::fs::create_dir_all(&rust_output_dir).unwrap();
            }

            for output in &rust_outputs {
                let file = Path::new(&output);
                if !file.exists() {
                    std::fs::File::create(&file).unwrap();
                }
            }

            codegen.arg("--rust-input");
            codegen.args([
                format!("{}/rust/src/api.rs", &pwd),
                format!("{}/rust/src/json.rs", &pwd),
            ]);
            codegen.arg("--dart-output");
            codegen.args([
                format!("{}/lib/bridge_generated/api.dart", &pwd),
                format!("{}/lib/bridge_generated/json.dart", &pwd),
            ]);
            codegen.arg("--rust-output");
            codegen.args(&rust_outputs);
            codegen.arg("--class-name");
            codegen.args(["ApiPlatform", "ApiJson"]);
            codegen.arg("--c-output");
            codegen.args([format!("{}/macos/Runner/brdge_generated.h", &pwd)]);
            codegen.arg("--skip-add-mod-to-lib");

            codegen.spawn().expect("failed to gen").wait().unwrap();
        }
        &_ => panic!("Unexpected arguments!"),
    }
}
