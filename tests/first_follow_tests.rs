use std::collections::BTreeSet;
use std::path::Path;

use grammar_analyse::first_follow::{
    compute_first_sets, compute_follow_sets, first_of_sequence, FirstSets,
};
use grammar_analyse::grammar::Symbol;
use grammar_analyse::loader::load_problem;

#[test]
fn compute_first_and_follow_for_expression_grammar() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");

    let first = compute_first_sets(&problem.grammar);
    let follow = compute_follow_sets(&problem.grammar, &first);

    assert_eq!(first["Expr"], set_of(&["(", "id"]));
    assert_eq!(first["ExprP"], set_of(&["+", "ε"]));
    assert_eq!(follow["Expr"], set_of(&["$", ")"]));
    assert_eq!(follow["Term"], set_of(&["$", ")", "+"]));
}

#[test]
fn first_of_expression_prime_contains_epsilon() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");

    let first = compute_first_sets(&problem.grammar);

    assert!(first["ExprP"].contains("ε"));
}

#[test]
fn first_of_sequence_uses_consistent_nullable_contract() {
    let empty_first_sets = FirstSets::new();
    assert_eq!(
        first_of_sequence(&[], &empty_first_sets),
        (BTreeSet::new(), true)
    );
    assert_eq!(
        first_of_sequence(&[Symbol::Epsilon], &empty_first_sets),
        (BTreeSet::new(), true)
    );

    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");
    let first = compute_first_sets(&problem.grammar);

    let sequence = vec![
        Symbol::NonTerminal("ExprP".to_string()),
        Symbol::Terminal("id".to_string()),
    ];
    assert_eq!(first_of_sequence(&sequence, &first), (set_of(&["+", "id"]), false));
}

#[test]
#[should_panic(expected = "missing FIRST set for non-terminal: Missing")]
fn first_of_sequence_panics_when_first_set_is_missing() {
    let _ = first_of_sequence(
        &[Symbol::NonTerminal("Missing".to_string())],
        &FirstSets::new(),
    );
}

fn set_of(values: &[&str]) -> BTreeSet<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
