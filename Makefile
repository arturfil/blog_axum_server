DB_DOCKER_CONTAINER=blog_axum_container
DB_NAME=blog_axum_db


install:
	cargo add axum
	cargo add axum-extra -F cookie
	cargo add time
	cargo add tokio -F full
	cargo add tower-http -F "cors"
	cargo add serde_json
	cargo add serde -F derive
	cargo add chrono -F serde
	cargo add dotenv
	cargo add uuid -F "serde v4"
	cargo add sqlx -F "runtime-async-std-native-tls postgres chrono uuid"
	cargo add jsonwebtoken
	cargo add argon2
	cargo add rand_core --features "std"

create_docker_container:
	docker run --name ${DB_DOCKER_CONTAINER} -p 5432:5432 -e POSTGRES_USER=root -e POSTGRES_PASSWORD=secret -d postgres:12-alpine

create_postgres_db:
	docker exec -it ${DB_DOCKER_CONTAINER} createdb --username=root --owner=root ${DB_NAME}

start_docker_db:
	docker start ${DB_DOCKER_CONTAINER}

create_migrations:
	sqlx migrate add -r init

migrate-up:
	sqlx migrate run --database-url "postgresql://root:secret@localhost:5432/blog_axum_db?sslmode=disable"

migrate-down:
	sqlx migrate revert

stop_containers:
	@echo "Stopping all docker containers..."
	if [ $$(docker ps -q) ]; then \
		echo "found and stopped containers..."; \
		docker stop $$(docker ps -q); \
	else \
		echo "no active containers found..."; \
	fi


init_docker: stop_containers start_docker_db

run: init_docker
	cargo watch -q -c -w src/ -x run


