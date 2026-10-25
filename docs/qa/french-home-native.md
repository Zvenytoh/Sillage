# Accueil français violet — état de vérification

Date : 2026-10-25. Branche : `codex/french-home-implementation`.
Direction approuvée : A Équilibré, violet `#AD91FF`.

## Livré en source

- Accueil français : récents persistants, tendances France, nouveaux clips,
  recherche globale, favoris et panneau morceau/file. Ouverture locale retirée
  de l'interface ; outils de diagnostic conservés.
- Données issues du service, sans titres ou pochettes d'exemple des maquettes.
  Cache d'accueil daté, six heures, actualisation explicite et conservation
  des anciennes données sur panne.
- Historique qualifié par progression PCM consommée, indépendant de la visibilité
  de la fenêtre ; migration JSON v1 vers v2 au prochain enregistrement, avec
  verrou exclusif et conservation des formats inconnus/fichiers de récupération.
- Pochettes partagées entre vues et persistantes sur disque. URL validée, repli
  sur le même videoId, file et mémoire bornées. Miniatures player réutilisées
  lors de la résolution sans nouvel appel de métadonnées.
- Volume : valeur UI immédiate, commandes coalescées et confirmations anciennes
  ignorées. La piste et la poignée utilisent le même calcul, clic/drag/clavier
  conservent les bornes 0–100. Remplissage violet.

## Catalogue réellement interrogé

La sonde `examples/catalog-probe.rs --metadata-only` utilise le client Rust
de production et uniquement les métadonnées publiques. Elle ne crée aucune
fenêtre et n'ouvre aucun périphérique audio.

Le service a retourné la collection « Les 20 clips tendance France » depuis
`FEmusic_charts`. L'identifiant de playlist retourné est ensuite parcouru ;
aucun identifiant de classement n'est figé dans le code. Découverte : section
« Nouveaux clips musicaux » de `FEmusic_explore`, limitée à 24 pistes.

Première sonde : 20 tendances et 23 nouveautés, un hôte de miniature légitime
`yt3.googleusercontent.com` rejeté par l'ancienne liste exacte. Correction de
cette liste et normalisation des images non autorisées sans perdre le morceau.
Dernière sonde : sortie 0, 20 tendances, 21 nouveautés, 41 références de pochette,
toutes les pistes valides, aucun compte ni audio. Les contenus publics varient.
Cette sonde ne prouve pas le téléchargement ni l'affichage de chaque image.

## Revue statique

Revue effectuée par l'agent courant, sans sous-agent : annulation/générations,
bornes réseau, conversion des modèles, conservation des données, références
d'images évincées des vues, qualification d'historique et fermeture des workers.
Lecture de bibliothèque obligatoire avant toute sauvegarde ; les écoutes
survenues pendant son chargement sont fusionnées. Aucun polling ajouté.

Adaptations du plan : `HomePage` contient directement les deux collections
actuellement utilisées ; pas de hiérarchie de sections spéculative. Un seul
jeton d'opération catalogue et une demande remplaçable ; les données déjà
reçues restent indépendantes. Deux priorités d'images suffisent. Résolutions
bornées 384/512 px, adaptation DPI fine encore à inspecter.

Cas préparés et compilés, sans exécution : historique/seuil court/dédoublonnage,
migration v1, préservation de récupération/version inconnue, route du classement,
objets non jouables et rejet d'hôtes. Les tests existants couvrent aussi les
volumes NaN/hors bornes. Aucune réussite de test n'est revendiquée.

## Contrôle natif restant

Le contrôle de l'ancienne fenêtre pour diagnostiquer le volume a été interrompu
par Échap avant toute observation exploitable. Aucune reproduction visuelle de
la panne n'est établie. La nouvelle révision n'a pas été lancée ni capturée.
L'utilisateur joue sur son PC et demande de continuer : contrôle du bureau et
ouverture de fenêtres reportés pour respecter son activité.

À vérifier quand il sera disponible : accueil vide/rempli, recherche/favoris/
récents/file, vraie résolution des images, volume à 0/25/50/75/100, mute et
clavier, navigation/focus, scroll et titres longs à 1000×700, 1360×900 et
1920×1080, DPI disponibles sans modifier les réglages système.

L'exécutable est une préversion compilée, sans verdict visuel « parfait ».
Ni écoute, ni DRM, ni consommation CPU/RAM/GPU, ni performance validés.
Commandes et empreinte du binaire : `french-home-build-results-2026-10-25.json`.

Préversion distincte : `dist/Sillage-fr-violet-preview.exe`, 18 046 976 octets.
Raccourci `Sillage.lnk` du Bureau mis à jour puis relu. Ancien exécutable préservé,
aucune fenêtre fermée ni lancée. Fermer l'ancienne fenêtre avant de lancer la
nouvelle : le verrou exclusif de bibliothèque empêche deux instances d'écrire.

Vérifications finales exécutées : `cargo fmt --all -- --check`,
`cargo clippy --locked --all-targets --target-dir .tools/app-target -- -D warnings`,
`cargo build --locked --release --bins --target-dir .tools/app-target`.
Sorties 0, aucun avertissement Rust/Slint. Deux jobs, priorité BelowNormal.

## Retour utilisateur — géométrie de l'accueil

La capture native fournie ensuite par l'utilisateur montre le vrai accueil à
1920 px : titre « À découvrir » séparé des cartes par un grand vide, couvertures
empiétant sur les titres, deux colonnes de tendances de largeurs variables,
placeholder de recherche centré dans l'espace restant. Ce retour est une preuve
du rendu précédent, pas une capture de la correction.

Cause confirmée par inspection du compilateur Slint 1.18.1
(`passes/default_geometry.rs`) : un élément dimensionné hors layout reçoit un
centrage automatique lorsque x/y ne sont pas définis. La couverture MusicCard
était ainsi centrée verticalement dans la carte alors que les légendes étaient
positionnées depuis son bord haut. Le layout de la page et les hauteurs de
titres n'étaient pas suffisamment contraints ; les rangées de tendances
résolvaient chacune leurs largeurs indépendamment.

Correction ciblée : ancrages x/y explicites dans MusicCard, hauteur de carte
calculée sur l'image et ses légendes, viewport de l'accueil lié à la hauteur
préférée de son contenu, titres/rangées bornés, colonnes égales. Les nouveaux
clips ont un cadre 16:9 et les récents gardent leur cadre carré. Quatre, six
ou huit cartes selon la largeur utile ; placeholder ancré à gauche.

Aucune commande du bureau, ouverture, écoute ou mesure effectuée pour cette
correction. La nouvelle géométrie attend encore une vérification native.

Formatage, Clippy toutes cibles sans avertissement et build Release --bins
réexécutés avec succès après cette correction. Préversion distincte
`dist/Sillage-fr-violet-layout-preview.exe` ; raccourci Bureau mis à jour sans
fermer ni remplacer l'exécutable précédent. Résultats et empreinte :
`french-home-layout-build-results-2026-10-25.json`.
