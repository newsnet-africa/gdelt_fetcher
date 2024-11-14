FROM rust:1.67

WORKDIR /library
COPY . .

CMD ["cargo", "build"]
