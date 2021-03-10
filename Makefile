export PG_HOST=127.0.0.1
export PG_PORT=5432
export PG_USER=postgres
export PG_PASS=
export PG_DBNAME=postgres
export PG_DSN=postgresql://${PG_USER}:${PG_PASS}@${PG_HOST}:${PG_PORT}/${PG_DBNAME}?connect_timeout=10
export MONGODB_URI=mongodb://localhost:27017
export MONGODB_DBNAME="rustmicroservices"

KIND_CREATE_CLUSTER_SCRIPT=$(CURDIR)/kubernetes/kind/kind-create-cluster-with-registry
KIND_NETWORK_NAME=kind
KIND_CLUSTER_NAME=cluster-dev01
KIND_KUBECONFIG_DIR=${HOME}/.kube
KIND_KUBECONFIG_FILE=${KIND_KUBECONFIG_DIR}/kind-kubernetes-clusters-${KIND_CLUSTER_NAME}.kubeconfig
KIND_NODE_IMAGE=kindest/node:v1.20.2@sha256:8f7ea6e7642c0da54f04a7ee10431549c0257315b3a634f6ef2fecaaedb19bab
KIND_REGISTRY_NAME=kind-registry
KIND_REGISTRY_PORT=5000
KIND_KUBERNETES_ADMIN_USER=admin-user

DOCKER_CONTAINER_NAME = rust-container
DOCKER_IMAGE_NAME = rust-container:latest
DEV_CONTAINER_IMAGE_NAME = rust-devcontainer:latest
CURRENT_IMAGE_ID = $(shell docker images -q ${DEV_CONTAINER_IMAGE_NAME})
DELETE_IMAGE_CMD =

ifneq ($(strip ${CURRENT_IMAGE_ID}),)
DELETE_IMAGE_CMD = docker rmi ${CURRENT_IMAGE_ID}
endif

BUILD_IN_CONTAINER_CMD = docker run --rm --user "$$(id -u)":"$$(id -g)" -v ${PWD}:/usr/src/build -w /usr/src/build $(DEV_CONTAINER_IMAGE_NAME) bash -c 'CC=$(CC) cargo build --release --target ${TARGET}'

export KUBECONFIG=${KIND_KUBECONFIG_FILE}

.PHONY: build-devcontainer kind-create-cluster get-cluster-token deploy-kubernetes undeploy-kubernetes
.SILENT: kind-create-cluster get-cluster-token deploy-kubernetes undeploy-kubernetes
.ONESHELL: kind-create-cluster deploy-kubernetes undeploy-kubernetes

build-devcontainer:
	$(DELETE_IMAGE_CMD)
	docker build --pull -t $(DEV_CONTAINER_IMAGE_NAME) -f ./docker/devcontainer/Dockerfile ./docker/devcontainer

arm64-release: TARGET=aarch64-unknown-linux-musl
arm64-release: CC="/opt/musl-toolchains/aarch64-linux-musl-cross/bin/aarch64-linux-musl-gcc"
arm64-release:
	$(BUILD_IN_CONTAINER_CMD)

amd64-release: TARGET=x86_64-unknown-linux-musl
amd64-release: CC="/opt/musl-toolchains/x86_64-linux-musl-cross/bin/x86_64-linux-musl-gcc"
amd64-release:
	$(BUILD_IN_CONTAINER_CMD)

kind-create-cluster:
	(
		( kind get clusters | grep -q ${KIND_CLUSTER_NAME} ) && ( echo "Cluster ${KIND_CLUSTER_NAME} already exists" && exit 0; )
	) || (

		echo "Creating Kubernetes cluster...."

		if [ -f "${KIND_CREATE_CLUSTER_SCRIPT}" ]; then
			$(KIND_CREATE_CLUSTER_SCRIPT) ${KIND_NETWORK_NAME} ${KIND_CLUSTER_NAME} ${KIND_KUBECONFIG_DIR} ${KIND_KUBECONFIG_FILE} ${KIND_NODE_IMAGE} ${KIND_REGISTRY_NAME} ${KIND_REGISTRY_PORT} ${KIND_KUBERNETES_ADMIN_USER}
		else
			( echo "${KIND_CREATE_CLUSTER_SCRIPT} not Found!" && exit 1; )
		fi
	)

kind-delete-cluster:
	docker network disconnect "${KIND_NETWORK_NAME}" "${KIND_REGISTRY_NAME}"
	kind delete cluster --name ${KIND_CLUSTER_NAME}

run:
	cargo run

start:
	target/x86_64-unknown-linux-musl/release/RustMicroservicesSandbox

docker-build-image:
	docker rmi localhost:5000/${DOCKER_IMAGE_NAME}
	docker rmi ${DOCKER_IMAGE_NAME}
	docker build --pull -t ${DOCKER_IMAGE_NAME} -f ./docker/images/Dockerfile .
	docker tag ${DOCKER_IMAGE_NAME} localhost:5000/${DOCKER_IMAGE_NAME}
	docker push localhost:5000/${DOCKER_IMAGE_NAME}

docker-start:
	docker run --rm -d --net=host --name ${DOCKER_CONTAINER_NAME} -e PG_HOST -e PG_PORT -e PG_USER -e PG_PASS -e PG_DBNAME -p 9000:9000 ${DOCKER_IMAGE_NAME}

docker-stop:
	docker stop ${DOCKER_CONTAINER_NAME}

get-cluster-token:
	kubectl -n kubernetes-dashboard describe secret $$(kubectl -n kubernetes-dashboard get secret | grep admin-user | awk '{print $$1}') | grep token: | awk -F 'token:' '{print $$2}' | sed 's/ //g'

deploy-kubernetes:
	kubectl apply -f kubernetes/01-namespace.yaml
	kubectl apply -f kubernetes/02-mongo-configmap.yaml
	kubectl apply -f kubernetes/03-postgresql-configmap.yaml
	kubectl apply -f kubernetes/04-application-configmap.yaml
	kubectl apply -f kubernetes/05-application-deployment.yaml

undeploy-kubernetes:
	kubectl delete -f kubernetes/05-application-deployment.yaml
	kubectl delete -f kubernetes/04-application-configmap.yaml
	kubectl delete -f kubernetes/03-postgresql-configmap.yaml
	kubectl delete -f kubernetes/02-mongo-configmap.yaml
	kubectl delete -f kubernetes/01-namespace.yaml

test:
	cargo test --color=always -- --nocapture
