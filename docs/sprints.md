# Planification des Sprints

Cadence : sprints de 2 semaines. Les objectifs, stories clés, critères d'acceptation et risques sont détaillés ci‑dessous. Les points de story estimés seront renseignés lors de la planification Sprint (Poker Planning).

## Sprint 0 – Cadrage & Proof of Concept
- **Objectif** : Valider la faisabilité technique et aligner l'équipe.
- **Stories clés** :
  - Collecte exigences métier (workshops PO/Support).
  - Élaboration architecture high-level + CDC (docs finalisés).
  - PoC signalisation SIP (REGISTER/OPTIONS) en Rust (`rvoip`) avec instrumentation de base.
  - Mise en place dépôt, outillage base (`cargo fmt`, `cargo clippy`, Git hooks) + pipeline de tests mémoire (`cargo miri`, `cargo valgrind`).
- **Livrables** : README consolidé, `docs/cdc.md`, architecture overview, backlog priorisé, PoC SIP Rust démontrable + rapport mémoire initial.
- **Critères d'acceptation** : documentation validée par PO/Tech Lead, PoC tournant localement, tests mémoire exécutés et archivés.
- **Risques surveillés** : couverture protocolaire `rvoip`, dépendances externes, dette éventuelle si fallback Kamailio.

## Sprint 1 – MVP Signalisation & Environnement Dev
- **Objectif** : Fournir un service de signalisation opérationnel et un environnement de dev reproductible.
- **Stories clés** :
  - Implémenter service SIP Rust (REGISTER, INVITE basique) + tests unitaires (`cargo test`).
  - Docker Compose dev : `rvoip`, RTPengine stub, API mock Rust, PostgreSQL, Redis.
  - Pipeline CI (`cargo fmt`, `cargo clippy`, `cargo test`, build image, scan vulnérabilités avec `cargo-audit`) + job mémoire (`cargo miri`/`heaptrack`) sur chaque PR critique.
  - Gestion configuration 12-factor (fichiers `config/environments/`).
  - Documentation runbook développeur (`docs/architecture/deployment.md`) incluant protocole de tests mémoire.
- **Livrables** : docker-compose dev, pipeline CI sur dépôt, doc config, tests basiques, rapports mémoire automatisés.
- **Critères d'acceptation** : appel boucle locale UA↔UA fonctionnel, pipeline vert, doc suivie par nouvelle recrue, budget mémoire stable vs baseline.
- **Risques** : maturité `rvoip`, temps de build containers Rust.

## Sprint 2 – Média & Provisioning
- **Objectif** : Activer le relais média et exposer les APIs de provisioning.
- **Stories clés** :
  - Intégrer RTPengine, support SRTP, métriques QoS (service Rust asynchrone).
  - Développer API REST/gRPC (users, devices, trunks, routes) en Rust (`axum`/`tonic`).
  - Persistance PostgreSQL (migrations `sqlx-cli`/`refinery`), Redis sessions.
  - Observabilité : exporter métriques, logs structurés (`tracing`), dashboards initiaux + dashboards mémoire (RSS/allocations).
  - Tests d'intégration (SIPp + scénarios e2e) et charges de base avec profilage mémoire.
- **Livrables** : API documentée (OpenAPI/gRPC proto), dashboards Grafana, tests e2e, rapport mémoire média/services.
- **Critères d'acceptation** : appel UA↔Trunk tracké bout en bout (signaling + média + logs), absence de fuite mémoire détectée sur bancs de charge.
- **Risques** : performance RTP, complexité mapping CRM.

## Sprint 3 – Sécurité, Scalabilité, Pilotage Ops
- **Objectif** : Durcir la plateforme et préparer un pilote production.
- **Stories clés** :
  - Authentification forte (MFA), RBAC, audit trail complet.
  - Routage avancé (overflow, files d'attente simples, routage skills-based).
  - Tests de performance et tuning kernel (RTP), autoscaling HPA, stress mémoire (long-run) et surveillance fragmentation.
  - Documentation runbook ops, playbooks incidents, plan de sauvegarde (incluant procédures de vérif mémoire).
  - Mise en place pipelines CD vers staging + contrôle Go/No-Go.
- **Livrables** : runbook complet, résultats tests perf, politique monitoring/alerting, rapports mémoire long-run.
- **Critères d'acceptation** : SLA respecté en staging ≥ 1 semaine, audit log complet, plan DR validé, alertes mémoire configurées.
- **Risques** : contraintes sécurité, délai d'intégration CRM, charge tests.

## Gestion Continue
- **Backlog grooming** hebdomadaire avec PO & Tech Lead.
- **Daily stand-up** 15 minutes ; **Review/Retro** à la fin de chaque sprint (rapports dans `docs/retros/`).
- **Risk Register** tenu à jour (`docs/risks/register.md`).
- **Definition of Done** : code + tests verts, doc mise à jour, monitoring & alerting configurés, rapports mémoire générés, revue approuvée.

Mettre à jour ce plan après chaque revue de sprint pour refléter l'état réel et les ajustements.
