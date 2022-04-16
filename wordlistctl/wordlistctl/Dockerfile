FROM rust:buster

LABEL version="1.0.0" \
    author="Author BlackArch (https://github.com/BlackArch)" \
    docker_build="docker build -t blackarch/wordlistctl:1.0 ." \
    docker_run_basic="docker run --rm blackarch/wordlistctl:1.0 -h"

WORKDIR /wordlistctl
COPY . .

RUN apt-get update -y
RUN apt-get upgrade -y
RUN apt-get install perl -y
RUN cargo install cargo-make
RUN cargo make all
RUN cd out

ENTRYPOINT ["./wordlistctl"]

CMD ["-h"]