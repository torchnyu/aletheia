# From parent Rust dockerfile
FROM rustlang/rust:nightly

WORKDIR /usr/src/aletheia
COPY . .

# RUN rustup toolchain install nightly-2019-02-12
# RUN rustup override set nightly-2019-03-12
RUN cargo install --path .

CMD ["aletheia"]
