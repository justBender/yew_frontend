# ---------- build stage ---------- #
FROM rust:1.99 as builder

WORKDIR /app

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

COPY . .

RUN trunk build --release

# ---- runtime ----

FROM nginx:alpine
RUN rm -rf /usr/share/nginx/html/*
COPY --from=builder nginx.conf /etc/nginx/conf.d/default.conf
COPY --from=builder /app/dist/ /usr/share/nginx/html/

EXPOSE 80
