FROM rust:latest as builder

WORKDIR /usr/src/arewegrillin
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim

ENV ROCKET_TEMPLATE_DIR=/usr/local/share/templates

EXPOSE 8000

COPY --from=builder /usr/local/cargo/bin/arewegrillin /usr/local/bin/arewegrillin
COPY --from=builder /usr/src/arewegrillin/templates $ROCKET_TEMPLATE_DIR

CMD ["arewegrillin"]
