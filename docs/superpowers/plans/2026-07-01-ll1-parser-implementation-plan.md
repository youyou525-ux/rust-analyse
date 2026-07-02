# LL(1) 语法分析器实现计划

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** 使用 Rust 实现一个最小完整的 LL(1) 语法分析器，支持从文件读取文法和输入串，计算 `FIRST/FOLLOW`、构造预测分析表、执行预测分析，并输出终端摘要和 Markdown 报告。

**Architecture:** 项目采用单向流水线结构：`loader -> first_follow -> parse_table -> analyzer -> report`。所有算法模块返回结构化数据，`main` 只负责串联流程和处理命令行参数，以保证实现简单、流程清晰、便于展示与测试。

**Tech Stack:** Rust stable、Cargo、Rust 标准库、`std::process::Command` 集成测试

---

## 文件结构

### 计划创建的文件

- `Cargo.toml`
  责任：定义二进制项目基础信息。
- `src/main.rs`
  责任：解析命令行参数，串联完整分析流程，打印终端摘要。
- `src/lib.rs`
  责任：统一导出各模块。
- `src/grammar.rs`
  责任：定义 `Symbol`、`Production`、`Grammar`、`ParseStep`、`AnalysisReport` 等核心结构。
- `src/loader.rs`
  责任：解析输入文件并构建 `Grammar` 与输入串列表。
- `src/first_follow.rs`
  责任：计算 `FIRST` 与 `FOLLOW` 集。
- `src/parse_table.rs`
  责任：构造预测分析表并检测冲突。
- `src/analyzer.rs`
  责任：执行栈驱动预测分析并记录过程。
- `src/report.rs`
  责任：生成 Markdown 报告文本并写入文件。
- `tests/fixtures/expression_grammar.txt`
  责任：经典表达式文法样例，用于主流程测试与展示。
- `tests/fixtures/epsilon_grammar.txt`
  责任：包含空产生式的简单样例。
- `tests/fixtures/conflict_grammar.txt`
  责任：冲突文法样例，用于负向测试。
- `tests/loader_tests.rs`
  责任：验证输入文件解析。
- `tests/first_follow_tests.rs`
  责任：验证 `FIRST/FOLLOW` 计算。
- `tests/parse_table_tests.rs`
  责任：验证分析表构造和冲突检测。
- `tests/analyzer_tests.rs`
  责任：验证预测分析过程。
- `tests/end_to_end.rs`
  责任：验证完整命令行流程和报告生成。

### 关键接口约定

- `loader::load_problem(path: &Path) -> Result<ProblemInput, String>`
- `first_follow::compute_first_sets(grammar: &Grammar) -> BTreeMap<String, BTreeSet<String>>`
- `first_follow::compute_follow_sets(grammar: &Grammar, first_sets: &FirstSets) -> BTreeMap<String, BTreeSet<String>>`
- `parse_table::build_parse_table(grammar: &Grammar, first_sets: &FirstSets, follow_sets: &FollowSets) -> Result<ParseTable, TableConflict>`
- `analyzer::analyze_input(grammar: &Grammar, table: &ParseTable, input: &[String]) -> ParseResult`
- `report::render_markdown(report: &AnalysisReport) -> String`

这些接口在后续任务中保持一致，不再改名。

### 样例文法约定

为避免实现被输入细节拖慢，统一采用空格分词：

- 文法右部使用空格分隔符号
- 输入串每个符号之间也用空格分隔
- 空串固定写作 `ε`

这样可以把词法问题压到最低，只保留 LL(1) 分析核心。

### Task 1: 项目骨架与命令行入口

**Files:**
- Create: `Cargo.toml`
- Create: `src/main.rs`
- Create: `src/lib.rs`
- Create: `tests/end_to_end.rs`

- [ ] **Step 1: 先写一个失败的端到端测试，约束命令行入口行为**

```rust
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
```

- [ ] **Step 2: 运行测试，确认当前状态失败**

Run: `rtk cargo test --test end_to_end cli_without_arguments_prints_usage`

Expected: FAIL，原因应为缺少 `Cargo.toml` 或二进制尚未定义。

- [ ] **Step 3: 创建最小可运行项目骨架**

`Cargo.toml`

```toml
[package]
name = "grammar-analyse"
version = "0.1.0"
edition = "2021"

[dependencies]
```

`src/lib.rs`

```rust
pub fn version_banner() -> &'static str {
    "grammar-analyse"
}
```

`src/main.rs`

```rust
fn main() {
    let mut args = std::env::args().skip(1);

    if args.next().is_none() {
        eprintln!("用法: grammar-analyse <输入文件> [输出目录]");
        std::process::exit(1);
    }

    println!("{}", grammar_analyse::version_banner());
}
```

- [ ] **Step 4: 重新运行测试，确认命令行最小行为满足约束**

Run: `rtk cargo test --test end_to_end cli_without_arguments_prints_usage`

Expected: PASS

- [ ] **Step 5: 初始化 Git 并提交项目骨架**

Run:

```bash
git init
git add Cargo.toml src/main.rs src/lib.rs tests/end_to_end.rs
git commit -m "chore: bootstrap rust ll1 analyzer"
```

Expected: 成功创建首个提交。

### Task 2: 文法模型与输入文件解析

**Files:**
- Modify: `src/lib.rs`
- Create: `src/grammar.rs`
- Create: `src/loader.rs`
- Create: `tests/fixtures/expression_grammar.txt`
- Create: `tests/fixtures/conflict_grammar.txt`
- Create: `tests/loader_tests.rs`

- [ ] **Step 1: 先写解析测试，固定输入文件格式**

`tests/loader_tests.rs`

```rust
use std::path::Path;

use grammar_analyse::loader::load_problem;

#[test]
fn load_expression_grammar_file() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");

    assert_eq!(problem.grammar.start_symbol, "Expr");
    assert!(problem.grammar.terminals.contains("+"));
    assert_eq!(problem.grammar.productions.len(), 8);
    assert_eq!(problem.inputs.len(), 3);
    assert_eq!(problem.inputs[0], vec!["id", "+", "id", "*", "id"]);
}

#[test]
fn reject_conflict_fixture_with_unknown_start_symbol() {
    let error = load_problem(Path::new("tests/fixtures/conflict_grammar.txt"))
        .err()
        .expect("expected error");

    assert!(error.contains("开始符号"));
}
```

`tests/fixtures/expression_grammar.txt`

```txt
%start Expr
%token id + * ( )
%%
Expr -> Term ExprP
ExprP -> + Term ExprP | ε
Term -> Factor TermP
TermP -> * Factor TermP | ε
Factor -> ( Expr ) | id
%%
id + id * id
( id + id ) * id
id + + id
```

`tests/fixtures/conflict_grammar.txt`

```txt
%start Missing
%token a b
%%
S -> a A | a B
A -> b
B -> ε
%%
a b
```

- [ ] **Step 2: 运行解析测试，确认其失败**

Run: `rtk cargo test --test loader_tests`

Expected: FAIL，原因应为 `grammar_analyse::loader` 模块尚不存在。

- [ ] **Step 3: 实现核心文法结构和文件解析**

`src/lib.rs`

```rust
pub mod grammar;
pub mod loader;

pub fn version_banner() -> &'static str {
    "grammar-analyse"
}
```

`src/grammar.rs`

```rust
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
```

`src/loader.rs`

```rust
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use crate::grammar::{Grammar, ProblemInput, Production, Symbol};

pub fn load_problem(path: &Path) -> Result<ProblemInput, String> {
    let raw = fs::read_to_string(path).map_err(|e| format!("读取文件失败: {e}"))?;
    let sections: Vec<&str> = raw.split("\n%%\n").collect();

    if sections.len() != 3 {
        return Err("输入文件必须包含两个 %% 分隔段".to_string());
    }

    let header = sections[0];
    let grammar_block = sections[1];
    let input_block = sections[2];

    let mut start_symbol = None;
    let mut terminals = BTreeSet::new();

    for line in header.lines().map(str::trim).filter(|line| !line.is_empty()) {
        if let Some(rest) = line.strip_prefix("%start ") {
            start_symbol = Some(rest.trim().to_string());
        } else if let Some(rest) = line.strip_prefix("%token ") {
            for token in rest.split_whitespace() {
                terminals.insert(token.to_string());
            }
        }
    }

    let start_symbol = start_symbol.ok_or_else(|| "缺少开始符号声明".to_string())?;

    let mut productions = Vec::new();
    let mut non_terminals = BTreeSet::new();

    for line in grammar_block.lines().map(str::trim).filter(|line| !line.is_empty()) {
        let (left, right) = line
            .split_once("->")
            .ok_or_else(|| format!("产生式格式错误: {line}"))?;

        let left = left.trim().to_string();
        non_terminals.insert(left.clone());

        for branch in right.trim().split('|') {
            let symbols = parse_right_side(branch.trim());
            productions.push(Production {
                id: productions.len() + 1,
                left: left.clone(),
                right: symbols,
            });
        }
    }

    if !non_terminals.contains(&start_symbol) {
        return Err(format!("开始符号不存在于产生式左部: {start_symbol}"));
    }

    validate_symbols(&productions, &terminals, &non_terminals)?;

    let mut productions_by_left: BTreeMap<String, Vec<usize>> = BTreeMap::new();
    for production in &productions {
        productions_by_left
            .entry(production.left.clone())
            .or_default()
            .push(production.id);
    }

    let inputs = input_block
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.split_whitespace().map(str::to_string).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    Ok(ProblemInput {
        grammar: Grammar {
            start_symbol,
            terminals,
            non_terminals,
            productions,
            productions_by_left,
        },
        inputs,
    })
}

fn parse_right_side(branch: &str) -> Vec<Symbol> {
    if branch == "ε" {
        return vec![Symbol::Epsilon];
    }

    branch
        .split_whitespace()
        .map(|part| Symbol::Terminal(part.to_string()))
        .collect()
}

fn validate_symbols(
    productions: &[Production],
    terminals: &BTreeSet<String>,
    non_terminals: &BTreeSet<String>,
) -> Result<(), String> {
    for production in productions {
        for symbol in &production.right {
            if let Symbol::Terminal(value) = symbol {
                if value == "ε" {
                    continue;
                }

                if !terminals.contains(value) && !non_terminals.contains(value) {
                    return Err(format!("符号未声明: {value}"));
                }
            }
        }
    }

    Ok(())
}
```

- [ ] **Step 4: 修正右部符号分类，使非终结符不被当作终结符**

将 `src/loader.rs` 中的 `parse_right_side` 调整为接收 `non_terminals`，并在第二次遍历中完成右部分类：

```rust
fn parse_right_side(branch: &str, non_terminals: &BTreeSet<String>) -> Vec<Symbol> {
    if branch == "ε" {
        return vec![Symbol::Epsilon];
    }

    branch
        .split_whitespace()
        .map(|part| {
            if non_terminals.contains(part) {
                Symbol::NonTerminal(part.to_string())
            } else {
                Symbol::Terminal(part.to_string())
            }
        })
        .collect()
}
```

同时将 `load_problem` 拆成两轮：

- 第一轮：只收集左部，确定所有非终结符
- 第二轮：真正生成 `Production`

Run: `rtk cargo test --test loader_tests`

Expected: PASS

- [ ] **Step 5: 提交文法模型和加载器**

```bash
git add src/lib.rs src/grammar.rs src/loader.rs tests/fixtures/expression_grammar.txt tests/fixtures/conflict_grammar.txt tests/loader_tests.rs
git commit -m "feat: add grammar model and loader"
```

### Task 3: `FIRST` 与 `FOLLOW` 集计算

**Files:**
- Modify: `src/lib.rs`
- Create: `src/first_follow.rs`
- Create: `tests/first_follow_tests.rs`

- [ ] **Step 1: 先写集合计算测试**

`tests/first_follow_tests.rs`

```rust
use std::path::Path;

use grammar_analyse::first_follow::{compute_first_sets, compute_follow_sets};
use grammar_analyse::loader::load_problem;

#[test]
fn compute_first_and_follow_for_expression_grammar() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");

    let first = compute_first_sets(&problem.grammar);
    let follow = compute_follow_sets(&problem.grammar, &first);

    assert_eq!(first["Expr"], ["(", "id"].into_iter().map(str::to_string).collect());
    assert_eq!(first["ExprP"], ["+", "ε"].into_iter().map(str::to_string).collect());
    assert_eq!(follow["Expr"], ["$", ")"].into_iter().map(str::to_string).collect());
    assert_eq!(follow["Term"], ["$", ")", "+"].into_iter().map(str::to_string).collect());
}
```

- [ ] **Step 2: 运行测试，确认失败**

Run: `rtk cargo test --test first_follow_tests`

Expected: FAIL，原因应为 `first_follow` 模块不存在。

- [ ] **Step 3: 实现 `FIRST` 与 `FOLLOW` 计算**

`src/lib.rs`

```rust
pub mod analyzer;
pub mod first_follow;
pub mod grammar;
pub mod loader;
pub mod parse_table;
pub mod report;

pub fn version_banner() -> &'static str {
    "grammar-analyse"
}
```

`src/first_follow.rs`

```rust
use std::collections::{BTreeMap, BTreeSet};

use crate::grammar::{Grammar, Symbol};

pub type FirstSets = BTreeMap<String, BTreeSet<String>>;
pub type FollowSets = BTreeMap<String, BTreeSet<String>>;

pub fn compute_first_sets(grammar: &Grammar) -> FirstSets {
    let mut first = grammar
        .non_terminals
        .iter()
        .map(|nt| (nt.clone(), BTreeSet::new()))
        .collect::<FirstSets>();

    let mut changed = true;
    while changed {
        changed = false;

        for production in &grammar.productions {
            let left = production.left.clone();
            let mut all_nullable = true;

            for symbol in &production.right {
                match symbol {
                    Symbol::Terminal(value) => {
                        changed |= first.get_mut(&left).unwrap().insert(value.clone());
                        all_nullable = false;
                        break;
                    }
                    Symbol::NonTerminal(value) => {
                        let snapshot = first.get(value).cloned().unwrap_or_default();
                        for item in snapshot.iter().filter(|item| item.as_str() != "ε") {
                            changed |= first.get_mut(&left).unwrap().insert(item.clone());
                        }
                        if !snapshot.contains("ε") {
                            all_nullable = false;
                            break;
                        }
                    }
                    Symbol::Epsilon => {
                        changed |= first.get_mut(&left).unwrap().insert("ε".to_string());
                        all_nullable = false;
                        break;
                    }
                    Symbol::EndMarker => {
                        all_nullable = false;
                        break;
                    }
                }
            }

            if all_nullable {
                changed |= first.get_mut(&left).unwrap().insert("ε".to_string());
            }
        }
    }

    first
}

pub fn compute_follow_sets(grammar: &Grammar, first_sets: &FirstSets) -> FollowSets {
    let mut follow = grammar
        .non_terminals
        .iter()
        .map(|nt| (nt.clone(), BTreeSet::new()))
        .collect::<FollowSets>();

    follow
        .get_mut(&grammar.start_symbol)
        .unwrap()
        .insert("$".to_string());

    let mut changed = true;
    while changed {
        changed = false;

        for production in &grammar.productions {
            for index in 0..production.right.len() {
                let Symbol::NonTerminal(current) = &production.right[index] else {
                    continue;
                };

                let (suffix_first, suffix_nullable) =
                    first_of_sequence(&production.right[index + 1..], first_sets);

                for item in suffix_first.iter().filter(|item| item.as_str() != "ε") {
                    changed |= follow.get_mut(current).unwrap().insert(item.clone());
                }

                if suffix_nullable {
                    let left_follow = follow.get(&production.left).cloned().unwrap_or_default();
                    for item in left_follow {
                        changed |= follow.get_mut(current).unwrap().insert(item);
                    }
                }
            }
        }
    }

    follow
}

pub fn first_of_sequence(
    symbols: &[Symbol],
    first_sets: &FirstSets,
) -> (BTreeSet<String>, bool) {
    let mut result = BTreeSet::new();
    if symbols.is_empty() {
        return (result, true);
    }

    let mut nullable = true;
    for symbol in symbols {
        match symbol {
            Symbol::Terminal(value) => {
                result.insert(value.clone());
                nullable = false;
                break;
            }
            Symbol::NonTerminal(value) => {
                let set = first_sets.get(value).cloned().unwrap_or_default();
                for item in set.iter().filter(|item| item.as_str() != "ε") {
                    result.insert(item.clone());
                }
                if !set.contains("ε") {
                    nullable = false;
                    break;
                }
            }
            Symbol::Epsilon => {
                result.insert("ε".to_string());
                break;
            }
            Symbol::EndMarker => {
                result.insert("$".to_string());
                nullable = false;
                break;
            }
        }
    }

    if nullable {
        result.insert("ε".to_string());
    }

    (result, nullable)
}
```

- [ ] **Step 4: 运行测试并补一个空串场景断言**

在 `tests/first_follow_tests.rs` 追加：

```rust
#[test]
fn first_of_expression_prime_contains_epsilon() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");

    let first = compute_first_sets(&problem.grammar);
    assert!(first["ExprP"].contains("ε"));
}
```

Run: `rtk cargo test --test first_follow_tests`

Expected: PASS

- [ ] **Step 5: 提交集合计算模块**

```bash
git add src/lib.rs src/first_follow.rs tests/first_follow_tests.rs
git commit -m "feat: add first and follow set calculation"
```

### Task 4: 预测分析表构造与冲突检测

**Files:**
- Create: `src/parse_table.rs`
- Create: `tests/parse_table_tests.rs`

- [ ] **Step 1: 先写分析表测试**

`tests/parse_table_tests.rs`

```rust
use std::path::Path;

use grammar_analyse::first_follow::{compute_first_sets, compute_follow_sets};
use grammar_analyse::loader::load_problem;
use grammar_analyse::parse_table::build_parse_table;

#[test]
fn build_table_for_expression_grammar() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");
    let first = compute_first_sets(&problem.grammar);
    let follow = compute_follow_sets(&problem.grammar, &first);

    let table = build_parse_table(&problem.grammar, &first, &follow).expect("build table");

    assert_eq!(table.entries[&("Expr".to_string(), "id".to_string())], 1);
    assert!(table.entries.contains_key(&("Factor".to_string(), "(".to_string())));
}
```

- [ ] **Step 2: 追加冲突测试并确认失败**

在 `tests/parse_table_tests.rs` 追加：

```rust
#[test]
fn detect_conflict_for_non_ll1_grammar() {
    let fixture = std::path::Path::new("tests/fixtures/table_conflict_grammar.txt");
    let problem = load_problem(fixture).expect("load fixture");
    let first = compute_first_sets(&problem.grammar);
    let follow = compute_follow_sets(&problem.grammar, &first);

    let error = build_parse_table(&problem.grammar, &first, &follow)
        .err()
        .expect("expected conflict");

    assert_eq!(error.non_terminal, "S");
    assert_eq!(error.terminal, "a");
}
```

`tests/fixtures/table_conflict_grammar.txt`

```txt
%start S
%token a b
%%
S -> a A | a B
A -> b
B -> ε
%%
a b
```

Run: `rtk cargo test --test parse_table_tests`

Expected: FAIL，原因应为 `parse_table` 模块尚未实现。

- [ ] **Step 3: 实现分析表与冲突结构**

`src/parse_table.rs`

```rust
use std::collections::BTreeMap;

use crate::first_follow::{first_of_sequence, FirstSets, FollowSets};
use crate::grammar::Grammar;

#[derive(Debug, Clone)]
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

pub fn build_parse_table(
    grammar: &Grammar,
    first_sets: &FirstSets,
    follow_sets: &FollowSets,
) -> Result<ParseTable, TableConflict> {
    let mut entries = BTreeMap::new();

    for production in &grammar.productions {
        let (first_alpha, nullable) = first_of_sequence(&production.right, first_sets);

        for terminal in first_alpha.iter().filter(|item| item.as_str() != "ε") {
            insert_entry(
                &mut entries,
                production.left.clone(),
                terminal.clone(),
                production.id,
            )?;
        }

        if nullable || first_alpha.contains("ε") {
            if let Some(follow) = follow_sets.get(&production.left) {
                for terminal in follow {
                    insert_entry(
                        &mut entries,
                        production.left.clone(),
                        terminal.clone(),
                        production.id,
                    )?;
                }
            }
        }
    }

    Ok(ParseTable { entries })
}

fn insert_entry(
    entries: &mut BTreeMap<(String, String), usize>,
    non_terminal: String,
    terminal: String,
    production_id: usize,
) -> Result<(), TableConflict> {
    let key = (non_terminal.clone(), terminal.clone());

    if let Some(existing) = entries.insert(key, production_id) {
        if existing != production_id {
            return Err(TableConflict {
                non_terminal,
                terminal,
                existing_production: existing,
                new_production: production_id,
            });
        }
    }

    Ok(())
}
```

- [ ] **Step 4: 运行测试并修正 `insert_entry` 的覆盖写入缺陷**

将 `insert_entry` 改成先查询、后写入，避免冲突时破坏原值：

```rust
fn insert_entry(
    entries: &mut BTreeMap<(String, String), usize>,
    non_terminal: String,
    terminal: String,
    production_id: usize,
) -> Result<(), TableConflict> {
    let key = (non_terminal.clone(), terminal.clone());

    if let Some(existing) = entries.get(&key) {
        if *existing != production_id {
            return Err(TableConflict {
                non_terminal,
                terminal,
                existing_production: *existing,
                new_production: production_id,
            });
        }
        return Ok(());
    }

    entries.insert(key, production_id);
    Ok(())
}
```

Run: `rtk cargo test --test parse_table_tests`

Expected: PASS

- [ ] **Step 5: 提交分析表模块**

```bash
git add src/parse_table.rs tests/fixtures/table_conflict_grammar.txt tests/parse_table_tests.rs
git commit -m "feat: add ll1 parse table builder"
```

### Task 5: 栈驱动预测分析

**Files:**
- Modify: `src/grammar.rs`
- Create: `src/analyzer.rs`
- Create: `tests/fixtures/epsilon_grammar.txt`
- Create: `tests/analyzer_tests.rs`

- [ ] **Step 1: 先写预测分析测试**

`tests/analyzer_tests.rs`

```rust
use std::path::Path;

use grammar_analyse::analyzer::analyze_input;
use grammar_analyse::first_follow::{compute_first_sets, compute_follow_sets};
use grammar_analyse::loader::load_problem;
use grammar_analyse::parse_table::build_parse_table;

#[test]
fn accept_expression_input() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");
    let first = compute_first_sets(&problem.grammar);
    let follow = compute_follow_sets(&problem.grammar, &first);
    let table = build_parse_table(&problem.grammar, &first, &follow).expect("build table");

    let result = analyze_input(&problem.grammar, &table, &problem.inputs[0]);

    assert!(result.accepted);
    assert!(!result.steps.is_empty());
}

#[test]
fn reject_invalid_expression_input() {
    let problem = load_problem(Path::new("tests/fixtures/expression_grammar.txt"))
        .expect("load fixture");
    let first = compute_first_sets(&problem.grammar);
    let follow = compute_follow_sets(&problem.grammar, &first);
    let table = build_parse_table(&problem.grammar, &first, &follow).expect("build table");

    let result = analyze_input(&problem.grammar, &table, &problem.inputs[2]);

    assert!(!result.accepted);
    assert!(result.error_message.is_some());
}
```

- [ ] **Step 2: 运行测试，确认失败**

Run: `rtk cargo test --test analyzer_tests`

Expected: FAIL，原因应为 `analyzer` 模块尚未实现。

- [ ] **Step 3: 扩展步骤结构并实现分析器**

在 `src/grammar.rs` 末尾追加：

```rust
#[derive(Debug, Clone)]
pub struct ParseStep {
    pub step: usize,
    pub stack: Vec<String>,
    pub remaining_input: Vec<String>,
    pub action: String,
    pub error: bool,
}

#[derive(Debug, Clone)]
pub struct ParseResult {
    pub input: Vec<String>,
    pub accepted: bool,
    pub steps: Vec<ParseStep>,
    pub error_message: Option<String>,
}
```

`src/analyzer.rs`

```rust
use crate::grammar::{Grammar, ParseResult, ParseStep, Symbol};
use crate::parse_table::ParseTable;

pub fn analyze_input(grammar: &Grammar, table: &ParseTable, input: &[String]) -> ParseResult {
    let mut stack = vec!["$".to_string(), grammar.start_symbol.clone()];
    let mut tokens = input.to_vec();
    tokens.push("$".to_string());

    let mut index = 0usize;
    let mut steps = Vec::new();

    loop {
        let top = stack.last().cloned().unwrap_or_default();
        let current = tokens.get(index).cloned().unwrap_or_else(|| "$".to_string());

        let stack_snapshot = stack.clone();
        let remaining_input = tokens[index..].to_vec();

        if top == "$" && current == "$" {
            steps.push(ParseStep {
                step: steps.len() + 1,
                stack: stack_snapshot,
                remaining_input,
                action: "接受".to_string(),
                error: false,
            });
            return ParseResult {
                input: input.to_vec(),
                accepted: true,
                steps,
                error_message: None,
            };
        }

        if grammar.terminals.contains(&top) || top == "$" {
            if top == current {
                stack.pop();
                index += 1;
                steps.push(ParseStep {
                    step: steps.len() + 1,
                    stack: stack_snapshot,
                    remaining_input,
                    action: format!("匹配终结符 {current}"),
                    error: false,
                });
                continue;
            }

            return reject(input, steps, stack_snapshot, remaining_input, format!("终结符不匹配: 栈顶 {top}, 当前输入 {current}"));
        }

        let key = (top.clone(), current.clone());
        let Some(production_id) = table.entries.get(&key).copied() else {
            return reject(input, steps, stack_snapshot, remaining_input, format!("分析表查询失败: ({top}, {current})"));
        };

        let production = grammar
            .productions
            .iter()
            .find(|item| item.id == production_id)
            .expect("production exists");

        stack.pop();
        for symbol in production.right.iter().rev() {
            match symbol {
                Symbol::Terminal(value) | Symbol::NonTerminal(value) => stack.push(value.clone()),
                Symbol::Epsilon => {}
                Symbol::EndMarker => stack.push("$".to_string()),
            }
        }

        steps.push(ParseStep {
            step: steps.len() + 1,
            stack: stack_snapshot,
            remaining_input,
            action: format!("使用产生式 {}: {} -> ...", production.id, production.left),
            error: false,
        });
    }
}

fn reject(
    input: &[String],
    mut steps: Vec<ParseStep>,
    stack: Vec<String>,
    remaining_input: Vec<String>,
    message: String,
) -> ParseResult {
    steps.push(ParseStep {
        step: steps.len() + 1,
        stack,
        remaining_input,
        action: message.clone(),
        error: true,
    });

    ParseResult {
        input: input.to_vec(),
        accepted: false,
        steps,
        error_message: Some(message),
    }
}
```

- [ ] **Step 4: 补一个空产生式测试并运行**

`tests/fixtures/epsilon_grammar.txt`

```txt
%start S
%token a
%%
S -> A
A -> a A | ε
%%
a a

```

在 `tests/analyzer_tests.rs` 追加：

```rust
#[test]
fn accept_input_with_epsilon_production() {
    let problem = load_problem(Path::new("tests/fixtures/epsilon_grammar.txt"))
        .expect("load fixture");
    let first = compute_first_sets(&problem.grammar);
    let follow = compute_follow_sets(&problem.grammar, &first);
    let table = build_parse_table(&problem.grammar, &first, &follow).expect("build table");

    let result = analyze_input(&problem.grammar, &table, &problem.inputs[0]);
    assert!(result.accepted);
}
```

Run: `rtk cargo test --test analyzer_tests`

Expected: PASS

- [ ] **Step 5: 提交预测分析模块**

```bash
git add src/grammar.rs src/analyzer.rs tests/fixtures/epsilon_grammar.txt tests/analyzer_tests.rs
git commit -m "feat: add predictive parser"
```

### Task 6: 报告生成与主流程串联

**Files:**
- Create: `src/report.rs`
- Modify: `src/main.rs`
- Modify: `src/grammar.rs`
- Modify: `tests/end_to_end.rs`

- [ ] **Step 1: 先写完整流程测试**

将 `tests/end_to_end.rs` 扩展为：

```rust
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
}

#[test]
fn cli_generates_report_for_valid_fixture() {
    let binary = env!("CARGO_BIN_EXE_grammar-analyse");
    let output_dir = PathBuf::from("target/test-output");
    let _ = fs::remove_dir_all(&output_dir);
    fs::create_dir_all(&output_dir).expect("create output dir");

    let output = Command::new(binary)
        .arg("tests/fixtures/expression_grammar.txt")
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
```

- [ ] **Step 2: 运行测试，确认失败**

Run: `rtk cargo test --test end_to_end cli_generates_report_for_valid_fixture`

Expected: FAIL，原因应为主流程尚未串联、报告尚未生成。

- [ ] **Step 3: 定义总报告结构并实现 Markdown 输出**

在 `src/grammar.rs` 末尾追加：

```rust
#[derive(Debug, Clone)]
pub struct AnalysisReport {
    pub grammar: Grammar,
    pub first_sets: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    pub follow_sets: std::collections::BTreeMap<String, std::collections::BTreeSet<String>>,
    pub table_entries: std::collections::BTreeMap<(String, String), usize>,
    pub parse_results: Vec<ParseResult>,
}
```

`src/report.rs`

```rust
use std::fs;
use std::path::Path;

use crate::grammar::AnalysisReport;

pub fn render_markdown(report: &AnalysisReport) -> String {
    let mut out = String::new();
    out.push_str("# LL(1) 分析报告\n\n");
    out.push_str("## 原始文法\n\n");
    for production in &report.grammar.productions {
        out.push_str(&format!("{}: {} -> {}\n\n", production.id, production.left, format_right(&production.right)));
    }

    out.push_str("## FIRST 集\n\n");
    for (name, set) in &report.first_sets {
        out.push_str(&format!("- {} = {:?}\n", name, set));
    }

    out.push_str("\n## FOLLOW 集\n\n");
    for (name, set) in &report.follow_sets {
        out.push_str(&format!("- {} = {:?}\n", name, set));
    }

    out.push_str("\n## 预测分析结果\n\n");
    for result in &report.parse_results {
        out.push_str(&format!("### 输入串: {}\n\n", result.input.join(" ")));
        out.push_str(&format!("- 结论: {}\n", if result.accepted { "接受" } else { "拒绝" }));
        for step in &result.steps {
            out.push_str(&format!(
                "- 第 {} 步 | 栈: {:?} | 输入: {:?} | 动作: {}\n",
                step.step, step.stack, step.remaining_input, step.action
            ));
        }
        out.push('\n');
    }

    out
}

pub fn write_report(output_dir: &Path, markdown: &str) -> Result<(), String> {
    fs::create_dir_all(output_dir).map_err(|e| format!("创建输出目录失败: {e}"))?;
    fs::write(output_dir.join("analysis_report.md"), markdown)
        .map_err(|e| format!("写入报告失败: {e}"))
}

fn format_right(right: &[crate::grammar::Symbol]) -> String {
    right.iter()
        .map(|symbol| match symbol {
            crate::grammar::Symbol::Terminal(value) => value.clone(),
            crate::grammar::Symbol::NonTerminal(value) => value.clone(),
            crate::grammar::Symbol::Epsilon => "ε".to_string(),
            crate::grammar::Symbol::EndMarker => "$".to_string(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}
```

- [ ] **Step 4: 串联 `main` 完整流程并运行所有测试**

将 `src/main.rs` 替换为：

```rust
use std::path::{Path, PathBuf};

use grammar_analyse::analyzer::analyze_input;
use grammar_analyse::first_follow::{compute_first_sets, compute_follow_sets};
use grammar_analyse::grammar::AnalysisReport;
use grammar_analyse::loader::load_problem;
use grammar_analyse::parse_table::build_parse_table;
use grammar_analyse::report::{render_markdown, write_report};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let input_path = args.next().ok_or_else(|| "用法: grammar-analyse <输入文件> [输出目录]".to_string())?;
    let output_dir = args.next().map(PathBuf::from).unwrap_or_else(|| PathBuf::from("output"));

    let problem = load_problem(Path::new(&input_path))?;
    let first_sets = compute_first_sets(&problem.grammar);
    let follow_sets = compute_follow_sets(&problem.grammar, &first_sets);
    let table = build_parse_table(&problem.grammar, &first_sets, &follow_sets)
        .map_err(|conflict| format!(
            "分析表构造失败: ({}, {}) 在产生式 {} 和 {} 间冲突",
            conflict.non_terminal, conflict.terminal, conflict.existing_production, conflict.new_production
        ))?;

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
```

Run: `rtk cargo test`

Expected: PASS

- [ ] **Step 5: 提交报告与完整流程**

```bash
git add src/main.rs src/grammar.rs src/report.rs tests/end_to_end.rs
git commit -m "feat: wire cli flow and markdown report"
```

## 自检清单

### Spec 覆盖检查

- 输入文件读取：Task 2
- `FIRST/FOLLOW`：Task 3
- 预测分析表：Task 4
- 输入串分析：Task 5
- 终端摘要与 Markdown 报告：Task 6
- 单元测试与集成测试：Task 2 到 Task 6

当前无缺口。

### 占位符扫描

本计划未使用 `TBD`、`TODO`、`implement later` 等占位描述。

### 类型与接口一致性

计划中统一使用以下核心类型与函数：

- `ProblemInput`
- `Grammar`
- `ParseResult`
- `AnalysisReport`
- `load_problem`
- `compute_first_sets`
- `compute_follow_sets`
- `build_parse_table`
- `analyze_input`
- `render_markdown`

后续执行时不应再改名，以免破坏任务间衔接。
