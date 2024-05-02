FROM --platform=linux/amd64 rust:latest as build 
RUN apt-get -y update && apt-get install -y sqlite3 libsqlite3-dev

COPY . /app

WORKDIR /app
RUN cargo build --release 

FROM --platform=linux/amd64 ubuntu:latest as runtime

COPY --from=build /app/target/release/data-minded /
EXPOSE 3000
ENV RUST_LOG=debug
CMD ["./data-minded"]