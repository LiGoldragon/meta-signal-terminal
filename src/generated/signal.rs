#![allow(dead_code, non_camel_case_types, non_snake_case)]
pub type TerminalCommandExecutable = String;
pub type TerminalCommandArgument = String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalCommand {
    pub terminal_command_executable: TerminalCommandExecutable,
    pub terminal_command_arguments: TerminalCommandArguments,
}
pub type TerminalCommandArguments = std::vec::Vec<TerminalCommandArgument>;
pub type TerminalEnvironmentName = String;
pub type TerminalEnvironmentValue = String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct TerminalEnvironmentBinding {
    pub terminal_environment_name: TerminalEnvironmentName,
    pub terminal_environment_value: TerminalEnvironmentValue,
}
pub type TerminalEnvironment = std::vec::Vec<TerminalEnvironmentBinding>;
pub type TerminalWorkingDirectory = String;
pub type WirePath = String;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct CreateSession {
    pub terminal_name: signal_terminal::TerminalName,
    pub terminal_command: TerminalCommand,
    pub terminal_environment: TerminalEnvironment,
    pub selected_working_directory: SelectedWorkingDirectory,
}
pub type SelectedWorkingDirectory = std::option::Option<TerminalWorkingDirectory>;
pub type RetireSession = signal_terminal::TerminalName;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionCreated {
    pub terminal_name: signal_terminal::TerminalName,
    pub wire_path: WirePath,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct SessionRetired {
    pub terminal_name: signal_terminal::TerminalName,
    pub selected_exit_status: SelectedExitStatus,
}
pub type SelectedExitStatus = std::option::Option<signal_terminal::TerminalExitStatus>;
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum MetaTerminalOperationKind {
    CreateSession(CreateSession),
    RetireSession(RetireSession),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub struct MetaTerminalRequestUnimplemented {
    pub terminal_name: signal_terminal::TerminalName,
    pub meta_terminal_operation_kind: MetaTerminalOperationKind,
    pub meta_terminal_unimplemented_reason: MetaTerminalUnimplementedReason,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum MetaTerminalUnimplementedReason {
    NotBuiltYet,
    DependencyTrackNotLanded,
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Query {
    CreateSession(CreateSession),
    RetireSession(RetireSession),
}
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(
    feature = "datom",
    derive(datom_codec::Datomizable, datom_codec::Compositional)
)]
pub enum Response {
    SessionCreated(SessionCreated),
    SessionRetired(SessionRetired),
    MetaTerminalRequestUnimplemented(MetaTerminalRequestUnimplemented),
}
