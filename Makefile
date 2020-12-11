export PG_HOST=127.0.0.1
export PG_PORT=5432
export PG_USER=postgres
export PG_PASS=Pass2020!
export PG_DBNAME=postgres
export PG_DSN=postgresql://${PG_USER}:${PG_PASS}@${PG_HOST}:${PG_PORT}/${PG_DBNAME}?connect_timeout=10
# export PG_HOST=127.0.0.1
# export PG_PORT=5433
# export PG_USER=postgres
# export PG_PASS=
# export PG_DBNAME=postgres
# export PG_DSN=postgresql://${PG_USER}:${PG_PASS}@${PG_HOST}:${PG_PORT}/${PG_DBNAME}?connect_timeout=10
# dev on docker vars
# export PG_HOST_DEV_DK=172.17.0.2
# export PG_PORT_DEV_DK=5432
# export PG_USER_DEV_DK=postgres
# export PG_PASS_DEV_DK=Pass2020!
# export PG_DBNAME_DEV_DK=postgres
# export PG_DSN_DEV_DK=postgresql://${PG_USER_DEV_DK}:${PG_PASS_DEV_DK}@${PG_HOST_DEV_DK}:${PG_PORT_DEV_DK}/${PG_DBNAME_DEV_DK}?connect_timeout=10
# export PGADMIN_DEFAULT_EMAILE_DEV_DK=user@domain.local
# export PGADMIN_DEFAULT_PASSWORDE_DEV_DK=SuperSecret

DEV_CONTAINER_IMAGE_NAME = rust-devcontainer:latest
DOCKER_IMAGE_NAME = rust-container:latest
CURRENT_IMAGE_ID = $(shell docker images -q ${DEV_CONTAINER_IMAGE_NAME})
DELETE_IMAGE =

ifneq ($(strip ${CURRENT_IMAGE_ID}),)
DELETE_IMAGE_CMD = docker rmi ${CURRENT_IMAGE_ID}
endif

#BUILD_IN_CONTAINER_CMD = docker run --rm --user "$(id -u)":"$(id -g)" -v ${PWD}:/usr/src/build -w /usr/src/build $(DEV_CONTAINER_IMAGE_NAME) bash -c 'CC=$(CC) cargo build --release --target ${TARGET}'
BUILD_IN_CONTAINER_CMD = docker run --rm -v ${PWD}:/usr/src/build -w /usr/src/build $(DEV_CONTAINER_IMAGE_NAME) bash -c 'CC=$(CC) cargo build --release --target ${TARGET}'

.PHONY: build-devcontainer

build-devcontainer:
	$(DELETE_IMAGE_CMD)
	docker build -t $(DEV_CONTAINER_IMAGE_NAME) -f ./docker/devcontainer/Dockerfile ./docker/devcontainer

arm64-release: TARGET=aarch64-unknown-linux-musl
arm64-release: CC="/opt/musl-toolchains/aarch64-linux-musl-cross/bin/aarch64-linux-musl-gcc"
arm64-release:
	$(BUILD_IN_CONTAINER_CMD)

amd64-release: TARGET=x86_64-unknown-linux-musl
amd64-release: CC="/opt/musl-toolchains/x86_64-linux-musl-cross/bin/x86_64-linux-musl-gcc"
amd64-release:
	$(BUILD_IN_CONTAINER_CMD)

kind-create-cluster:
	@echo "Building k8s cluster"
	@mkdir -p ~/.kube
	kind create cluster --kubeconfig ~/.kube/kind-kubernetes-clusters.kubeconfig --image="kindest/node:v1.20.0@sha256:b40ecf8bcb188f6a0d0f5d406089c48588b75edc112c6f635d26be5de1c89040" --name="cluster-dev01"

run:
	cargo run

start:
	target/x86_64-unknown-linux-musl/release/RustMicroservicesSandbox

docker-images:
	docker build -t $(DEV_CONTAINER_IMAGE_NAME) -f ./docker/images/Dockerfile .

docker-start:
	docker run --rm -d -e PG_HOST -e PG_PORT -e PG_USER -e PG_PASS -e PG_DBNAME -p 9000:9000 rust-devcontainer:latest


# PG dev on Docker
# docker run -d \
# 	--name dev-postgres \
# 	-e POSTGRES_PASSWORD=Pass2020! \
# 	-v ${HOME}/postgres-data/:/var/lib/postgresql/data \
#         -p 5432:5432 \
#         postgres



#  docker exec -it dev-postgres bash
# psql -h localhost -U postgres


# docker run \
#     -p 80:80 \
#     -e 'PGADMIN_DEFAULT_EMAIL=user@domain.local' \
#     -e 'PGADMIN_DEFAULT_PASSWORD=SuperSecret' \
#     --name dev-pgadmin \
#     -d dpage/pgadmin4


# docker inspect dev-postgres -f "{{json .NetworkSettings.Networks }}"


# curl -d '{"username":"pobo","password":"secret1!","email":"pobo@rust.com"}' -H "Content-Type: application/json" -X POST http://localhost:9000/api/v1/users/