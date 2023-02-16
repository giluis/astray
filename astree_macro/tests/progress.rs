#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic/mod.rs");

    t.pass("tests/struct/stateless_leaf.rs");
    // t.pass("tests/struct/stateful_leaf.rs");
    t.pass("tests/struct/ref.rs");
    // t.pass("tests/struct/all_together.rs");

    // t.pass("tests/enum/one_variant.rs");
    // t.pass("tests/enum/many_variants.rs");
    // t.pass("tests/enum/ref_to_enum.rs");
    // t.pass("tests/enum/enum_and_struct.rs");

    // t.pass("tests/std_types/optional.rs");

    // t.pass("tests/compiles.rs");
    // t.pass("tests/simple_leaf.rs");
    // t.pass("tests/linkedtypes.rs");
    // t.pass("tests/enumtype.rs");
    // t.pass("tests/call-build.rs");
    // t.pass("tests/method-chaining.rs");
    // t.pass("tests/optional-field.rs");
    // t.pass("tests/repeated-field.rs");
    // t.compile_fail("tests/unrecognized-attribute.rs");
    // t.pass("tests/redefined-prelude-types.rs");
}
