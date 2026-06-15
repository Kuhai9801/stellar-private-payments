#[path = "../build_support.rs"]
mod build_support;

use build_support::circom_include_path;

#[test]
fn include_parser_finds_includes_after_pragma_lines() {
    let source = [
        "pragma circom 2.2.2;",
        "",
        "include \"./policyTransaction.circom\";",
        "include '../circomlib/circuits/poseidon.circom';",
        "component main = PolicyTransaction();",
    ];

    let includes: Vec<_> = source
        .iter()
        .filter_map(|line| circom_include_path(line))
        .collect();

    assert_eq!(
        includes,
        [
            "./policyTransaction.circom",
            "../circomlib/circuits/poseidon.circom"
        ]
    );
}

#[test]
fn include_parser_ignores_comments_and_similar_identifiers() {
    let source = [
        "// include \"ignored.circom\";",
        "included \"ignored.circom\";",
        "include \"kept.circom\"; // trailing comment",
    ];

    let includes: Vec<_> = source
        .iter()
        .filter_map(|line| circom_include_path(line))
        .collect();

    assert_eq!(includes, ["kept.circom"]);
}
