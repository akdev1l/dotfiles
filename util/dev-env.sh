#!/bin/bash

set -eux

dnf install -y \
    podman buildah skopeo \
    ansible vim-enhanced \
    tmux fira-code-fonts openssh-clients \
    make 'gcc-c++' 'gcc' clang cmake \
    zsh git java-latest-openjdk \
    nodejs /usr/bin/chsh

git clone https://github.com/jimeh/tmux-themepack.git /root/.config/tmux-themepack

chsh -s /usr/bin/zsh

rm -rf /tmp/* /var/cache/* /var/log/*
