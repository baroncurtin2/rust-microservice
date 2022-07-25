FROM rustlang/rust:nightly
MAINTAINER <knowledge3@gmail.com>

WORKDIR /var/www/microservice/
COPY . .

RUN rustc --version
RUN cargo install --path .

CMD ["microservice"]