# Architecture cible

Le socle local M1, la tranche recherche publique → lecture progressive et les
pochettes bornées des premiers résultats sont implémentés. Stockage, cache durable,
navigation et mises à jour restent à construire.
Le lecteur n'a pas été lancé ; son rendu et son comportement audio ne sont pas validés.

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
| `src/search.rs` | Worker de recherche unique, dernière requête conservée, notification Slint |
| `src/model.rs` | Types partagés de pistes, pages, commandes et erreurs |
| `src/queue.rs` | Ordre de lecture, répétition, mélange et déplacements |
| `src/service/mod.rs` | Client du service et opérations métier |
| `src/service/transport.rs` | HTTPS, limites, redirections, annulation et délais |
| `src/service/parse.rs` | Réponses distantes vers données typées |
| `src/service/stream.rs` | Lecture HTTP progressive bornée, annulation et refus du seek |
| `src/service/auth.rs` | Connexion explicite et état de session |
| `src/playback/mod.rs` | Machine de lecture et commandes audio |
| `src/playback/source.rs` | Décodage local/distant sur worker et petite file PCM bornée |
| `src/playback/buffer.rs` | Source progressive, fenêtre mémoire et recherche temporelle |
| `src/storage.rs` | Paramètres, reprise atomique et versions du format local |
| `src/artwork.rs` | Chargement, redimensionnement et éviction des pochettes |
| `src/platform/mod.rs`, `media.rs`, `tray.rs`, `instance.rs`, `secrets.rs` | Frontières Windows et ressources natives |
| `src/update.rs` | Vérification et préparation des mises à jour |
| `src/bin/audio-output-check.rs` | Contrôle sonore explicite, distinct du lancement UI |
| `ui/app.slint`, `theme.slint`, `player.slint`, `library.slint` | Présentation, tokens, lecteur, navigation et listes |
| `scripts/`, `installer/`, `.github/workflows/` | Mesures, installation et vérifications autorisées |

Créer ces fichiers avec les tâches qui en ont besoin. Ne pas générer une forêt
de modules vides. Scinder un fichier uniquement lorsque ses responsabilités le justifient.

## Tranche publique implémentée

`Client::search` retourne des `Track` ; `Client::resolve` produit un `ResolvedStream`
sans Debug ni sérialisation. `PlaybackCommand::OpenRemote(Track)` confie résolution
et décodage au worker audio, hors UI. Aucun bus générique de requêtes n'est nécessaire
pour ces deux opérations. La recherche et la lecture ont chacune un worker réseau,
une seule opération courante, et des jetons d'annulation partagés.

Métadonnées : HTTPS vers les deux hôtes YouTube fixes, redirections refusées,
4 Mio maximum, délai global de 15 s par requête. Audio : uniquement sous-domaines
DNS de googlevideo.com, HTTPS 443, trois redirections revérifiées maximum,
Content-Range et taille contrôlés. Premier bloc de 64 Kio, suivants de 256 Kio ;
aucun fichier audio complet chargé en mémoire ou écrit sur disque. Le décodage
alimente 16 blocs PCM de 1 024 trames, au plus huit canaux à 192 kHz. Le callback
audio ne fait ni réseau, ni décodage, ni attente du producteur ; il remplit une
sous-alimentation par du silence. Ces bornes de code ne sont pas des mesures mémoire.

Stop, remplacement de source et fermeture invalident les tâches précédentes.
Le worker de recherche affiche d'abord les métadonnées, puis reçoit et réduit
jusqu'à 32 pochettes. `image` décode hors UI avec bornes de dimensions/allocation ;
les pixels RGBA traversent la notification Slint, la création d'Image reste sur
le thread UI. Maximum 192×192×4 octets par pochette conservée (textures GPU à part).
Les hôtes image sont une liste exacte distincte de l'audio ; aucune redirection.

Une requête bloquée peut attendre son délai réseau avant de se terminer ;
l'annulation n'interrompt pas l'appel système immédiatement. Un échec nécessite
une reprise manuelle. Pas de compte, cookies, pagination, seek ou renouvellement
automatique des URL dans cette tranche. L'UI sombre suit `DESIGN.md` ; le lecteur
fixe pleine largeur reste indépendant du contenu de recherche.

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
