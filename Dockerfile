FROM rust:1.44

RUN apt-get update && apt-get install -qy linux-perf
# Looks like perf is broken inside this image.
RUN rm -rf /usr/bin/perf && ln -s /usr/bin/perf_4.19 /usr/bin/perf
RUN cargo install flamegraph

WORKDIR /usr/src/mlog
COPY . .

RUN cargo install --path .

