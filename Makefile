default: all


all:
	@echo ; echo "====> TESTING parse_input" ; echo ; \
		cd parse_input && cargo test
	@set -e ; for d in $$( ls . | grep '^d[0-9][0-9]' ) ; \
	do \
		echo ; echo "====> TESTING $$d" ; echo ; \
		(cd $$d && cargo test) ; \
	done

clean:
	@find . -name Cargo.toml | xargs dirname | while read pkg ; \
	do \
		echo "====> CLEANING $$pkg" ; \
		(cd $$pkg && cargo clean) ; \
	done
