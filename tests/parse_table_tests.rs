use std::path::Path;

use grammar_analyse::first_follow::{compute_first_sets, compute_follow_sets};
use grammar_analyse::loader::load_problem;

pub mod grammar {
    pub use grammar_analyse::grammar::*;
}

pub mod first_follow {
    pub use grammar_analyse::first_follow::*;
}

#[path = "../src/parse_table.rs"]
mod parse_table;

use parse_table::{build_parse_table, ParseTableError};

#[test]
fn build_parse_table_for_expression_grammar() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");
    let first_sets = compute_first_sets(&problem.grammar);
    let follow_sets = compute_follow_sets(&problem.grammar, &first_sets);

    let table = build_parse_table(&problem.grammar, &first_sets, &follow_sets)
        .expect("build parse table");

    assert_eq!(table.entries.get(&key("Expr", "id")), Some(&1));
    assert!(table.entries.contains_key(&key("Factor", "(")));
}

#[test]
fn build_parse_table_reports_conflict_for_non_ll1_grammar() {
    let problem = load_problem(Path::new("tests/fixtures/table_conflict_grammar.txt"))
        .expect("load fixture");
    let first_sets = compute_first_sets(&problem.grammar);
    let follow_sets = compute_follow_sets(&problem.grammar, &first_sets);

    let error = build_parse_table(&problem.grammar, &first_sets, &follow_sets)
        .err()
        .expect("expected conflict");

    match error {
        ParseTableError::Conflict(conflict) => {
            assert_eq!(conflict.non_terminal, "S");
            assert_eq!(conflict.terminal, "a");
            assert_eq!(conflict.existing_production, 1);
            assert_eq!(conflict.new_production, 2);
        }
        ParseTableError::MissingFollowSet { non_terminal } => {
            panic!("unexpected missing follow set error for {non_terminal}");
        }
    }
}

#[test]
fn build_parse_table_reports_missing_follow_set_explicitly() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");
    let first_sets = compute_first_sets(&problem.grammar);
    let mut follow_sets = compute_follow_sets(&problem.grammar, &first_sets);
    follow_sets.remove("ExprP");

    let error = build_parse_table(&problem.grammar, &first_sets, &follow_sets)
        .err()
        .expect("expected missing follow set");

    assert_eq!(
        error,
        ParseTableError::MissingFollowSet {
            non_terminal: "ExprP".to_string()
        }
    );
}

fn key(non_terminal: &str, terminal: &str) -> (String, String) {
    (non_terminal.to_string(), terminal.to_string())
}
