# akdev - devenv

Personal development environment as code. Provisions the same shell, editor, and tooling experience whether you're dropping into a container or setting up a Fedora workstation.

## What's included

| Tool | Config |
|------|--------|
| **zsh** | Oh My Zsh + Powerlevel10k, syntax highlighting, autosuggestions, AWS plugin |
| **vim** | Relative line numbers, 4-space tabs, 100-char column, per-filetype overrides |
| **tmux** | Backtick prefix, red Powerline theme, SSH agent forwarding |
| **git** | GPG commit signing, AWS CodeCommit credential helper |

Dev tools installed: `gcc`, `g++`, `gdb`, `clang`, `cmake`, `make`, `git`, `gh`, `awscli2`, `podman`, `buildah`, `skopeo`, `nodejs`, `java`, `python`, and Claude Code CLI.

## Usage

### Container (quick, ephemeral)

Pull and run against your current directory:

```sh
podman run --rm -it -v $PWD:$PWD -w $PWD akdev1l/devenv:latest
```

Images are published to Docker Hub and GHCR on every push to `master`:

```sh
docker.io/akdev1l/devenv:latest
ghcr.io/akdev1l/devenv:latest
```

### Ansible (workstation provisioning)

Provisions a Fedora machine in place — installs packages, deploys dotfiles, and sets up zsh:

```sh
ansible-playbook devbox.yml
```

Requires Ansible and a Fedora/RHEL-based host. Runs against `localhost` by default.

## Development

Build and publish the container image manually:

```sh
make          # lint + build + publish
make build    # build only
make publish  # push to registries
```

Requires `podman` and credentials for Docker Hub and GHCR.

## Works great with Fedora Kinoite
