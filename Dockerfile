# Create an image to run justfile with
FROM rustlang/rust:nightly-buster-slim AS just
RUN cargo install just
RUN cp $(which just) /just

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

FROM rustlang/rust:nightly-buster-slim AS server
# Setup build directory
RUN mkdir /build
RUN apt-get update && apt-get install -y libpq-dev
RUN cargo install diesel_cli --no-default-features --features postgres
WORKDIR /build
# Only build dependencies first since it is the most time-consuming part
ADD Cargo.toml Cargo.toml
ADD Cargo.lock Cargo.lock
ADD schema/Cargo.toml schema/Cargo.toml
RUN mkdir -p src schema/src
RUN echo "fn main() {}" > src/main.rs
RUN touch schema/src/lib.rs
RUN cargo build --release
# Now build the server executable
RUN rm -r src schema
ADD schema schema
ADD src src
RUN rm target/release/webcord* target/release/deps/webcord* target/release/deps/libwebcord*
RUN cargo build --release

RUN mkdir /webcord
WORKDIR /webcord
COPY --from=client /build/build /webcord/build
RUN cp /build/target/release/webcord /webcord/webcord

RUN useradd webcord
RUN chown -R webcord:webcord .
USER webcord

CMD (cd /build/schema && diesel migration run) && ./webcord
