ARG rustVersion=1.89-bookworm
ARG phpVersion=8.2-apache


FROM rust:$rustVersion AS builder_rs
COPY ./bridge_php.rs/ /repo
WORKDIR /repo
RUN cargo build --lib --release
RUN mv ./target/release/libcore.so ./packages/php_data/root/public/core.so


FROM php:$phpVersion
COPY --from=builder_rs \
        /repo/packages/php_data/conf/ffi.ini \
        /usr/local/etc/php/conf.d/99-ffi.ini
COPY --from=builder_rs \
        /repo/packages/php_data/root \
        /var/www/html
RUN apt-get update

# 啟用 FFI
RUN apt-get install -y libffi-dev && \
    docker-php-ext-install ffi

# 啟用 RewriteEngine On
RUN a2enmod rewrite

EXPOSE 80
