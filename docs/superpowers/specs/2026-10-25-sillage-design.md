# Sillage — design du client natif

Date : 2026-10-25. Statut : proposition complète à relire avant implémentation.

## Intention et décisions

L'utilisateur veut un vrai client YouTube Music, sans charger le site dans une
WebView : démarrage rapide, interface réactive, consommation minimale et rendu
soigné. Il a choisi le nom Sillage et un nouveau dépôt GitHub distinct d'UltraMusic.
Il demande AGENTS.md, Graphify, Ponytail, Superpowers, Caveman et une roadmap entière.

La recommandation technique retenue est Rust + Slint, Symphonia + rodio et les API
Windows. C'est une décision d'ingénierie pour ce besoin, pas la preuve d'une
supériorité universelle ou d'une consommation moindre qu'egui.

Choix de cadrage faits par l'agent : Windows 11 x64 en V1 ; interface personnalisée,
un seul compte actif, pas de base SQL initiale, pas de fork de TubeFast. Ils doivent
être revus avec le document ; ils n'ont pas été expressément confirmés séparément.

## Contraintes globales

- Cible initiale : Windows 11 x64.
- Stack : Rust + Slint ; Symphonia + rodio ; API Windows via windows-rs.
- Aucun navigateur embarqué ni WebView dans l'interface ou la lecture audio.
- Aucun sous-agent sans demande explicite de l'utilisateur.
- Aucun test unitaire ou fonctionnel sans demande explicite de l'utilisateur.
- Aucun lancement du lecteur ni contrôle du bureau sans réautorisation explicite.
- Aucun changement des données, identifiants ou fichiers d'UltraMusic.
- Aucun secret ni URL de flux signée dans Git, les logs ou les rapports.
- Aucun contournement de DRM ni promesse de qualité ou de fonctionnalité non vérifiée.

## Expérience attendue

L'application ouvre immédiatement sa structure locale ; le réseau ne bloque pas
la première interaction. La navigation conserve le lecteur et sa file. Recherche,
albums, artistes et playlists utilisent des listes progressives et virtualisées.
Une requête ancienne ne peut pas remplacer la recherche courante.

Le lecteur expose lecture/pause, précédent/suivant, volume, position, répétition
et mélange. La file est modifiable et restaurée au lancement, en pause par défaut.
Les commandes clavier et système suivent le même état que l'interface.

Le rendu privilégie typographie lisible, bonnes pochettes, espacement cohérent,
focus visible et transitions courtes. Pas de visualiseur permanent, fond vidéo
ou animation décorative qui tourne pendant l'écoute. Le thème sombre est initial ;
thème clair, contraste élevé et mouvement réduit sont prévus avant la V1 publique.
Les vues s'adaptent aux petits écrans, au zoom Windows et aux titres longs.

Les erreurs réseau laissent la navigation utilisable. Une piste indisponible
affiche la raison et une action de reprise ; elle ne déclenche pas une boucle
de retries. Aucun basculement silencieux vers un navigateur pour lire l'audio.

## Périmètre par jalon

| Jalon | Livrable utilisable | Condition de sortie |
|---|---|---|
| M0 | Dossier de faisabilité du service | Recherche, flux, formats, auth et limites établis ou blocage explicite |
| M1 | Lecteur local natif | Fichier audio autorisé, UI Slint, file et état cohérents |
| M2 | Alpha publique sans compte | Recherche, navigation, streaming, reprise locale, commandes Windows |
| M3 | Bêta avec compte | Session protégée et données personnelles réellement accessibles |
| M4 | Candidate V1 | Finition, accessibilité, mesures, installation et mises à jour vérifiées |
| V1 | Release initiale | Matrice de capacités et vérifications réelles publiée |
| Après V1 | Extensions choisies | Besoin utilisateur et capacité du service démontrés |

Les tâches d'accès au compte peuvent bloquer M3 sans empêcher M1. Une absence
de connexion viable doit être annoncée ; une alpha déconnectée n'est pas une
V1 avec compte. Aucun jalon ne donne un calendrier avant M0 et M1.

## Stack et responsabilité des choix

Slint est compilé avec Rust, via Winit et FemtoVG/OpenGL au départ. Ce moteur
limite les composants ; il n'est pas qualifié de plus rapide sans mesure.
Un changement de moteur nécessite une anomalie ou un gain mesuré sur le même écran.

`ureq` traite HTTPS dans des workers bornés ; `serde`/`serde_json` convertissent
les données vers des types métier. Paramètres et reprise utilisent un format JSON
versionné, borné et remplacé atomiquement. Les pochettes ont un cache mémoire/disque.
Pas de Tokio, SQLite, serveur local ou moteur de plugins par anticipation.

Symphonia et rodio traitent les formats réellement obtenus ; activer seulement
les décodeurs nécessaires. La version Rust minimale et les versions exactes sont
fixées au premier squelette depuis les exigences des versions stables retenues,
puis inscrites dans `Cargo.toml`, `Cargo.lock` et la CI. Aucun numéro inventé ici.

## Service, compte et qualité

L'API interne utilisée par des clients comme TubeFast peut évoluer sans préavis.
La tâche 0 doit examiner son implémentation, sa licence, les clients du service,
les flux signés et les dépendances éventuelles au lecteur JavaScript. Ne pas
supposer que copier un fichier de paramètres suffit à garantir la lecture.

Commencer par les contenus publics accessibles. Pour le compte, évaluer une
connexion explicite via navigateur externe avec contexte dédié et session
transférée de façon contrôlée. Ne pas prétendre qu'un OAuth Google standard
donne automatiquement accès à la bibliothèque et aux flux YouTube Music.
Pas de collecte du mot de passe, de lecture implicite du profil navigateur
principal ou de port de débogage laissé ouvert. Tout navigateur externe est
temporaire, déclenché par l'utilisateur et distinct du lecteur.

La faisabilité doit fixer le mécanisme concret avant la tâche 7. S'il exige
un composant incompatible avec les contraintes, documenter le blocage et faire
revoir le périmètre ; ne pas introduire ce composant en cachette.

La qualité affichée correspond au codec et débit observés. Le support Premium,
des formats supérieurs et des modifications distantes des likes/playlists est
conditionné aux capacités établies. Pas de téléchargement hors ligne de contenus
protégés, de vidéo, de Cast ou de promesse de parité complète dans la V1.

## Budgets initiaux proposés

Ces valeurs sont des cibles de conception. Les mesures nécessitent une autorisation
runtime ; aucun résultat n'a été obtenu à ce jour.

| Indicateur | Cible et protocole |
|---|---|
| Première interaction locale | P95 <= 500 ms à chaud et <= 1 500 ms à froid, 20 lancements chacun |
| Retour visuel d'une commande locale | P95 <= 50 ms, hors réseau et acquisition du périphérique audio |
| Fluidité du défilement | P95 des images <= 16,7 ms sur écran 60 Hz, 1 000 pistes |
| Mémoire privée du processus | <= 150 Mio après 30 min de lecture avec cache chaud ; autres métriques consignées séparément |
| CPU fenêtre masquée en lecture | Moyenne <= 1 % de la capacité CPU totale pendant 10 min ; processeur documenté |
| CPU fenêtre masquée en pause | Moyenne <= 0,1 % de la capacité CPU totale pendant 5 min |
| Rendu masqué | Aucun timer de repaint UI ; aucun rendu demandé par la progression |
| Cache pochettes mémoire | <= 32 Mio de pixels décodés ; textures GPU comptées séparément |
| Cache pochettes disque | <= 256 Mio, éviction des éléments les moins récemment utilisés |
| Tampon audio compressé | <= 8 Mio ; ne croît pas avec la durée du morceau |
| Réseau service | 2 requêtes métier simultanées ; 2 téléchargements de pochettes simultanés |
| Recherche | Déclenchement après 200 ms de frappe inactive ; résultats périmés ignorés |
| Transport | Métadonnées <= 4 Mio ; image encodée <= 8 Mio et <= 4 mégapixels ; délai lecture réseau 15 s |
| Reprise locale | Fichier <= 2 Mio ; <= 1 000 pistes restaurées ; fichier invalide conservé pour récupération |

Mesurer aussi le GPU, les réveils CPU, le working set et la mémoire GPU ; ne pas
additionner des métriques incompatibles. L'énergie est comparée avec le même
codec, débit, périphérique, volume et scénario. Un chiffre CPU isolé ne prouve
pas un gain d'autonomie. Le temps avant le premier son est mesuré séparément
de l'ouverture de fenêtre et ventilé entre réseau, décodage et sortie audio.

## Sécurité, résilience et distribution

Les listes d'hôtes distinguent service, CDN audio, pochettes et mises à jour.
L'audit M0 doit établir les domaines et schémas acceptés, y compris les redirections.
Chaque validation se refait après redirection. Les entrées distantes sont bornées.
Les sessions sont chiffrées avec DPAPI et supprimées à la déconnexion.

L'écriture de reprise utilise un fichier temporaire sur le même volume et un
remplacement atomique ; un échec garde l'ancienne version. Aucun cookie UltraMusic
n'est importé. L'installateur utilise un AppId et des chemins Sillage propres.

Les mises à jour viennent de ce dépôt, vérifient taille et digest attendus avant
exécution. Un digest fourni par le même canal apporte l'intégrité de transport,
pas une authentification indépendante : protéger les releases, prévoir la signature
de code et ne jamais qualifier le système d'infaillible. Installation explicite,
profil conservé, restauration limitée documentée.

## Travail assisté et validation

`AGENTS.md` fixe le comportement effectif. Skills présents et chargés à la demande.
Graphify utilise un graphe local quand il existe ; pas de génération sur un dépôt
vide, de watcher ou de rebuild systématique. RTK et recherches bornées évitent
les gros dumps. L'économie de tokens n'est pas une raison de réduire la preuve.

Les cas de tests sont définis dans la roadmap, mais les exécutions sont opt-in.
Sans autorisation UI/runtime, les jalons correspondants restent en attente de
validation ; les builds et revues statiques ne les remplacent pas.

## Suite après V1

Évaluer, dans cet ordre : parité compte et synchronisation si non disponibles,
qualité audio supérieure réellement accessible, paroles avec attribution et
confidentialité explicites, mini-lecteur, puis ARM64 et autres plateformes.
Discord, multi-comptes, égaliseur, Cast et plugins restent hors engagement initial.
Chaque extension reçoit son petit design et son plan au moment du besoin.

## Sources consultées le 25 octobre 2026

- [TubeFast : fonctionnement et limites](https://github.com/yigitbozyaka/tubefast#how-it-works)
- [Dépendances TubeFast](https://github.com/yigitbozyaka/tubefast/blob/master/Cargo.toml)
- [Slint : réactivité](https://docs.slint.dev/latest/docs/slint/guide/language/concepts/reactivity/)
- [Slint : moteurs](https://docs.slint.dev/latest/docs/slint/guide/backends-and-renderers/backend_winit/)
- [Slint : ListView](https://docs.slint.dev/latest/docs/slint/reference/std-widgets/views/listview/)
- [Slint : licences](https://slint.dev/pricing)

La licence Slint applicable et les obligations de redistribution sont à consigner
en tâche 0. Ne pas présenter automatiquement l'ensemble comme MIT parce que
le code original ou TubeFast le sont.
