ARG version=1.85-bookworm
ARG debianVersion=12


FROM rust:$version As builder
ARG features=meminfo-fn
COPY ./resource_management.rs /repo
WORKDIR /repo
RUN cargo build --features "${features}" --release


FROM debian:$debianVersion
COPY --from=builder /repo/target/release/try_lib  /app
CMD ["/app"]
