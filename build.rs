use std::{env, path::PathBuf};

use schema_rust::build::{DependencySchema, GenerationDriver, GenerationPlan};

fn main() {
    SchemaBuild::from_environment().run();
}

struct SchemaBuild {
    crate_root: PathBuf,
}

impl SchemaBuild {
    fn from_environment() -> Self {
        Self {
            crate_root: PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("manifest dir set")),
        }
    }

    fn run(&self) {
        println!("cargo:rerun-if-changed=schema/lib.schema");
        println!("cargo:rerun-if-changed=src/schema/lib.rs");
        println!("cargo:rerun-if-env-changed=DEP_SIGNAL_TERMINAL_SCHEMA_DIR");
        self.print_schema_directory_metadata();

        let terminal_signal =
            DependencySchema::from_cargo_metadata("signal-terminal", "signal-terminal", "0.2.3")
                .expect("read signal-terminal schema metadata")
                .expect(
                    "signal-terminal schema directory exposed via DEP_SIGNAL_TERMINAL_SCHEMA_DIR",
                );

        GenerationDriver::new(
            GenerationPlan::wire_contract(&self.crate_root, "meta-signal-terminal", "0.1.0")
                .with_dependency_schema(terminal_signal),
        )
        .generate()
        .expect("generate meta-signal-terminal schema artifacts")
        .write_or_check("META_SIGNAL_TERMINAL_UPDATE_SCHEMA_ARTIFACTS")
        .expect("checked-in meta-signal-terminal schema artifacts are fresh");
    }

    fn print_schema_directory_metadata(&self) {
        println!(
            "cargo::metadata=schema-dir={}",
            self.crate_root.join("schema").display()
        );
    }
}
