FROM alpine:3.11

WORKDIR /root/

RUN mkdir tekster
COPY ./syfotekster-rs .

EXPOSE 8080
CMD ["./syfotekster-rs"]
