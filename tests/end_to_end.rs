use std::fs;
use std::path::PathBuf;
use std::process::Command;

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
    assert!(report.contains("FIRST 集"));
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
