# BikeStationAPI

Une petite API Axum construite pour explorer le tracing, les timeouts et le rate limiting, sans se concentrer sur une logique métier complexe.

Le domaine — un réseau de vélos en libre-service qui expose la disponibilité des stations — est volontairement mince. L'intérêt du projet est tout ce qui entoure les endpoints : logs structurés, timeouts de requête, rate limiting par client, une tâche de fond de rafraîchissement, et un arrêt propre.

## Endpoints

| Méthode | Chemin | Description | Réponses |
|---------|--------|-------------|----------|
| GET | `/stations` | Liste toutes les stations | `200` tableau JSON |
| GET | `/stations/{id}` | Une station par id | `200` JSON, `404` si inconnue |
| POST | `/stations/{id}/report` | Signale un vélo défectueux (décrémente de un les vélos disponibles) | `200` station mise à jour, `404` si inconnue, `409` si aucun disponible |
| GET | `/health` | Sonde de vie | `200` |

Une station est `{ id, name, total_docks, available_bikes, free_docks }`. L'état est conservé en mémoire (pas de base de données) et initialisé avec quatre stations au démarrage.

## Fonctionnement

Chaque requête traverse trois couches tower, de la plus externe à la plus interne :

1. **Tracing** (`TraceLayer` de `tower-http`) — logue méthode, chemin, statut et latence pour chaque requête, y compris celles rejetées plus bas.
2. **Rate limiting** (`axum-governor`) — par IP client, 50 requêtes/seconde. Au-delà de la limite : `429` avec les en-têtes `RateLimit` et `Retry-After`.
3. **Timeout** (`TimeoutLayer` de `tower-http`) — 5 s par requête ; au-delà, `408`.

Cet ordre est volontaire : le tracing est le plus externe pour que les requêtes rejetées apparaissent quand même dans les logs, et le rate limiting passe avant le timeout pour couper le trafic abusif le plus tôt possible.

À côté du serveur, une tâche de fond rafraîchit l'état des stations toutes les 5 s depuis une source simulée (une fonction avec une latence artificielle). Cet appel est borné par son propre timeout de 2 s ; s'il est dépassé, le rafraîchissement est sauté et la tâche continue de tourner. Le rafraîchissement remplace l'état en mémoire, donc un `report` n'est visible que jusqu'au prochain cycle.

À `Ctrl-C`, le serveur cesse d'accepter des connexions et la tâche de fond est annulée ; le processus attend la fin des deux avant de se terminer.

## Lancer

Nécessite une toolchain Rust stable (edition 2024, 1.85+).

```
cargo run
```

Le serveur écoute sur `127.0.0.1:3000`.

```
curl -i http://127.0.0.1:3000/stations
curl -i http://127.0.0.1:3000/stations/1
curl -i -X POST http://127.0.0.1:3000/stations/1/report
```

## Configuration

Le niveau de log est contrôlé par `RUST_LOG` (par défaut `info` s'il n'est pas défini) :

```
RUST_LOG=debug cargo run
```

Les access logs HTTP (la couche de tracing) sont émis en `DEBUG` ; le niveau `info` par défaut n'affiche donc que les événements applicatifs : démarrage, timeouts du rafraîchissement de fond, arrêt. Utilise `RUST_LOG=debug` ou `RUST_LOG=info,tower_http=debug` pour voir les logs par requête.

## Tests

```
cargo test
```

Couvre le comportement des endpoints (codes de statut et JSON), le rafraîchissement de fond et son timeout, l'acceptation et le rejet du rate limiting (y compris l'isolation par client), et l'arrêt propre de la tâche de fond.

## Structure du projet

```
src/
  main.rs        démarrage du serveur, init du tracing, tâche de fond, arrêt propre
  routes.rs      routeur et pile de couches tower
  state.rs       état partagé en mémoire (Arc<RwLock<Vec<Station>>>)
  models.rs      modèle Station et données initiales
  handlers/      handlers des endpoints
  sources.rs     source simulée + tâche de fond de rafraîchissement
  shutdown.rs    signal Ctrl-C
tests/           tests d'intégration
```

## Notes

- La CI (GitHub Actions) lance `fmt`, `clippy -D warnings`, `check`, `test` et un build release à chaque push et pull request sur `main`.
- Le `Cargo.toml` embarque quelques dépendances liées à l'authentification (`argon2`, `jsonwebtoken`, `dotenvy`) qui ne sont pas encore utilisées — l'auth JWT est une suite possible, pas une partie du code actuel.
- Aucun fichier de licence n'est inclus.