#   ██╗    ██╗ ██████╗ ██████╗ ██████╗ ██╗     ███████╗    ██╗  ██╗    ██████╗  ██████╗  ██████╗██╗  ██╗███████╗██████╗
#   ██║    ██║██╔═══██╗██╔══██╗██╔══██╗██║     ██╔════╝    ╚██╗██╔╝    ██╔══██╗██╔═══██╗██╔════╝██║ ██╔╝██╔════╝██╔══██╗
#   ██║ █╗ ██║██║   ██║██████╔╝██║  ██║██║     █████╗       ╚███╔╝     ██║  ██║██║   ██║██║     █████╔╝ █████╗  ██████╔╝
#   ██║███╗██║██║   ██║██╔══██╗██║  ██║██║     ██╔══╝       ██╔██╗     ██║  ██║██║   ██║██║     ██╔═██╗ ██╔══╝  ██╔══██╗
#   ╚███╔███╔╝╚██████╔╝██║  ██║██████╔╝███████╗███████╗    ██╔╝ ██╗    ██████╔╝╚██████╔╝╚██████╗██║  ██╗███████╗██║  ██║
#    ╚══╝╚══╝  ╚═════╝ ╚═╝  ╚═╝╚═════╝ ╚══════╝╚══════╝    ╚═╝  ╚═╝    ╚═════╝  ╚═════╝  ╚═════╝╚═╝  ╚═╝╚══════╝╚═╝  ╚═╝


# Build the wordle game binary
# ----------------------------

FROM rust:slim-bullseye as build

COPY . /wordle-rs
WORKDIR /wordle-rs
RUN rustup target add x86_64-unknown-linux-musl
RUN cargo build --target=x86_64-unknown-linux-musl --no-default-features --release
RUN strip /wordle-rs/target/x86_64-unknown-linux-musl/release/wordle-rs


# Run the game in a lightweight and fresh container
# -------------------------------------------------

FROM scratch

COPY --from=build /wordle-rs/target/x86_64-unknown-linux-musl/release/wordle-rs /wordle-rs
ENTRYPOINT ["/wordle-rs"]

LABEL maintainer="Tanguy Segarra <tanguy.segarra@epita.fr>"
