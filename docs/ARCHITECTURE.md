# Architecture cible

Ce document décrit le futur client. Aucun module applicatif n'existe encore.

## Flux principal

```text
Slint : vues et modèles visibles
    commandes utilisateur -> app : état et générations
    événements terminés   <- app
app -> service : requêtes YouTube Music, parsing et résolution de flux
app -> playback : téléchargement borné, décodage Symphonia, sortie rodio
app -> storage : paramètres et reprise atomique, cache de pochettes
app <-> platform : fenêtres, commandes média, tray, DPAPI
```

Un exécutable et une boucle UI. Les workers ont une durée de vie possédée par
l'application. Les notifications réveillent l'UI ; l'UI ne sonde pas les workers.
L'audio poursuit la lecture lorsque la fenêtre est masquée.

## Carte de fichiers prévue

| Fichier ou dossier | Responsabilité |
|---|---|
| `src/main.rs`, `build.rs`, `Cargo.toml` | Entrée, compilation Slint, dépendances |
| `src/app.rs` | État, commandes, réception des événements, invalidation des générations |
| `src/model.rs` | Types partagés de pistes, pages, commandes et erreurs |
| `src/queue.rs` | Ordre de lecture, répétition, mélange et déplacements |
| `src/service/mod.rs` | Client du service et opérations métier |
| `src/service/transport.rs` | HTTPS, limites, redirections, annulation et délais |
| `src/service/parse.rs` | Réponses distantes vers données typées |
| `src/service/stream.rs` | Résolution du flux et de ses capacités |
| `src/service/auth.rs` | Connexion explicite et état de session |
| `src/playback/mod.rs` | Machine de lecture et commandes audio |
| `src/playback/buffer.rs` | Source progressive, fenêtre mémoire et recherche temporelle |
| `src/storage.rs` | Paramètres, reprise atomique et versions du format local |
| `src/artwork.rs` | Chargement, redimensionnement et éviction des pochettes |
| `src/platform/mod.rs`, `media.rs`, `tray.rs`, `instance.rs`, `secrets.rs` | Frontières Windows et ressources natives |
| `src/update.rs` | Vérification et préparation des mises à jour |
| `ui/app.slint`, `theme.slint`, `player.slint`, `library.slint` | Présentation, tokens, lecteur, navigation et listes |
| `scripts/`, `installer/`, `.github/workflows/` | Mesures, installation et vérifications autorisées |

Créer ces fichiers avec les tâches qui en ont besoin. Ne pas générer une forêt
de modules vides. Scinder un fichier uniquement lorsque ses responsabilités le justifient.

## Décisions

- Rust pour la logique ; Slint compilé pour la présentation. Aucun interpréteur UI ajouté.
- Backend Winit, moteur FemtoVG/OpenGL retenu au départ pour limiter les composants graphiques.
- Changer de moteur seulement pour résoudre un problème reproduit et comparé ; ne pas compiler tous les moteurs par précaution.
- Symphonia décode les formats réellement exposés par le service. rodio possède la sortie audio.
- HTTPS bloquant avec `ureq` sur workers bornés ; pas de runtime asynchrone supplémentaire par défaut.
- `serde`/`serde_json` pour paramètres et reprise atomique. Pas de base SQL tant que le besoin de requêtes/migrations ne le justifie pas.
- API Windows via `windows-rs` pour les commandes média, la notification et DPAPI.
- Le thread UI possède les objets Slint ; les workers envoient des données Rust, jamais des références UI.
- Les noms et signatures partagés sont fixés dans le plan ; tout changement de contrat met à jour producteurs et consommateurs ensemble.

## Frontières

L'accès au service est non officiel et isolé dans `service/`. La capacité de
connexion, la lecture et les formats doivent être établis avant de dépendre
de fonctionnalités personnelles. Aucun fallback caché vers une WebView.

Les sessions appartiennent à Sillage. Les flux signés sont éphémères, ne vont
ni dans la reprise disque ni dans les logs. Une action de déconnexion retire
la session mémorisée et invalide les opérations liées au compte.

Le dossier de données prévu est `%LOCALAPPDATA%\Sillage`. L'AppId de l'installeur,
le mutex et les répertoires doivent être propres à Sillage.
