ARG version=1.85-bookworm
ARG dotnetVersion=9.0


FROM rust:$version AS builder_rs
ARG project=my_app.cs
COPY ./link_lib.rs /repo
WORKDIR /repo
RUN cargo build --lib --release
    # cargo run --features headers --release \
    #     --bin generate_headers -- --lang cs --file ./packages/$project/src/LinkLib.g.cs
RUN mkdir ./packages/$project/lib && \
    mv ./target/release/libffi_test.so ./packages/$project/lib/FfiTest.so && \
    rm -rf ./target


# dotnet 編譯
FROM mcr.microsoft.com/dotnet/sdk:$dotnetVersion AS builder_cs
ARG project=my_app.cs
COPY --from=builder_rs /repo/packages/$project /repo
WORKDIR /repo
RUN dotnet build -c Release -o ./out && \
    mv ./out/my_app ./out/myapp


# dotnet 運行
FROM mcr.microsoft.com/dotnet/runtime:$dotnetVersion
COPY --from=builder_cs /repo/out /app
CMD ["/app/myapp"]
