# État de reprise

Date : 2026-10-25.

- Nom choisi par l'utilisateur : **Sillage**.
- Dépôt : `https://github.com/Zvenytoh/Sillage` ; projet séparé d'UltraMusic.
- Demande actuelle : suivre précisément la roadmap, lire les fichiers pertinents et appliquer Caveman, i-have-adhd, Graphify, Using-superpowers et Ponytail. Logo et README publiés sur main au commit `137ea86`.
- Direction technique : Rust + Slint, Symphonia + rodio, sans WebView.
- Choix de cadrage de l'agent : Windows 11 x64, interface personnalisée, moteur FemtoVG au départ.
- Livrables : README anglais avec bannière, icône PNG générée et ICO multitailles, AGENTS, architecture, design, plan et skills locaux.
- Produit : aucun code applicatif, aucune dépendance Cargo installée, aucun lecteur lancé.
- Autorisations conservées : pas de sous-agent ; tests unitaires/fonctionnels opt-in ; contrôle du bureau interdit sans réautorisation. Exception explicite : sonde publique de recherche et préfixe audio autorisée puis exécutée le 25 octobre 2026.
- Vérifications de cette préparation : lecture des documents, contrôle des liens locaux, cohérence du plan, absence de whitespace problématique et vérification Git après publication. Le résultat exact est rapporté dans le chat de livraison.
- Identité visuelle : symbole de sillage en S, citron sur pin sombre. Master raster ImageGen. Icônes 16/32/64/128 px et bannière inspectées sur fichiers, sans contrôle du bureau ; voir `docs/APP-ICON.md`.
- Tâche active : 0, étude de faisabilité. Inspection statique de TubeFast au commit `c6c55dd6b1ef60f0339737e7279cda2ff3b433e2`, matrice de capacités et dépendances dans `docs/research/` ; M0 n'est pas validé à l'exécution.
- Décision de workflow : reprise manuelle dans ce fichier plutôt que helpers Bash dépendant de skills non vendoriés ; revue par l'agent courant, sans sous-agent. Les restrictions utilisateur priment sur ces helpers.
- Résultat M0 : sonde publique réussie, sortie 0, le 25 octobre 2026 à 16:00:33 UTC. Identifiant vidéo trouvé, flux direct itag 140, 65 536 octets reçus en HTTP 206 et préfixe ftyp. Preuve expurgée : `docs/research/probe-result-2026-10-25.json`.
- Vérification de la sonde : analyse syntaxique AST PowerShell réussie, revue statique des bornes/destinations/redirections et absence de sortie d'URL signée. Ce contrôle ne prouve pas le fonctionnement réseau ni le décodage.
- Prochaine action : préparer une vérification du décodage, sans sortie audio. Aucun décodeur, lecteur, compte ou navigateur n'a été lancé. M0 complet demandera une preuve audio distincte ; la licence applicative reste à choisir avant publication du code produit.
- Points avant implémentation : confirmer Windows 11 x64, accepter les cibles de performance et le périmètre V1 ; établir une méthode viable de connexion au compte.

Ne pas marquer la conception comme une validation audio, visuelle ou de performance.
