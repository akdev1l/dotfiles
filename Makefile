GIT_TAG := $(shell git rev-parse --short HEAD)

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
	podman tag \
		akdev1l/devenv:latest \
		ghcr.io/akdev1l/devenv:${GIT_TAG} \
		ghcr.io/akdev1l/devenv:latest
	podman push \
		ghcr.io/akdev1l/devenv:${GIT_TAG} \
		ghcr.io/akdev1l/devenv:latest
		
