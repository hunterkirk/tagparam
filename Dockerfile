FROM debian:bullseye-20220622-slim as base
RUN apt-get update -y -qq && \
apt-get install -y --no-install-recommends -qq wget=1.21-1+deb11u1 ca-certificates=20210119 && \
wget -q https://github.com/hunterkirk/tagparam/releases/download/v0.3.7/tagparam_v0.3.7_x86_64-unknown-linux-musl.tar.gz -O /tmp/tagparam.tar.gz
WORKDIR /tmp
RUN tar -xzvf /tmp/tagparam.tar.gz

FROM scratch as release
COPY --from=base /tmp/tagparam /bin/tagparam
ENTRYPOINT [ "/bin/tagparam" ]
