
FROM debian:bullseye-slim
WORKDIR /
RUN wget "https://github.com/Mruiz-99/SOPES1_Rust_P1/releases/latest/download/api"

ENV DATABASE_URL mongodb://admindb:1234@35.209.237.73:27017
ENV DATABASE_NAME fase2
ENV USER_COLLECTION_NAME fase2
ENV SERVER_URL 0.0.0.0:8080
EXPOSE 8080
# Run the web service on container startup.
CMD ["./api"]


#FROM rust:latest as build
#WORKDIR /
#COPY . .
#ENV MONGO_HOST 34.122.108.75
#ENV MONGO_USER admingrupo19
#ENV MONGO_PASS so1-fase2
#ENV MONGO_DB so-proyecto-f2
#ENV MONGO_COLLECTION logs
#RUN cargo build

#FROM debian:bullseye-slim
#WORKDIR /
#COPY --from=build ./target/debug/api ./
#EXPOSE 8080
#CMD ["./api"]*/
