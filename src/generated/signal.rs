#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
pub type TerminalCommandExecutable = String;
#[rustfmt::skip]
pub type TerminalCommandArgument = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalCommand {
    pub terminal_command_executable: TerminalCommandExecutable,
    pub terminal_command_arguments: TerminalCommandArguments,
}
#[rustfmt::skip]
pub type TerminalCommandArguments = std::vec::Vec<TerminalCommandArgument>;
#[rustfmt::skip]
pub type TerminalEnvironmentName = String;
#[rustfmt::skip]
pub type TerminalEnvironmentValue = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct TerminalEnvironmentBinding {
    pub terminal_environment_name: TerminalEnvironmentName,
    pub terminal_environment_value: TerminalEnvironmentValue,
}
#[rustfmt::skip]
pub type TerminalEnvironment = std::vec::Vec<TerminalEnvironmentBinding>;
#[rustfmt::skip]
pub type TerminalWorkingDirectory = String;
#[rustfmt::skip]
pub type WirePath = String;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct CreateSession {
    pub terminal_name: signal_terminal::TerminalName,
    pub terminal_command: TerminalCommand,
    pub terminal_environment: TerminalEnvironment,
    pub selected_working_directory: SelectedWorkingDirectory,
}
#[rustfmt::skip]
pub type SelectedWorkingDirectory = std::option::Option<TerminalWorkingDirectory>;
#[rustfmt::skip]
pub type RetireSession = signal_terminal::TerminalName;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionCreated {
    pub terminal_name: signal_terminal::TerminalName,
    pub wire_path: WirePath,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct SessionRetired {
    pub terminal_name: signal_terminal::TerminalName,
    pub selected_exit_status: SelectedExitStatus,
}
#[rustfmt::skip]
pub type SelectedExitStatus = std::option::Option<signal_terminal::TerminalExitStatus>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MetaTerminalOperationKind {
    CreateSession(CreateSession),
    RetireSession(RetireSession),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct MetaTerminalRequestUnimplemented {
    pub terminal_name: signal_terminal::TerminalName,
    pub meta_terminal_operation_kind: MetaTerminalOperationKind,
    pub meta_terminal_unimplemented_reason: MetaTerminalUnimplementedReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MetaTerminalUnimplementedReason {
    NotBuiltYet,
    DependencyTrackNotLanded,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    CreateSession(CreateSession),
    RetireSession(RetireSession),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    SessionCreated(SessionCreated),
    SessionRetired(SessionRetired),
    MetaTerminalRequestUnimplemented(MetaTerminalRequestUnimplemented),
}
