#### akdev - devenv container image

This contains my containerized development environment.
It allows me to quickly deploy a container with my preferred config
that way I always feel at home :).

Works great with Fedora Kinoite.

#### Usage

Spin up a new container - use the development tools inside:

```
podman run --rm -it -v $PWD:$PWD -w $PWD akdev1l/devenv:latest
```
