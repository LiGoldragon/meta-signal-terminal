#[test]
fn terminal_meta_contract_declares_local_dotos_text_feature_for_signal_frame_macros() {
    let cargo_toml = include_str!("../Cargo.toml");

    assert!(
        cargo_toml.contains("default = [\"dotos-text\"]"),
        "direct meta-signal-terminal users keep the DOTOS projection by default",
    );
    assert!(
        cargo_toml.contains("dotos-text = [\"signal-frame/dotos-text\"]"),
        "signal-frame macro-generated DOTOS traits are gated through a local feature",
    );
}
