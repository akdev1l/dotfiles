FROM fedora:36

COPY ./util/dev-env.sh /tmp/dev-env.sh
RUN /tmp/dev-env.sh 

COPY ./util/install-oh-my-zsh.sh /tmp/install-oh-my-zsh.sh
RUN /tmp/install-oh-my-zsh.sh

COPY ./files/vim/vimrc /root/.vimrc
COPY ./files/tmux/tmux.conf /root/.tmux.conf
COPY ./files/zsh/zshrc /root/.zshrc
COPY ./files/git/gitconfig /root/.gitconfig
