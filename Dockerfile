FROM rust

RUN mkdir "Vhenn_coin"
COPY . ./

RUN cargo build --release
EXPOSE 8000
CMD ["./target/Vhenn_coin"]