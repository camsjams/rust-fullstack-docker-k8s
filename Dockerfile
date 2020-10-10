FROM rust:1.47.0-slim-buster as builder

# copy files from this directory '.' to '/var/www'
ADD . /var/www

# set cwd for RUN and CMD
WORKDIR /var/www

RUN apt-get update \
	&& apt-get -y install pkg-config libssl-dev curl \
	&& curl -sL https://deb.nodesource.com/setup_14.x | bash - \
	&& apt-get install -y nodejs

RUN cargo fetch

RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

WORKDIR /var/www/app

# install all node dependencies
RUN NODE_ENV=development npm install --no-optional

# build web
RUN NODE_ENV=production npm run build

WORKDIR /var/www/

# build server
RUN cargo build --release

################
# second stage #
FROM rust:1.47.0-slim-buster

RUN mkdir /var/www
WORKDIR /var/www

# adjust permissions
RUN groupadd --gid 1000 rust \
	&& useradd --uid 1000 --gid rust --shell /bin/bash --create-home rust

RUN chown -R rust:rust /var/www

COPY --from=builder /var/www/target/release/rust_fullstack_docker_k8s .
COPY --from=builder /var/www/app/dist app/dist

USER rust

# run the daemon
ENTRYPOINT ["./rust_fullstack_docker_k8s"]

# profit || throw new Error('unable to profit')
