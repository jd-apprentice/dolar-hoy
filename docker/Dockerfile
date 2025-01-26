FROM rust:slim-bookworm AS build

RUN USER=root cargo new --bin dolar_hoy
WORKDIR /tmp/app

RUN apt-get update && apt-get install -y \
    build-essential \
    ## Add additional dependencies here
    && rm -rf /var/lib/apt/lists/*

COPY . .
RUN cargo build --release --bin dolar_hoy

FROM rust:slim-bookworm AS runtime

WORKDIR /app
COPY --from=build /tmp/app/target/release/dolar_hoy /app/dolar_hoy

RUN groupadd user \
&& useradd -m -g user user \
&& chown -R user:user /app

USER user
ENTRYPOINT ["/app/dolar_hoy"]