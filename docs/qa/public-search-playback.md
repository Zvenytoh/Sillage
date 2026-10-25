# Recherche publique et lecture progressive — 2026-10-25

## Périmètre livré en source

L'utilisateur ne souhaite pas fournir de fichier audio et demande de poursuivre.
La tranche relie recherche publique, résultats virtualisés et lecture directe.
Aucun lancement UI, son, test fonctionnel ou nouvel appel au service n'est déduit
de cette instruction. Les exceptions runtime précédentes restent historiques.

- Recherche après 200 ms d'inactivité ou validation explicite ; une requête
  courante, dernière attente conservée, réponses annulées ignorées.
- Jusqu'à 100 identifiants de piste uniques ; titres et artistes bornés.
- Résolution publique selon les paramètres éprouvés par la sonde M0 : WEB_REMIX,
  visitor_id WEB, player VISIONOS, AAC/M4A itag 140 direct.
- Choix d'un résultat → préparation hors UI → Play ; Pause, Stop, volume,
  durée/erreurs ; fichier local facultatif.
- Plages HTTP de 64 Kio puis 256 Kio, décodage sur producteur séparé et file PCM
  bornée. Aucune copie complète du morceau, aucun flux signé sérialisé ou affiché.

## Revue statique

HTTPS uniquement ; métadonnées limitées à 4 Mio, délai global 15 s par requête,
redirections de métadonnées refusées. Destinations audio validées à chaque saut,
trois redirections maximum ; réponse 206 et Content-Range cohérent obligatoires.
Pas de boucle de reprise sur 429, pas d'authentification ni de lecture de cookies.
Annulation vérifiée entre phases/lectures et avant installation de la source.
La fermeture rejoint les workers ; une requête système déjà engagée peut attendre
son délai réseau. Callback audio sans réseau, décodage ou attente explicite.

## Vérifications

Résultat exact des commandes et empreinte de l'exécutable :
`public-search-build-results-2026-10-25.json`.

Cas préparés et vérifiés par le compilateur : identifiants vidéo, champs inconnus,
déduplication, réponse vide/invalide, annulation partagée, destinations audio.
Aucun `cargo test` exécuté. La revue statique ne remplace pas ces tests.

## Travail restant

Validation intégrée réseau/UI/audio, panne et annulation en situation réelle,
navigation albums/artistes/playlists, pagination, pochettes, file, progression,
seek et renouvellement des flux expirés. Aucun résultat de consommation, latence,
rendu ou qualité sonore revendiqué. M0/M1 runtime et M2 restent ouverts.
