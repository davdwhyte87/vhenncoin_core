FROM rust

RUN mkdir "vhenn_coin"
COPY . ./

RUN cargo build 
EXPOSE 8000
RUN ["./target/Vhenn_coin"]