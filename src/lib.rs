//! Schema-derived meta Signal contract for privileged `terminal` session lifecycle.
//!
//! Ordinary terminal transport lives in `signal-terminal`. This crate carries
//! the meta-only vocabulary that starts and retires terminal sessions.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

pub type MetaTerminalRequest = Input;
pub type MetaTerminalReply = Output;
pub type MetaTerminalFrame = Frame;
pub type MetaTerminalFrameBody = FrameBody;
pub type MetaTerminalRequestBuilder = RequestBuilder;
pub type ChannelRequest = Input;
pub type ChannelReply = Output;

impl TerminalCommandExecutable {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl TerminalCommandArgument {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl TerminalEnvironmentName {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl TerminalEnvironmentValue {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl TerminalWorkingDirectory {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl WirePath {
    pub fn as_str(&self) -> &str {
        self.payload().as_str()
    }
}

impl Input {
    pub fn operation_kind(&self) -> MetaTerminalOperationKind {
        match self {
            Self::CreateSession(_) => MetaTerminalOperationKind::CreateSession,
            Self::RetireSession(_) => MetaTerminalOperationKind::RetireSession,
        }
    }
}
