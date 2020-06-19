FROM rustlang/rust:nightly AS build
WORKDIR /usr/src/ygo_server
COPY . .
RUN cargo install --path .


FROM debian:buster-slim
RUN apt update && apt install -y libpq5
COPY --from=build /usr/local/cargo/bin/ygo_server /usr/local/bin/ygo_server

ENV ROCKET_DATABASES='{ database = { url = "postgres://postgres:postgres@database:5432/postgres" } }'
ENV ADMIN_KEY="admin"
EXPOSE 8000
CMD ["ygo_server"]
