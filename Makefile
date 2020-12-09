export PG_HOST=127.0.0.1
export PG_PORT=5433
export PG_USER=postgres
export PG_PASS=
export PG_DBNAME=postgres
export PG_DSN=postgresql://${PG_USER}:${PG_PASS}@${PG_HOST}:${PG_PORT}/${PG_DBNAME}?connect_timeout=10

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