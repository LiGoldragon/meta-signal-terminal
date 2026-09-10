use meta_signal_terminal::{
    ByteViewable, Query, Response, Restorable, SessionRetired, Signal, Signalizable,
};

#[test]
fn query_and_response_round_trip_through_received_bytes() {
    let query = Query::RetireSession("terminal".into());
    let received =
        Signal::<Query>::from(query.signalize().expect("query archives").bytes().to_vec());
    assert_eq!(received.restore().expect("query restores"), query);

    let response = Response::SessionRetired(SessionRetired {
        terminal_name: "terminal".into(),
        selected_exit_status: None,
    });
    let received = Signal::<Response>::from(
        response
            .signalize()
            .expect("response archives")
            .bytes()
            .to_vec(),
    );
    assert_eq!(received.restore().expect("response restores"), response);
}

#[test]
fn malformed_archive_is_rejected() {
    assert!(Signal::<Query>::from(vec![1, 2, 3]).restore().is_err());
}

#[cfg(feature = "datom")]
#[test]
fn query_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let query = Query::RetireSession("terminal".into());
    let text = query.clone().datomize(vec![]).protosize().textualize();
    let restored = Potential::<Query>::from(text)
        .actualize(&mut Budget {
            remaining: 1024,
            reader: ReaderBudget { remaining: 1024 },
            depth: 0,
            maximum_depth: 1024,
        })
        .expect("Datom restores");
    assert_eq!(restored, query);
}
