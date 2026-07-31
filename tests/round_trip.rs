use meta_signal_terminal::{
    CreateSession, Frame, FrameBody, MetaTerminalOperationKind, MetaTerminalReply,
    MetaTerminalRequest, MetaTerminalRequestUnimplemented, MetaTerminalUnimplementedReason,
    RetireSession, SessionCreated, SessionRetired, TerminalCommand, TerminalCommandArgument,
    TerminalCommandExecutable, TerminalEnvironmentBinding, TerminalEnvironmentName,
    TerminalEnvironmentValue, TerminalExitStatus, TerminalName, TerminalWorkingDirectory, WirePath,
};
#[cfg(feature = "dotos-text")]
use dotos::{DotosDecode, DotosEncode, DotosSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, SessionEpoch,
    RootCode, SignalOperationHeads, SubReply, VariantCode, WireRoute,
};

#[cfg(feature = "dotos-text")]
const CANONICAL: &str = include_str!("../examples/canonical.dotos");

fn terminal() -> TerminalName {
    TerminalName::new("operator".to_string())
}

fn command() -> TerminalCommand {
    TerminalCommand {
        executable: TerminalCommandExecutable::new("pi"),
        arguments: vec![TerminalCommandArgument::new("--model")],
    }
}

fn environment() -> TerminalEnvironmentBinding {
    TerminalEnvironmentBinding {
        name: TerminalEnvironmentName::new("TERM"),
        value: TerminalEnvironmentValue::new("xterm-256color"),
    }
}

fn data_socket_path() -> WirePath {
    WirePath::new("/run/persona/terminal/sessions/operator/data.sock")
}

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: MetaTerminalRequest) -> MetaTerminalRequest {
    let frame = request.into_frame(exchange()).expect("build request frame");
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => {
            let (payload, tail) = request.payloads.into_head_and_tail();
            assert!(tail.is_empty(), "one request payload is expected");
            payload
        }
        other => panic!("expected request operation, got {other:?}"),
    }
}

fn round_trip_reply(reply: MetaTerminalReply) -> MetaTerminalReply {
    let frame = Frame::new(
        WireRoute::new(RootCode::new(0), VariantCode::new(0)),
        FrameBody::Reply {
            exchange: exchange(),
            reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply))),
        },
    );
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            Reply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply operation, got {other:?}"),
    }
}

#[cfg(feature = "dotos-text")]
fn round_trip_dotos<T>(value: T, expected: &str)
where
    T: DotosEncode + DotosDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_dotos();
    assert_eq!(encoded, expected);

    let recovered = DotosSource::new(&encoded)
        .parse::<T>()
        .expect("decode dotos text");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.dotos missing line: {expected}"
    );
}

#[test]
fn meta_terminal_requests_round_trip() {
    let create = MetaTerminalRequest::CreateSession(CreateSession {
        name: terminal(),
        command: command(),
        environment: vec![environment()],
        working_directory: Some(TerminalWorkingDirectory::new("/workspace")),
    });
    assert_eq!(round_trip_request(create.clone()), create);

    let retire = MetaTerminalRequest::RetireSession(RetireSession { name: terminal() });
    assert_eq!(round_trip_request(retire.clone()), retire);
}

#[test]
fn meta_terminal_replies_round_trip() {
    let created = MetaTerminalReply::SessionCreated(SessionCreated {
        name: terminal(),
        data_socket_path: data_socket_path(),
    });
    assert_eq!(round_trip_reply(created.clone()), created);

    let retired = MetaTerminalReply::SessionRetired(SessionRetired {
        name: terminal(),
        exit_status: Some(TerminalExitStatus::StatusUnavailable),
    });
    assert_eq!(round_trip_reply(retired.clone()), retired);

    let unimplemented =
        MetaTerminalReply::MetaTerminalRequestUnimplemented(MetaTerminalRequestUnimplemented {
            terminal: terminal(),
            operation: MetaTerminalOperationKind::CreateSession,
            reason: MetaTerminalUnimplementedReason::NotBuiltYet,
        });
    assert_eq!(round_trip_reply(unimplemented.clone()), unimplemented);
}

#[test]
fn meta_terminal_request_heads_are_contract_local_operations() {
    assert_eq!(
        <MetaTerminalRequest as SignalOperationHeads>::HEADS,
        &["CreateSession", "RetireSession"]
    );
}

#[test]
fn meta_terminal_request_exposes_contract_owned_operation_kind() {
    let create = MetaTerminalRequest::CreateSession(CreateSession {
        name: terminal(),
        command: command(),
        environment: vec![environment()],
        working_directory: Some(TerminalWorkingDirectory::new("/workspace")),
    });
    assert_eq!(
        create.operation_kind(),
        MetaTerminalOperationKind::CreateSession
    );

    let retire = MetaTerminalRequest::RetireSession(RetireSession { name: terminal() });
    assert_eq!(
        retire.operation_kind(),
        MetaTerminalOperationKind::RetireSession
    );
}

#[cfg(feature = "dotos-text")]
#[test]
fn meta_terminal_canonical_examples_round_trip() {
    round_trip_dotos(
        MetaTerminalRequest::CreateSession(CreateSession {
            name: terminal(),
            command: TerminalCommand {
                executable: TerminalCommandExecutable::new("pi"),
                arguments: Vec::new(),
            },
            environment: Vec::new(),
            working_directory: None,
        }),
        "(CreateSession {operator {pi []} [] None})",
    );
    round_trip_dotos(
        MetaTerminalRequest::RetireSession(RetireSession { name: terminal() }),
        "(RetireSession {operator})",
    );
    round_trip_dotos(
        MetaTerminalReply::SessionCreated(SessionCreated {
            name: terminal(),
            data_socket_path: data_socket_path(),
        }),
        "(SessionCreated {operator /run/persona/terminal/sessions/operator/data.sock})",
    );
    round_trip_dotos(
        MetaTerminalReply::SessionRetired(SessionRetired {
            name: terminal(),
            exit_status: Some(TerminalExitStatus::StatusUnavailable),
        }),
        "(SessionRetired {operator Some.StatusUnavailable})",
    );
    round_trip_dotos(
        MetaTerminalReply::MetaTerminalRequestUnimplemented(MetaTerminalRequestUnimplemented {
            terminal: terminal(),
            operation: MetaTerminalOperationKind::CreateSession,
            reason: MetaTerminalUnimplementedReason::NotBuiltYet,
        }),
        "(MetaTerminalRequestUnimplemented {operator CreateSession NotBuiltYet})",
    );
}
