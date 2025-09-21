# ADR-001 — Choix de la Stack de Signalisation

- **Date** : 20 septembre 2025
- **Statut** : Accepté (révisé post-recherche)

## Contexte
L'équipe doit sélectionner la stack technique pour la signalisation SIP. Les premières hypothèses combinaient Kamailio avec des modules Rust (KEMI). De nouvelles recherches menées en 2025 montrent que l'écosystème Rust (librairie `rvoip`, async avec `tokio`, exposition via `axum`) couvre désormais les besoins SIP/VoIP à l'échelle attendue. Le Tech Lead impose néanmoins une validation mémoire stricte à chaque étape du projet.

## Décision
Adopter une stack 100 % Rust (bibliothèque `rvoip` + runtime `tokio` + exposition `axum`) pour la signalisation. Kamailio reste un fallback si des protocoles périphériques manquent.

## Conséquences
- Accélère le Sprint 0 : compilation pure Cargo, intégration facilitée avec le reste des services Rust.
- Réduit la dette liée à l'embed KEMI et la maintenance Kamailio.
- Exige une surveillance rapprochée des performances et de la mémoire en tests de charge (`cargo miri`, `heaptrack`, stress long-run).

## Critères de Révision
- Si le throughput mesuré sur le PoC < 100 appels/seconde, ouvrir un nouvel ADR pour basculer vers une architecture hybride (Rust + Kamailio).
- Si le budget mémoire RSS dépasse 250 Mo/service ou si des fuites sont détectées malgré les tests, réévaluer la décision.

## Suivi
- Mettre à jour la documentation d'architecture et le CDC pour refléter cette décision.
- Planifier des tests de charge SIP + profilage mémoire dès Sprint 1/Sprint 2 pour valider les critères de révision.
 
 