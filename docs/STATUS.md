# État de reprise

Date : 2026-10-25.

- Nom choisi par l'utilisateur : **Sillage**.
- Dépôt : `https://github.com/Zvenytoh/Sillage` ; projet séparé d'UltraMusic.
- Demande actuelle : suivre précisément la roadmap, lire les fichiers pertinents et appliquer Caveman, i-have-adhd, Graphify, Using-superpowers et Ponytail. Logo et README publiés sur main au commit `137ea86`.
- Direction technique : Rust + Slint, Symphonia + rodio, sans WebView.
- Choix de cadrage de l'agent : Windows 11 x64, interface personnalisée, moteur FemtoVG au départ.
- Livrables : README anglais avec bannière, icône PNG générée et ICO multitailles, AGENTS, architecture, design, plan et skills locaux.
- Produit : aucun code applicatif, aucune dépendance Cargo installée, aucun lecteur lancé.
- Autorisations conservées : pas de sous-agent ; tests unitaires/fonctionnels opt-in ; contrôle du bureau interdit sans réautorisation.
- Vérifications de cette préparation : lecture des documents, contrôle des liens locaux, cohérence du plan, absence de whitespace problématique et vérification Git après publication. Le résultat exact est rapporté dans le chat de livraison.
- Identité visuelle : symbole de sillage en S, citron sur pin sombre. Master raster ImageGen. Icônes 16/32/64/128 px et bannière inspectées sur fichiers, sans contrôle du bureau ; voir `docs/APP-ICON.md`.
- Tâche active : 0, étude de faisabilité. Inspection statique de TubeFast au commit `c6c55dd6b1ef60f0339737e7279cda2ff3b433e2`, matrice de capacités et dépendances dans `docs/research/` ; M0 n'est pas validé à l'exécution.
- Décision de workflow : reprise manuelle dans ce fichier plutôt que helpers Bash dépendant de skills non vendoriés ; revue par l'agent courant, sans sous-agent ni test. Les restrictions utilisateur priment sur ces helpers.
- Préparation M0 : politique réseau et sonde publique bornée dans `docs/research/network-policy.md` et `scripts/probe-service.ps1`. Aucun appel fonctionnel exécuté ; aucun compte ni navigateur utilisé.
- Vérification de la sonde : analyse syntaxique AST PowerShell réussie, revue statique des bornes/destinations/redirections et absence de sortie d'URL signée. Ce contrôle ne prouve pas le fonctionnement réseau ni le décodage.
- Prochaine action : autoriser explicitement la sonde de recherche publique et de préfixe audio. Elle ne décode ni ne joue le son ; M0 complet demandera une preuve audio distincte. La licence applicative reste à choisir avant publication du code produit.
- Points avant implémentation : confirmer Windows 11 x64, accepter les cibles de performance et le périmètre V1 ; établir une méthode viable de connexion au compte.

Ne pas marquer la conception comme une validation audio, visuelle ou de performance.
