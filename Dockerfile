FROM rust:1.67

COPY . .

RUN apt-get update
RUN apt-get -y install libopus-dev
RUN apt-get -y install build-essential autoconf automake libtool m4
RUN apt-get -y install ffmpeg

RUN cargo build --release

CMD ["./target/release/abc_chatter_bot"]