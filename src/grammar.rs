use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Symbol {
    Terminal(String),
    NonTerminal(String),
    Epsilon,
    EndMarker,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Production {
    pub id: usize,
    pub left: String,
    pub right: Vec<Symbol>,
}

#[derive(Debug, Clone)]
pub struct Grammar {
    pub start_symbol: String,
    pub terminals: BTreeSet<String>,
    pub non_terminals: BTreeSet<String>,
    pub productions: Vec<Production>,
    pub productions_by_left: BTreeMap<String, Vec<usize>>,
}

#[derive(Debug, Clone)]
pub struct ProblemInput {
    pub grammar: Grammar,
    pub inputs: Vec<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseStep {
    pub step: usize,
    pub stack: Vec<String>,
    pub remaining_input: Vec<String>,
    pub action: String,
    pub error: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseResult {
    pub input: Vec<String>,
    pub accepted: bool,
    pub steps: Vec<ParseStep>,
    pub error_message: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AnalysisReport {
    pub grammar: Grammar,
    pub first_sets: BTreeMap<String, BTreeSet<String>>,
    pub follow_sets: BTreeMap<String, BTreeSet<String>>,
    pub table_entries: BTreeMap<(String, String), usize>,
    pub parse_results: Vec<ParseResult>,
}
