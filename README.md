# Testing rust for web services

Using `rustc 1.5.0-nightly (6108e8c3e 2015-09-28)`.
The Makefile will create/start/remove a docker instance of postgres.

```
$ make create && make start
$ cargo run
```

The server lives on `localhost:5000` and the only url is `http://localhost:5000/passwords`.

## Get
Will return all the passwords in the db in JSON format.
```
$ curl http://localhost:5000/passwords 
```

## Post
Will insert a password in the db and return 201
```
$ curl -X POST -v -d '{"id": 42, "name": "hey", "encrypted": "hey"}' http://localhost:5000/passwords

```
