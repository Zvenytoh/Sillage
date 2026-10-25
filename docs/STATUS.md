# État de reprise

Date : 2026-10-25.

- Nom choisi par l'utilisateur : **Sillage**.
- Dépôt : `https://github.com/Zvenytoh/Sillage` ; projet séparé d'UltraMusic.
- Demande actuelle : suivre précisément la roadmap, lire les fichiers pertinents et appliquer Caveman, i-have-adhd, Graphify, Using-superpowers et Ponytail. Logo et README publiés sur main au commit `137ea86`.
- Direction technique : Rust + Slint, Symphonia + rodio, sans WebView.
- Choix de cadrage de l'agent : Windows 11 x64, interface personnalisée, moteur FemtoVG au départ.
- Livrables : README anglais avec bannière, icône PNG générée et ICO multitailles, AGENTS, architecture, design, plan et skills locaux.
- Produit : aucun code applicatif ni lecteur lancé. Outil de décodage distinct préparé et compilé ; ses dépendances et artefacts sont dans `.tools/`, avec manifeste et lockfile de l'outil versionnés.
- Autorisations conservées : pas de sous-agent ; tests unitaires/fonctionnels opt-in ; contrôle du bureau interdit sans réautorisation. Exceptions explicites exécutées : sonde publique et contrôle silencieux de décodage, le 25 octobre 2026. Aucun accord de lecture audible ou d'accès au compte.
- Vérifications de cette préparation : lecture des documents, contrôle des liens locaux, cohérence du plan, absence de whitespace problématique et vérification Git après publication. Le résultat exact est rapporté dans le chat de livraison.
- Identité visuelle : symbole de sillage en S, citron sur pin sombre. Master raster ImageGen. Icônes 16/32/64/128 px et bannière inspectées sur fichiers, sans contrôle du bureau ; voir `docs/APP-ICON.md`.
- Tâche active : 0, étude de faisabilité. Inspection statique de TubeFast au commit `c6c55dd6b1ef60f0339737e7279cda2ff3b433e2`, matrice de capacités et dépendances dans `docs/research/` ; M0 n'est pas validé à l'exécution.
- Décision de workflow : reprise manuelle dans ce fichier plutôt que helpers Bash dépendant de skills non vendoriés ; revue par l'agent courant, sans sous-agent. Les restrictions utilisateur priment sur ces helpers.
- Résultat M0 : sonde publique réussie, sortie 0, le 25 octobre 2026 à 16:00:33 UTC. Identifiant vidéo trouvé, flux direct itag 140, 65 536 octets reçus en HTTP 206 et préfixe ftyp. Preuve expurgée : `docs/research/probe-result-2026-10-25.json`.
- Vérification de la sonde : analyse syntaxique AST PowerShell réussie, revue statique des bornes/destinations/redirections et absence de sortie d'URL signée. Ce contrôle ne prouve pas le fonctionnement réseau ni le décodage.
- Préparation du contrôle silencieux : `tools/decode-probe/` utilise Symphonia seul ; entrée <= 64 Kio, au moins une seconde de PCM fini requise, aucun périphérique audio. L'option `-SaveDecodeSample` prépare la capture explicite sans remplacer un échantillon existant. Voir `docs/research/decode-check.md`.
- Vérifications de préparation : rustfmt, cargo check toutes cibles, Clippy sans avertissement, cargo build et analyse AST PowerShell réussis. Test unitaire de frontière écrit mais non exécuté.
- Contrôle silencieux exécuté : capture <= 64 Kio réussie ; premier décodage en échec car préfixe déclaré navigable, correction en ReadOnlySource, puis réussite sur le même fichier. Résultat : 45 056 trames/canal, 44 100 Hz, stéréo, PCM fini, sortie 0. Preuve : `docs/research/decode-result-2026-10-25.json`. Aucun son ni fichier PCM produit.
- Prochaine action : préparer la vérification de sortie audio pour terminer la preuve de lecture M0, sans la lancer avant autorisation explicite. Aucun compte ou navigateur n'a été lancé. M0 reste ouvert ; licence applicative encore à choisir avant publication du code produit.
- Points avant implémentation : confirmer Windows 11 x64, accepter les cibles de performance et le périmètre V1 ; établir une méthode viable de connexion au compte.

Ne pas marquer la conception comme une validation audio, visuelle ou de performance.
