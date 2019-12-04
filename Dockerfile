# Base file
FROM rustlang/rust:nightly

COPY . .

# Common tasks
#RUN apt-get update
#RUN apt-get upgrade -y

# Before build
#RUN dpkg --configure -a
#RUN apt install build-essential -y
#RUN apt install libssl-dev -y
#RUN apt install curl -y
#RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -y | sh
#RUN source $HOME/.cargo/env
#RUN rustup override set nightly
RUN cargo build --bin website --release

#CMD ./target/release/website

ENTRYPOINT ["./target/release/website"]
EXPOSE 8000/tcp
