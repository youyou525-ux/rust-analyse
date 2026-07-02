.PHONY: help test run_expr run_epsilon run_conflict report_expr report_epsilon clean_output

OUTPUT_DIR ?= export

help:
	@echo "可用命令:"
	@echo "  make test            运行全部测试"
	@echo "  make run_expr        运行表达式文法样例"
	@echo "  make run_epsilon     运行空产生式样例"
	@echo "  make run_conflict    运行冲突文法样例"
	@echo "  make report_expr     生成表达式文法报告到 OUTPUT_DIR"
	@echo "  make report_epsilon  生成空产生式样例报告到 OUTPUT_DIR"
	@echo "  make clean_output    清理 OUTPUT_DIR"
	@echo ""
	@echo "可通过 make <命令> OUTPUT_DIR=export 或 OUTPUT_DIR=/tmp/xxx 自定义报告目录"

test:
	cargo test

run_expr:
	cargo run -- tests/fixtures/expression_grammar.txt $(OUTPUT_DIR)

run_epsilon:
	cargo run -- tests/fixtures/epsilon_grammar.txt $(OUTPUT_DIR)

run_conflict:
	cargo run -- tests/fixtures/table_conflict_grammar.txt $(OUTPUT_DIR)

report_expr:
	@mkdir -p $(OUTPUT_DIR)
	cargo run -- tests/fixtures/expression_grammar.txt $(OUTPUT_DIR)
	@echo "报告位置: $(OUTPUT_DIR)/analysis_report.md"

report_epsilon:
	@mkdir -p $(OUTPUT_DIR)
	cargo run -- tests/fixtures/epsilon_grammar.txt $(OUTPUT_DIR)
	@echo "报告位置: $(OUTPUT_DIR)/analysis_report.md"

clean_output:
	rm -rf $(OUTPUT_DIR)
