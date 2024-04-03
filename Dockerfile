FROM rust:latest as cargobuild
WORKDIR /usr/app
COPY . .
RUN rustup default nightly
RUN cargo build  --release 
FROM rust:latest
WORKDIR /app
COPY --from=cargobuild /usr/app/target/release/emailvalidator ./mailvalidator
COPY ./start.sh .
COPY ./template ./templates
RUN chmod +x start.sh
EXPOSE 8383
ENTRYPOINT ["/app/start.sh"]