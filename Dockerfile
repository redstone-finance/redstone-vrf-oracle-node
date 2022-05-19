FROM rust:1.60

RUN mkdir /app
WORKDIR /app

# TODO: improve this Dockerfile later
# We can only build once and run web server in slim environment
# Example here: https://blog.logrocket.com/packaging-a-rust-web-service-using-docker/

COPY . .

RUN cargo build --release

# Configure env variables
ENV PRIVATE_KEY=

EXPOSE 8080

# Running the app
CMD "./target/release/redstone-vrf-oracle-node"
