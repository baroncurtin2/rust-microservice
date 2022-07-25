FROM rustlang/rust:nightly
MAINTAINER <knowledge3@gmail.com>

ENV SSL_CERT_FILE=/etc/ssl/certs/ca-certificates.crt
# Install vim for in-place editing.

WORKDIR /var/www/microservice/
COPY . .

RUN rustc --version
RUN cargo install --path .

CMD ["microservice"]