# Create an image to run justfile with
FROM rustlang/rust:nightly-buster-slim AS just
RUN cargo install just
RUN cp $(which just) /just

FROM rustlang/rust:nightly-buster-slim AS server
# Setup build directory
RUN mkdir /build
WORKDIR /build
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
# Only build dependencies first since it is the most time-consuming part
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs
RUN cargo build --release
# Now build the server executable
RUN rm -r src
ADD src src
RUN touch src/main.rs
RUN cargo build --release

FROM node:10.17-buster-slim as client
# Setup build directory
RUN mkdir /build
WORKDIR /build
COPY --from=just /just /just
ADD justfile justfile
ADD client client
WORKDIR /build/client
# Install dependencies
RUN npm install
# Build JavaScript aggregate
RUN /just js
# Build CSS aggregate
RUN /just css

FROM debian:buster-slim
RUN mkdir /webcord
WORKDIR /webcord
COPY --from=client /build/build /webcord/build
COPY --from=server /build/target/release/webcord /webcord/webcord
ADD templates templates

CMD ./webcord
