.DEFAULT_GOAL := help
.PHONY : all

help:
		@echo "Please use 'make <target>' where <target> is one of"
		@echo ""
		@echo "  lint        - rust project run the linter"
		@echo "  format      - rust project format all files"
		@echo "  test        - rust project run all tests"
		@echo ""
		@echo "Check the Makefile to know exactly what each target is doing."

lint:
	cd rust && cargo clippy

format:
	cd rust && cargo fmt

test:
	cd rust && cargo test
