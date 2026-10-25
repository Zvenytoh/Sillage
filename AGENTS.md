# Sillage — instructions pour les agents

## Priorités et périmètre

- Respecter d'abord les instructions explicites de l'utilisateur. Ce fichier prime sur les workflows des skills.
- Lire `README.md`, `docs/STATUS.md`, puis la section pertinente de `docs/ARCHITECTURE.md` avant une modification.
- Lire le design et la tâche active du plan avant de coder. Le dépôt contient actuellement une conception à revoir, pas une autorisation d'implémenter le lecteur.
- Projet distinct d'UltraMusic. Ne modifier ni son dépôt, ni ses profils, cookies, réglages, assets ou identifiants d'installation.
- Cible de cadrage : Windows 11 x64. Stack : Rust + Slint, Symphonia + rodio, intégration Windows par `windows-rs`.
- Aucun WebView, Electron, Chromium embarqué, serveur local permanent ou frontend JavaScript dans le lecteur.

## Travail et skills

- Ponytail est la règle par défaut : plus petit changement complet, réutilisation avant ajout, aucune abstraction spéculative.
- Caveman concerne la concision du chat. Garder un français clair ; conserver les erreurs, négations, unités et chemins exacts.
- Charger les skills locaux à la demande, jamais tous à chaque tour : voir `docs/TOOLING.md`.
- Superpowers : brainstorming pour préciser un besoin, writing-plans pour planifier, executing-plans pour exécuter une tâche approuvée, systematic-debugging pour diagnostiquer, verification-before-completion avant toute annonce de réussite.
- Aucun sous-agent, délégation, audit par un autre modèle ou travail parallèle d'agents sans demande explicite de l'utilisateur. Effectuer la revue soi-même.
- Les prescriptions TDD des skills ne déclenchent pas de tests : les tests unitaires et fonctionnels restent opt-in.
- Ne pas installer de CLI, proxy, moteur Caveman, télémétrie, hooks ou service permanent simplement pour économiser des tokens.

## Graphify et contexte

- Si `graphify-out/graph.json` existe, utiliser d'abord `graphify query "<question>" --budget 1200` pour une question d'architecture ou de relations.
- Lire ensuite les seuls symboles/fichiers concernés. Un graphe ancien n'est pas une preuve du code actuel.
- Sans graphe : `rg --files`, puis `rg -n` ciblé. Ne pas reconstruire tout le graphe à chaque tour.
- Créer le premier graphe quand les modules applicatifs existent ; le mettre à jour après un jalon d'architecture, à la demande ou si une requête révèle une obsolescence.
- Garder `graphify-out/` local et ignoré par Git. Exclure sessions, cookies, clés, journaux privés, dépendances et artefacts.
- Pas de watcher, hook automatique, visualisation ou extraction sémantique globale par défaut. Ne pas lancer d'agent pour Graphify.

## Économie de tokens

- Une recherche ciblée avant une lecture ; ne pas afficher un fichier entier pour quelques symboles.
- Grouper les lectures indépendantes. Exécuter les mutations et leurs vérifications dans l'ordre.
- Utiliser RTK si installé et applicable ; sinon la commande native. Ne pas ajouter de dépendance à RTK dans l'application.
- Sorties compactes par défaut ; demander la sortie complète seulement pour diagnostiquer une anomalie.
- Ne pas relire les mêmes skills ou documents dans le même contexte sans changement pertinent.
- Une tâche active à la fois. Résumé de reprise dans `docs/STATUS.md` : décision, fichiers touchés, vérification réelle, prochaine action.
- Réponses brèves : résultat, preuve utile, limite réelle. Pas de promesse chiffrée d'économie de tokens sans mesure.
- Ne jamais économiser sur validation, accessibilité, sécurité, gestion d'erreurs ou vérification nécessaire.

## Implémentation

- Rust idiomatique, modules possédant leur comportement, dépendances justifiées et versions verrouillées dans `Cargo.lock`.
- Aucun `unwrap`/`expect` dans les chemins de production récupérables. Erreurs typées à la frontière du service.
- Localiser `unsafe` dans l'intégration native ; documenter les invariants et propriétaires des ressources.
- UI événementielle ; aucun travail réseau, disque lent ou décodage audio sur son thread.
- Aucun rafraîchissement permanent ni polling périodique du morceau ; progression dérivée de l'horloge audio, réactualisée seulement si visible.
- Files, caches et tampons bornés. Annuler ou ignorer les anciennes générations de recherches et de lectures.
- Ne jamais journaliser cookies, jetons, URL de flux signées ou réponses contenant des secrets.
- Stocker les sessions Windows avec DPAPI ; ne pas lire les profils d'autres applications ou navigateurs sans action explicite.
- Valider schémas, tailles, délais, redirections et destinations réseau. Ne jamais exécuter du code reçu d'un serveur de configuration.
- Préserver les données lors des écritures, migrations, installations et mises à jour.

## Vérification et livraison

- Aucun test unitaire/fonctionnel, y compris dans la CI, sauf demande explicite. Les étapes correspondantes du plan sont conditionnelles.
- Aucun lancement du lecteur, contrôle du bureau, automatisation UI ou écoute de validation sans réautorisation explicite.
- Les commandes de formatage, lint et compilation proposées par le plan ne sont exécutées qu'au périmètre autorisé ; rapporter exactement celles qui ont tourné.
- Une compilation ne prouve pas le rendu, l'audio, le DRM, la consommation ou l'autonomie. Distinguer inspection statique, tests et mesures runtime.
- Marquer une tâche terminée seulement avec sa preuve ; sinon documenter la vérification manquante.
- Commits limités à la tâche, jamais `git add .` sur un arbre partagé avec des modifications étrangères. Branches de développement `codex/…`.
- Pas de push forcé, effacement de données, publication de secrets ou changement des permissions du dépôt.
