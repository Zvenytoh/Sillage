# Architecture cible

Le socle local M1, la recherche publique → lecture progressive et l'accueil
français sont implémentés. Le contrôle visuel natif de la nouvelle révision,
la reprise de file, les comptes et les mises à jour restent à faire.
La compilation ne valide ni le rendu ni le comportement audio.

## Fenêtre sans barre système

`ui/window-controls.slint` contient les trois contrôles intégrés. `AppWindow`
utilise `no-frame`, `resize-border-width`, `minimized`, `maximized` et `close()`
de Slint ; `WindowMoveArea` délègue le déplacement au backend Winit. Aucun code
Win32 unsafe, hook, polling ou dépendance supplémentaire ajouté. La fermeture
conserve le callback existant et la sauvegarde de session après `ui.run()`.

## Révision accueil français

`catalog.rs` remplace `search.rs` : un worker pour recherche/accueil, une requête
active et une attente remplaçable. La navigation conserve les modèles déjà reçus ;
une nouvelle demande annule la précédente. Cache normalisé `home-fr.json` ≤2 Mio,
fraîcheur six heures, actualisation explicite, ancien contenu conservé sur panne.
`Client::home` suit la playlist « Les 20 clips tendance France » retournée par
`FEmusic_charts`, puis les nouveaux clips de `FEmusic_explore` (24 maximum).
Aucun classement simulé, compte ou personnalisation supposée. 429 arrête le lot.

`history.rs` contient l'historique pur ; `session.rs` possède favoris, récents et
images, `view.rs` les conversions Slint. Qualification après 30 secondes PCM
consommées, ou moitié d'un morceau connu de moins de 60 secondes ; 200 récents
dédupliqués. Les notifications cachées alimentent l'historique sans rafraîchir
le compteur visible. Sauvegarde uniquement si modifié, sur qualification,
pause, changement et fermeture, jamais à chaque position. JSON v1 lu en favoris,
v2 écrit au prochain changement ; limites 500 favoris/200 récents/2 Mio.

`artwork.rs` partage une file de 64 demandes, courant avant visible, un fetch
actif. Même identifiant dans toutes les vues, sans annulation lors d'une recherche.
Référence publique validée ou miniature i.ytimg.com du même videoId ; aucun
homonyme. Réception ≤512 Kio, décodage ≤2048²/16 Mio, réduction 384 px (visible)
ou 512 px (courant). Cache encodé ≤128 Mio/1024 fichiers ; buffers conservés
≤32 Mio, références évincées aussi dans les vues. Textures GPU non mesurées.
`cache.rs` fournit lectures bornées et écritures temporaires atomiques.

Le volume Slint est l'état immédiat de l'interaction. Le worker conserve la
dernière valeur atomique avec au plus une notification en attente ; les anciens
accusés `VolumeChanged` ne repositionnent plus la poignée. Cette correction
statique reste à vérifier par glissement/clavier dans le lecteur natif.

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
| `src/catalog.rs` | Worker accueil/recherche, dernière demande conservée, cache normalisé |
| `src/artwork.rs` | Worker d'images distinct, dernier lot seulement, réception/décodage bornés |
| `src/session.rs` | File locale, historique, passage suivant/précédent, mélange et favoris UI |
| `src/library.rs` | Bibliothèque locale versionnée, verrou Windows et écritures atomiques hors UI |
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
Le worker de recherche affiche les métadonnées puis confie jusqu'à 32 pochettes
à un worker distinct, sans attendre ses requêtes. `image` décode hors UI avec bornes de dimensions/allocation ;
les pixels RGBA traversent la notification Slint, la création d'Image reste sur
le thread UI. Maximum 320×320×4 octets par pochette conservée (textures GPU à part).
Les hôtes image sont une liste exacte distincte de l'audio ; aucune redirection.

Une requête bloquée peut attendre son délai réseau avant de se terminer ;
l'annulation n'interrompt pas l'appel système immédiatement. Un échec nécessite
une reprise manuelle. Pas de compte, cookies, pagination, seek ou renouvellement
automatique des URL dans cette tranche. L'UI sombre suit `DESIGN.md` ; le lecteur
fixe pleine largeur reste indépendant du contenu de recherche.

La reconstruction desktop ajoute `session.rs` (file de 500 titres et historique
de 128 entrées) et `library.rs` (favoris locaux, pas une bibliothèque du compte).
Les événements de fin sont dédupliqués avant l'avance automatique. La position
est comptée sur les blocs PCM réellement consommés ; le silence de buffering
n'avance pas le morceau. Notifications ≤4/s, position UI modifiée uniquement si
la fenêtre est visible/non minimisée ; aucun timer de progression.

La bibliothèque JSON version 1 est limitée à 2 Mio/500 titres. Un worker possède
le verrou de partage exclusif Windows, charge avant d'autoriser l'écriture,
écrit un fichier CreateNew, synchronise puis renomme. Formats inconnus, fichiers
invalides et récupération déjà présente sont conservés. Le verrou OS est libéré
à la sortie, y compris anormale. Pas de sérialisation de pochettes ou d'URL de flux.

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
