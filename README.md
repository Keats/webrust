# Testing rust for web services

```
$ make create && make start
$ cargo run
```

The server lives on localhost:5000 and the only url is http://localhost:5000/passwords.

## Get
Will return all the passwords in the db in JSON format.

## Post
Will insert a password in the db and return 201
