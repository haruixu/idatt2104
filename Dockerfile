#Use rust:lastest as base image
FROM rust:latest 
#Name the workdir in container
WORKDIR /rust_docker
#Copy dependency files over first (not sure if this helps, but I think this prevents redownloading dependencies if nothing was changed)
COPY Cargo.lock Cargo.toml .
#Copy over binary and make this main.rs
COPY src/bin/temp.rs ./src/main.rs
#On-default command: the container runs "cargo run" when it is created 
CMD ["cargo", "run"]

# New virtualized file structure looks like this: 
# Docker-container (root)
# |-- rust-docker
# |   |-- src
# |       |--main.rs
# |   |-- Cargo.toml
# |   |-- Cargo.lock
# |-- Dockerfile ??unsure abothisone
