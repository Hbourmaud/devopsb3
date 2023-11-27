FROM rust:1.74.0
RUN useradd -ms /bin/bash web-user
USER web-user
WORKDIR /home/web-user
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo 'fn main() {}' > ./src/main.rs
RUN cargo build --release
RUN rm -rf ./src
COPY ./src ./src
RUN cargo build --release
CMD cargo run