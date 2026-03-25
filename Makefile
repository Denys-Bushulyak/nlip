.PHONY: test

test:
	@echo "Test..."
	@echo ""
	cargo test

coverage:
	@echo "Coverage..."
	@echo ""
	cargo llvm-cov test --html --open
