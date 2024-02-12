FROM rust

RUN mkdir "Vhenn_coin"
COPY . ./

RUN cargo build

CMD ["./target/Vhenn_coin"]