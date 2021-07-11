use rust_workshop::answer;

#[test]
fn check_answer_validity_integrationtest() {
    assert_eq!(answer(), 42);
}
