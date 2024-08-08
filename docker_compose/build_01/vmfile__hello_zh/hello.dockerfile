
ARG version=latest

FROM alpine:$version

ARG name=

ENV NAME=$name
COPY ./helloworld.sh /helloworld

CMD ["/helloworld"]

