# README

## ENV
```shell
docker run -itd -e POSTGRES_USER=stars -e POSTGRES_PASSWORD=stars123 -p 5432:5432 --name share-stars-postgres postgres
```

###Db
* sqlx database create
* sqlx migrate add -r users
* sqlx migrate run
