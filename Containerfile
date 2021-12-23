FROM fedora:35

COPY ./util/dev-env.sh /tmp/dev-env.sh

RUN /tmp/dev-env.sh
