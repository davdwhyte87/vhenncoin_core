FROM rust

RUN mkdir "vhenn_coin"
COPY . ./

RUN cargo build 
EXPOSE 8000
CMD ["./target/Vhenn_coin"]