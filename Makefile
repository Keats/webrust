create:
		docker run --name rust-postgresql -d \
		-e 'POSTGRES_USER=pg' \
		-e 'POSTGRES_PASSWORD=pg' \
		-e 'POSTGRES_DB=safe' \
		-p 5432:5432 \
		postgres:9.4.4

remove:
		docker rm rust-postgresql

stop:
		docker stop rust-postgresql

start:
		docker start rust-postgresql
