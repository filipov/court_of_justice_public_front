FROM node:12 as build
WORKDIR /app
COPY . .
ENV RUSTUP_HOME=/usr/local/rustup \
    CARGO_HOME=/usr/local/cargo \
    PATH=/usr/local/cargo/bin:$PATH \
    RUST_VERSION=1.41.1
RUN apt-get update
RUN apt-get install -y \
    build-essential \
    curl
# Update new packages
RUN apt-get update
# Get Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
RUN echo 'source $HOME/.cargo/env' >> $HOME/.bashrc
RUN cargo install cargo-web
RUN cargo install wasm-pack
# Build
RUN npm i
RUN npm run build

FROM nginx:1.15.2-alpine
COPY --from=build /app/dist /var/www
COPY --from=build /app/nginx/nginx.conf /etc/nginx/nginx.conf
COPY --from=build /app/nginx/mime.types /etc/nginx/mime.types
EXPOSE 80
ENTRYPOINT ["nginx","-g","daemon off;"]