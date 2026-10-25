# Géométrie du lecteur et du volume

Retour utilisateur : le volume fonctionne, mais sa représentation et les
alignements du lecteur sont incorrects. Capture native fournie, sans prise de
contrôle du bureau.

Cause du remplissage : le rectangle violet possédait une largeur calculée,
mais aucun x. Slint le centrait automatiquement dans sa piste. À 25 %, le
segment apparaissait au milieu au lieu de partir de la gauche. Le remplissage
de progression avait la même omission. Les textes de durée n'avaient pas de
hauteur explicite ; les groupes gauche et droit étaient centrés à y=48 plutôt
qu'à y=44 dans la barre de 88 px.

Correction ciblée : x=0/y=0 pour les deux remplissages, fraction de volume
commune et bornée pour le segment et la poignée ; pochette 56 px à y=16,
commandes de droite centrées à y=44, transport 232 px exactement centré,
progression plafonnée à 620 px, cases de durée de 20 px de haut et jauge 96 px.
Commandes de volume, accessibilité clavier et moteur audio conservés.

La correction précédente des confirmations audio périmées traitait une autre
piste statique ; elle ne prouvait pas la disparition de ce défaut géométrique.
La présente capture permet de distinguer les deux. Aucune manipulation du
curseur ni écoute n'est revendiquée ici. Le rendu corrigé reste à observer.

Vérifications et artefact : `playback-bar-build-results-2026-10-25.json`.
Tests unitaires/fonctionnels non exécutés ; aucune mesure de performance.

Formatage, Clippy toutes cibles sans avertissement Rust/Slint et Release --bins
réussis après modification. Nouvelle préversion `dist/Sillage-player-preview.exe` ;
raccourci du Bureau mis à jour puis relu. Ancien exécutable conservé, aucune
fenêtre lancée ni fermée. Deux jobs de compilation, priorité BelowNormal.
