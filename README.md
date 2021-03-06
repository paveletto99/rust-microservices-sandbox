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

> The above Docker image supports Rust and is built from scratch by installing the following components:
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
> The executables are **optimized** and **statically linked** and can be used for building **distroless** Docker images in order to deploy Microservices.


### kind - How to run Kubernetes in Docker for developement purpose

> Prerequisites: download the following executables and put them where you prefer in the **PATH**
>
> 1. **kubectl**
> 2. **kind**
>
> For both see **"Install and Set Up kubectl"** and **"kind - Quick Start"** links at the end of this document
>
> The following make target will create a **Kubernetes Cluster** in a Docker container
>
> A local Docker Registry will be set up too
>

```bash
$ make kind-create-cluster
```
>
>The cluster configuration will be saved in the file located here:
>

```bash
$ ls ~/.kube/kind-kubernetes-clusters-${CLUSTER_NAME}.kubeconfig
```
>
>With **kind** you can create as many **Kubernetes Clusters** you want also outside this project and of different type like **Multi-node clusters** (***one control-plane*** and ***many worker node***) or **Control-plane HA** (***multiple control-plane in HA*** and ***many worker node*** )
>
>You can use above configuration file for storing in different separate places the configurations of all **Kubernetes Clusters** created and managed with **kind**
>
>
>The path to this configuration file must be exported into an environment variable to be able to interact with the **Kubernetes Clusters**

```bash
$ export KUBECONFIG=~/.kube/kind-kubernetes-clusters-${CLUSTER_NAME}.kubeconfig
```
>
>Then you can use the **kubectl** command as usual to interact with your Kubernetes Cluster
>

```bash
$ kubectl get pods -A
```
>
>If you have instead more than one **kind Kubernetes Cluster** running on Docker in your host machine configured in the same **KUBECONFIG** configuration file, you can use the following option to specify the cluster:
>> **--context kind-<CLUSTER_NAME>**
>

```bash
$ kubectl get pods --context kind-<CLUSTER_NAME> -A
```
>
>If you prefer, when you create a **kind Kubernetes Cluster** you can specify a separate **KUBECONFIG** configuration file for all of them based on the cluster name.
>

```bash
export CLUSTER_NAME=cluster-dev01
kind create cluster --kubeconfig ~/.kube/kind-kubernetes-clusters-${CLUSTER_NAME}.kubeconfig --image="kindest/node:v1.20.2@sha256:8f7ea6e7642c0da54f04a7ee10431549c0257315b3a634f6ef2fecaaedb19bab" --name="${CLUSTER_NAME}"
```

>
>Export the path to the configuration file of the cluster you want to use
>

```bash
$ export KUBECONFIG=~/.kube/kind-kubernetes-clusters-cluster-dev01.kubeconfig
```

## Installing the Kubernetes Dashboard UI (Optional)
>
>NOTE: **Some of the following setup commands are used at Kubernetes Cluster creation time**
>
>Reported here for a detailed explaination
>
>They are already defined in the set up script used by the **make kind-create-cluster** target
>

```bash
$ kubectl apply -f https://raw.githubusercontent.com/kubernetes/dashboard/v2.1.0/aio/deploy/recommended.yaml
```

>Creating a **ServiceAccount** user with **cluster-admin** RoleBinding (see the link at the end of the document)

```yaml
$ cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: ServiceAccount
metadata:
  name: admin-user
  namespace: kubernetes-dashboard
EOF

$ cat <<EOF | kubectl apply -f -
apiVersion: rbac.authorization.k8s.io/v1
kind: ClusterRoleBinding
metadata:
  name: admin-user
roleRef:
  apiGroup: rbac.authorization.k8s.io
  kind: ClusterRole
  name: cluster-admin
subjects:
- kind: ServiceAccount
  name: admin-user
  namespace: kubernetes-dashboard
EOF
```

### Kubernetes Cluster interaction

>
>Check the **ClusterRoleBinding** exists
>

```bash
$ kubectl get clusterrolebindings -A | grep cluster-admin
```

>Getting a Bearer Token for the UI dashboard login for the **admin-user**

```bash
$ kubectl -n kubernetes-dashboard describe secret $(kubectl -n kubernetes-dashboard get secret | grep admin-user | awk '{print $1}') | grep token: | awk -F 'token:' '{print $2}' | sed 's/ //g'
```

>After exporting the environment variable **KUBECONFIG** you can also use the following useful command to get a token for accessing the UI Dashboard

```bash
$ make get-cluster-token
```

>Or get the default token

```bash
$ kubectl -n kubernetes-dashboard describe secret $(kubectl -n kubernetes-dashboard get secret | grep default-token | awk '{print $1}') | grep token: | awk -F 'token:' '{print $2}' | sed 's/ //g'
```

>Start the proxy for accessing the UI dashboard

```bash
$ kubectl proxy
```

>Open the brower pointing to the following link
>
><http://localhost:8001/api/v1/namespaces/kubernetes-dashboard/services/https:kubernetes-dashboard:/proxy/>
>
>

### Build Docker image and deploy the application

>
>Buil the image and publish to the local registry

```bash
$ make docker-build-image
```
>
>Create a **dev** namespace and check that it's correctly created
>

```bash
$ kubectl apply -f kubernetes/01-namespace.yaml
$ kubectl get namespaces -A | grep dev
```
>
>Create the **ConfigMap** for storing services environment variables and check that they are correctly created
>

```bash
$ kubectl apply -f kubernetes/02-mongo-configmap.yaml
$ kubectl apply -f kubernetes/03-postgresql-configmap.yaml
$ kubectl apply -f kubernetes/04-application-configmap.yaml
$ kubectl get configmaps -n dev
```
>
>For deploying the application run
>

```bash
$ kubectl apply -f kubernetes/05-application-deployment.yaml
```

>
>You can find useful running this command for deploying/undeploying the Kubernetes configurations of the application
>

```bash
$ make deploy-kubernetes

$ make undeploy-kubernetes
```

>
>Open the brower pointing to the following link to access the application and check if it works
>
>https://localhost/www/index.html
>

>For removing a cluster simply run

```bash
$ kind delete cluster --name <CLUSTER_NAME>
```

> Or for cleaning up also the connection of the registry container with the **kind** network

```bash
$ make kind-delete-cluster
```

## Troubleshooting NGINX Ingress Controller

### Find the deployment

```bash
$ kubectl get namespaces
$ kubectl get deployments -n ingress-nginx
$ kubectl get pods -n ingress-nginx
```

### Customization

>
>The value for **use-proxy-protocol** needs to be false if you do not use haproxy, elb or similar reverse proxies in front.
>
>https://github.com/kubernetes-sigs/kind/issues/1618
>

```bash
$ kubectl edit configmaps -n ingress-nginx ingress-nginx-controller
```

```yaml
apiVersion: v1
data:
  proxy_protocol: "off"
  use-proxy-protocol: "false"
```

### Restart the deployments

```bash
$ kubectl rollout restart -n ingress-nginx deployments/ingress-nginx-controller
```
>
> or
>

```bash
$ kubectl scale deployment -n ingress-nginx ingress-nginx-controller --replicas=0

$ kubectl scale deployment -n ingress-nginx ingress-nginx-controller --replicas=1

$ kubectl scale deployment -n dev rust-microservices-sandbox-deployment --replicas=0

$ kubectl scale deployment -n dev rust-microservices-sandbox-deployment --replicas=1
```

### Check what is going on

```bash
$ kubectl logs -f -n dev rust-microservices-sandbox-deployment-b4f99b546-74ccx rust-service
```


## Misc useful commands

>
>Removing service account
>

```bash
$ kubectl -n kubernetes-dashboard delete serviceaccount admin-user
$ kubectl -n kubernetes-dashboard delete clusterrolebinding admin-user
```

>
>Editing an Ingress configuration
>
```bash
$ kubectl describe ingress -A
$ kubectl edit ingress ingress-rust-microservices-sandbox -n dev
```
>
>Getting informations about deployments and services
>

```bash
$ kubectl get deployment -l app=RustMicroservicesSandbox -o wide -n dev
$ kubectl describe svc -n dev
$ kubectl describe deployments -A
$ kubectl describe deployments -n dev
$ kubectl get svc svc-rust-microservices-sandbox -n dev
$ kubectl describe svc svc-rust-microservices-sandbox -n dev
$ kubectl get ep svc-rust-microservices-sandbox -n dev
```

### Kind PROs

>
>The main advantages of using **kind** are:
>
> 1. It's possible clean up all with a simple command
> 2. Kubernetes Clusters running on a docker container that shares the resources with the host machine; this differs from how **minikube** works as it requires a VM with preallocated resources
>

## The Rust application

> For the Rust application and the setup of this project the following command was used:

```bash
$ cargo init RustMicroservicesSandbox
```

> For initilaizing Rust projects in Devcontainer (Ctrl + Shift + P or equivalent one) you must use the following commad instead

```bash
$ export USER=root && cargo init RustMicroservicesSandbox
```

>The application uses **Actix Web** as web framework
>
>Settings and **crates** dependencies are specified in **Cargo.toml** file
>
>For updating project packages dependencies run the following command:

```bash
$ cargo update
```

>The web application structure aims to apply the ***Separation of Concerns*** (SoC) principle of Software Design.
>
>From this principle derive the **S**ingle Responsibility and the **I**nterface Segregation principles of the **S.O.L.I.D.** Design.
>
>To accomplish that the web application adheres to the concept of ***"separating code by features not functionalities"***
>
>Simply put, ***"package by feature, not by layers"***, **DO NOT** group togheter routing handlers, service or data access layers:

```bash
$ tree --dirsfirst -L 2 src/api/services/
src/api/services/
????????? customers
???   ????????? Controller.rs
???   ????????? Model.rs
???   ????????? mod.rs
???   ????????? Repository.rs
???   ????????? Resource.rs
???   ????????? Service.rs
????????? invoices
???   ????????? Controller.rs
???   ????????? Model.rs
???   ????????? mod.rs
???   ????????? Repository.rs
???   ????????? Resource.rs
???   ????????? Service.rs
????????? orders
???   ????????? Controller.rs
???   ????????? Model.rs
???   ????????? mod.rs
???   ????????? Repository.rs
???   ????????? Resource.rs
???   ????????? Service.rs
????????? users
???   ????????? Controller.rs
???   ????????? Model.rs
???   ????????? mod.rs
???   ????????? Repository.rs
???   ????????? Resource.rs
???   ????????? Service.rs
????????? mod.rs
```

>This will allow application modularity by loading or unloading features modules and mounting or umounting features routes in the **main.rs**

## IDEs

### IntelliJ IDEA Rust plugin

> IntelliJ Rust <https://www.jetbrains.com/rust/>

### Recomended VSCode extensions

> 1. Rust analyzer <https://rust-analyzer.github.io>
> 2. Rust support for Visual Studio Code <https://github.com/rust-lang/vscode-rust>

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
* [Kubernetes - Creating sample admin ServiceAccount user with cluster-admin RoleBinding](https://github.com/kubernetes/dashboard/blob/master/docs/user/access-control/creating-sample-user.md)

## Must-read

### Articles

* [ExpressJS vs Actix-Web. It is exactly what you think](https://medium.com/@maxsparr0w/performance-of-node-js-compared-to-actix-web-37f20810fb1a)
  
  >Actix-web and Rust ecosystem are a good fit for developing efficient web services, requiring **~6 times less CPU power and less memory** it would allow significant **75%-95% runtime cost saving** with just basic not optimized setup.

### Blogs

* [Luca Palmieri's Blog - A learning journal](https://www.lpalmieri.com)
  * [Book - **"Zero To Production In Rust"**](https://www.lpalmieri.com/posts/2020-05-24-zero-to-production-0-foreword)
  * [Using Types To Guarantee Domain Invariants - Validation - Type-Driven Development - Error As Values](https://www.lpalmieri.com/posts/2020-12-11-zero-to-production-6-domain-modelling/)
  * [How To Bootstrap A Rust Web API From Scratch - Our First Integration Test](https://www.lpalmieri.com/posts/2020-08-09-zero-to-production-3-how-to-bootstrap-a-new-rust-web-api-from-scratch/)
