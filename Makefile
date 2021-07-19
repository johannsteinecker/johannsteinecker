.PHONY: johannsteinecker-github-readme
johannsteinecker-github-readme:
	@cargo build --release --quiet
	@target/release/johannsteinecker.exe
