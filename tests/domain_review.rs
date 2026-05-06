use zenith_dev_deps_deck::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 58, slack: 36, drag: 17, confidence: 85 };
    assert_eq!(review_score(case), 186);
    assert_eq!(review_lane(case), "ship");
}
