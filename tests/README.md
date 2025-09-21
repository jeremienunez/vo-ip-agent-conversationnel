# Stratégie de Tests

- `unit/` : tests unitaires par composant, mocks pour dépendances externes.
- `integration/` : scénarios multi-composants (ex. SIPp + services API + DB).
- Tests de charge : outils SIPp/RTPp, scripts dans `tests/integration/`.
- Tests contractuels : gRPC/REST schema enforcement, utiliser Pact ou Dredd.

Automatiser via CI (`make test`, `make test-integration`). Conserver rapports (coverage, junit) dans `output/`.
