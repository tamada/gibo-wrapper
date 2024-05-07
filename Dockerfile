FROM rust:slim-buster as build

# Fast + Small Docker Image Builds for Rust Apps
# https://shaneutt.com/blog/rust-fast-small-docker-image-builds/
WORKDIR /opt/gibo-wrapper

COPY Cargo.toml Cargo.toml
RUN mkdir src/
RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/gibo-wrapper*

COPY . .
RUN cargo build --release

FROM golang:1.22.2-bullseye as gibo

WORKDIR /opt/gibo-wrapper
ENV HOME=/opt/gibo-wrapper
ENV XDG_CACHE_HOME=/opt/cache

RUN git clone https://github.com/simonwhitaker/gibo.git
RUN  cd gibo \
  && go build -o gibo \
  && mkdir -p ${XDG_CACHE_HOME}/gibo/gibo-boilerplates \
  && ./gibo update

FROM gcr.io/distroless/cc-debian11

ENV PATH="/opt/gibo-wrapper:${PATH}"
ENV XDG_CACHE_HOME=/opt/cache

COPY --chown=nonroot:nonroot --from=gibo  /opt/gibo-wrapper/gibo/gibo /opt/gibo-wrapper/gibo
COPY --chown=nonroot:nonroot --from=gibo  /opt/cache/gibo/gibo-boilerplates /opt/cache/gibo/gibo-boilerplates
COPY --chown=nonroot:nonroot --from=build /opt/gibo-wrapper/target/release/gibo-wrapper /opt/gibo-wrapper/gibo-wrapper

USER nonroot
WORKDIR /app

ENTRYPOINT ["/opt/gibo-wrapper/gibo-wrapper"]
