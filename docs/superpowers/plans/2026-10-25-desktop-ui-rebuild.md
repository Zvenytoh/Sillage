# Reconstruction de l'interface desktop

Demande explicite : Spotify Desktop comme référence principale, suppression du
vert citron de l'UI, travail substantiel plutôt qu'un changement de palette.
La capture utilisateur est une preuve du rendu rejeté ; les autorisations
restent statiques uniquement. Aucune promesse de perfection sans rendu vérifié.

1. Corriger la géométrie : chrome global en haut, bibliothèque utile à gauche,
   contenu central structuré, panneau de lecture à droite et transport en bas.
   Icônes centrées explicitement, largeurs contraintes, aucun séparateur traversant
   la pochette ou les commandes. Monochrome noir/charbon/blanc.
2. Construire accueil, recherche All/Songs et bibliothèque locale : cartes issues
   des vrais résultats, meilleur résultat, liste compacte et états de chargement,
   vide/erreur. Les raccourcis de découverte déclenchent de vraies recherches.
3. Relier les éléments manquants à un comportement réel : favoris locaux bornés
   enregistrés séparément, file de session, précédent/suivant, répétition,
   panneau du morceau/file, volume/muet et progression issue des trames audio.
   Le seek distant reste indisponible, avec indication explicite.
4. Vérifier statiquement les contrats, générations, bornes, écritures et clavier.
   Formatage, Clippy et build Release ; aucun test, lancement ou mesure runtime.
5. Documenter précisément ce qui existe, préparer un nouvel exécutable et mettre
   à jour le raccourci utilisateur sans écraser un binaire ouvert ni lancer l'app.

Revue et documentation effectuées par l'agent courant, sans délégation.
Les pages album/artiste/playlist distantes et la connexion restent des tâches
distinctes ; ne pas présenter de navigation factice pour simuler leur présence.
