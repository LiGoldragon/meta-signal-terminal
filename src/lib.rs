//! OwnerSignal contract — privileged `persona-terminal` session lifecycle.
//!
//! Ordinary terminal transport lives in `signal-persona-terminal`. This crate
//! carries the owner-only vocabulary that starts and retires terminal sessions.

use nota_codec::{NotaEnum, NotaRecord, NotaTransparent};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;
pub use signal_persona_terminal::{TerminalExitStatus, TerminalName};

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
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
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalCommand {
    pub executable: TerminalCommandExecutable,
    pub arguments: Vec<TerminalCommandArgument>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
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
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct TerminalEnvironmentBinding {
    pub name: TerminalEnvironmentName,
    pub value: TerminalEnvironmentValue,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaTransparent, Debug, Clone, PartialEq, Eq, Hash,
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

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct CreateSession {
    pub name: TerminalName,
    pub command: TerminalCommand,
    pub environment: Vec<TerminalEnvironmentBinding>,
    pub working_directory: Option<TerminalWorkingDirectory>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct RetireSession {
    pub name: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SessionCreated {
    pub name: TerminalName,
    pub data_socket_path: signal_persona::WirePath,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct SessionRetired {
    pub name: TerminalName,
    pub exit_status: Option<TerminalExitStatus>,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerTerminalOperationKind {
    CreateSession,
    RetireSession,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, NotaRecord, Debug, Clone, PartialEq, Eq)]
pub struct OwnerTerminalRequestUnimplemented {
    pub terminal: TerminalName,
    pub operation: OwnerTerminalOperationKind,
    pub reason: OwnerTerminalUnimplementedReason,
}

#[derive(
    Archive, RkyvSerialize, RkyvDeserialize, NotaEnum, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
pub enum OwnerTerminalUnimplementedReason {
    NotBuiltYet,
    DependencyTrackNotLanded,
}

signal_channel! {
    channel OwnerTerminal {
        request OwnerTerminalRequest {
            Mutate CreateSession(CreateSession),
            Retract RetireSession(RetireSession),
        }
        reply OwnerTerminalReply {
            SessionCreated(SessionCreated),
            SessionRetired(SessionRetired),
            OwnerTerminalRequestUnimplemented(OwnerTerminalRequestUnimplemented),
        }
    }
}

pub type Frame = OwnerTerminalFrame;
pub type FrameBody = OwnerTerminalFrameBody;
pub type ChannelRequest = OwnerTerminalChannelRequest;
pub type ChannelReply = OwnerTerminalChannelReply;
pub type RequestBuilder = OwnerTerminalRequestBuilder;

impl OwnerTerminalRequest {
    pub fn operation_kind(&self) -> OwnerTerminalOperationKind {
        match self {
            Self::CreateSession(_) => OwnerTerminalOperationKind::CreateSession,
            Self::RetireSession(_) => OwnerTerminalOperationKind::RetireSession,
        }
    }
}

impl From<CreateSession> for OwnerTerminalRequest {
    fn from(payload: CreateSession) -> Self {
        Self::CreateSession(payload)
    }
}

impl From<RetireSession> for OwnerTerminalRequest {
    fn from(payload: RetireSession) -> Self {
        Self::RetireSession(payload)
    }
}

impl From<SessionCreated> for OwnerTerminalReply {
    fn from(payload: SessionCreated) -> Self {
        Self::SessionCreated(payload)
    }
}

impl From<SessionRetired> for OwnerTerminalReply {
    fn from(payload: SessionRetired) -> Self {
        Self::SessionRetired(payload)
    }
}

impl From<OwnerTerminalRequestUnimplemented> for OwnerTerminalReply {
    fn from(payload: OwnerTerminalRequestUnimplemented) -> Self {
        Self::OwnerTerminalRequestUnimplemented(payload)
    }
}
