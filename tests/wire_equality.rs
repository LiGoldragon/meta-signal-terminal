//! The 4.1.0 newtypes keep the wire and the text of the type aliases they replace.
//!
//! `support/previous_400.rs` is `src/generated/signal.rs` at 4.0.0 (40219a6) with one mechanical
//! substitution, `signal_terminal::` to `crate::old_terminal::`, because the old file names the aliases of
//! the signal-terminal it was built on; `old_terminal` below is that crate's 4.0.0 shape for the
//! four items the file uses. Each sample is built once in the old family and once in the new from
//! the same data; the archived bytes (and, with `datom`, the text) must be equal. Representative
//! values, not a proof for every value.

#[path = "support/previous_400.rs"]
#[allow(clippy::all, dead_code)]
mod previous;

use meta_signal_terminal as current;
use rkyv::rancor::Error;
use signal_terminal as new_terminal;

/// signal-terminal 4.0.0 as the old file uses it: the aliases, and the exit status enum.
#[allow(dead_code)]
mod old_terminal {
    pub type TerminalName = String;
    pub type ExitCode = i64;
    pub type TerminalSignalNumber = i64;
    #[derive(
        rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq, Eq, Hash,
    )]
    #[cfg_attr(
        feature = "datom",
        derive(datom_codec::Datomizable, datom_codec::Composing)
    )]
    pub enum TerminalExitStatus {
        Exited(ExitCode),
        Signaled(TerminalSignalNumber),
        StatusUnavailable,
    }
}

macro_rules! old_w {
    ($p:path, $e:expr) => {
        $e
    };
}
macro_rules! new_w {
    ($p:path, $e:expr) => {
        $p($e)
    };
}

macro_rules! family {
    ($name:ident, $m:ident, $t:ident, $w:ident) => {
        mod $name {
            #![allow(unused_imports)]
            use super::$m::*;
            pub fn terminal(s: &str) -> super::$t::TerminalName {
                $w!(super::$t::TerminalName, s.to_string())
            }
            pub fn command() -> TerminalCommand {
                TerminalCommand {
                    terminal_command_executable: $w!(
                        TerminalCommandExecutable,
                        "/bin/sh".to_string()
                    ),
                    terminal_command_arguments: $w!(
                        TerminalCommandArguments,
                        vec![$w!(TerminalCommandArgument, "-l".to_string())]
                    ),
                }
            }
            pub fn environment() -> TerminalEnvironment {
                $w!(
                    TerminalEnvironment,
                    vec![TerminalEnvironmentBinding {
                        terminal_environment_name: $w!(TerminalEnvironmentName, "TERM".to_string()),
                        terminal_environment_value: $w!(
                            TerminalEnvironmentValue,
                            "xterm".to_string()
                        ),
                    }]
                )
            }
            pub fn directory(s: Option<&str>) -> SelectedWorkingDirectory {
                $w!(
                    SelectedWorkingDirectory,
                    s.map(|s| $w!(TerminalWorkingDirectory, s.to_string()))
                )
            }
            pub fn wire_path(s: &str) -> WirePath {
                $w!(WirePath, s.to_string())
            }
            pub fn retire(s: &str) -> RetireSession {
                $w!(RetireSession, terminal(s))
            }
            pub fn status(kind: u8) -> SelectedExitStatus {
                $w!(
                    SelectedExitStatus,
                    match kind {
                        0 => None,
                        1 => Some(super::$t::TerminalExitStatus::Exited($w!(
                            super::$t::ExitCode,
                            0
                        ))),
                        2 => Some(super::$t::TerminalExitStatus::Signaled($w!(
                            super::$t::TerminalSignalNumber,
                            9
                        ))),
                        _ => Some(super::$t::TerminalExitStatus::StatusUnavailable),
                    }
                )
            }
            pub fn create(name: &str, directory_: Option<&str>) -> CreateSession {
                CreateSession {
                    terminal_name: terminal(name),
                    terminal_command: command(),
                    terminal_environment: environment(),
                    selected_working_directory: directory(directory_),
                }
            }
        }
    };
}

family!(oldk, previous, old_terminal, old_w);
family!(newk, current, new_terminal, new_w);

macro_rules! samples {
    ($m:ident, $k:ident) => {{
        use $m::*;
        Samples {
            queries: vec![
                Query::CreateSession($k::create("main", Some("/home/user"))),
                Query::CreateSession($k::create("bare", None)),
                Query::RetireSession($k::retire("main")),
            ],
            responses: vec![
                Response::SessionCreated(SessionCreated {
                    terminal_name: $k::terminal("main"),
                    wire_path: $k::wire_path("/run/terminal/main.sock"),
                }),
                Response::SessionRetired(SessionRetired {
                    terminal_name: $k::terminal("main"),
                    selected_exit_status: $k::status(0),
                }),
                Response::SessionRetired(SessionRetired {
                    terminal_name: $k::terminal("main"),
                    selected_exit_status: $k::status(1),
                }),
                Response::SessionRetired(SessionRetired {
                    terminal_name: $k::terminal("main"),
                    selected_exit_status: $k::status(2),
                }),
                Response::SessionRetired(SessionRetired {
                    terminal_name: $k::terminal("main"),
                    selected_exit_status: $k::status(3),
                }),
                Response::MetaTerminalRequestUnimplemented(MetaTerminalRequestUnimplemented {
                    terminal_name: $k::terminal("main"),
                    meta_terminal_operation_kind: MetaTerminalOperationKind::RetireSession(
                        $k::retire("main"),
                    ),
                    meta_terminal_unimplemented_reason:
                        MetaTerminalUnimplementedReason::NotBuiltYet,
                }),
                Response::MetaTerminalRequestUnimplemented(MetaTerminalRequestUnimplemented {
                    terminal_name: $k::terminal("main"),
                    meta_terminal_operation_kind: MetaTerminalOperationKind::CreateSession(
                        $k::create("main", None),
                    ),
                    meta_terminal_unimplemented_reason:
                        MetaTerminalUnimplementedReason::DependencyTrackNotLanded,
                }),
            ],
        }
    }};
}

struct Samples<Q, R> {
    queries: Vec<Q>,
    responses: Vec<R>,
}

fn old_samples() -> Samples<previous::Query, previous::Response> {
    samples!(previous, oldk)
}

fn new_samples() -> Samples<current::Query, current::Response> {
    samples!(current, newk)
}

macro_rules! archived {
    ($value:expr) => {
        rkyv::to_bytes::<Error>($value).expect("archive").to_vec()
    };
}

#[test]
fn the_current_types_archive_to_the_bytes_of_the_400_types() {
    let (old, new) = (old_samples(), new_samples());
    assert_eq!(old.queries.len(), new.queries.len());
    for (index, (old, new)) in old.queries.iter().zip(&new.queries).enumerate() {
        assert_eq!(archived!(old), archived!(new), "Query sample {index}");
    }
    assert_eq!(old.responses.len(), new.responses.len());
    for (index, (old, new)) in old.responses.iter().zip(&new.responses).enumerate() {
        assert_eq!(archived!(old), archived!(new), "Response sample {index}");
    }
}

#[test]
fn an_old_archive_restores_as_the_current_value() {
    let (old, new) = (old_samples(), new_samples());
    for (index, (old, new)) in old.queries.iter().zip(&new.queries).enumerate() {
        let restored = rkyv::from_bytes::<current::Query, Error>(&archived!(old)).expect("restore");
        assert_eq!(&restored, new, "Query sample {index}");
    }
    for (index, (old, new)) in old.responses.iter().zip(&new.responses).enumerate() {
        let restored =
            rkyv::from_bytes::<current::Response, Error>(&archived!(old)).expect("restore");
        assert_eq!(&restored, new, "Response sample {index}");
    }
}

#[cfg(feature = "datom")]
mod text {
    use super::*;
    use datom_codec::{Actualizing, Budget, Composing, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    fn written<T: Datomizable>(value: &T) -> String {
        value.datomize(Vec::new()).protosize().textualize()
    }

    fn read<T: Composing>(text: &str) -> T {
        let mut budget = Budget {
            remaining: 100_000,
            reader: ReaderBudget::try_from(100_000usize).unwrap(),
            depth: 0,
            maximum_depth: 128,
        };
        Potential::<T>::from(text)
            .actualize(&mut budget)
            .expect("read the datom text")
    }

    #[test]
    fn the_current_types_write_the_text_of_the_400_types_and_read_it_back() {
        let (old, new) = (old_samples(), new_samples());
        for (index, (old, new)) in old.queries.iter().zip(&new.queries).enumerate() {
            let text = written(old);
            assert_eq!(text, written(new), "Query sample {index}");
            assert_eq!(&read::<current::Query>(&text), new, "Query sample {index}");
        }
        for (index, (old, new)) in old.responses.iter().zip(&new.responses).enumerate() {
            let text = written(old);
            assert_eq!(text, written(new), "Response sample {index}");
            assert_eq!(
                &read::<current::Response>(&text),
                new,
                "Response sample {index}"
            );
        }
    }
}
