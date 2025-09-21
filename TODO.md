# TODO Backlog

## Sprint 1 - Urgent (Semaine du 23 septembre 2025)

### Infrastructure technique
- [ ] **Réactiver OpenTelemetry** avec nouvelle API TracerProvider::builder()
- [ ] **Support Serde pour proto types** - Wrapper types ou migration vers prost-serde
- [ ] **Connexion NATS réelle** - Implémenter EventBus avec vraie connexion
- [ ] **Corriger les 10 warnings** de compilation (unused imports, dead code)
- [ ] **Tests d'intégration** - Au moins 30% de coverage

### Documentation
- [ ] **Guide de démarrage rapide** - README avec quick start
- [ ] **Documentation API REST** - OpenAPI/Swagger pour endpoints
- [ ] **Architecture as-built** - Diagrammes de l'architecture réelle

### Services core
- [ ] **Health checks complets** - /health avec status détaillé par composant
- [ ] **Métriques Prometheus** - Exposer /metrics endpoint
- [ ] **Graceful shutdown** - Gérer proprement SIGTERM/SIGINT

## Sprint 2 - SIP Core (Début octobre 2025)

### Signalisation SIP
- [ ] **Parser SIP** - Intégrer crate SIP ou implémenter parser basique
- [ ] **REGISTER handler** - Gestion des enregistrements SIP
- [ ] **OPTIONS handler** - Keep-alive et capability discovery
- [ ] **Session state machine** - FSM pour les appels
- [ ] **Tests SIPp** - Scénarios automatisés

### Base de données
- [ ] **Schema PostgreSQL** - Tables pour users, devices, calls
- [ ] **Migrations SQLx** - Setup avec versioning
- [ ] **Repository pattern** - Abstraction d'accès aux données

## Sprint 3 - Media & RTP (Mi-octobre 2025)

### Relais média
- [ ] **RTP parser/builder** - Traitement des paquets RTP
- [ ] **SRTP support** - Chiffrement des flux média
- [ ] **Codec negotiation** - G.711, G.729, Opus
- [ ] **Jitter buffer** - Gestion de la QoS
- [ ] **DTMF detection** - RFC 2833

### Monitoring média
- [ ] **Statistiques RTP** - Perte de paquets, jitter, latence
- [ ] **MOS scoring** - Qualité perçue des appels
- [ ] **Recording API** - Interface pour enregistrement

## Backlog Technique (Non priorisé)

### Performance
- [ ] Benchmark avec `criterion` - Baseline de performance
- [ ] Profiling mémoire systématique - `miri`, `heaptrack`
- [ ] Load testing - 1000 appels simultanés objectif
- [ ] Connection pooling - Redis, PostgreSQL

### Sécurité
- [ ] TLS mutual authentication - mTLS entre services
- [ ] Rate limiting - Par IP et par user
- [ ] Audit logging - Journalisation sécurisée
- [ ] Secrets management - Vault ou similar

### DevOps
- [ ] Pipeline CI complet - Tests, linting, security scan
- [ ] Dockerfile multi-stage - Images optimisées
- [ ] Helm charts - Déploiement Kubernetes
- [ ] Monitoring stack - Grafana dashboards

### Intégrations
- [ ] Webhook system - Notifications d'événements
- [ ] REST API v2 - Version améliorée
- [ ] GraphQL endpoint - API alternative
- [ ] WebSocket support - Real-time updates

## Priorité Haute (ADR-001 legacy)
- [x] ~~Propager ADR-001 auprès de l'équipe~~ - Complété avec migration Rust
- [ ] Préparer PoC rvoip avec instrumentation mémoire
- [ ] Industrialiser pipeline de tests avec job mémoire
- [ ] Esquisser config/environments/dev.env.template

## Priorité Moyenne
- [ ] Plan de formation équipe support et runbook incident
- [ ] Cartographier intégrations CRM/ERP disponibles
- [ ] Vérifier licences des dépendances crates

## Priorité Basse
- [ ] Options d'enregistrement appels chiffrés
- [ ] Stratégie multi-tenant (isolation données)
- [ ] Impact RGPD sur analytics et conservation

## Bugs connus

### Critiques
- OpenTelemetry désactivé (temporaire)
- EventBus non connecté à NATS

### Mineurs
- 10 warnings de compilation
- Dead code dans plusieurs modules
- Imports non utilisés

## Notes

- Mettre à jour à chaque Sprint Planning/Review
- Versioning semantic : 0.x.y tant qu'en développement
- Objectif v1.0.0 : Q2 2026 avec SIP complet

---
*Dernière mise à jour : 2025-09-21 par Claude*