//! Meta Signal contract — privileged `terminal` session lifecycle.
//!
//! Ordinary terminal transport lives in `signal-terminal`. This crate
//! carries the meta-only vocabulary that starts and retires terminal sessions.

use nota_next::{NotaDecode, NotaEncode};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_frame::signal_channel;
pub use signal_terminal::{TerminalExitStatus, TerminalName};

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalCommandExecutable(String);

impl TerminalCommandExecutable {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalCommandArgument(String);

impl TerminalCommandArgument {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalCommand {
    pub executable: TerminalCommandExecutable,
    pub arguments: Vec<TerminalCommandArgument>,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalEnvironmentName(String);

impl TerminalEnvironmentName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalEnvironmentValue(String);

impl TerminalEnvironmentValue {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct TerminalEnvironmentBinding {
    pub name: TerminalEnvironmentName,
    pub value: TerminalEnvironmentValue,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct TerminalWorkingDirectory(String);

impl TerminalWorkingDirectory {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
)]
pub struct WirePath(String);

impl WirePath {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct CreateSession {
    pub name: TerminalName,
    pub command: TerminalCommand,
    pub environment: Vec<TerminalEnvironmentBinding>,
    pub working_directory: Option<TerminalWorkingDirectory>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct RetireSession {
    pub name: TerminalName,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionCreated {
    pub name: TerminalName,
    pub data_socket_path: WirePath,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct SessionRetired {
    pub name: TerminalName,
    pub exit_status: Option<TerminalExitStatus>,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MetaTerminalOperationKind {
    CreateSession,
    RetireSession,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEncode, NotaDecode, Debug, Clone, PartialEq, Eq,
)]
pub struct MetaTerminalRequestUnimplemented {
    pub terminal: TerminalName,
    pub operation: MetaTerminalOperationKind,
    pub reason: MetaTerminalUnimplementedReason,
}

#[derive(
    Archive,
    RkyvSerialize,
    RkyvDeserialize,
    NotaEncode,
    NotaDecode,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Hash,
)]
pub enum MetaTerminalUnimplementedReason {
    NotBuiltYet,
    DependencyTrackNotLanded,
}

signal_channel! {
    channel MetaTerminal {
        operation CreateSession(CreateSession),
        operation RetireSession(RetireSession),
    }
    reply MetaTerminalReply {
        SessionCreated(SessionCreated),
        SessionRetired(SessionRetired),
        MetaTerminalRequestUnimplemented(MetaTerminalRequestUnimplemented),
    }
}

pub type MetaTerminalRequest = Operation;
pub type MetaTerminalFrame = Frame;
pub type MetaTerminalFrameBody = FrameBody;
pub type MetaTerminalRequestBuilder = RequestBuilder;
pub type ChannelRequest = Operation;
pub type ChannelReply = MetaTerminalReply;

impl MetaTerminalRequest {
    pub fn operation_kind(&self) -> MetaTerminalOperationKind {
        match self {
            Self::CreateSession(_) => MetaTerminalOperationKind::CreateSession,
            Self::RetireSession(_) => MetaTerminalOperationKind::RetireSession,
        }
    }
}

impl From<CreateSession> for MetaTerminalRequest {
    fn from(payload: CreateSession) -> Self {
        Self::CreateSession(payload)
    }
}

impl From<RetireSession> for MetaTerminalRequest {
    fn from(payload: RetireSession) -> Self {
        Self::RetireSession(payload)
    }
}
