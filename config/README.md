# Configuration

- `environments/` : modèles de fichiers `.env` spécifiques aux environnements (`dev`, `staging`, `prod`). Copier un modèle et renseigner les secrets localement.
- `docker/` : manifestes Docker Compose / override pour le développement, tests, démonstrations.

## Bonnes pratiques
- Ne versionnez jamais les secrets effectifs. Utilisez des fichiers suffixés `.template`.
- Documentez chaque variable dans un tableau (nom, description, défaut) dans `docs/architecture/deployment.md`.
- Synchronisez la configuration avec Vault/Secrets Manager.
