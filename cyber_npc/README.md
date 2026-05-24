# Applied Project Cluster

Cyber NPC infrastructure for VMware vSphere automation with Matrix integration and autonomous AI agents.

## Overview

Cyber NPC generates realistic benign user activity in cyber security training environments using LLM-powered agents that interact with virtual machines through out-of-band hypervisor interfaces. The system captures VM screens, analyzes them with vision models, and sends keyboard input to simulate human users.

## Prerequisites

- Kubernetes cluster (v1.28+)
- Helm (v3.12+)
- kubectl configured for your cluster
- VMware vSphere environment (7.0+)
- Container registry access (Docker Hub or private)

## Quick Start

### 1. Clone the Repository

```bash
git clone https://github.com/hattmo/applied_project_cluster.git
cd applied_project_cluster
```

### 2. Create the Namespace

```bash
kubectl create namespace npc
```

### 3. Generate SSH Keys for Ollama

```bash
mkdir -p ./ssh-keys
ssh-keygen -t ed25519 -f ./ssh-keys/id_ed25519 -N "" -C "ollama@npc"
```

### 4. Create the Credentials Secret

The `creds` secret contains all sensitive configuration:

```bash
kubectl create secret generic creds -n npc \
  --from-literal=ollama-api-key="ollama-api-key" \
  --from-literal=matrix-password="your-matrix-password" \
  --from-literal=vcenter-hostname="vcenter.example.com" \
  --from-literal=vcenter-username="openclaw@vsphere.local" \
  --from-literal=vcenter-password="your-vcenter-password" \
  --from-literal=vcenter-port="443" \
  --from-literal=vcenter-white-list="^training-.*$" \
  --from-file=ssh-keys=./ssh-keys/
```

**Secret Keys Explained:**

| Key | Description | Example |
|-----|-------------|---------|
| `ollama-api-key` | Ollama API key | `""` |
| `matrix-password` | Password for Matrix agent accounts | `SecurePass123!` |
| `vcenter-hostname` | vSphere FQDN or IP | `vcenter.lab.local` |
| `vcenter-username` | vSphere service account | `openclaw@vsphere.local` |
| `vcenter-password` | vSphere account password | `VCenterPass!` |
| `vcenter-port` | vSphere API port | `443` |
| `vcenter-white-list` | Regex for allowed VM names | `^training-.*$` |
| `ssh-keys/id_ed25519` | SSH private key | (from step 3) |
| `ssh-keys/id_ed25519.pub` | SSH public key | (from step 3) |

### 5. Deploy with Helm

```bash
helm install npc ./chart \
  --namespace npc \
  --create-namespace \
  --wait \
  --timeout 10m
```

## Repository Structure

```
applied_project_cluster/
├── chart/                          # Helm chart
│   ├── Chart.yaml
│   ├── values.yaml                 # Image versions
│   └── templates/
│       ├── agent.yaml              # OpenClaw + Ollama StatefulSet
│       ├── controller.yaml         # Controller Deployment + Service
│       ├── vmware-gateway.yaml     # VMware Gateway Deployment + Service
│       └── matrix.yaml             # Matrix Synapse with TLS
├── controller/                     # Rust controller backend
│   ├── backend/
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── main.rs
│   │       ├── tasks.rs
│   │       ├── assignments.rs
│   │       ├── vms.rs
│   │       └── scale.rs
│   └── frontend/                   # React UI (Vite)
├── vmware_gateway/                 # Python vSphere proxy
│   ├── main.py
│   ├── pyproject.toml
│   └── README.md
├── openclaw/workspace-build/       # OpenClaw agent configuration
│   ├── AGENT.md
│   ├── SOUL.md
│   ├── HEARTBEAT.md
│   ├── TOOLS.md
│   └── skills/vmware-vm-operator/
│       ├── SKILL.md
│       └── vmware_operator.py
├── system/                         # ArgoCD and infrastructure (optional)
│   ├── dev/
│   └── prod/
└── .github/workflows/
    └── build-images.yml            # CI/CD for Docker images
```

## Version Management

Edit `chart/values.yaml` to change image versions:

```yaml
images:
  controller:
    tag: "0.1.28"
  openclaw:
    tag: "0.1.11"
  vmwareGateway:
    tag: "0.1.1"
```

## Components

| Component | Image | Description |
|-----------|-------|-------------|
| **Controller** | `hattmo/controller:0.1.28` | Rust/Axum REST API for task queues, agent assignments, and Matrix integration |
| **Agent** | `hattmo/openclaw:0.1.11` + `ollama/ollama:latest` | OpenClaw agents with Ollama sidecar for LLM inference |
| **VMware Gateway** | `hattmo/vmware-gateway:0.1.1` | Python/Flask proxy for vSphere keyboard/screen APIs |
| **Matrix** | `matrixdotorg/synapse:latest` | Synapse homeserver with CA-signed TLS for secure communication |

## External Access

To expose services externally:

```bash
# Manually create LoadBalancer:
cat <<EOF | kubectl apply -f -
apiVersion: v1
kind: Service
metadata:
  name: controller-lb
  namespace: npc
spec:
  type: LoadBalancer
  selector:
    app: controller
  ports:
  - port: 80
    targetPort: 8080
EOF
```

## Security Considerations

- **VM Whitelist**: Configure `vcenter-white-list` to restrict which VMs agents can access (e.g., `^training-.*$`)

## Development

### Build Images Locally

```bash
# VMware Gateway
docker build -t hattmo/vmware-gateway:local ./vmware_gateway

# Controller
docker build -t hattmo/controller:local ./controller

# Push to registry and update values.yaml
```
