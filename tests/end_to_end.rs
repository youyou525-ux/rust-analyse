use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[path = "../src/grammar.rs"]
mod grammar;
#[path = "../src/report.rs"]
mod report;

use std::collections::{BTreeMap, BTreeSet};

use grammar::{AnalysisReport, Grammar, ParseResult, ParseStep, Production, Symbol};

#[test]
fn cli_without_arguments_prints_usage() {
    let binary = env!("CARGO_BIN_EXE_grammar-analyse");
    let output = Command::new(binary).output().expect("run binary");

    assert!(!output.status.success());

    let stderr = String::from_utf8(output.stderr).expect("stderr utf8");
    assert!(stderr.contains("用法"));
    assert!(stderr.contains("grammar-analyse <输入文件>"));
}

#[test]
fn cli_generates_report_for_valid_fixture() {
    let binary = env!("CARGO_BIN_EXE_grammar-analyse");
    let fixture = PathBuf::from("tests/fixtures/expression_grammar.txt");
    let output_dir = unique_output_dir("valid-fixture");

    let output = Command::new(binary)
        .arg(&fixture)
        .arg(&output_dir)
        .output()
        .expect("run binary");

    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).expect("stdout utf8");
    assert!(stdout.contains("分析表构造成功"));
    assert!(stdout.contains("接受"));

    let report_path = output_dir.join("analysis_report.md");
    assert!(report_path.exists());

    let report = fs::read_to_string(report_path).expect("read report");
    assert!(report.contains("# LL(1) 分析报告"));
    assert!(report.contains("符号说明"));
    assert!(report.contains("FIRST 集"));
    assert!(report.contains("Expr -> Term ExprP"));
    assert!(report.contains("M[Expr, id]"));
    assert!(report.contains("$：输入结束符"));
    assert!(report.contains("Expr：表达式（expression）"));
    assert!(report.contains("ExprP：表达式后续部分"));
    assert!(report.contains("ε：空串"));
    assert!(report.contains("id：终结符 id"));
}

fn unique_output_dir(label: &str) -> PathBuf {
    let mut path = std::env::temp_dir();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .expect("system time before epoch")
        .as_nanos();
    path.push(format!("grammar-analyse-{label}-{nanos}"));
    fs::create_dir_all(&path).expect("create output dir");
    path
}

#[test]
fn render_markdown_escapes_dynamic_content() {
    let mut terminals = BTreeSet::new();
    terminals.insert("id".to_string());
    terminals.insert("x|y".to_string());

    let mut non_terminals = BTreeSet::new();
    non_terminals.insert("Expr".to_string());

    let productions = vec![Production {
        id: 1,
        left: "Expr".to_string(),
        right: vec![
            Symbol::Terminal("x|y".to_string()),
            Symbol::Terminal("line\nbreak".to_string()),
            Symbol::Terminal("tick`mark".to_string()),
        ],
    }];

    let grammar = Grammar {
        start_symbol: "Expr".to_string(),
        terminals,
        non_terminals,
        productions,
        productions_by_left: BTreeMap::from([("Expr".to_string(), vec![1])]),
    };

    let first_sets = BTreeMap::from([(
        "Expr".to_string(),
        BTreeSet::from(["x|y".to_string(), "line\nbreak".to_string()]),
    )]);
    let follow_sets = BTreeMap::from([(
        "Expr".to_string(),
        BTreeSet::from(["tick`mark".to_string(), "carriage\rreturn".to_string()]),
    )]);
    let table_entries = BTreeMap::from([(("Expr".to_string(), "x|y".to_string()), 1usize)]);
    let parse_results = vec![ParseResult {
        input: vec!["x|y".to_string(), "line\nbreak".to_string()],
        accepted: false,
        steps: vec![ParseStep {
            step: 1,
            stack: vec!["Expr".to_string(), "tick`mark".to_string()],
            remaining_input: vec!["x|y".to_string(), "line\nbreak".to_string()],
            action: "expand Expr -> x|y `tick`\r\nnext".to_string(),
            error: true,
        }],
        error_message: Some("bad|token\r\nwith`tick".to_string()),
    }];

    let report = report::render_markdown(&AnalysisReport {
        grammar,
        first_sets,
        follow_sets,
        table_entries,
        parse_results,
    });

    assert!(report.contains("x\\|y line break tick\\`mark"));
    assert!(report.contains("Expr = {line break, x\\|y}"));
    assert!(report.contains("Expr = {carriage return, tick\\`mark}"));
    assert!(report.contains("### 输入串: x\\|y line break"));
    assert!(report.contains("- 错误: bad\\|token with\\`tick"));
    assert!(report.contains("| 1 | Expr tick\\`mark | x\\|y line break | expand Expr -> x\\|y \\`tick\\` next |"));

    let input_heading = report
        .lines()
        .find(|line| line.starts_with("### 输入串:"))
        .expect("input heading exists");
    assert!(!input_heading.contains('\n'));
    assert!(!input_heading.contains('\r'));
}
