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
            escape_markdown_text(&production.left),
            format_right(&production.right)
        ));
    }

    out.push_str("\n## FIRST 集\n\n");
    for (name, set) in &report.first_sets {
        out.push_str(&format!(
            "- {} = {{{}}}\n",
            escape_markdown_text(name),
            join_set(set)
        ));
    }

    out.push_str("\n## FOLLOW 集\n\n");
    for (name, set) in &report.follow_sets {
        out.push_str(&format!(
            "- {} = {{{}}}\n",
            escape_markdown_text(name),
            join_set(set)
        ));
    }

    out.push_str("\n## 预测分析表\n\n");
    for ((non_terminal, terminal), production_id) in &report.table_entries {
        out.push_str(&format!(
            "- M[{}, {}] = {}\n",
            escape_markdown_text(non_terminal),
            escape_markdown_text(terminal),
            production_id
        ));
    }

    out.push_str("\n## 预测分析结果\n\n");
    for result in &report.parse_results {
        out.push_str(&format!(
            "### 输入串: {}\n\n",
            escape_markdown_text(&result.input.join(" "))
        ));
        out.push_str(&format!(
            "- 结论: {}\n",
            if result.accepted { "接受" } else { "拒绝" }
        ));
        if let Some(error_message) = &result.error_message {
            out.push_str(&format!(
                "- 错误: {}\n",
                escape_markdown_text(error_message)
            ));
        }
        out.push_str("\n| 步骤 | 分析栈 | 剩余输入 | 动作 |\n");
        out.push_str("| --- | --- | --- | --- |\n");
        for step in &result.steps {
            out.push_str(&format!(
                "| {} | {} | {} | {} |\n",
                step.step,
                escape_markdown_table_cell(&step.stack.join(" ")),
                escape_markdown_table_cell(&step.remaining_input.join(" ")),
                escape_markdown_table_cell(&step.action)
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
    right
        .iter()
        .map(format_symbol)
        .map(|symbol| escape_markdown_text(&symbol))
        .collect::<Vec<_>>()
        .join(" ")
}

fn format_symbol(symbol: &Symbol) -> String {
    match symbol {
        Symbol::Terminal(value) | Symbol::NonTerminal(value) => value.clone(),
        Symbol::Epsilon => "ε".to_string(),
        Symbol::EndMarker => "$".to_string(),
    }
}

fn join_set(set: &std::collections::BTreeSet<String>) -> String {
    set.iter()
        .map(|item| escape_markdown_text(item))
        .collect::<Vec<_>>()
        .join(", ")
}

fn escape_markdown_table_cell(value: &str) -> String {
    escape_markdown_text(value)
}

fn escape_markdown_text(value: &str) -> String {
    let mut escaped = String::with_capacity(value.len());

    for ch in value.chars() {
        match ch {
            '|' => escaped.push_str("\\|"),
            '`' => escaped.push_str("\\`"),
            '\n' | '\r' => escaped.push(' '),
            _ => escaped.push(ch),
        }
    }

    escaped.split_whitespace().collect::<Vec<_>>().join(" ")
}
