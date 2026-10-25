# Refonte UI — Spotify / YouTube Music

Demande explicite du 2026-10-25 : remplacer l'UI rejetée et suivre les codes de
Spotify et YouTube Music. Cette demande fait passer la refonte visuelle avant
la suite de la navigation catalogue. Stack et comportement audio conservés.

## Changements

- Remplacement complet crème/pin → noir/charbon/blanc, accent citron discret.
- Rail de navigation, recherche persistante, résultats denses avec pochettes,
  sélection visible et lecteur fixe sur toute la largeur.
- Boutons SVG cohérents, focus, clavier et accessibilité explicite.
- Suggestions de genres déclenchant une vraie recherche ; aucun faux album,
  aucune commande de file, seek ou compte sans implémentation.
- Métadonnées affichées immédiatement, puis jusqu'à 32 pochettes chargées sur
  le worker de recherche. Rejet des résultats périmés, entrée et décodage bornés.

## Références de composition

[Spotify Desktop](https://newsroom.spotify.com/2023-06-20/spotify-desktop-experience-redesign-your-library-now-playing-views-customize/)
et [présentation du Web Player](https://webplayer.byspotify.com/) : navigation,
artwork et séparation du contenu/lecteur. L'objectif est une interface familière,
pas une promesse de parité fonctionnelle ou la reproduction d'un écran 2026 précis.

## Revue et vérifications

Revue effectuée par l'agent courant, sans sous-agent. Documentation extraite du
code dans `DESIGN.md`. Compilation Slint/Rust, Clippy et formatage consignés dans
`dark-interface-build-results-2026-10-25.json` après leur exécution.

La première vérification a signalé une boucle de layout dans une largeur/padding
conditionnels ; remplacement par des contraintes de taille et padding constant.
Le lint suivant ne signale plus cette boucle.

Pas de tests exécutés, lecteur non lancé, pas de capture du rendu réel ni de
validation réseau des pochettes. Les contrôles 800×560, 1280×800, DPI 125/150/200%,
focus clavier, troncature et contraste des widgets rendus restent ouverts selon
AGENTS.md. Le build ne suffit pas pour déclarer le rendu impeccable.

## Limites conservées

Pas encore de progression/seek, file, albums, artistes, playlists ni compte.
Les pochettes au-delà des 32 premiers résultats restent sur l'icône neutre.
Un échec réseau arrête le lot de pochettes, sans cacher les résultats ni arrêter
l'audio. Le prochain terme saisi peut attendre la fin d'une requête d'image déjà
engagée (délai HTTP de 15 s maximum par requête). Les textures GPU sont distinctes
du budget de pixels CPU ; consommation et fluidité non mesurées.
