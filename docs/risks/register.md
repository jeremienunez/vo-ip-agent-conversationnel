# Registre des Risques

| ID | Description | Impact | Probabilité | Propriétaire | Plan d'atténuation | Statut |
|----|-------------|--------|-------------|--------------|--------------------|--------|
| R1 | Maturité `rvoip` (couverture protocolaire/perf/mémoire) | Haut | Moyen | Tech Lead | Tests PoC précoces, benchmark SIPp, profilage mémoire, fallback Kamailio documenté | Ouvert |
| R2 | Performance RTP insuffisante | Haut | Moyen | DevOps | Tests charge Sprint 2, tuning kernel, scaling horizontal | Ouvert |
| R3 | Conformité RGPD (logs/enregistrements) | Moyen | Moyen | Responsable Sécurité | Anonymisation/pseudonymisation, politique rétention | Ouvert |
| R4 | Disponibilité infra (Kubernetes) | Haut | Faible | DevOps | Multi-zone, backup automatisé, monitoring | Ouvert |
| R5 | Régression mémoire non détectée en prod | Haut | Faible | QA | Pipelines `cargo miri`/`heaptrack`, alertes Prometheus mémoire, revues des rapports par sprint | Ouvert |
