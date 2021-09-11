FROM rust:1.55 as builder

LABEL maintainer="dannypen@gmail.com"

WORKDIR /usr/src/standup

COPY . .

RUN cargo install --path .

FROM debian:buster-slim

COPY --from=builder /usr/local/cargo/bin/standup /usr/local/bin/standup

COPY ./assets /usr/local/bin/standup/assets

WORKDIR /usr/local/bin/standup

EXPOSE 4200

CMD ["bash", "-c", "/usr/local/bin/standup"]
