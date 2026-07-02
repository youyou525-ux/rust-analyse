use std::collections::{BTreeMap, BTreeSet};

use crate::grammar::{Grammar, Symbol};

pub type FirstSets = BTreeMap<String, BTreeSet<String>>;
pub type FollowSets = BTreeMap<String, BTreeSet<String>>;

pub fn compute_first_sets(grammar: &Grammar) -> FirstSets {
    let mut first_sets = grammar
        .non_terminals
        .iter()
        .cloned()
        .map(|name| (name, BTreeSet::new()))
        .collect::<FirstSets>();

    let mut changed = true;
    while changed {
        changed = false;

        for production in &grammar.productions {
            let (sequence_first, nullable) = first_of_sequence(&production.right, &first_sets);

            let target = first_sets
                .get_mut(&production.left)
                .expect("production left side should be registered");
            for symbol in sequence_first {
                changed |= target.insert(symbol);
            }
            if nullable {
                changed |= target.insert("ε".to_string());
            }
        }
    }

    first_sets
}

pub fn compute_follow_sets(grammar: &Grammar, first_sets: &FirstSets) -> FollowSets {
    let mut follow_sets = grammar
        .non_terminals
        .iter()
        .cloned()
        .map(|name| (name, BTreeSet::new()))
        .collect::<FollowSets>();

    follow_sets
        .get_mut(&grammar.start_symbol)
        .expect("start symbol should be registered")
        .insert("$".to_string());

    let mut changed = true;
    while changed {
        changed = false;

        for production in &grammar.productions {
            for (index, symbol) in production.right.iter().enumerate() {
                let Symbol::NonTerminal(current) = symbol else {
                    continue;
                };

                let (suffix_first, suffix_nullable) =
                    first_of_sequence(&production.right[index + 1..], first_sets);

                {
                    let target = follow_sets
                        .get_mut(current)
                        .expect("non-terminal should be registered");
                    for entry in suffix_first {
                        if entry != "ε" {
                            changed |= target.insert(entry);
                        }
                    }
                }

                if suffix_nullable {
                    let left_follow = follow_sets
                        .get(&production.left)
                        .cloned()
                        .expect("production left side should be registered");
                    let target = follow_sets
                        .get_mut(current)
                        .expect("non-terminal should be registered");
                    for entry in left_follow {
                        changed |= target.insert(entry);
                    }
                }
            }
        }
    }

    follow_sets
}

pub fn first_of_sequence(symbols: &[Symbol], first_sets: &FirstSets) -> (BTreeSet<String>, bool) {
    if symbols.is_empty() {
        return (BTreeSet::new(), true);
    }

    let mut first = BTreeSet::new();
    let mut nullable = true;

    for (index, symbol) in symbols.iter().enumerate() {
        match symbol {
            Symbol::Terminal(value) => {
                first.insert(value.clone());
                nullable = false;
                break;
            }
            Symbol::NonTerminal(name) => {
                let nested_first = first_sets
                    .get(name)
                    .unwrap_or_else(|| panic!("missing FIRST set for non-terminal: {name}"));
                for entry in nested_first {
                    if entry != "ε" {
                        first.insert(entry.clone());
                    }
                }
                if !nested_first.contains("ε") {
                    nullable = false;
                    break;
                }
            }
            Symbol::Epsilon => {
                assert!(
                    symbols.len() == 1 && index == 0,
                    "epsilon must appear alone in a sequence"
                );
                break;
            }
            Symbol::EndMarker => {
                first.insert("$".to_string());
                nullable = false;
                break;
            }
        }
    }

    (first, nullable)
}
