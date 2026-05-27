#[test]
fn core_must_not_depend_on_adapter() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/core_depends_on_l1_adapter.rs");
}

#[test]
fn core_must_not_depend_on_infrastructure() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/fail/core_depends_on_revm.rs");
}
