FROM rust

RUN mkdir "speedforce_server"
COPY . /bin/speedforce_server

RUN cargo build

CMD ["./target/hdos_api"]