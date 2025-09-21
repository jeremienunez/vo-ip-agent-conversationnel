# Cahier des Charges – Plateforme VoIP

## 1. Contexte
L'entreprise souhaite disposer d'une plateforme VoIP interne capable de gérer des flux d'appels entrants/sortants, de fournir des services de téléphonie avancés (routage dynamique, intégration CRM) et de garantir la conformité réglementaire (journalisation, protection des données). Le projet doit être déployable on-premise ou dans le cloud hybride et supporte une montée en charge progressive.

## 2. Périmètre Fonctionnel
- Gestion des terminaux SIP (softphones, IP phones, gateways).
- Etablissement, maintien et fin de sessions d'appels voix sur IP.
- Routage contextuel des appels (compétence, horaires, scénarios failover).
- Provisioning des utilisateurs, groupes, trunks opérateurs.
- Gestion des files d'attente et distribution automatique d'appels basique.
- Journalisation des événements clés (audit, diagnostic, facturation).
- Interfaces d'administration (API REST/gRPC, portail à venir).

Hors périmètre initial : messagerie vocale, vidéo, SMS, intégration temps réel avec WebRTC (à évaluer après Sprint 3).

## 3. Exigences Fonctionnelles
1. **Enregistrement SIP** : support REGISTER/OPTIONS, expiration configurable, authentification Digest.
2. **Signalisation appels** : INVITE, ACK, BYE, CANCEL, REFER, reINVITE pour modification de session.
3. **Relais média** : support RTP/RTCP, codecs G.711/G.729/Opus, enregistrement optionnel.
4. **Provisioning** : APIs CRUD pour utilisateurs, numéros, plans de numérotation, trunks.
5. **Routage** : règles basées sur horaires, priorité agent, overflow, numéros d'urgence.
6. **Observabilité** : métriques Prometheus, logs JSON (application/audit/diagnostic), traces distribuées.
7. **Sécurité** : authentification multi-facteurs pour administrateurs, RBAC, chiffrement TLS pour SIP/TLS et SRTP.
8. **Intégration** : connecteurs REST/AMQP pour CRM/ERP, webhooks d'événements importantes.

## 4. Exigences Non Fonctionnelles
- Disponibilité cible : 99,95% (avec plan de bascule actif-passif).
- Latence de signalisation : < 150 ms sur réseau interne, < 300 ms inter-sites.
- Scalabilité horizontale via autoscaling Kubernetes (HPA).
- Observabilité : 95% des événements critiques corrélés via IDs.
- Sécurité : conformité RGPD, chiffrement au repos (base de données, fichiers sensibles).
- Maintenabilité : couverture de tests unitaires ≥ 70% dès Sprint 2, intégration continue obligatoire.
- Portabilité : déploiement possible sur Kubernetes ≥1.27 et Docker Compose pour dev.
- **Maîtrise mémoire** : budget RSS/processus < 250 Mo par service SIP/Media en charge nominale, alertes > 80% du budget, tests mémoire (`cargo miri`, `heaptrack`, stress long-run) exécutés à chaque sprint.

## 5. Architecture Cible
- **Couche Signalisation** : microservice SIP 100 % Rust (bibliothèque `rvoip` + `tokio` + `axum`) exposant les interfaces internes. Kamailio reste un fallback si des protocoles périphériques critiques sont absents.
- **Couche Média** : relais RTP (RTPengine / MediaSoup) orchestré via service Rust.
- **Couche Services** : microservices Rust (frameworks Actix Web / Axum / Tonic) pour API REST/gRPC, orchestrés par un bus événementiel (NATS/Kafka).
- **Persistance** : PostgreSQL (configuration), Redis (sessions), MinIO/S3 (enregistrements), Elasticsearch (logs longue rétention).
- **Observabilité** : Prometheus, Grafana, Loki, Tempo/Jaeger.
- **Infra** : Kubernetes (prod/staging), Docker Compose (dev), Terraform/Ansible pour IaC.

Le détail des flux et diagrammes se trouve dans `docs/architecture/`.

## 6. Environnements Cibles
- **Développement** : Docker Compose local, données de test anonymisées.
- **Staging** : cluster Kubernetes mirroring production, tests de charge et DRT.
- **Production** : cluster haute disponibilité multi-zone, supervision et sauvegardes automatisées.

## 7. Roadmap & Jalons
- Sprint 0 : cadrage, architecture high-level, backlog, PoC signalisation Rust + rapport mémoire initial.
- Sprint 1 : MVP signalisation + pipeline CI Rust + job mémoire automatisé.
- Sprint 2 : média + provisioning + observabilité (dashboards mémoire).
- Sprint 3 : sécurité, scalabilité, préparation pilote (stress mémoire long-run).
- Go/No-Go production pilot à l'issue de Sprint 3.

## 8. Budget & Contraintes
- Équipe dédiée : 1 PO, 1 Tech Lead, 3 devs backend Rust, 1 devops, 1 QA.
- Budget licences : prioriser open-source, prévoir coûts monitoring/call recording.
- Contraintes légales : conservation des logs d'audit 12 mois, consentement enregistrement appels.

## 9. Critères d'Acceptation Globaux
- Passage des tests fonctionnels SIP et scénarios métiers critiques.
- SLA observé sur 30 jours consécutifs en staging.
- Documentation à jour (README, CDC, architecture, playbooks).
- Processus de support opérationnel défini (astreinte, runbook, escalade).
- **Validation mémoire** : absence de fuite détectée sur bancs de charge, respect budgets RSS et fragmentation maîtrisée (<10%).

## 10. Risques & Plans d'Atténuation
| Risque | Impact | Probabilité | Atténuation |
|--------|--------|-------------|-------------|
| Couverture protocolaire `rvoip` incomplète | Fort | Moyen | Prévoir fallback Kamailio, contributions open-source, veille écosystème |
| Performance/consommation mémoire `rvoip` | Fort | Moyen | Tests PoC précoces, profilage `heaptrack`, seuils d'alerte, fallback hybride |
| Performance RTP | Fort | Moyen | Tests de charge (SIPp, RTPp) dès Sprint 2, tuning kernel |
| Sécurité données | Fort | Moyen | Audit sécurité, chiffrement systématique, pen-test |
| Adoption équipe support | Moyen | Moyen | Formation, documentation runbook, shadowing |

## 11. Validation
Le CDC est validé par : Product Owner, Tech Lead, Responsable Sécurité. Les révisions sont journalisées dans `docs/decisions/`.
