# Sillage — conception française et accueil musical

Statut : **brouillon de brainstorming, non approuvé**. Aucune modification produit
autorisée par ce document seul. L'utilisateur précise vouloir un accueil de type
Apple Music/Spotify et accepte la comparaison de maquettes. Aucun choix visuel
ni approbation de la conception complète n'a encore été reçu.

## Demande confirmée

Application en français, pochettes des vrais morceaux, accueil avec tendances et
écoutes récentes, suppression de « Open local file », correction du volume et des
incohérences visuelles. La capture TubeFast fournie fixe une référence concrète :
navigation latérale, recherche dans l'en-tête du contenu, sections musicales et
panneau du morceau à droite. Chercher une couleur de mise en valeur distincte de
son orange et de l'ancien citron Sillage. Le natif et les budgets restent requis.

La dernière demande autorise explicitement l'ouverture et le contrôle visuel de
l'application. Cela remplace le refus antérieur pour ce périmètre. Aucun accord
de lecture audible, de tests unitaires ou d'accès au compte n'en découle.

## Décisions proposées par l'agent, à valider

- Accueil personnel d'abord : reprendre ses écoutes, puis découvrir les tendances.
- France comme pays initial des tendances ; interface française, catalogue mondial.
- Accent pervenche `#9CA9FF`, surfaces noires/charbon, contraste conservé.
- Historique local, dédupliqué, limité à 200 titres ; aucune synchronisation de compte.
- Une écoute rejoint l'historique après 30 s de PCM consommé, ou après la moitié
  d'un titre de moins de 60 s. Une ouverture ratée, recherche ou préparation seule
  ne compte pas comme écoute. Une reprise réordonne l'entrée existante.
- Cache de classement valable six heures, rafraîchissement explicite possible.

Ces valeurs sont des choix proposés, pas des accords de l'utilisateur ni des mesures.

## Direction visuelle

La capture devient une référence de composition ; il ne faut pas reproduire ses
identifiants, son orange ou ses données de compte. Trois directions ont été
considérées : ambre doux (trop proche de la chaleur de la référence), rose poudré
(distinct mais plus expressif), pervenche (retenu pour une lecture nocturne et
une séparation claire des deux accents rejetés). La comparaison visuelle est disponible ci-dessous ; les choix restent ouverts.

## Comparaison visuelle présentée

Trois compositions interactives ont été ouvertes dans le navigateur intégré :
A Équilibré (récents puis tendances), B Éditorial (sélection à la une puis récents),
C Immédiat (raccourcis compacts puis tendances). Accents indépendants : pervenche
`#9CA9FF`, rose `#E89CB8`, ambre `#E5BB7A`.
Recommandation de l'agent : A/pervenche, à confirmer par l'utilisateur.

Maquettes locales non livrées dans l'app : `.tools/french-home-mocks/content/`.
Pochettes réelles du catalogue public Apple/iTunes, données d'écoute et classement
clairement illustratifs, aucun son. La future implémentation devra résoudre les
images du catalogue YouTube Music par identifiant ; aucune dépendance Apple
ajoutée au produit. Captures A/B/C dans `.tools/french-home-mocks/`.

Inspection visuelle du prototype à 1360×950 et au viewport intégré par défaut ;
24 images affichées, aucune manquante lors du contrôle A. Volume de la maquette
contrôlé au clavier à 100 %, 0 %, puis restauration à 55 %. Ces observations
ne valident ni le volume natif ni le rendu du lecteur Rust. Serveur de comparaison
temporaire sur loopback, télémétrie désactivée ; aucune modification produit.

Palette proposée : fond `#090A0D`, surfaces `#14151B`, survol `#242630`, texte
`#F4F4F7`, secondaire `#ADB0BD`, accent `#9CA9FF`, texte sur accent `#111326`.
L'accent porte lecture principale, navigation active, sélection et progression.
Les titres ordinaires restent blancs ; les pochettes portent l'essentiel de la
couleur. Pas de halo ni de flou permanent. Un dégradé statique discret dans
l'en-tête peut reprendre une teinte de la pochette, sans changer la couleur des
commandes ni réduire la lisibilité. Les contrastes doivent être calculés puis
vérifiés sur le rendu ; les hexadécimaux seuls ne constituent pas une validation.

Segoe UI, hiérarchie resserrée : corps 14–16 px, secondaire 12–13 px, sections
22–24 px. Titres de cartes sur deux lignes maximum, artistes sur une ligne.
Le panneau du morceau accepte davantage de lignes ; le titre complet reste
accessible. Aucun titre brut ne doit imposer sa largeur à une colonne.

## Composition et navigation

Rail de 232–256 px : identité compacte, **Accueil**, **Titres favoris**,
**Écoutés récemment**, puis bibliothèque. Retirer l'ouverture locale de la
navigation et des actions visibles ; ne supprimer aucun fichier utilisateur.
Ne pas ajouter de bouton de connexion fictif ou de bibliothèque distante vide.

Zone centrale : en-tête avec retour/avance réels et recherche de 420–520 px,
puis contenu défilant. Historique de navigation borné et état de recherche
préservé au retour. La musique continue pendant la navigation.

Panneau droit de 288–320 px : grande pochette carrée, titre, artiste, favori,
**À suivre** avec les prochains titres réels. Sa fermeture rend l'espace au
contenu central. Sur petite fenêtre, l'ouvrir comme un volet explicite ; ne pas
recouvrir silencieusement les commandes de la liste.

Lecteur fixe de 80–96 px : pochette et métadonnées à gauche, transport centré,
file et volume à droite. Même axe de centrage pour toutes les icônes. Les groupes
ont des largeurs minimales mesurées ; aucune ligne ne traverse les pochettes.
La progression est distincte du volume. Tant que le seek n'existe pas, la position
reste une information sans poignée suggérant un glissement fonctionnel.

## Accueil

1. **Reprendre l'écoute** : les 6–12 dernières écoutes réelles, pochettes et accès
   direct. Éventuelle reprise à la position précédente uniquement si le moteur
   permet effectivement le seek ; sinon redémarrer et le rendre explicite.
2. **Tendances en France** : source de classement identifiable, rang et fraîcheur
   si disponibles. Prévoir une liste compacte sur deux colonnes ou un rail de
   cartes selon la largeur, sans hauteur vide artificielle.
3. **À découvrir** : contenu public réellement reçu du service. Suggestions liées
   au dernier morceau seulement si l'endpoint correspondant fonctionne ; ne pas
   appeler une recherche par mots-clés une recommandation personnalisée.

Premier lancement : tendances et découverte d'abord ; la section récente n'affiche
pas de fausses cartes. Une phrase discrète explique où apparaîtront les écoutes.
Hors ligne : historique et images locales restent consultables ; classement en
cache marqué avec sa date. Échec distant : section concernée en erreur avec
**Réessayer**, pas une page entière blanche. Chargement : conserver le cache et
des emplacements aux bonnes dimensions pour éviter les sauts de mise en page.

La recherche reçoit ses propres états et ses propres résultats. Elle ne sert plus
de magasin global improvisé pour alimenter l'accueil ou l'historique.

## Pochettes et données musicales

Privilégier les références d'image fournies par le catalogue pour le même
identifiant de morceau. Conserver l'identifiant et une référence publique d'image
validée, indépendamment des URL audio signées. Choisir la résolution correspondant
à la taille affichée et au DPI, avec une limite supérieure explicite.

Une miniature vidéo 16:9 ne doit pas être recadrée arbitrairement en une fausse
pochette carrée. Détecter les marges/ratios usuels et conserver l'image lorsque
le recadrage perdrait l'information ; une vraie couverture d'album est prioritaire.
Ne pas générer de fausses couvertures et ne pas rechercher des images au nom du
titre sur un moteur généraliste : homonymes et mauvais artistes sont possibles.

Le cache doit servir recherche, favoris, historique, file et panneau du morceau.
Prévoir cache disque borné et éviction, cache décodé CPU borné, textures GPU
comptées à part. Le premier écran visible est prioritaire ; les images hors écran
ne doivent pas retarder les images utiles. La fin d'une recherche ne doit pas
invalider les images encore nécessaires au morceau ou à l'historique.

Image manquante : repli neutre cohérent, tentative bornée ou autre URL du même
objet si le service la fournit. Ne pas laisser la note musicale comme état final
quand une image valide est connue. Erreurs individuelles isolées, 429 respecté,
pas de boucle permanente. Hôtes, tailles et délais restent contrôlés.

## Français

Traduire navigation, états, erreurs, fenêtres secondaires, descriptions de
commandes et noms accessibles. Exemples : **Rechercher un titre ou un artiste**,
**Écouter**, **Mettre en pause**, **Lecture aléatoire**, **Répéter ce titre**,
**Ajouter à la file**, **Retirer des favoris**, **Couper le son**.
Les titres/artistes gardent leur langue d'origine. Localiser les réponses du
service quand possible sans exclure la musique internationale. Le README reste
en anglais, conformément à la demande antérieure ; cette tâche concerne l'app.

## Volume : diagnostic avant correction

Le défaut doit être reproduit dans l'app ouverte, sans musique. Le code actuel
envoie les mouvements au worker puis réapplique les confirmations du worker à la
valeur liée au curseur : un retour tardif pendant un glissement est une piste de
saut, pas une cause démontrée. Vérifier aussi largeur réelle, zones de capture,
coordonnées, bornes et chevauchement des autres boutons.

Contrat attendu : une seule géométrie pour rail/poignée/pointeur ; 0/100 atteignables,
glissement hors du rail capturé et borné ; valeur affichée immédiate et stable.
Coalescer les commandes pendant l'interaction si nécessaire, sans timer permanent ;
la confirmation d'une ancienne valeur ne doit pas ramener la poignée en arrière.
Muet restaure le dernier volume non nul. Flèches, Home/End, focus et noms accessibles
doivent être cohérents. Une désactivation temporaire ne fait pas perdre la valeur.

## Faisabilité constatée et limites

Le code de Sillage expose recherche, résolution et images ; il n'expose pas encore
de navigation de catalogue, de classement ou d'historique d'écoute persistant.
Le TubeFast local inspecté possède des chemins `FEmusic_home` et `FEmusic_explore` :
cela fournit une piste technique, pas une preuve de disponibilité pour Sillage.
Les fonctionnalités de compte et les recommandations authentifiées restent séparées.

Source primaire : [YouTube Charts & Insights](https://support.google.com/youtubemusic/answer/9014376?hl=en).
YouTube distingue tendances et classements quotidiens/hebdomadaires. Le choix du
jeu de données et sa récupération doivent être vérifiés avant d'étiqueter une
section « Tendances ». Aucun classement actuel n'a été récupéré pendant ce cadrage.

## Critères de réception proposés

- Tous les textes visibles et accessibles sont en français, hors noms propres.
- Accueil distinct de la recherche ; récents fondés sur l'écoute et restaurés.
- Source et fraîcheur des tendances identifiées ; états hors ligne/erreur honnêtes.
- Pochettes cohérentes entre vues et après redémarrage ; réseau/mémoire bornés.
- Aucune entrée d'ouverture locale ; navigation et boutons correspondent à une action réelle.
- Volume contrôlé visuellement à 0/25/50/75/100, glissement et clavier vérifiés.
- Inspection de l'app réelle à 1000/1360/1920 px et DPI disponibles ; captures
  regroupées des états utiles, corrections puis nouvelle inspection après changement.
- Vérifier troncature, alignements, espaces, scroll, focus et lecture des erreurs.
- Formatage, Clippy et build restent distincts du verdict visuel. Tests unitaires,
  son et performances ne sont revendiqués que s'ils ont été autorisés et exécutés.

## Ordre de travail après approbation

Plan d'implémentation : modèles accueil/historique/images et contrats, source
distante vérifiée, cache partagé, localisation, composition, diagnostic volume,
puis vérification visuelle réelle. Livrer une version cohérente et contrôler
le raccourci du Bureau. Ne pas répartir une mauvaise maquette sur plusieurs builds
en annonçant successivement une finition non vérifiée.
