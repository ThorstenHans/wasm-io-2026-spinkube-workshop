# Wasm I/O 2026: SpinKube Workshop

This repository contains the starting point for the SpinKube workshop at [Wasm I/O 2026](https://wasm.io). Spin apps are already created and bring some boilerplate.

During the workshop we'll deploy SpinKube and necessary dependencies to a pristine Kubernetes cluster and implement both Spin applications.

## Prerequisites

To code along, you must have the following tools / languages installed on your machine:

- The `spin` CLI (including the `kube` plugin)
- Rust with the `wasm32-wasip1` target
- The `kubectl` and `helm` CLIs

Additionally, you must have a Kubernetes cluster. This could either be a fully managed cluster (such as LKE, AKS, or GKE) or a local Kubernetes distribution such as `k3d` or `kind`.

## Provisioning a Kubernetes Cluster

Instructions shown here use `k3d` to run a simple Kubernetes cluster for development purposes on your local machine leveraging Docker containers.

_If you wanna use `k3d`, you must have the `k3d` CLI and Docker installed on your machine._

To create a new Kubernetes cluster with two nodes, you can use the following command:

```bash
k3d cluster create spinkube-cluster --agents 2
```

Once the cluster has been created, it should automatically be set as current context for `kubectl`, verify this using:

```bash
kubectl config get-contexts
```


## Deploying the Database

```bash
helm repo add bitnami https://charts.bitnami.com/bitnami
helm repo update
helm install wasm-db bitnami/postgresql -f pg.values.yaml

export DB_SVC=$(kubectl get svc -l app.kubernetes.io/name=postgresql,app.kubernetes.io/instance=wasm-db -o jsonpath='{.items[0].metadata.name}')
export DB_NS=$(kubectl get svc $DB_SVC -o jsonpath='{.metadata.namespace}')
export DB_USER="wasmio26"
export DB_PASS="wasmio26"
export DB_NAME="wasmio_db"
export DB_PORT="5432"
```

## Creating the DB Credentials Secret

```bash
kubectl create secret generic db-credentials \
  --from-literal=username=$DB_USER \
  --from-literal=password=$DB_PASS \
  --from-literal=host="$DB_SVC.$DB_NS.svc.cluster.local" \
  --from-literal=port=$DB_PORT \
  --from-literal=database=$DB_NAME
```
