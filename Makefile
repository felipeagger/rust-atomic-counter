add:
	@if [ "$(v)" == "" ]; then \
			echo "You need to specify the package name. Ex: make add pkg=time"; \
			exit 1; \
		fi
	@cargo add ${v}

build:
	@echo "---- Building ----"
	@cargo build

run:
	@echo "---- Running ----"
	@cargo run