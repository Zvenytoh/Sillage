# Commandes de fenêtre intégrées

Demande : supprimer la barre de titre Windows, garder les trois boutons dans
l'application, ordre Windows réduire → agrandir/restaurer → fermer.

Implémentation exclusivement Slint : fenêtre sans cadre, contrôles 46×32 px en
haut à droite, libellés accessibles en français, activation clavier, survol rouge
pour fermer. L'état agrandi/restauré est celui de la fenêtre, sans état dupliqué
ni timer. Fermeture par `root.close()`, donc même callback et sauvegarde existants.

Déplacement natif demandé par `WindowMoveArea` dans la marque et le bord haut
libre ; double-clic pour agrandir/restaurer. Bord de redimensionnement de 6 px
en mode restauré, nul en mode agrandi. En-tête du panneau droit décalé sous les
contrôles. Aucun changement de taille système, du bureau, de l'audio ou des
données utilisateur.

API vérifiée dans les sources locales de Slint 1.18.1 : `WindowItem`,
`WindowMoveArea` et `WinitWindowAdapter::start_window_move`. Aucun ajout de paquet.

Vérification native non effectuée : état restauré/agrandi, limites de l'écran et
barre des tâches, drag, double-clic, redimensionnement, clavier, réduction et
fermeture restent à exercer. Tests unitaires/fonctionnels non exécutés.
Les résultats de compilation et le nouvel artefact sont consignés dans
`integrated-window-build-results-2026-10-25.json`.

Formatage, Clippy toutes cibles sans avertissement Rust/Slint et compilation
Release --bins réussis, deux jobs à priorité BelowNormal. Préversion distincte
`dist/Sillage-frameless-preview.exe`, raccourci Bureau mis à jour puis relu.
Les anciens exécutables et fenêtres sont conservés.
