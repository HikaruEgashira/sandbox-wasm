SHELL := /usr/bin/env bash

all:

code:
	DIR="$$(fd cargo.toml --exec echo {//} | fzf)"; \
	code -r "$$DIR"