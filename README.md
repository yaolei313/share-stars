# README

### postgres

```shell
docker run -itd -e POSTGRES_USER=stars -e POSTGRES_PASSWORD=stars123 -p 5432:5432 --name share-stars-postgres postgres
```

## rsa key

```shell
openssl genrsa -out private_key1.pem 2048
openssl rsa -in private_key1.pem -pubout -out public_key1.pem
openssl genrsa -out private_key2.pem 2048
openssl rsa -in private_key2.pem -pubout -out public_key2.pem
```

## sqlx

* sqlx database create
* sqlx migrate add -r users
* sqlx migrate run
