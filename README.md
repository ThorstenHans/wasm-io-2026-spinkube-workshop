# Wasm I/O 2026: Spin Workshop

This repository contains the starting point for the workshop. Spin apps are already created and bring some boilerplate.

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
