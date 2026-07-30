# Serveur de test Docker — simuler la réception des données

Ce document explique le serveur de simulation utilisé pour valider l'envoi HTTP distant du client Tracker (`src/remote_export.rs`, voir `README_data_how_they_are_send.md` §4), en attendant l'implémentation du vrai serveur de réception.

## 1. But

Aucun serveur de réception définitif n'existe encore. `mock-server/` est un serveur HTTP minimal, conteneurisé, qui :

- reçoit les rapports POSTés par le client Tracker,
- vérifie (optionnellement) un token d'authentification Bearer,
- les logue et les stocke sur disque pour inspection,

afin de pouvoir tester tout le chemin d'envoi (config UI → `send_report` → réseau → réponse) sans dépendre du futur vrai serveur.

## 2. Démarrage rapide

```bash
MOCK_SERVER_TOKEN=changeme docker compose up --build
```

Le serveur écoute sur `http://localhost:8080`. Sans `MOCK_SERVER_TOKEN` défini, il démarre quand même mais n'applique **aucune vérification d'authentification** (un avertissement est loggué au démarrage).

Pour l'arrêter : `docker compose down`.

## 3. Endpoints

| Endpoint | Méthode | Comportement |
|---|---|---|
| `/report` | `POST` | Vérifie `Authorization: Bearer <token>` si `MOCK_SERVER_TOKEN` est défini côté serveur (sinon accepte tout). Stocke le corps JSON reçu tel quel dans `/data/reports/<timestamp>.json`. Répond `200 {"status":"received"}`, ou `401 {"status":"unauthorized"}` si le token est absent/incorrect. |
| `/health` | `GET` | `200 OK`, utilisé par le `HEALTHCHECK` Docker. |

## 4. Configuration

| Variable d'environnement | Défaut | Rôle |
|---|---|---|
| `MOCK_SERVER_TOKEN` | absent (pas de vérification) | Token attendu dans `Authorization: Bearer <token>` |
| `MOCK_SERVER_DATA_DIR` | `/data/reports` | Répertoire de stockage des rapports reçus |
| `RUST_LOG` | — | Niveau de log (`info` recommandé, défini dans `docker-compose.yml`) |

`docker-compose.yml` monte `./mock-server/data` (hôte) sur `/data` (conteneur) : les rapports reçus persistent après un `docker compose down`.

## 5. Relier le client Tracker au serveur de test

Dans l'application GUI, onglet **Paramètres** :

1. Activer l'export HTTP automatique.
2. URL du serveur : `http://localhost:8080/report`.
3. Jeton d'authentification : la même valeur que `MOCK_SERVER_TOKEN`.
4. Sauvegarder, puis lancer une collecte depuis l'écran d'accueil — le statut d'envoi ("Envoyé au serveur ✓" ou message d'erreur) s'affiche juste après le résultat de l'export fichiers.

Un token absent ou incorrect côté client doit se traduire par une erreur `401` visible dans ce statut (pas de retry sur ce cas, voir `README_data_how_they_are_send.md` §4).

## 6. Vérifier la réception

```bash
# Rapports reçus, un fichier JSON par requête
ls mock-server/data/reports/
cat mock-server/data/reports/<fichier>.json

# Logs du conteneur (requêtes reçues, rejets d'authentification, erreurs)
docker compose logs mock-server
```

Test manuel rapide sans passer par l'app GUI :

```bash
curl -i -X POST http://localhost:8080/report \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer changeme" \
  -d '{"hello":"world"}'
```

## 7. Structure des fichiers

- `mock-server/Cargo.toml`, `mock-server/src/main.rs` — crate Rust (Axum + Tokio), membre du workspace Cargo racine.
- `mock-server/Dockerfile` — build multi-stage (`rust:1-slim` → `debian:bookworm-slim`).
- `docker-compose.yml` (racine du repo) — orchestration du service `mock-server`.

## 8. Limites assumées

Ce serveur est un outil de test, pas une base pour le futur serveur de réception :

- pas d'authentification par défaut si `MOCK_SERVER_TOKEN` n'est pas défini,
- aucune validation du schéma du JSON reçu (accepté tel quel, y compris invalide),
- pas de nettoyage automatique de `/data/reports` — les fichiers s'accumulent,
- pas de persistance en base de données, pas d'API de consultation au-delà des fichiers bruts,
- exécution en un seul processus, sans authentification par client ni gestion multi-token.
