use meta_signal_terminal::{
    CreateSession, Frame, FrameBody, InputRoute, MetaTerminalOperationKind, MetaTerminalReply,
    MetaTerminalRequest, MetaTerminalRequestUnimplemented, MetaTerminalUnimplementedReason,
    OutputRoute, RetireSession, SessionCreated, SessionRetired, TerminalCommand,
    TerminalCommandArgument, TerminalCommandExecutable, TerminalEnvironmentBinding,
    TerminalEnvironmentName, TerminalEnvironmentValue, TerminalExitStatus, TerminalName,
    TerminalWorkingDirectory, WirePath,
};
#[cfg(feature = "nota-text")]
use nota::{NotaDecode, NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply, Request as FrameRequest,
    SessionEpoch, SignalOperationHeads, SubReply,
};

#[cfg(feature = "nota-text")]
const CANONICAL: &str = include_str!("../examples/canonical.nota");

fn terminal() -> TerminalName {
    TerminalName::new("operator")
}

fn command() -> TerminalCommand {
    TerminalCommand {
        executable: TerminalCommandExecutable::new("pi").into(),
        arguments: vec![TerminalCommandArgument::new("--model")].into(),
    }
}

fn create_session(working_directory: Option<TerminalWorkingDirectory>) -> CreateSession {
    CreateSession {
        name: terminal().into(),
        command: command().into(),
        environment: vec![environment()].into(),
        working_directory: working_directory.into(),
    }
}

fn minimal_create_session() -> CreateSession {
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

fn retire_session() -> RetireSession {
    RetireSession::new(terminal().into())
}

fn environment() -> TerminalEnvironmentBinding {
    TerminalEnvironmentBinding {
        environment_name: TerminalEnvironmentName::new("TERM").into(),
        environment_value: TerminalEnvironmentValue::new("xterm-256color").into(),
    }
}

fn data_socket_path() -> WirePath {
    WirePath::new("/run/persona/terminal/sessions/operator/data.sock")
}

fn session_created() -> SessionCreated {
    SessionCreated {
        name: terminal().into(),
        data_socket_path: data_socket_path().into(),
    }
}

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn round_trip_request(request: MetaTerminalRequest) -> MetaTerminalRequest {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: FrameRequest::from_payload(request.clone()),
    });
    let bytes = frame.encode_length_prefixed().expect("encode");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode");
    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request frame, got {other:?}"),
    }
}

fn round_trip_reply(reply: MetaTerminalReply) -> MetaTerminalReply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: Reply::committed(NonEmpty::single(SubReply::Ok(reply.clone()))),
    });
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
        other => panic!("expected reply frame, got {other:?}"),
    }
}

#[cfg(feature = "nota-text")]
fn round_trip_nota<T>(value: T, expected: &str)
where
    T: NotaEncode + NotaDecode + PartialEq + std::fmt::Debug,
{
    let encoded = value.to_nota();
    assert_eq!(encoded, expected);

    let recovered = NotaSource::new(&encoded)
        .parse::<T>()
        .expect("decode nota text");
    assert_eq!(recovered, value);
    assert!(
        CANONICAL.contains(expected),
        "examples/canonical.nota missing line: {expected}"
    );
}

#[test]
fn meta_terminal_requests_round_trip() {
    let create = MetaTerminalRequest::CreateSession(create_session(Some(
        TerminalWorkingDirectory::new("/workspace"),
    )));
    assert_eq!(round_trip_request(create.clone()), create);

    let retire = MetaTerminalRequest::RetireSession(retire_session());
    assert_eq!(round_trip_request(retire.clone()), retire);
}

#[test]
fn meta_terminal_replies_round_trip() {
    let created = MetaTerminalReply::SessionCreated(session_created());
    assert_eq!(round_trip_reply(created.clone()), created);

    let retired = MetaTerminalReply::SessionRetired(SessionRetired {
        name: terminal().into(),
        exit_status: Some(TerminalExitStatus::StatusUnavailable).into(),
    });
    assert_eq!(round_trip_reply(retired.clone()), retired);

    let unimplemented =
        MetaTerminalReply::MetaTerminalRequestUnimplemented(MetaTerminalRequestUnimplemented {
            terminal: terminal().into(),
            operation: MetaTerminalOperationKind::CreateSession.into(),
            reason: MetaTerminalUnimplementedReason::NotBuiltYet.into(),
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
fn meta_terminal_routes_are_closed_and_named() {
    let create = MetaTerminalRequest::CreateSession(create_session(Some(
        TerminalWorkingDirectory::new("/workspace"),
    )));
    assert_eq!(create.route(), InputRoute::CreateSession);
    assert_eq!(
        create.operation_kind(),
        MetaTerminalOperationKind::CreateSession
    );

    let retire = MetaTerminalRequest::RetireSession(retire_session());
    assert_eq!(retire.route(), InputRoute::RetireSession);
    assert_eq!(
        retire.operation_kind(),
        MetaTerminalOperationKind::RetireSession
    );

    let reply = MetaTerminalReply::SessionCreated(session_created());
    assert_eq!(reply.route(), OutputRoute::SessionCreated);
}

#[cfg(feature = "nota-text")]
#[test]
fn meta_terminal_canonical_examples_round_trip() {
    round_trip_nota(
        MetaTerminalRequest::CreateSession(minimal_create_session()),
        "(CreateSession (operator (pi []) [] None))",
    );
    round_trip_nota(
        MetaTerminalRequest::RetireSession(retire_session()),
        "(RetireSession operator)",
    );
    round_trip_nota(
        MetaTerminalReply::SessionCreated(session_created()),
        "(SessionCreated (operator /run/persona/terminal/sessions/operator/data.sock))",
    );
    round_trip_nota(
        MetaTerminalReply::SessionRetired(SessionRetired {
            name: terminal().into(),
            exit_status: Some(TerminalExitStatus::StatusUnavailable).into(),
        }),
        "(SessionRetired (operator (Some StatusUnavailable)))",
    );
    round_trip_nota(
        MetaTerminalReply::MetaTerminalRequestUnimplemented(MetaTerminalRequestUnimplemented {
            terminal: terminal().into(),
            operation: MetaTerminalOperationKind::CreateSession.into(),
            reason: MetaTerminalUnimplementedReason::NotBuiltYet.into(),
        }),
        "(MetaTerminalRequestUnimplemented (operator CreateSession NotBuiltYet))",
    );
}
