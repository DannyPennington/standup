FROM rust:1.51

LABEL maintainer="dannypen@gmail.com"

RUN cargo install --path .

COPY bin/ /usr/local/bin/

CMD ["bash", "-c", "/usr/local/bin/standup"]
