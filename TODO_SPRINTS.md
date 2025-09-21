# TODO - Organisation en Sprints Détaillés

## Vue d'Ensemble
- **Durée Sprint** : 2 semaines
- **Équipe** : 1 PO, 1 Tech Lead, 3 Backend Rust, 1 DevOps, 1 QA
- **Stack** : Rust (signalisation/API), PostgreSQL, Redis, Docker/K8s
- **Objectif** : Plateforme VoIP production-ready avec SLA 99.95%

---

## 🚀 SPRINT 0 - Cadrage & PoC (En cours)
**Objectif** : Valider l'architecture et la faisabilité technique

### ✅ Complété
- [x] Structure workspace Rust avec 4 crates (core, signalling, media, api)
- [x] Documentation architecture (README, CDC, sprints)
- [x] Configuration workspace avec lints partagés
- [x] Squelette des services avec tests de base

### 📋 À faire
- [ ] **Installation Toolchain Rust**
  - [ ] Exécuter `./scripts/bootstrap.sh`
  - [ ] Vérifier `cargo --version` >= 1.76
  - [ ] Installer outils mémoire : `cargo install cargo-miri cargo-valgrind`

- [ ] **Validation Build Initial**
  - [ ] `cargo build --workspace`
  - [ ] `cargo fmt --all -- --check`
  - [ ] `cargo clippy --all-targets --all-features`
  - [ ] `cargo test --workspace`

- [ ] **PoC Signalisation SIP**
  - [ ] Intégrer crate `rvoip` dans voip-signalling
  - [ ] Implémenter REGISTER handler basique
  - [ ] Implémenter OPTIONS handler (keepalive)
  - [ ] Test avec `pjsua` ou `linphone`

- [ ] **Tests Mémoire Baseline**
  - [ ] Exécuter `cargo miri test`
  - [ ] Profiler avec `heaptrack cargo run`
  - [ ] Générer rapport mémoire initial dans `docs/memory/sprint0.md`

### 📊 Métriques de succès
- Build complet sans erreurs
- PoC SIP répond à REGISTER/OPTIONS
- Rapport mémoire < 50MB RSS idle

---

## 🛠 SPRINT 1 - MVP Signalisation & CI/CD
**Objectif** : Service SIP fonctionnel avec pipeline automatisé

### 🎯 User Stories

#### 1. Service Signalisation Complet (13 pts)
```rust
// voip-signalling/src/sip/
├── registry.rs       // Gestion des registres UA
├── session.rs        // Machine d'état INVITE/BYE
├── transport.rs      // UDP/TCP/TLS listeners
└── auth.rs          // Digest authentication
```
- [ ] REGISTER avec authentification Digest
- [ ] INVITE/ACK/BYE flow complet
- [ ] CANCEL et timeout handling
- [ ] Re-INVITE pour hold/resume
- [ ] Tests unitaires > 80% couverture

#### 2. Environnement Docker Dev (8 pts)
```yaml
# docker-compose.dev.yml
services:
  voip-signalling:
    build: ./crates/signalling
    ports: ["5060:5060/udp", "5060:5060/tcp"]

  postgres:
    image: postgres:16-alpine
    volumes: ["./sql/init.sql:/docker-entrypoint-initdb.d/"]

  redis:
    image: redis:7-alpine
```
- [ ] Dockerfile multi-stage pour Rust
- [ ] Scripts SQL migration (users, devices, CDRs)
- [ ] Configuration via env vars
- [ ] Health checks pour tous services

#### 3. Pipeline CI GitHub Actions (5 pts)
```yaml
# .github/workflows/ci.yml
- cargo fmt --check
- cargo clippy -- -D warnings
- cargo test --workspace
- cargo audit
- cargo miri test (weekly)
- docker build & push
```
- [ ] Tests sur chaque PR
- [ ] Build images Docker
- [ ] Scan sécurité dependencies
- [ ] Badge coverage dans README

#### 4. Configuration 12-Factor (3 pts)
```toml
# config/dev.toml
[sip]
listen = "0.0.0.0:5060"
transport = ["udp", "tcp"]
max_registrations = 1000

[database]
url = "${DATABASE_URL}"
pool_size = 20
```
- [ ] Parser TOML avec validation
- [ ] Override par env vars
- [ ] Templates pour dev/staging/prod

### 📊 Métriques Sprint 1
- Appel UA↔UA local réussi
- Pipeline CI 100% vert
- Temps build Docker < 5 min
- Mémoire service SIP < 100MB sous 100 REGISTER

---

## 🎬 SPRINT 2 - Média & APIs
**Objectif** : Relais RTP et APIs de provisioning

### 🎯 User Stories

#### 1. Service Média RTP (13 pts)
```rust
// voip-media/src/
├── relay.rs          // RTP forwarding engine
├── codec.rs          // G.711, G.729, Opus
├── recording.rs      // Capture streams
└── qos.rs           // Jitter buffer, packet loss
```
- [ ] Négociation SDP (offer/answer)
- [ ] Relais RTP/RTCP bidirectionnel
- [ ] Support SRTP (chiffrement)
- [ ] Métriques QoS temps réel
- [ ] Enregistrement WAV optionnel

#### 2. API REST Provisioning (8 pts)
```rust
// voip-api/src/routes/
├── users.rs         // CRUD utilisateurs
├── devices.rs       // Gestion terminaux
├── trunks.rs        // Config opérateurs
└── routes.rs        // Plans numérotation
```
- [ ] OpenAPI 3.0 spec
- [ ] Validation requêtes (validator)
- [ ] Auth JWT/OAuth2
- [ ] Rate limiting
- [ ] Tests Postman collection

#### 3. API gRPC Temps Réel (5 pts)
```proto
service VoIPControl {
  rpc GetActiveCalls(Empty) returns (stream Call);
  rpc HangupCall(CallId) returns (Result);
  rpc TransferCall(TransferRequest) returns (Result);
}
```
- [ ] Protobuf definitions
- [ ] Service Tonic async
- [ ] Client CLI pour tests

#### 4. Persistance & Cache (8 pts)
```sql
-- PostgreSQL schemas
CREATE TABLE users (id, email, password_hash, ...);
CREATE TABLE devices (id, user_id, mac, ip, ...);
CREATE TABLE cdrs (id, call_id, from, to, duration, ...);
```
- [ ] Migrations SQLx
- [ ] Connection pooling (deadpool)
- [ ] Redis session cache
- [ ] Backup strategy

#### 5. Observabilité (5 pts)
- [ ] Prometheus metrics endpoint
- [ ] Grafana dashboards (SIP, RTP, API)
- [ ] Tracing avec OpenTelemetry
- [ ] Alerting rules (> 500ms latency, > 80% CPU)
- [ ] Dashboard mémoire dédié

### 📊 Métriques Sprint 2
- 100 appels simultanés sans dégradation
- API latency p99 < 200ms
- Zero packet loss RTP local
- Dashboards opérationnels

---

## 🔒 SPRINT 3 - Production Readiness
**Objectif** : Sécurité, scalabilité et ops

### 🎯 User Stories

#### 1. Sécurité & Compliance (8 pts)
- [ ] MFA pour admin (TOTP)
- [ ] RBAC avec Casbin
- [ ] Audit trail complet
- [ ] Chiffrement données au repos
- [ ] Rate limiting SIP (anti-flood)
- [ ] Géo-blocking par IP

#### 2. Routage Avancé (13 pts)
```rust
// Moteur de règles
enum RouteStrategy {
    RoundRobin,
    LeastCost,
    SkillBased,
    TimeOfDay,
    Geographic,
}
```
- [ ] Files d'attente (FIFO/priorité)
- [ ] Routage skill-based
- [ ] Failover automatique
- [ ] Load balancing trunks
- [ ] Emergency numbers (112, 911)

#### 3. Tests Performance (8 pts)
- [ ] SIPp scenarios (10K registrations)
- [ ] RTP stress test (1K concurrent)
- [ ] Memory leak detection (72h run)
- [ ] Chaos engineering (network loss)
- [ ] Load test report

#### 4. Kubernetes Deployment (8 pts)
```yaml
# k8s/production/
├── namespace.yaml
├── configmap.yaml
├── secrets.yaml
├── deployments/
├── services/
├── ingress.yaml
└── hpa.yaml
```
- [ ] Helm charts
- [ ] HPA (auto-scaling)
- [ ] Network policies
- [ ] PVC pour recordings
- [ ] Backup CronJobs

#### 5. Documentation Ops (5 pts)
- [ ] Runbook incidents
- [ ] Playbooks (deploy, rollback)
- [ ] Architecture Decision Records
- [ ] Formation support L1/L2
- [ ] SLA monitoring

### 📊 Métriques Sprint 3
- SLA 99.95% sur 30 jours
- Recovery time < 5 min
- Zero données perdues
- Conformité audit sécurité

---

## 📈 SPRINT 4 - Optimisations & Features
**Objectif** : Performance et fonctionnalités avancées

### 🎯 Optimisations
- [ ] SIMD pour codec processing
- [ ] io_uring pour network I/O
- [ ] Zero-copy RTP relay
- [ ] Connection pooling optimisé
- [ ] Caching stratégique Redis

### 🎯 Features Avancées
- [ ] WebRTC gateway
- [ ] Transcription temps réel (Whisper)
- [ ] Analytics dashboard
- [ ] API webhooks
- [ ] Multi-tenant isolation

---

## 📋 Backlog Long Terme

### Intégrations
- [ ] CRM (Salesforce, HubSpot)
- [ ] Microsoft Teams Direct Routing
- [ ] SMS/MMS gateway
- [ ] Vidéo conférence
- [ ] Call center features (IVR, ACD)

### Infrastructure
- [ ] Multi-région active-active
- [ ] CDN pour médias
- [ ] Blockchain CDR (immutabilité)
- [ ] ML-based fraud detection
- [ ] IPv6 support complet

---

## 🎯 Definition of Done

### Pour chaque User Story
- [ ] Code reviewed par 2 devs
- [ ] Tests unitaires écrits (> 80% coverage)
- [ ] Tests d'intégration passés
- [ ] Documentation mise à jour
- [ ] Pas de TODOs/FIXMEs critiques
- [ ] Métriques Prometheus exposées
- [ ] Logs structurés avec correlation ID
- [ ] Memory profiling exécuté
- [ ] Security scan passed

### Pour chaque Sprint
- [ ] Demo au PO/Stakeholders
- [ ] Retrospective documentée
- [ ] Backlog re-priorisé
- [ ] Risks register mis à jour
- [ ] Performance baseline capturée
- [ ] Documentation ops/dev à jour

---

## 🚨 Risques Identifiés

| Risque | Impact | Mitigation | Owner |
|--------|--------|------------|-------|
| Maturité `rvoip` | High | Fallback Kamailio, contribute upstream | Tech Lead |
| Performance RTP | High | Kernel tuning, DPDK option | DevOps |
| Latence réseau | Medium | Edge deployment, QoS config | DevOps |
| Sécurité 0-day | High | Security monitoring, patches rapides | Security |
| Adoption users | Medium | Training, UX simple | PO |

---

## 📊 KPIs Projet

### Techniques
- Uptime : > 99.95%
- Call setup time : < 2s
- Audio quality MOS : > 4.0
- Concurrent calls : > 1000
- Memory per call : < 250KB

### Business
- Cost per minute : < €0.01
- User satisfaction : > 85%
- Support tickets : < 5/semaine
- Time to market : 3 mois

---

## 🔄 Process Agile

### Daily Standup (15 min)
- What I did yesterday
- What I'll do today
- Blockers

### Sprint Planning (4h)
- Review backlog
- Estimate stories (Fibonacci)
- Commit to sprint goal

### Sprint Review (2h)
- Demo to stakeholders
- Gather feedback
- Update backlog

### Sprint Retrospective (1h)
- What went well
- What could improve
- Action items

---

## 📝 Notes

- Utiliser `cargo-nextest` pour tests parallèles
- Profiler avec `flamegraph` avant optimisation
- Toujours tester sur réseau avec latence/perte
- Documenter les décisions dans ADR
- Maintenir une roadmap publique