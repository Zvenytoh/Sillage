# État de reprise

Date : 2026-10-25.

## Reprise active — accueil français violet

- Demande suivante : retirer la barre de titre Windows et intégrer uniquement
  réduire, agrandir/restaurer, fermer, dans cet ordre. Source :
  `ui/window-controls.slint`, `ui/app.slint`, quatre SVG. Déplacement et
  redimensionnement via Slint existant ; aucune dépendance ni logique Rust
  supplémentaire. Aucun contrôle du bureau ni lancement pour cette révision.
  Formatage, Clippy toutes cibles sans avertissement et Release --bins réussis.
  Raccourci Bureau vers `dist/Sillage-frameless-preview.exe`, anciens binaires
  conservés ; preuve dans `docs/qa/integrated-window-build-results-2026-10-25.json`.

- Dernier retour : capture native fournie par l'utilisateur. Correction ciblée
  du centrage implicite des pochettes, du vide entre sections, des largeurs de
  colonnes et du placeholder. Découverte 16:9, 4/6/8 cartes selon largeur.
  Formatage, Clippy et Release réussis. Raccourci vers la nouvelle préversion
  `dist/Sillage-fr-violet-layout-preview.exe`, ancienne version préservée.
  Preuve : `docs/qa/french-home-layout-build-results-2026-10-25.json`.
  Aucune ouverture/commande du bureau ; rendu corrigé à confirmer dans le natif.

- Choix A Équilibré + violet `#AD91FF`, implémentation approuvée « nickel go fonce ».
  Branche `codex/french-home-implementation`. Les mentions de brouillon plus bas
  décrivent les étapes antérieures.
- Source : catalogue accueil/recherche, vraies tendances France et découverte,
  historique PCM persistant/migration v1→v2, cache commun de pochettes, interface
  française sans ouverture locale, refonte A/violet et volume coalescé.
- Sonde publique Rust réussie : 20 tendances, 21 nouveautés, 41 références
  d'images valides ; aucun compte, fenêtre ou son. Preuves et limites dans
  `docs/qa/french-home-native.md` et le JSON de build associé.
- Dernière consigne : l'utilisateur joue sur son PC. Continuer en arrière-plan,
  aucune fenêtre ni contrôle UI pendant cette exécution. Compilation limitée
  à deux jobs, priorité BelowNormal. Tests unitaires/fonctionnels non exécutés.
- Vérifications finales : fmt --check, Clippy toutes cibles sans avertissement
  Rust/Slint, build Release --bins réussis. Nouvelle préversion distincte
  `dist/Sillage-fr-violet-preview.exe`, 18 046 976 octets ; raccourci Bureau
  actualisé et relu, ancienne version préservée. Aucune ouverture de la nouvelle
  révision. Empreinte dans le JSON de QA.
- Prochaine action après livraison : inspection native de la nouvelle préversion
  et du volume quand l'utilisateur est disponible, puis corrections visuelles
  nécessaires. La roadmap reste ouverte sur ces vérifications.

## Historique des étapes

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
- Tâche active : brainstorming demandé explicitement pour une app française, accueil tendances/récents, pochettes persistantes, suppression de l'ouverture locale, correction du volume et accent distinct de l'orange TubeFast/citron Sillage. Référence : nouvelle capture TubeFast fournie. Brouillon non approuvé : `docs/superpowers/specs/2026-10-25-accueil-francais-design.md` ; aucun code produit modifié dans ce cadrage.
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
- Reconstruction : navigation globale Home/recherche, bibliothèque locale réductible, vues All/Songs et meilleur résultat, panneau morceau/file, lecteur fixe avec contrôles centrés. Palette noir/charbon/blanc, volume monochrome. `PRODUCT.md`, `DESIGN.md` et `docs/qa/desktop-ui-rebuild.md` consignent les faits ; pas de verdict visuel validé.
- Données/commandes : favoris locaux versionnés ≤500 titres/2 Mio dans `%LOCALAPPDATA%\Sillage`, worker d'écriture atomique et verrou exclusif Windows ; refus d'écraser un fichier de récupération ou une version inconnue. File de session ≤500, historique précédent ≤128, suivant, shuffle, répétition du morceau courant et fin naturelle dédupliquée. File non restaurée, seek indisponible, aucun compte synchronisé.
- Pochettes : worker séparé du worker de recherche, ≤512 Kio reçus, décodage ≤2048×2048/16 Mio, réduction à 320×320, ≤32 par recherche, ≤24 couvertures retenues dans chacun des modèles favoris/file. Annulation, dernier lot en attente seulement, 429 sans retry. Trois workers réseau bornés, aucun appel depuis l'UI.
- Vérifications refonte : formatage, Clippy toutes cibles sans avertissement et build Release `--bins` réussis. Exécutable ignoré `dist/Sillage-dark-preview.exe` ; empreinte/commandes dans `docs/qa/dark-interface-build-results-2026-10-25.json`. Revue/documentation par l'agent courant, sans délégation. Aucun lancement, contrôle visuel réel, test, son ou mesure.
- Vérifications reconstruction : voir `docs/qa/desktop-ui-build-results-2026-10-25.json`. Formatage, Clippy toutes cibles et build Release ; trois nouveaux cas préparés/typecheckés, non exécutés. Exécutable distinct `dist/Sillage-desktop-preview.exe` et raccourci Bureau actualisé sans lancer ni écraser l'ancienne version ouverte.
- Nouvelle autorisation explicite : « prendre le temps d'ouvrir vérifier » autorise le lancement/contrôle visuel dans le périmètre de cette refonte. Le refus statique antérieur est remplacé pour ce contrôle UI. Aucun son, test unitaire, compte ou sous-agent autorisé par déduction. Aucun lancement effectué pendant le brainstorming ; M0/M1/M4 runtime non validés.
- Choix visuel explicite reçu : **« équilibre accent violet »**. A Équilibré retenu, violet `#AD91FF`, lavis `#342D45`, texte sur accent `#19112A`. Maquette mise à jour et inspectée ; capture `.tools/french-home-mocks/a-equilibre-violet.png`. Ce contrôle concerne uniquement le prototype.
- Plan prêt à relire : `docs/superpowers/plans/2026-10-25-accueil-francais.md` ; six tâches, exécution par l'agent courant sans sous-agent, tests opt-in. Source réelle de tendances, migration/historique, cache d'images, UI française, volume natif puis revue visuelle/livraison. Aucun code produit modifié à ce stade ; revue du plan avant exécution selon Writing Plans.
- Points avant implémentation : confirmer Windows 11 x64, accepter les cibles de performance et le périmètre V1 ; établir une méthode viable de connexion au compte.

Ne pas marquer la conception comme une validation audio, visuelle ou de performance.
