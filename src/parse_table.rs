use std::collections::BTreeMap;

use crate::first_follow::{first_of_sequence, FirstSets, FollowSets};
use crate::grammar::Grammar;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseTable {
    pub entries: BTreeMap<(String, String), usize>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableConflict {
    pub non_terminal: String,
    pub terminal: String,
    pub existing_production: usize,
    pub new_production: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParseTableError {
    Conflict(TableConflict),
    MissingFirstSet { non_terminal: String },
    MissingFollowSet { non_terminal: String },
}

pub fn build_parse_table(
    grammar: &Grammar,
    first_sets: &FirstSets,
    follow_sets: &FollowSets,
) -> Result<ParseTable, ParseTableError> {
    for non_terminal in &grammar.non_terminals {
        if !first_sets.contains_key(non_terminal) {
            return Err(ParseTableError::MissingFirstSet {
                non_terminal: non_terminal.clone(),
            });
        }
    }

    let mut table = ParseTable {
        entries: BTreeMap::new(),
    };

    for production in &grammar.productions {
        let (sequence_first, nullable) = first_of_sequence(&production.right, first_sets);

        for terminal in sequence_first.iter() {
            insert_entry(&mut table, &production.left, terminal, production.id)?;
        }

        if nullable {
            let follow = follow_sets.get(&production.left).ok_or_else(|| {
                ParseTableError::MissingFollowSet {
                    non_terminal: production.left.clone(),
                }
            })?;
            for terminal in follow {
                insert_entry(&mut table, &production.left, terminal, production.id)?;
            }
        }
    }

    Ok(table)
}

fn insert_entry(
    table: &mut ParseTable,
    non_terminal: &str,
    terminal: &str,
    production_id: usize,
) -> Result<(), ParseTableError> {
    let key = (non_terminal.to_string(), terminal.to_string());

    if let Some(existing_production) = table.entries.get(&key) {
        if *existing_production != production_id {
            return Err(ParseTableError::Conflict(TableConflict {
                non_terminal: non_terminal.to_string(),
                terminal: terminal.to_string(),
                existing_production: *existing_production,
                new_production: production_id,
            }));
        }

        return Ok(());
    }

    table.entries.insert(key, production_id);
    Ok(())
}
