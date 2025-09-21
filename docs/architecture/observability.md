# Observabilité

## 1. Journaux
- **application.log** : événements métier (appel établi, transfert, échec routage). Format JSON, champs : `timestamp`, `level`, `call_id`, `component`, `message`, `metadata`.
- **audit.log** : actions administratives (création utilisateur, modification route). Format JSON signé.
- **diagnostics.log** : détails bas niveau (négociation SDP, erreurs RTP). Rotation quotidienne + relai vers Loki.

## 2. Métriques Prometheus
- `voip_calls_active_total` (labels: tenant, route, codec).
- `voip_call_setup_duration_seconds` (histogram) pour latence INVITE → 200 OK.
- `voip_rtp_packet_loss_ratio` (gauge) par session.
- `voip_registration_total` (counter) par état (success/failure).
- `voip_api_requests_total` + `voip_api_request_duration_seconds` (API REST/gRPC).
- **Mémoire** : `voip_process_resident_memory_bytes`, `voip_process_allocations_total`, `voip_process_fragmentation_ratio`.

Exporter via :
- Service de signalisation Rust (`rvoip`) exposant métriques via `prometheus` crate / `metrics-exporter-prometheus`.
- RTPengine exporter.
- Services API Rust (`prometheus` crate / `metrics` + `metrics-exporter-prometheus`).
- Sidecars `node-exporter`/`process-exporter` pour corroborer la consommation mémoire.

## 3. Traces Distribuées
- Propagation W3C Trace Context (`traceparent`), corrélation par `Call-ID`.
- Ingestion vers Tempo/Jaeger, visualisation Grafana.
- Span types : `sip.invite`, `sip.register`, `rtp.session`, `api.request`, `db.query`.

## 4. Alertes
- `HighCallSetupFailureRate` : taux d'échec INVITE > 5% sur 5 min.
- `HighRTPPacketLoss` : perte > 5% sur 1 min.
- `RegistrationDrop` : baisse 30% du nombre de REGISTER vs baseline.
- `APIErrorRate` : erreurs 5xx > 2%.
- **MemoryBudgetExceeded** : `voip_process_resident_memory_bytes` > 80% du budget pendant 5 min.

## 5. Tableau de Bord
- Vue temps réel (nombre d'appels, MOS moyen, latence setup).
- Vue capacité (utilisation trunks, CPU/GPU si transcodage).
- Vue qualité (jitter, packet loss, retransmissions).
- Vue mémoire (RSS par service, allocations cumulées, fragmentation, historiques).

## 6. Gestion des Logs
- Standardiser via `scripts/logging-config` (à créer).
- Intégration Loki : promtail sidecar, retention 30 jours (diagnostic) / 365 jours (audit).
- Conformité : purges automatisées, signature des fichiers d'audit.

Mettre à jour ce document lors d'ajout de nouvelles métriques ou politiques d'alerte.
