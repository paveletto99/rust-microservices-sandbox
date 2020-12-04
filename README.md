# Rust Microservices Sandbox

## Tools used in this project

### DevContainer

> Prerequisites: install **Docker** in your host machine
>
> The following command builds a Docker Image that allows working in a consistent environment inside a container.
>
> This image will be used for two main pourposes:
>
> 1. Developing and debugging using **vscode**
> 2. Building artifacts
>
> This gives us homogeneous and consistent results.
>
> Every time the command runs it will remove the previous image and then builds the new one

```bash
$ make build-devcontainer
```

### **Musl libc** static cross toolchains

> The above Docker image suppors Rust and is built from scratch installing the following components:
>
>1. The latest version of **Rust** programming language
>2. The **Musl** libc toolchains for cross compiling Rust artifacts for **aarch64** and **x86_64** architectures
>
> The following commands will cross compile artifacts inside the container:

```bash
$ make arm64-release
```

```bash
$ make amd64-release
```

>The executables files could be found here:
>
>> **target/aarch64-unknown-linux-musl/release/<EXECUTABLE_NAME>**
>
>> **target/x86_64-unknown-linux-musl/release/<EXECUTABLE_NAME>**
>
> The executables are **optimized** and **statically linked** and can be used for building **distroless** Docker images


### kind - How to run Kubernetes in Docker for developement pourpose

> Prerequisites: download the following executables and put them where you prefer in the **PATH**
>
> 1. **kubectl**
> 2. **kind**
>
> For both see **"Install and Set Up kubectl"** and **"kind - Quick Start"** links at the end of this document
>
> With the following make target will be created a **Kubernetes Cluster** as a Docker container
>

```bash
$ make kind-create-cluster
```
>
>The cluster will be created with a separate configuration file located here:
>

```bash
$ ls ~/.kube/kind-kubernetes-clusters.kubeconfig
```
>
>With **kind** you can create as many **Kubernetes Clusters** you want also outside this project and of different type like **Multi-node clusters** (***one control-plane*** and ***many worker node***) or **Control-plane HA** (***multiple control-plane in HA*** and ***many worker node*** )
>
>You can use above configurazion file for storing in one separate place the configurations of all **Kubernetes Clusters** created and managed with **kind**
>
>
>The path to this configuration file must be exported into an environment variable to be able to interact with the **Kubernetes Clusters**

```bash
$ export KUBECONFIG=~/.kube/kind-kubernetes-clusters.kubeconfig
```
>
>Then you can use the **kubectl** command as usual to interact with your Kubernetes Cluster
>

```bash
$ kubectl get pods -A
```
>
>If you have more than one **kind Kubernetes Cluster** running on Docker in your host machine configured in the same **KUBECONFIG** configuration file, you can use the following option to specify the cluster:
>> **--context kind-<CLUSTER_NAME>**
>

```bash
$ kubectl get pods --context kind-<CLUSTER_NAME> -A
```
>
>If you prefer, when you create a **kind Kubernetes Cluster** you can specify a separate **KUBECONFIG** configuration file for all of them and then export the specific path to the configuration file of the cluster you want interact with
>

```bash
export CLUSTER_NAME=cluster-dev01
kind create cluster --kubeconfig ~/.kube/kind-kubernetes-cluster-${CLUSTER_NAME}.kubeconfig --image="kindest/node:v1.19.4@sha256:796d09e217d93bed01ecf8502633e48fd806fe42f9d02fdd468b81cd4e3bd40b" --name="${CLUSTER_NAME}"
```
>Then you can use the specific configuration file
```bash
$ export KUBECONFIG=~/.kube/kind-kubernetes-clusters-cluster-dev01.kubeconfig
```

>For removing a cluster simply run

```bash
$ kind delete cluster --name <CLUSTER_NAME>
```
>
>The main advantages of using **kind** are:
>
> 1. It's possible clean up all with simple commands
> 2. Kubernetes Clusters running on a docker container share the resources with the host machine; this differs from how **minikube** works as it'is require a VM with preallocated resources
>

## The Rust application

> For the Rust application and the setup of this project the following command was used:

```bash
$ cargo init rust-microservices
```

> For initilaizing Rust projects in Devcontainer (Ctrl + Shift + P) you must use the following commad instead

```bash
$ export USER=root && cargo init rust-microservices
```

>The application uses **Actix Web** as web framework
>
>Settings and **crates** dependencies are specified in **Cargo.toml** file
>
>For updating project packages dependencies run the following command:

```bash
$ cargo update
```

## Recomended VSCode extensions

> 1. Rust analyzer <https://rust-analyzer.github.io>

## Useful links

* [Rust programming language](https://www.rust-lang.org/ "Rust programming languages's Homepage")
* [Using Dev Container](https://code.visualstudio.com/docs/remote/create-dev-container)
* [Rust Dev Container](https://github.com/microsoft/vscode-remote-try-rust)
* [Rust Dev Container - Develop Rust based applications](https://github.com/microsoft/vscode-dev-containers/tree/v0.152.0/containers/rust "Vscode Rust dev container support repository")
* [Actix Web - A powerful, pragmatic, and extremely fast web framework for Rust](https://actix.rs)
* [Musl libc](https://musl.libc.org "Musl libc Homepage")
* [Musl libc static multi-arch pre-built toolchains](https://musl.cc "Musl libc toolchains | static cross/native toolchains Homepage")
* [Install and Set Up kubectl](https://kubernetes.io/docs/tasks/tools/install-kubectl)
* [kind - Running Kubernetes Clusters in Docker](https://kind.sigs.k8s.io)
* [kind - Quick Start](https://kind.sigs.k8s.io/docs/user/quick-start)