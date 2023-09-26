FROM rust:1.72.1-slim
WORKDIR /usr/src/id23
COPY . .
RUN cargo install --path .
CMD ["id23"]