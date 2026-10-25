# Sidebar, active buttons and square artwork

Demande reçue avec capture native : retirer « À propos » et « Votre musique,
sur ce PC », supprimer le glyphe coloré du libellé Sillage, neutraliser la
surbrillance violette des bascules et rendre les vignettes musicales carrées.

- Barre latérale : seul le texte blanc Sillage reste à côté des commandes.
  Les deux libellés et le bouton À propos sont supprimés.
- Le composant AboutSlint est retiré du binaire. La licence Slint 1.18.1 autorise
  l'attribution par badge sur une page publique ; le badge officiel est ajouté
  au README GitHub. La fermeture appelle désormais la fermeture réelle de la
  fenêtre et permet au code de sortie existant de sauvegarder l'état.
- Lecture aléatoire/répétition gardent un état actif lisible par une icône blanche
  et un point neutre ; aucune teinte violette ne reste sur ces deux boutons.
- Le parseur choisit parmi les URL autorisées celle dont les dimensions sont
  le plus proches du carré, puis de 512 px. Titres en lignes, favoris, récents
  et résultat principal utilisent un cadre 1:1 rempli ; les vidéos de découverte
  gardent leur ratio 16:9. Les anciens fichiers sont conservés, avec de nouveaux
  noms de cache `home-fr-square-v2.json` et `artwork-v2` pour ne pas réutiliser
  les métadonnées ou pixels persistés avec l'ancien choix de vignette.

Aucune donnée utilisateur n'est effacée. Le badge officiel Slint n'est pas placé
dans l'application et ne rajoute aucune couleur à sa barre latérale.

Formatage, Clippy toutes cibles et build Release : voir
`sidebar-squares-build-results-2026-10-25.json`.
Tests non exécutés ; aucune fenêtre ouverte pour vérifier le rendu ou les clics.
