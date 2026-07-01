use crate::grammar::{Grammar, ParseResult, ParseStep, Symbol};
use crate::parse_table::ParseTable;

pub fn analyze_input(grammar: &Grammar, table: &ParseTable, input: &[String]) -> ParseResult {
    let mut stack = vec![
        Symbol::EndMarker,
        Symbol::NonTerminal(grammar.start_symbol.clone()),
    ];
    let mut remaining_input = input.to_vec();
    remaining_input.push("$".to_string());

    let mut steps = Vec::new();
    let mut step_index = 1;

    loop {
        let stack_snapshot = snapshot_stack(&stack);
        let input_snapshot = remaining_input.clone();
        let Some(top) = stack.pop() else {
            let message = "分析栈意外为空".to_string();
            record_step(
                &mut steps,
                step_index,
                stack_snapshot,
                input_snapshot,
                "error".to_string(),
                true,
            );
            return ParseResult {
                input: input.to_vec(),
                accepted: false,
                steps,
                error_message: Some(message),
            };
        };

        let lookahead = remaining_input
            .first()
            .cloned()
            .unwrap_or_else(|| "$".to_string());

        match top {
            Symbol::Terminal(expected) => {
                if expected == lookahead {
                    remaining_input.remove(0);
                    let action = format!("match {expected}");
                    record_step(
                        &mut steps,
                        step_index,
                        stack_snapshot,
                        input_snapshot,
                        action,
                        false,
                    );
                } else {
                    let message = format!(
                        "终结符不匹配: 期待 {expected}，实际 {lookahead}"
                    );
                    let action = format!(
                        "error: expected {expected}, found {lookahead}"
                    );
                    record_step(
                        &mut steps,
                        step_index,
                        stack_snapshot,
                        input_snapshot,
                        action,
                        true,
                    );
                    return ParseResult {
                        input: input.to_vec(),
                        accepted: false,
                        steps,
                        error_message: Some(message),
                    };
                }
            }
            Symbol::EndMarker => {
                if lookahead == "$" {
                    record_step(
                        &mut steps,
                        step_index,
                        stack_snapshot,
                        input_snapshot,
                        "accept".to_string(),
                        false,
                    );
                    return ParseResult {
                        input: input.to_vec(),
                        accepted: true,
                        steps,
                        error_message: None,
                    };
                }

                let message = format!("输入未消费完成: 当前符号 {lookahead}");
                record_step(
                    &mut steps,
                    step_index,
                    stack_snapshot,
                    input_snapshot,
                    format!("error: unexpected trailing input {lookahead}"),
                    true,
                );
                return ParseResult {
                    input: input.to_vec(),
                    accepted: false,
                    steps,
                    error_message: Some(message),
                };
            }
            Symbol::NonTerminal(non_terminal) => {
                let Some(production_id) = table
                    .entries
                    .get(&(non_terminal.clone(), lookahead.clone()))
                    .copied()
                else {
                    let message = format!(
                        "预测分析表缺少条目: ({non_terminal}, {lookahead})"
                    );
                    let action = format!("error: no rule for {non_terminal} with {lookahead}");
                    record_step(
                        &mut steps,
                        step_index,
                        stack_snapshot,
                        input_snapshot,
                        action,
                        true,
                    );
                    return ParseResult {
                        input: input.to_vec(),
                        accepted: false,
                        steps,
                        error_message: Some(message),
                    };
                };

                let Some(production) = grammar
                    .productions
                    .iter()
                    .find(|production| production.id == production_id)
                else {
                    let message =
                        format!("分析表中的产生式不存在: {production_id}");
                    let action = format!(
                        "error: missing production {production_id} for {non_terminal} with {lookahead}"
                    );
                    record_step(
                        &mut steps,
                        step_index,
                        stack_snapshot,
                        input_snapshot,
                        action,
                        true,
                    );
                    return ParseResult {
                        input: input.to_vec(),
                        accepted: false,
                        steps,
                        error_message: Some(message),
                    };
                };

                for symbol in production.right.iter().rev() {
                    if !matches!(symbol, Symbol::Epsilon) {
                        stack.push(symbol.clone());
                    }
                }

                let action = format!(
                    "expand {} -> {}",
                    production.left,
                    format_symbols(&production.right)
                );
                record_step(
                    &mut steps,
                    step_index,
                    stack_snapshot,
                    input_snapshot,
                    action,
                    false,
                );
            }
            Symbol::Epsilon => {
                record_step(
                    &mut steps,
                    step_index,
                    stack_snapshot,
                    input_snapshot,
                    "skip ε".to_string(),
                    false,
                );
            }
        }

        step_index += 1;
    }
}

fn record_step(
    steps: &mut Vec<ParseStep>,
    step: usize,
    stack: Vec<String>,
    remaining_input: Vec<String>,
    action: String,
    error: bool,
) {
    steps.push(ParseStep {
        step,
        stack,
        remaining_input,
        action,
        error,
    });
}

fn snapshot_stack(stack: &[Symbol]) -> Vec<String> {
    stack.iter().rev().map(format_symbol).collect()
}

fn format_symbols(symbols: &[Symbol]) -> String {
    symbols.iter().map(format_symbol).collect::<Vec<_>>().join(" ")
}

fn format_symbol(symbol: &Symbol) -> String {
    match symbol {
        Symbol::Terminal(value) | Symbol::NonTerminal(value) => value.clone(),
        Symbol::Epsilon => "ε".to_string(),
        Symbol::EndMarker => "$".to_string(),
    }
}
