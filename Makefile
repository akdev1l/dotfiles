GIT_TAG := $(shell git rev-parse -- HEAD)

all: lint build publish


.PHONY: lint
lint:
	echo linter

.PHONY: build
build:
	podman build -t akdev1l/devenv:latest \
		-t akdev1l/devenv:${GIT_TAG} .

.PHONY: publish
publish:
	echo podman push ghcr.io/akdev1l/base:latest
