FROM rust

RUN mkdir "Vhenn_coin"
COPY . /bin/Vhenn_coin

RUN cargo build

CMD ["./target/Vhenn_coin"]