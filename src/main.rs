mod analyzer;
mod first_follow;
mod grammar;
mod loader;
mod parse_table;
mod report;

use std::path::{Path, PathBuf};

use analyzer::analyze_input;
use first_follow::{compute_first_sets, compute_follow_sets};
use grammar::AnalysisReport;
use loader::load_problem;
use parse_table::{build_parse_table, ParseTableError};
use report::{render_markdown, write_report};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let input_path = args
        .next()
        .ok_or_else(|| "用法: grammar-analyse <输入文件> [输出目录]".to_string())?;
    let output_dir = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("output"));

    let problem = load_problem(Path::new(&input_path))?;
    let first_sets = compute_first_sets(&problem.grammar);
    let follow_sets = compute_follow_sets(&problem.grammar, &first_sets);
    let table = build_parse_table(&problem.grammar, &first_sets, &follow_sets)
        .map_err(format_parse_table_error)?;

    let parse_results = problem
        .inputs
        .iter()
        .map(|input| analyze_input(&problem.grammar, &table, input))
        .collect::<Vec<_>>();

    println!("分析表构造成功");
    for result in &parse_results {
        println!(
            "{} => {}",
            result.input.join(" "),
            if result.accepted { "接受" } else { "拒绝" }
        );
    }

    let report = AnalysisReport {
        grammar: problem.grammar,
        first_sets,
        follow_sets,
        table_entries: table.entries,
        parse_results,
    };

    let markdown = render_markdown(&report);
    write_report(&output_dir, &markdown)?;
    println!("报告已生成: {}", output_dir.join("analysis_report.md").display());

    Ok(())
}

fn format_parse_table_error(error: ParseTableError) -> String {
    match error {
        ParseTableError::Conflict(conflict) => format!(
            "分析表构造失败: ({}, {}) 在产生式 {} 和 {} 间冲突",
            conflict.non_terminal,
            conflict.terminal,
            conflict.existing_production,
            conflict.new_production
        ),
        ParseTableError::MissingFirstSet { non_terminal } => {
            format!("分析表构造失败: 缺少 FIRST 集: {non_terminal}")
        }
        ParseTableError::MissingFollowSet { non_terminal } => {
            format!("分析表构造失败: 缺少 FOLLOW 集: {non_terminal}")
        }
    }
}
