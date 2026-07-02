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

#[path = "../src/analyzer.rs"]
mod analyzer;

use analyzer::analyze_input;
use parse_table::{build_parse_table, ParseTable};
use std::collections::BTreeMap;

#[test]
fn analyze_expression_inputs_accepts_first_input() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");
    let first_sets = compute_first_sets(&problem.grammar);
    let follow_sets = compute_follow_sets(&problem.grammar, &first_sets);
    let table = build_parse_table(&problem.grammar, &first_sets, &follow_sets)
        .expect("build parse table");

    let result = analyze_input(&problem.grammar, &table, &problem.inputs[0]);

    assert!(result.accepted);
    assert_eq!(result.input, problem.inputs[0]);
    assert!(result.error_message.is_none());
    assert!(!result.steps.is_empty());
    assert_eq!(
        result.steps.first().expect("first step").stack,
        vec!["Expr".to_string(), "$".to_string()]
    );
    assert_eq!(
        result.steps.last().expect("last step").action,
        "accept"
    );
}

#[test]
fn analyze_expression_inputs_rejects_third_input_with_error_message() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");
    let first_sets = compute_first_sets(&problem.grammar);
    let follow_sets = compute_follow_sets(&problem.grammar, &first_sets);
    let table = build_parse_table(&problem.grammar, &first_sets, &follow_sets)
        .expect("build parse table");

    let result = analyze_input(&problem.grammar, &table, &problem.inputs[2]);

    assert!(!result.accepted);
    assert!(result.error_message.is_some());
    assert!(result.steps.last().expect("error step").error);
}

#[test]
fn analyze_epsilon_grammar_accepts_repeated_a_input() {
    let problem = load_problem(Path::new("tests/fixtures/epsilon_grammar.txt"))
        .expect("load fixture");
    let first_sets = compute_first_sets(&problem.grammar);
    let follow_sets = compute_follow_sets(&problem.grammar, &first_sets);
    let table = build_parse_table(&problem.grammar, &first_sets, &follow_sets)
        .expect("build parse table");

    let result = analyze_input(&problem.grammar, &table, &problem.inputs[0]);

    assert!(result.accepted);
    assert!(result.steps.iter().any(|step| step.action.contains("A -> ε")));
}

#[test]
fn analyze_input_rejects_when_parse_table_references_missing_production() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");
    let table = ParseTable {
        entries: BTreeMap::from([(("Expr".to_string(), "id".to_string()), 999usize)]),
    };

    let result = analyze_input(&problem.grammar, &table, &problem.inputs[0]);

    assert!(!result.accepted);
    assert!(result.error_message.is_some());
    assert_eq!(
        result.error_message.as_deref(),
        Some("分析表中的产生式不存在: 999")
    );
    assert!(result.steps.last().expect("error step").error);
}
