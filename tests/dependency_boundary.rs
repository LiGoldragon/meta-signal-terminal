#[test]
fn terminal_meta_contract_declares_true_schema_dependencies() {
    let cargo_toml = include_str!("../Cargo.toml");

    assert!(
        cargo_toml
            .contains("schema-rust.git\", rev = \"886a9009077bdf9ee0e71fa0fa31aaf6e5444dd3\""),
        "schema-rust must be pinned to the known TrueSchema producer",
    );
    assert!(
        cargo_toml.contains("schema.git\", rev = \"4a8aaf1de3aaf476577d5b4e93691ef47c135d1a\""),
        "schema must be pinned to the known TrueSchema producer",
    );
    assert!(
        cargo_toml
            .contains("signal-terminal.git\", rev = \"2faad3a8787c962a5555aa2abf5e2721661e7925\""),
        "meta-signal-terminal must consume the closed TrueSchema signal-terminal producer",
    );
}

#[test]
fn terminal_meta_contract_uses_the_renamed_nota_and_schema_crates() {
    let cargo_toml = include_str!("../Cargo.toml");

    for stale in ["schema-next", "schema-rust-next", "nota-next", "drop-next"] {
        assert!(
            !cargo_toml.contains(stale),
            "Cargo.toml still references stale pre-TrueSchema crate name {stale}",
        );
    }
    assert!(
        cargo_toml.contains("nota-text = [") && cargo_toml.contains("signal-terminal/nota-text"),
        "imported terminal nouns must enable their NOTA projection only through the local nota-text feature",
    );
}
