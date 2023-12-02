FROM rust:1.74.0 as builder
RUN useradd -ms /bin/bash web-user
USER web-user
WORKDIR /home/web-user
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo 'fn main() {}' > ./src/main.rs
RUN cargo build --release
RUN rm -rf ./src
COPY ./src ./src
ENV RUSTFLAGS='-C target-feature=+crt-static'
RUN cargo build --release

FROM scratch
COPY --from=builder /home/web-user/target/release/tp-wik-dps-tp01 /myapp
CMD ["./myapp"]