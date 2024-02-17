#Use rust:lastest as base image
FROM rust:latest 
#Specify workdir in container
WORKDIR /rust_docker
#Copy dependency files over first
COPY Cargo.lock Cargo.toml .
#Copy over binary and create default module structure with main.rs as the sole binary
COPY src/bin/temp.rs ./src/main
#Default command the container runs when it is created
CMD ["cargo", "run"]

# New virtualized file structure looks like this: 
# Docker-container (root)
# |-- rust-docker
# |   |-- src
# |       |--main.rs
# |   |-- Cargo.toml
# |   |-- Cargo.lock
# |-- Dockerfile ??unsure abothisone
