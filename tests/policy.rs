use zenith_dev_deps_deck::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 81, capacity: 107, latency: 8, risk: 12, weight: 13 };
    assert_eq!(score(signal), 205);
    assert_eq!(classify(signal), "accept");
    let signal = Signal { demand: 80, capacity: 106, latency: 9, risk: 17, weight: 8 };
    assert_eq!(score(signal), 143);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 83, capacity: 70, latency: 23, risk: 11, weight: 8 };
    assert_eq!(score(signal), 99);
    assert_eq!(classify(signal), "review");
}
