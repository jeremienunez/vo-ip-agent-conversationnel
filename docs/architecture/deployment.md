# Déploiement & Topologies

## 1. Environnements
- **Dev** : Docker Compose, services Rust (SIP Gateway `rvoip`, Media Relay stub, API mock), PostgreSQL, Redis, observabilité allégée.
- **Staging** : Kubernetes (namespace dédié), autoscaling activé, trafic simulé (SIPp) pour tests de charge, monitoring complet.
- **Production** : Kubernetes multi-zone, ingress haute disponibilité, passerelles SBC en frontal, services Rust containerisés, BDD gérée (PostgreSQL HA), stockage S3 compatible.

## 2. Topologie Réseau (production)
- **Zone DMZ** : SBC/Load Balancer (Envoy) -> SIP Gateway Rust (`rvoip`) -> Media Relay (RTPengine).
- **Zone Applicative** : Services API/Core, brokers, jobs.
- **Zone Data** : PostgreSQL HA, Redis cluster, MinIO/S3, Elastic cluster.
- **Observabilité** : Prometheus + Grafana + Loki + Tempo, exporters dédiés (`rvoip` metrics, RTPengine exporter).

Les communications sensibles sont chiffrées (TLS/SRTP). Gestion des certificats via cert-manager/ACME interne.

## 3. Pipelines CI/CD
1. `build` : `cargo fmt` + `cargo clippy` + tests → `cargo miri` (sanity mémoire) → build container multi-arch (Rust) → profilage `heaptrack`/`memray` sur scénarios critiques → scan (Trivy/cargo-audit) → push registry.
2. `deploy` : génération manifests (Helm/Kustomize) → déploiement Terraform/Helm vers staging → tests e2e (SIPp) + stress mémoire (long-run) → promotion manuelle vers production.
3. `ops` : runbook automatisé (scripts/ops), plan de rollback (Helm release previous), vérif post-déploiement (alertes mémoire, budgets RSS).

## 4. Configuration & Secrets
- ConfigMap/Helm values pour paramètres non sensibles.
- Secrets : Vault ou Sealed Secrets (Kubernetes), rotation automatique.
- Variables par environnement dans `config/environments/<env>.env` + `config/docker/*.compose.yml`.

## 5. Supervision & Alerting
- Tableaux de bord Grafana : signalisation Rust, médias, API, infrastructure, mémoire.
- Alertmanager : SLA appels, erreurs 5xx, latence, disponibilité base, jitter RTP, budgets mémoire.
- Logs structurés vers Loki, règles d'alerte sur taux d'échec.

## 6. Continuité d'Activité
- Backups PostgreSQL/MinIO quotidiens + tests de restauration mensuels.
- Clusters multi-zone, bascule automatique via kube-proxy/lb.
- Plan de désastre : redéploiement dans zone secondaire (Terraform + Ansible).

## 7. Conformité & Sécurité
- Politique réseau : NetworkPolicies, PodSecurityStandards (baseline).
- Traçabilité : audit log stocké 12 mois, accès restreint (IAM, RBAC).
- Pentest annuel, scans vulnérabilités en continu.

Mettez à jour ce document après chaque modification d'infrastructure ou pipeline.
