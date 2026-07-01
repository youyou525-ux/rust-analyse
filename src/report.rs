use std::fs;
use std::path::Path;

use crate::grammar::{AnalysisReport, Symbol};

pub fn render_markdown(report: &AnalysisReport) -> String {
    let mut out = String::new();
    out.push_str("# LL(1) 分析报告\n\n");

    out.push_str("## 原始文法\n\n");
    for production in &report.grammar.productions {
        out.push_str(&format!(
            "- {}. {} -> {}\n",
            production.id,
            production.left,
            format_right(&production.right)
        ));
    }

    out.push_str("\n## FIRST 集\n\n");
    for (name, set) in &report.first_sets {
        out.push_str(&format!("- {} = {{{}}}\n", name, join_set(set)));
    }

    out.push_str("\n## FOLLOW 集\n\n");
    for (name, set) in &report.follow_sets {
        out.push_str(&format!("- {} = {{{}}}\n", name, join_set(set)));
    }

    out.push_str("\n## 预测分析表\n\n");
    for ((non_terminal, terminal), production_id) in &report.table_entries {
        out.push_str(&format!(
            "- M[{}, {}] = {}\n",
            non_terminal, terminal, production_id
        ));
    }

    out.push_str("\n## 预测分析结果\n\n");
    for result in &report.parse_results {
        out.push_str(&format!("### 输入串: {}\n\n", result.input.join(" ")));
        out.push_str(&format!(
            "- 结论: {}\n",
            if result.accepted { "接受" } else { "拒绝" }
        ));
        if let Some(error_message) = &result.error_message {
            out.push_str(&format!("- 错误: {}\n", error_message));
        }
        out.push_str("\n| 步骤 | 分析栈 | 剩余输入 | 动作 |\n");
        out.push_str("| --- | --- | --- | --- |\n");
        for step in &result.steps {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                step.step,
                step.stack.join(" "),
                step.remaining_input.join(" "),
                step.action
            ));
        }
        out.push('\n');
    }

    out
}

pub fn write_report(output_dir: &Path, markdown: &str) -> Result<(), String> {
    fs::create_dir_all(output_dir).map_err(|error| format!("创建输出目录失败: {error}"))?;
    fs::write(output_dir.join("analysis_report.md"), markdown)
        .map_err(|error| format!("写入报告失败: {error}"))
}

fn format_right(right: &[Symbol]) -> String {
    right.iter().map(format_symbol).collect::<Vec<_>>().join(" ")
}

fn format_symbol(symbol: &Symbol) -> String {
    match symbol {
        Symbol::Terminal(value) | Symbol::NonTerminal(value) => value.clone(),
        Symbol::Epsilon => "ε".to_string(),
        Symbol::EndMarker => "$".to_string(),
    }
}

fn join_set(set: &std::collections::BTreeSet<String>) -> String {
    set.iter().cloned().collect::<Vec<_>>().join(", ")
}
