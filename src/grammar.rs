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
