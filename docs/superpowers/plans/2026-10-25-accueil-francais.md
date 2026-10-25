# Accueil musical français — plan d'implémentation

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Aucun sous-agent ni reviewer délégué : l'utilisateur les interdit. Les tests restent opt-in.

**Goal:** Réaliser dans Sillage natif l'accueil A Équilibré violet choisi, en français, avec écoutes récentes persistantes, tendances vérifiables, pochettes fiables et volume corrigé puis inspecté dans l'app.

**Architecture:** Les données de catalogue, l'historique, les favoris et les références d'images restent indépendants des modèles visibles. Un worker de métadonnées partage accueil/recherche ; un worker d'images et le worker audio existants restent distincts. Le stockage conserve les favoris existants et ajoute un historique versionné ; les commandes de volume ne réappliquent pas une confirmation périmée pendant un glissement.

**Tech Stack:** Rust, Slint 1.18.1, ureq 3.5.0, image 0.25.10, serde_json, rodio 0.23.0 / Symphonia 0.5.5, windows-rs. Pas de nouvelle dépendance initialement.

**Spec:** `docs/superpowers/specs/2026-10-25-accueil-francais-design.md`.

## Global Constraints

- Windows 11 x64, Rust + Slint ; aucun WebView ou frontend JavaScript dans le produit. Le prototype HTML est une référence de composition uniquement.
- Direction choisie : A Équilibré ; violet `#AD91FF`, texte sur accent `#19112A`, lavis `#342D45`, fonds `#090A0D` / `#14151B`.
- Tous les textes applicatifs et accessibles en français ; titres/artistes dans leur langue d'origine ; README conservé en anglais.
- Retirer l'ouverture de fichier local des actions visibles ; ne pas supprimer de fichiers utilisateur ni casser le contrôle de décodage existant.
- Historique ≤200 titres uniques ; qualification à 30 s de PCM consommé, ou moitié d'un titre connu de moins de 60 s. Recherche/préparation seules ne comptent pas.
- Tendances initialement France, source et fraîcheur explicites, cache six heures. Ne pas remplacer un classement par une recherche générique étiquetée « Tendances ».
- Images : HTTPS/hôtes validés, requêtes ≤512 Kio, dimensions ≤2048×2048, allocation de décodage ≤16 Mio ; cache décodé CPU ≤32 Mio, textures GPU et allocations temporaires distinctes. Budget disque proposé 128 Mio.
- Métadonnées ≤4 Mio, requête ≤15 s, redirections revérifiées/refusées selon le transport existant ; files bornées et réponses anciennes invalidées. Trois opérations réseau actives maximum : catalogue, images, audio.
- Favoris existants préservés. Pas de cookies, sessions d'autres applications, URL audio signées, identifiants ou compte ajoutés au stockage.
- Contrôle UI et réseau public nécessaires à cette refonte autorisés ; aucune lecture audible, unité/fonctionnel automatisé ou mesure audio par déduction.
- Une compilation ne valide pas le rendu. Faire la revue soi-même ; aucun sous-agent.

## Review Focus

1. Bibliothèque v1, version inconnue ou fichier de récupération : conserver les données et empêcher l'écriture destructive. Cas préparés dans tâche 2.
2. Recherche rapide, navigation et changement de piste pendant le chargement d'images : bonne pochette sur le bon identifiant, sans annulation du morceau courant. Cas préparés dans tâche 3.
3. Premier lancement, hors ligne, classement absent ou périmé : accueil utilisable, provenance et date honnêtes, aucune recommandation inventée. Cas préparés dans tâche 1, contrôlés dans tâche 6.
4. Buffering, lecture masquée et rafale de glissements du volume : récents fondés sur le PCM et poignée stable, erreurs 429/annulations bornées. Cas préparés dans tâches 2 et 5.
5. Titres français longs, fenêtre étroite et DPI élevé : textes tronqués intentionnellement, commandes accessibles et aucun chevauchement. Contrôle réel dans tâche 6.

## Carte des responsabilités

| Fichiers | Responsabilité |
|---|---|
| `src/model.rs`, `src/service/mod.rs`, `src/service/parse.rs` | Types de sections, données musicales, source et fraîcheur du classement |
| `src/catalog.rs` (évolution de `src/search.rs`) | Un worker accueil/recherche, cache des métadonnées et annulation |
| `src/history.rs` (nouveau) | Qualification/déduplication des écoutes, sans I/O ni UI |
| `src/library.rs` | Migration et sauvegarde versionnée des favoris/historique |
| `src/artwork.rs`, `src/service/transport.rs` | Images partagées, priorité des vues, cache borné et validation des destinations |
| `src/session.rs`, `src/app.rs` | Coordination de l'état, progression, volume, navigation et modèles visibles |
| `ui/app.slint`, `library.slint`, `tracks.slint`, `components.slint`, `theme.slint`, `volume.slint`, `player.slint`, `model.slint` | Composition approuvée et copie française |
| `docs/qa/`, `DESIGN.md`, `PRODUCT.md`, `docs/STATUS.md` | Preuves, système visuel observé et état réel |

## Tâche 1 — Établir les vrais contenus d'accueil

**Fichiers :** service/model/parse, nouveau `src/catalog.rs`, app/main ; fixtures expurgées dans `tests/fixtures/`, cas dans `tests/home.rs`.

**Interfaces :** `HomePage { sections: Vec<HomeSection>, country: String, fetched_at_unix_ms: u64 }` ; `HomeSection { id: String, title: String, tracks: Vec<Track>, source: Option<FeedSource> }` ; `FeedSource { label: String, country: String }` ; `Client::home(country: &str, cancel: &Cancellation) -> Result<HomePage, ServiceError>`.

- [ ] Inspecter les flux publics `FEmusic_home` / `FEmusic_explore` du référence TubeFast, puis vérifier par requête bornée les sections disponibles en `hl=fr`, `gl=FR`. Aucune donnée de compte ; aucune récupération audio.
- [ ] Identifier et consigner l'origine réelle des tendances. Si une section conduit à une playlist, parcourir ses identifiants retournés par le service pour obtenir des pistes ; ne pas construire un faux classement ni utiliser les exemples des maquettes.
- [ ] Ajouter un parseur borné et tolérant aux champs inconnus. Ne pas convertir un album/artiste sans `videoId` en morceau jouable. N'afficher que les objets dont l'action est implémentée.
- [ ] Remplacer le worker dédié à la seule recherche par un worker catalogue : `Catalog::search(String)`, `Catalog::home(String)`, annulations séparées par vue et dernière demande en attente seulement. Préserver l'accueil déjà reçu pendant une recherche.
- [ ] Mettre en cache la page normalisée, bornée, datée. Cache récent immédiatement visible ; requête à expiration/actualisation explicite. Une panne garde l'ancien contenu avec son âge. Une source non établie reste marquée indisponible.
- [ ] Préparer les cas `home_ignores_unknown_fields`, `empty_feed_is_valid`, `non_track_is_not_playable`, `stale_feed_is_marked`, `missing_chart_is_not_replaced_by_search` ; typechecker, sans les exécuter.
- [ ] Formatage/Clippy ; revue des hôtes, délais, pays et sources ; commit limité. La tâche reste ouverte si la récupération du classement est impossible.

## Tâche 2 — Historique fiable et migration sans perte

**Fichiers :** nouveau `src/history.rs`, model/library/session/app ; cas `tests/history.rs` et tests de stockage locaux préparés.

**Interfaces :** `RecentListen { track: Track, played_at_unix_ms: u64, position_ms: u64 }` ; `LibrarySnapshot { favorites: Vec<Track>, recent: Vec<RecentListen> }` ; `History::observe(track: &Track, position_ms: u64, now_unix_ms: u64) -> bool` ; `Library::save(&LibrarySnapshot)` et `Library::take_loaded() -> Option<LibrarySnapshot>`.

- [ ] Faire de `History` un modèle pur borné/dédupliqué ; position remise à zéro par génération, seuil fondé sur les échantillons consommés existants. Un titre inconnu ne se qualifie pas sur une fausse durée zéro.
- [ ] Garder le snapshot canonique dans l'état Rust de session. Le worker de stockage dépose le chargement en mémoire et réveille un callback UI ; `take_loaded` transfère l'objet Rust sans JSON volumineux à parser sur le thread UI.
- [ ] Lire v1 sans perte, migrer en v2 lors d'une écriture réussie seulement ; conserver titre/artiste/durée et référence publique d'image validée. Version inconnue, fichier invalide et `.pending` existant restent préservés.
- [ ] Conserver le verrou Windows et l'écriture atomique ; bornes v2 : 500 favoris, 200 récents, 2 Mio. Aucun fichier des autres applications.
- [ ] Écrire à la qualification, pause/changement/fermeture si dirty, jamais toutes les 250 ms. Conserver le dernier snapshot en attente lorsque des modifications se succèdent.
- [ ] Préparer `search_is_not_history`, `buffering_is_not_listening`, `short_track_halfway`, `repeat_deduplicates`, `hidden_playback_updates_history`, `v1_migration_preserves_favorites`, `pending_recovery_is_preserved` ; ne pas exécuter.
- [ ] Formatage/Clippy, revue fermeture/écriture et commit.

## Tâche 3 — Pochettes communes et persistantes

**Fichiers :** artwork/transport/service/model/session/library, `ui/model.slint`, cas préparés d'images/cache.

**Interfaces :** `ArtworkKey` = identifiant stable de morceau ; `ArtworkPriority::{Current, Visible, Background}` ; `Artwork::request(key, url, priority)` ; `Artwork::retain_visible(keys)` ; événements d'image indexés par identifiant, sans dépendre exclusivement des résultats de recherche.

- [ ] Préserver la référence d'image du catalogue dans les transformations Track → vue → stockage ; l'ancienne conversion de favoris perdait cette donnée. Si elle manque en v1, récupérer des métadonnées du même identifiant, jamais une image d'un homonyme.
- [ ] Extraire les miniatures déjà présentes dans la réponse player lors d'une résolution ; éviter un deuxième appel identique juste pour la pochette. Validation identique des URL avant requête et mise en cache.
- [ ] Donner l'image d'un même morceau à recherche, accueil, favoris, historique, file et panneau. Ne pas annuler une image du morceau courant lorsque l'utilisateur efface sa recherche.
- [ ] Prioriser morceau courant puis objets visibles ; file d'images ≤64 demandes uniques, un fetch actif. Prendre une résolution adaptée au DPI et traiter correctement les images 16:9 plutôt qu'un recadrage destructif automatique.
- [ ] Cache disque atomique ≤128 Mio, cache décodé CPU ≤32 Mio. Compter aussi les images encore retenues dans les modèles de vue : évincer un index LRU ne libère pas un pixel buffer toujours référencé par une vue. Invalider les références hors écran lors d'une éviction nécessaire.
- [ ] Préparer `same_id_same_cover`, `wrong_host_rejected`, `search_cancel_keeps_current_cover`, `restart_uses_cached_cover`, `decoded_budget_includes_view_handles`, `unavailable_cover_has_neutral_fallback` ; ne pas exécuter.
- [ ] Inspecter des vraies images à l'écran après intégration ; formatage/Clippy puis commit.

## Tâche 4 — Traduction et composition A violette

**Fichiers :** tous les UI cités, app/model/session/service/library/catalog ; `DESIGN.md` sera consolidé après inspection.

**Contrats de vue :** modèles distincts `recent_rows`, `trending_rows`, `discovery_rows`, `search_results`, `saved_rows`, `queue_rows`. Les boutons naviguent vers des données réelles ; les dates entières et l'état canonique restent en Rust, pas dans des floats Slint.

- [ ] Traduire navigation, états, erreurs, About, aides et accessibilité. Conserver noms propres et textes légaux requis. Chaînes cohérentes : Accueil, Titres favoris, Écoutes récentes, À suivre, Rechercher, Réessayer.
- [ ] Retirer le bouton et le callback d'ouverture locale dans la présentation/app ; conserver le moteur local et les outils de diagnostic hors UI pour ne pas casser leurs usages existants.
- [ ] Transposer A depuis `accueil-violet.html` / `a-equilibre-violet.png` : navigation latérale, en-tête de recherche, récents en cartes, tendances compactes, découverte, grande pochette et prochain titres à droite, transport fixe.
- [ ] Conserver les proportions observées à 1360 px : rail environ 210–232 px, panneau 288–304 px, recherche ≤470–520 px, cartes à ratio carré et sections sans hauteur vide artificielle. À 1000 px, panneau refermable sans commandes masquées ; à 1920 px, augmenter les colonnes utiles plutôt qu'étirer une seule liste.
- [ ] Appliquer violet `#AD91FF` et lavis `#342D45` discrètement. Icônes centrées explicitement, titres à deux lignes maximum dans les cartes, artiste une ligne, focus visible et contrastes calculés.
- [ ] Premier lancement sans récents : découverte/tendances réelles, aucune fausse écoute. Chargement, erreur, hors ligne et cache daté gardent une structure stable.
- [ ] Formatage/Clippy/build ; revue de toutes les actions visibles et commit. Pas de promesse de parité album/compte issue des textes de démonstration des maquettes.

## Tâche 5 — Diagnostiquer et corriger le volume natif

**Fichiers :** volume/player/components Slint ; app/playback si les confirmations sont en cause.

**Contrat :** une commande porte une valeur normalisée 0..1 et un identifiant monotone si nécessaire ; une confirmation périmée ne remplace pas la valeur affichée pendant un glissement. Les touches et l'accessibilité utilisent la même conversion que le pointeur.

- [ ] Ouvrir la version native concernée sans Play. Observer largeur réelle, rail, poignée et coordonnées ; reproduire glissement, clic, clavier et mute. Comparer avec le comportement attendu de la maquette, sans l'utiliser comme preuve du moteur natif.
- [ ] Vérifier la piste de réinjection des `VolumeChanged` pendant le drag. Corriger la cause constatée : géométrie commune ou confirmations séquencées et commandes coalescées. Aucun timer permanent ; coalescence uniquement pendant l'interaction si utile.
- [ ] Vérifier visuellement 0/25/50/75/100 %, drag hors limites, Home/End/flèches, changement de titre et sortie du mode muet. Aucune écoute nécessaire pour le contrôle UI.
- [ ] Préparer un cas de confirmation périmée et des bornes NaN/hors plage si la logique de protocole change ; ne pas lancer les tests.
- [ ] Formatage/Clippy/build ; documenter cause observée, changement et limites ; commit.

## Tâche 6 — Contrôle visuel et livraison

**Fichiers :** `docs/qa/french-home-native.md`, résultats de build JSON, STATUS/PRODUCT/DESIGN ; exécutable local ignoré et raccourci Bureau.

- [ ] Construire un nouvel exécutable distinct, préserver celui déjà ouvert. Ouvrir la nouvelle version par contrôle UI autorisé ; aucune lecture audible ni accès au compte.
- [ ] Inspecter ensemble accueil vide/rempli, recherche, favoris, récents, file et erreurs à 1000×700, environ 1360×900 et 1920×1080. Vérifier les DPI disponibles sans changer les réglages système non autorisés ; consigner ce qui n'est pas exercé.
- [ ] Vérifier texte français long, petites/grandes pochettes, scroll, focus, raccourcis, volet droit, volume, chargement/hors ligne via situations réellement accessibles. Une simulation injectée doit être identifiée comme telle.
- [ ] Faire une passe groupée de corrections, reconstruire après modification, puis capturer de nouveau les états affectés. Conserver des captures du natif correspondant au binaire livré.
- [ ] Exécuter `cargo fmt --all -- --check`, `cargo clippy --locked --all-targets -- -D warnings`, `cargo build --locked --release --bins`, avec le toolchain/target-dir locaux déjà documentés. Aucun `cargo test`.
- [ ] Documenter séparément source de tendances vérifiée, rendu natif observé, contrôles non faits, absence d'écoute/mesure. Mettre à jour le raccourci seulement vers l'artefact vérifié, sans fermer une fenêtre utilisateur.
- [ ] Commit/push limité à la refonte autorisée. Ne pas déclarer « parfait » ou terminé si une source essentielle ou un défaut visuel reste ouvert.

## Passage à l'exécution

Méthode déjà fixée : exécution native par l'agent courant, sans délégation.
Relire ce plan avant implémentation. La comparaison A/violet est retenue ; les
maquettes ne prouvent ni l'accès aux tendances ni le bon fonctionnement du natif.
