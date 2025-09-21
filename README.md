# VoIP Server Platform

Plateforme VoIP modulaire permettant de gérer la signalisation SIP, le relais média RTP et l'orchestration des appels métier. Le dépôt rassemble l'ensemble des artefacts du projet : code source Rust, documentation fonctionnelle, planification agile et outillage d'exploitation.

## Vision & Objectifs
- Offrir un coeur de signalisation SIP tolérant aux pannes pour l'inscription des terminaux et l'établissement des sessions.
- Assurer le transport RTP avec chiffrement optionnel et capacités de transcodage.
- Exposer des API de provisioning (utilisateurs, trunks, routage) et d'observabilité (métriques, journaux structurés, traces).
- Industrialiser le déploiement via pipelines CI/CD, orchestration conteneur (Docker/Kubernetes) et infrastructure as code.
- Respecter les contraintes de conformité (journalisation audit, rétention, traçabilité des décisions).

## Architecture Fonctionnelle
Le système est organisé autour de quatre domaines principaux — cf. `docs/architecture/overview.md` pour les détails :
1. **Signalisation** : gestion des registres SIP, négociation SDP, supervision d'état (implémentation 100 % Rust conformément à ADR-001, fallback Kamailio documenté).
2. **Média** : relais RTP, adaptation codec, QoS et enregistrement.
3. **Services métiers** : API REST/gRPC de provisioning et logique de routage.
4. **Plateforme** : persistance, observabilité, sécurité, automatisation.

Un schéma de déploiement cible (staging/production) est décrit dans `docs/architecture/deployment.md` avec l'empilement réseau, les composants externes (SBC, CRM, opérateurs) et les flux.

## Structure du Dépôt
```
├── config/              # Modèles de configuration applicative & infrastructure
├── docs/                # Documentation fonctionnelle, technique et management
│   ├── architecture/    # Blueprints & décisions d'architecture
│   ├── decisions/       # Journal des ADR (Architecture Decision Records)
│   ├── retros/          # Comptes-rendus de rétrospectives
│   ├── risks/           # Registre des risques & plans d'atténuation
│   └── sprints.md       # Planification des sprints et jalons
├── logs/                # Emplacements des journaux applicatifs/audit/diagnostic
├── scripts/             # Automatisation (build, CI, outils d'administration)
├── src/                 # Code source Rust (signalisation, média, APIs, core)
├── tests/               # Suites de tests unitaires, intégration et charges
├── TODO.md              # Backlog court terme et actions à planifier
└── README.md            # Point d'entrée projet
```

Les sous-répertoires contiennent chacun un `README.md` ou un `.gitkeep` pour guider l'équipe.

## Lancement Rapide
1. **Installer les prérequis** : Rust ≥ 1.76 (toolchain stable + `clippy`), Docker, Docker Compose, Make, Python 3.11 (scripts), outils SIP (pjsua, sipp). Node.js ≥ 18 reste optionnel pour les utilitaires UI/monitoring.
2. **Cloner le dépôt** puis initialiser l'environnement : `make bootstrap` (script à livrer Sprint 1, installe toolchain Rust & hooks).
3. **Configurer les secrets** dans `config/environments/<env>.env` (modèles fournis).
4. **Démarrer l'environnement local** : `docker compose -f config/docker/devenv.compose.yml up` (à livrer Sprint 1).
5. **Exécuter les tests** : `cargo test` pour les unités, `make test-integration` pour scénarios SIP/RTP (à livrer Sprint 2).

## Gouvernance & Qualité
- Développement en branches courtes dérivées de `main`, revues obligatoires via pull request.
- CI : `cargo fmt`, `cargo clippy`, `cargo test`, scan de sécurité (cargo-audit, Trivy) avant fusion.
- **Tests mémoire systématiques** : `cargo miri`, `cargo valgrind`, profilage `heaptrack`/`memray` sur chaque incrément critique (signalisation, média) et rapports versionnés.
- CD : déploiement automatisé vers staging, promotion vers production après validation manuelle.
- Journalisation structurée (JSON) avec corrélation par `Call-ID` et `Trace-ID` via `tracing`.
- Observabilité : Prometheus + Grafana, Loki pour les logs, Tempo/Jaeger pour le tracing (voir `docs/architecture/observability.md`).

## Planification & Documentation
- **CDC** : exigences fonctionnelles et non-fonctionnelles (`docs/cdc.md`).
- **Sprints** : objectifs, livrables, risques (`docs/sprints.md`).
- **Décisions** : ADR consignées dans `docs/decisions/` (cf. ADR-001 pour la stack de signalisation).
- **Risques** : identification & suivi dans `docs/risks/register.md`.
- **Rétros** : leçons apprises par sprint dans `docs/retros/`.

Mettez à jour ces artefacts à chaque incrément pour conserver une vision fiable.

## Contact & Collaboration
- Product Owner : à définir (Sprint 0).
- Tech Lead : à définir (Sprint 0).
- Canal de communication : Slack `#voip-platform`, réunions stand-up quotidiennes.
- Point d'entrée décisionnel : consigner toute décision majeure via ADR et informer le PO.

Pour toute modification majeure, synchronisez-vous avec l'équipe, mettez à jour la documentation associée et assurez la traçabilité dans les journaux de décisions.
