use std::path::Path;

use grammar_analyse::loader::load_problem;

#[test]
fn load_expression_grammar_file() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");

    assert_eq!(problem.grammar.start_symbol, "Expr");
    assert!(problem.grammar.terminals.contains("+"));
    assert_eq!(problem.grammar.productions.len(), 8);
    assert_eq!(problem.inputs.len(), 3);
    assert_eq!(problem.inputs[0], vec!["id", "+", "id", "*", "id"]);
}

#[test]
fn reject_conflict_fixture_with_unknown_start_symbol() {
    let error = load_problem(Path::new("tests/fixtures/conflict_grammar.txt"))
        .err()
        .expect("expected error");

    assert!(error.contains("开始符号"));
}
