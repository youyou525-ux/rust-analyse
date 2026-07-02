# LL(1) 语法分析器

一个基于 Rust 实现的 LL(1) 语法分析器，主要用于编译原理课程作业、LL(1) 分析流程演示和基础实验验证。

## 项目背景

在编译原理课程里，LL(1) 语法分析通常需要把以下几个知识点串起来：

- 文法表示
- `FIRST` 集和 `FOLLOW` 集
- 预测分析表
- 基于栈的预测分析过程

理论本身并不复杂，但如果没有一个真正可运行的程序，这些步骤往往只停留在概念层面。这个项目的作用就是把这条流程做成一个能执行、能测试、能输出结果的 Rust 程序。

## 项目作用

项目主要完成以下工作：

1. 从文件读取文法和输入串。
2. 计算 `FIRST` 集与 `FOLLOW` 集。
3. 构造 LL(1) 预测分析表。
4. 对输入串执行预测分析。
5. 输出终端摘要并生成 Markdown 报告。

项目重点不是“功能尽可能多”，而是“核心流程完整、结果清晰、容易展示”。

## 当前状态

项目已经完成本机验证：

```bash
cargo test
```

结果：

```txt
19 passed (8 suites)
```

## 项目目录

```txt
grammar-analyse/
├── src/                   核心源码
├── tests/                 测试代码与输入样例
├── env/                   使用与快速开始说明
├── docs/
│   ├── commit/            项目讲解与详细设计文档
│   └── superpowers/       设计/spec/计划过程文档
├── Cargo.toml             Rust 项目配置
└── README.md              项目总说明
```

## 项目结构

`src/` 下的主要模块如下：

- `loader.rs`
  负责解析输入文件，读取 `%start`、`%token`、产生式和输入串。

- `grammar.rs`
  负责定义核心数据结构，如 `Symbol`、`Production`、`Grammar`、`ParseStep`、`ParseResult`、`AnalysisReport`。

- `first_follow.rs`
  负责 `FIRST` 集和 `FOLLOW` 集计算。

- `parse_table.rs`
  负责 LL(1) 预测分析表构造与冲突检测。

- `analyzer.rs`
  负责栈驱动预测分析过程。

- `report.rs`
  负责 Markdown 报告生成。

- `main.rs`
  负责命令行入口和整体流程串联。

## 输入格式

项目采用固定文本输入格式，例如：

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

含义如下：

- `%start`：开始符号
- `%token`：终结符声明
- 第一段 `%%` 到第二段 `%%`：文法定义
- 第二段 `%%` 之后：待分析输入串

## 快速运行

运行测试：

```bash
cargo test
```

运行程序：

```bash
cargo run -- tests/fixtures/expression_grammar.txt export
```

示例输出：

```txt
分析表构造成功
id + id * id => 接受
( id + id ) * id => 接受
id + + id => 拒绝
报告已生成: export/analysis_report.md
```

## 相关文档

- 快速开始：[env/快速开始.md](../../env/快速开始.md:1)
- 详细设计：[项目详细设计文档.md](../../docs/commit/项目详细设计文档.md:1)
- 讲解：[视频讲解.md](../../docs/commit/讲解.md)

## 说明

当前项目定位为课程作业型 LL(1) 语法分析器，优先保证：

- 实现简单
- 逻辑完整
- 结果可展示
- 行为可验证
