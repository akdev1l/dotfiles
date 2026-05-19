FROM fedora:44

RUN dnf install -y ansible sudo && dnf clean all

COPY . /tmp/dotfiles/
WORKDIR /tmp/dotfiles

RUN ansible-playbook container.yml -c local

RUN rm -rf /tmp/dotfiles /tmp/* /var/cache/* /var/log/*

CMD ["/usr/bin/zsh"]
