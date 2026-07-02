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

    let glossary = build_symbol_glossary(report);
    if !glossary.is_empty() {
        out.push_str("\n## 符号说明\n\n");
        for (name, meaning) in glossary {
            out.push_str(&format!(
                "- {}：{}\n",
                escape_markdown_text(&name),
                escape_markdown_text(meaning)
            ));
        }
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

fn build_symbol_glossary(report: &AnalysisReport) -> Vec<(String, &'static str)> {
    let mut glossary = Vec::new();

    for name in &report.grammar.non_terminals {
        let meaning = match name.as_str() {
            "Expr" => Some("表达式（expression），表示这一套文法里最外层的运算结构，也是当前样例里的主要开始分析对象"),
            "ExprP" => Some("表达式后续部分，通常表示在一个项之后，还可以继续接上加法一类的递归扩展"),
            "Term" => Some("项（term），表示表达式里优先级更高的一层结构，通常先于加法层被识别"),
            "TermP" => Some("项的后续部分，通常表示在一个因子之后，还可以继续接上乘法一类的递归扩展"),
            "Factor" => Some("因子（factor），表示最基础的可参与运算单元，例如标识符或括号括起来的子表达式"),
            "S" => Some("开始符号（start symbol），表示整份文法推导时最先展开的入口非终结符"),
            "A" => Some("文法中的辅助非终结符 A，通常用来承接某一类中间推导或递归展开"),
            "B" => Some("文法中的辅助非终结符 B，通常和 A 一样作为辅助推导结构使用"),
            _ => None,
        };

        if let Some(meaning) = meaning {
            glossary.push((name.clone(), meaning));
        }
    }

    for name in &report.grammar.terminals {
        let meaning = match name.as_str() {
            "a" => Some("终结符 a，在教材记号里通常表示一个具体可直接匹配的输入符号"),
            "b" => Some("终结符 b，在教材记号里通常表示一个具体可直接匹配的输入符号"),
            "x" => Some("终结符 x，在教材记号里通常表示一个具体可直接匹配的输入符号"),
            "y" => Some("终结符 y，在教材记号里通常表示一个具体可直接匹配的输入符号"),
            "id" => Some("终结符 id，通常表示标识符，是可以被分析器直接匹配的输入记号"),
            "+" => Some("终结符 +，表示加法运算符，是输入串中可直接匹配的符号"),
            "*" => Some("终结符 *，表示乘法运算符，是输入串中可直接匹配的符号"),
            "(" => Some("终结符 (，表示左括号，是输入串中可直接匹配的符号"),
            ")" => Some("终结符 )，表示右括号，是输入串中可直接匹配的符号"),
            _ => None,
        };

        if let Some(meaning) = meaning {
            glossary.push((name.clone(), meaning));
        }
    }

    glossary.push((
        "$".to_string(),
        "输入结束符，在教材中通常表示输入串已经读完，也常作为分析栈的栈底标记",
    ));
    glossary.push((
        "ε".to_string(),
        "空串，在教材中表示该产生式可以不消耗任何输入符号而直接推出空",
    ));

    glossary.sort_by(|a, b| a.0.cmp(&b.0));
    glossary
}
