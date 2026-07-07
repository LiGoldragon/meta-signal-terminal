use meta_signal_terminal::{
    CreateSession, Input, InputRoute, MetaTerminalOperationKind, MetaTerminalRequestUnimplemented,
    MetaTerminalUnimplementedReason, Output, OutputRoute, SessionCreated, TerminalCommand,
    TerminalCommandExecutable, TerminalName, WirePath,
};

const SCHEMA_SOURCE: &str = include_str!("../schema/lib.schema");
const GENERATED_SCHEMA_RUST: &str = include_str!("../src/schema/lib.rs");

fn terminal() -> TerminalName {
    TerminalName::new("operator")
}

fn create_session() -> CreateSession {
    CreateSession {
        name: terminal().into(),
        command: TerminalCommand {
            executable: TerminalCommandExecutable::new("pi").into(),
            arguments: Vec::new().into(),
        }
        .into(),
        environment: Vec::new().into(),
        working_directory: None.into(),
    }
}

#[test]
fn generated_meta_input_owns_short_header_and_frame() {
    let input = Input::CreateSession(create_session());

    assert_eq!(input.route(), InputRoute::CreateSession);
    assert_eq!(
        input.operation_kind(),
        MetaTerminalOperationKind::CreateSession
    );

    let frame = input.encode_signal_frame().expect("encode generated input");
    let (route, decoded) = Input::decode_signal_frame(&frame).expect("decode generated input");

    assert_eq!(route, InputRoute::CreateSession);
    assert_eq!(decoded, input);
}

#[test]
fn generated_meta_output_owns_short_header_and_frame() {
    let output = Output::SessionCreated(SessionCreated {
        name: terminal().into(),
        data_socket_path: WirePath::new("/run/persona/terminal/sessions/operator/data.sock").into(),
    });

    assert_eq!(output.route(), OutputRoute::SessionCreated);

    let frame = output
        .encode_signal_frame()
        .expect("encode generated output");
    let (route, decoded) = Output::decode_signal_frame(&frame).expect("decode generated output");

    assert_eq!(route, OutputRoute::SessionCreated);
    assert_eq!(decoded, output);
}

#[test]
fn generated_meta_contract_imports_terminal_nouns_without_local_redeclaration() {
    assert!(
        GENERATED_SCHEMA_RUST
            .contains("pub use signal_terminal::schema::lib::TerminalName as TerminalName;"),
        "generated schema must import TerminalName from signal-terminal",
    );
    assert!(
        GENERATED_SCHEMA_RUST.contains(
            "pub use signal_terminal::schema::lib::TerminalExitStatus as TerminalExitStatus;"
        ),
        "generated schema must import TerminalExitStatus from signal-terminal",
    );
    assert!(
        !GENERATED_SCHEMA_RUST.contains("pub struct TerminalName"),
        "TerminalName must not be locally redeclared",
    );
    assert!(
        !GENERATED_SCHEMA_RUST.contains("pub enum TerminalExitStatus"),
        "TerminalExitStatus must not be locally redeclared",
    );
}

#[test]
fn generated_meta_contract_surface_excludes_runtime_plane_and_old_schema_terms() {
    let unimplemented =
        Output::MetaTerminalRequestUnimplemented(MetaTerminalRequestUnimplemented {
            terminal: terminal().into(),
            operation: MetaTerminalOperationKind::RetireSession.into(),
            reason: MetaTerminalUnimplementedReason::DependencyTrackNotLanded.into(),
        });
    assert_eq!(
        unimplemented.route(),
        OutputRoute::MetaTerminalRequestUnimplemented
    );

    for term in [
        "NexusWork",
        "NexusAction",
        "CommandSemaWrite",
        "SemaWriteInput",
        "SemaReadInput",
        "SignalEngine",
        "NexusEngine",
        "SemaEngine",
        "TraceEvent",
        "SpecifiedSchema",
        "Schema/SpecifiedSchema",
        "emit_signal",
        "emit_contract",
        "schema-next",
        "schema-rust-next",
        "nota-next",
        "drop-next",
    ] {
        assert!(
            !SCHEMA_SOURCE.contains(term),
            "contract schema must not declare stale/runtime term {term}"
        );
        assert!(
            !GENERATED_SCHEMA_RUST.contains(term),
            "generated contract module must not export stale/runtime term {term}"
        );
    }
}
