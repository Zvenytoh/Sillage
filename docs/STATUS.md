# État de reprise

Date : 2026-10-25.

- Nom choisi par l'utilisateur : **Sillage**.
- Dépôt : `https://github.com/Zvenytoh/Sillage` ; projet séparé d'UltraMusic.
- Demande actuelle : suivre précisément la roadmap, lire les fichiers pertinents et appliquer Caveman, i-have-adhd, Graphify, Using-superpowers et Ponytail. Logo et README publiés sur main au commit `137ea86`.
- Direction technique : Rust + Slint, Symphonia + rodio, sans WebView.
- Choix de cadrage de l'agent : Windows 11 x64, interface personnalisée, moteur FemtoVG au départ.
- Livrables : README anglais avec bannière, icône PNG générée et ICO multitailles, AGENTS, architecture, design, plan et skills locaux.
- Produit : lecteur natif Slint avec recherche publique et lecture progressive reliées en source, plus fichier local facultatif. Lecture/pause, Stop, volume, durée/état/erreurs et AboutSlint. Aucun compte intégré ; aucune exécution de ce parcours connecté.
- Autorisations conservées : pas de sous-agent ; tests unitaires/fonctionnels opt-in ; contrôle du bureau interdit sans réautorisation. Exceptions explicites exécutées : sonde publique et contrôle silencieux de décodage, le 25 octobre 2026. Aucun accord de lecture audible ou d'accès au compte.
- Vérifications de cette préparation : lecture des documents, contrôle des liens locaux, cohérence du plan, absence de whitespace problématique et vérification Git après publication. Le résultat exact est rapporté dans le chat de livraison.
- Identité visuelle : symbole de sillage en S, citron sur pin sombre. Master raster ImageGen. Icônes 16/32/64/128 px et bannière inspectées sur fichiers, sans contrôle du bureau ; voir `docs/APP-ICON.md`.
- Tâche active : tranche recherche → sélection → streaming, tâches 2 et 4 partielles. L'utilisateur ne souhaite pas fournir de fichier audio et demande de poursuivre ; le choix d'un résultat remplace ce prérequis. M0/M1 runtime restent ouverts.
- Décision de workflow : reprise manuelle dans ce fichier plutôt que helpers Bash dépendant de skills non vendoriés ; revue par l'agent courant, sans sous-agent. Les restrictions utilisateur priment sur ces helpers.
- Résultat M0 : sonde publique réussie, sortie 0, le 25 octobre 2026 à 16:00:33 UTC. Identifiant vidéo trouvé, flux direct itag 140, 65 536 octets reçus en HTTP 206 et préfixe ftyp. Preuve expurgée : `docs/research/probe-result-2026-10-25.json`.
- Vérification de la sonde : analyse syntaxique AST PowerShell réussie, revue statique des bornes/destinations/redirections et absence de sortie d'URL signée. Ce contrôle ne prouve pas le fonctionnement réseau ni le décodage.
- Préparation du contrôle silencieux : `tools/decode-probe/` utilise Symphonia seul ; entrée <= 64 Kio, au moins une seconde de PCM fini requise, aucun périphérique audio. L'option `-SaveDecodeSample` prépare la capture explicite sans remplacer un échantillon existant. Voir `docs/research/decode-check.md`.
- Vérifications de préparation : rustfmt, cargo check toutes cibles, Clippy sans avertissement, cargo build et analyse AST PowerShell réussis. Test unitaire de frontière écrit mais non exécuté.
- Contrôle silencieux exécuté : capture <= 64 Kio réussie ; premier décodage en échec car préfixe déclaré navigable, correction en ReadOnlySource, puis réussite sur le même fichier. Résultat : 45 056 trames/canal, 44 100 Hz, stéréo, PCM fini, sortie 0. Preuve : `docs/research/decode-result-2026-10-25.json`. Aucun son ni fichier PCM produit.
- Contrôle sonore préparé : `src/bin/audio-output-check.rs`, tonalité 440 Hz, 750 ms, amplitude 0,05 ; argument explicite `--play-tone` obligatoire. Aucun son exécuté. Voir `docs/research/audio-output-check.md`.
- Licence : MIT confirmée explicitement par l'utilisateur et ajoutée à la racine. Versions : Slint 1.18.1, rodio 0.23.0, CPAL 0.18.2, Symphonia 0.5.5 ; minimum Rust déclaré 1.92, compilation avec 1.99.1 GNU.
- Vérifications M1 : `cargo fmt --all -- --check`, Clippy `--locked --all-targets -- -D warnings` et `cargo build --locked --release --bins` réussis, cible locale `.tools/app-target`. Trois cas de tests préparés et vérifiés par le compilateur, sans `cargo test`.
- Exécutables locaux non publiés : `dist/Sillage-local-preview.exe` (13 327 872 octets) et `dist/Sillage-audio-output-check.exe` (590 336 octets). Sources et limites : `docs/qa/m1-local-player.md`. Aucun rendu, comportement audio ou chiffre de consommation M1 validé à l'exécution.
- Tranche publique : `service/{mod,transport,parse,stream}.rs`, worker `search.rs`, branchements app/lecture et vue `ui/library.slint`. Contrats simplifiés et bornes détaillés dans l'architecture ; aucune session, pagination, pochette, navigation ou reprise automatique ajoutée. Voir `docs/qa/public-search-playback.md`.
- Vérifications tranche publique : `cargo fmt --all -- --check`, Clippy toutes cibles avec `-D warnings` et build Release `--bins` réussis. Tests compilés, non exécutés. Exécutable local ignoré `dist/Sillage-online-preview.exe`, 15 917 568 octets ; SHA-256 et commandes dans `docs/qa/public-search-build-results-2026-10-25.json`. Aucun lancement, appel intégré au service, son ou mesure de performance.
- Prochaine action de développement : navigation albums/artistes/playlists (tâche 3), puis compléter la reprise réseau et la file. La validation intégrée réseau/UI/audio reste séparée et requiert l'autorisation runtime ; elle n'est pas remplacée par la compilation.
- Points avant implémentation : confirmer Windows 11 x64, accepter les cibles de performance et le périmètre V1 ; établir une méthode viable de connexion au compte.

Ne pas marquer la conception comme une validation audio, visuelle ou de performance.
