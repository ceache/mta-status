FROM scratch
ADD target/x86_64-unknown-linux-musl/release/mta_status /
EXPOSE 4000
CMD ["/mta_status"]

