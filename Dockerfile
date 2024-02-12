FROM rust

RUN mkdir "Vhenn_coin"
COPY . /Vhenn_coin

RUN cargo build

CMD ["./target/Vhenn_coin"]