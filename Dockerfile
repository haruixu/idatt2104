#Use rust:lastest as base image
FROM rust:latest 
#Name the workdir in container
WORKDIR /rust_docker
#Copy dependency files over first (not sure if this helps, but I think this prevents redownloading dependencies for new image if nothing was changed - see last paragraph)
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
#

# https://stackoverflow.com/questions/38613216/rebuild-same-docker-image-with-only-the-additional-changes-in-the-dockerfile
# FROM something
# RUN  cmd1
# RUN  cmd2
# RUN  cmd3
# RUN  cmd4
# If you change cmd1 then all layers will be rebuilt, because they could be different with respect to cmd1
# If you change cmd4 than only this command will be rebuilt, because it has not affect any other layers.
