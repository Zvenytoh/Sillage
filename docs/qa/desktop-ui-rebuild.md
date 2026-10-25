# Reconstruction desktop — 2026-10-25

## Autorité et méthode

L'utilisateur rejette la première refonte sombre et fournit sa capture 1920×1080.
Il exige une composition proche de Spotify Desktop et retire le vert/citron de
l'UI. Impeccable, ses références Operate/Craft floor, et la revue ciblée du code
guident cette reconstruction. Revue et documentation par l'agent courant, sans
sous-agent. La demande de refonte ne révoque pas le refus explicite de lancement.

Références primaires de composition :
[Spotify Desktop](https://newsroom.spotify.com/2023-06-20/spotify-desktop-experience-redesign-your-library-now-playing-views-customize/)
et [Web Player](https://webplayer.byspotify.com/). Ces références établissent
l'anatomie bibliothèque/contenu/lecture ; elles ne constituent pas une cible
pixel pour une version spécifique de Spotify en 2026.

## Défauts observés et changements de source

| Capture utilisateur | Changement |
|---|---|
| Rail presque vide, source statique | Bibliothèque persistante locale, sauvegarde/retrait de morceaux, volet réductible |
| Recherche étirée et liste pleine largeur | Navigation globale, champ borné, meilleur résultat + titres, filtre Songs |
| Aucun panneau du morceau | Volet 304 px, grande pochette, métadonnées, sauvegarde et état réels |
| Lecteur incomplet et icônes au bord haut | Conteneurs de centrage explicites, précédent/suivant, mélange, répétition, file, muet |
| Séparateur traversant le lecteur | Progression contenue dans la colonne centrale, temps écoulé et durée |
| Accent citron et volume rose | États blancs/gris et volume monochrome avec clavier/accessibilité |
| Pochettes manquantes ou lentes | Worker distinct, image proche de 320 px, échec isolé toléré, limites conservées |

## Comportements et données

La recherche alimente une file de session dédupliquée de 500 titres maximum.
Choix d'un élément de la file, précédent/suivant, passage naturel au suivant,
shuffle sans répétition immédiate et répétition du morceau courant sont câblés.
Clear queue conserve le morceau en cours ; la file n'est pas restaurée sur disque.
Les notifications de fin sont dédupliquées et les sources annulées ne déclenchent
pas de passage automatique au suivant.

La bibliothèque locale est `%LOCALAPPDATA%\Sillage\library.json`, version 1,
≤500 titres, ≤2 Mio. Pas de cookies, jetons, URL signées, pochettes ou compte.
Lecture/écriture sur un worker ; verrou Windows exclusif, fichier temporaire
CreateNew, synchronisation puis renommage. Une version inconnue/invalide désactive
l'écriture et conserve le fichier. Un fichier `.pending` existant n'est pas écrasé.
Les erreurs d'écriture sont visibles ; les changements récents restent en session.

La progression compte les échantillons PCM consommés par blocs de 1024 trames,
sans compter le silence ajouté lors d'une sous-alimentation. Notifications au
plus quatre fois par seconde ; aucune propriété de progression modifiée si la
fenêtre est cachée/minimisée. Il peut rester une avance liée au tampon de sortie,
non mesurée. Le seek est explicitement indisponible. Aucun timer de progression.

Pochettes : ≤32 résultats, ≤512 Kio reçus/image, décodage limité à 2048×2048 et
16 Mio, réduction ≤320×320 RGBA ; ≤24 couvertures retenues dans chacun des modèles
file/favoris. Dernier lot en attente seulement, annulation partagée. Trois workers
réseau au maximum : métadonnées, pochettes et flux audio ; un appel actif chacun.
Quatre erreurs consécutives arrêtent un lot, 429/cancellation arrêtent immédiatement.
Images GPU, mémoire de décodeur et données temporairement en transit restent séparées.

## Vérification réelle

Commandes, sorties et empreinte : `desktop-ui-build-results-2026-10-25.json`.
Formatage, compilation Slint/Rust, Clippy toutes cibles et build Release. Trois
cas préparés/typecheckés, non exécutés : silence de buffering sans avance de
position, conservation d'un fichier de récupération, version de bibliothèque
inconnue sans modification. Aucun `cargo test` ni test fonctionnel.

Revue des contrats UI/Rust, annulations, événements de fin, limites, sauvegardes
et géométrie écrite pour 1000/1280/1920 px. Cette inspection n'est pas un contrôle
des pixels : pas de lancement, nouvelle capture, validation du clavier, écoute,
panne réseau ou benchmark. Le verdict visuel reste **ouvert** ; aucune perfection
ni parité Spotify n'est revendiquée.

À vérifier après autorisation : accueil/recherche/favoris/file, volet étroit et
large, DPI 100/125/150/200%, textes longs, tabulation et raccourcis, progression
sur pause/buffering, fermeture pendant réseau/écriture, redémarrage et deux instances.
