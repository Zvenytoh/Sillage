# Skills et économie de tokens

Les skills de ce dépôt sont des instructions locales versionnées. Leur présence
ne signifie pas que leur CLI ou service associé est installé. Ils seront
découverts à l'ouverture du projet ou au prochain tour qui actualise le catalogue.

## Inventaire

| Skill local | Usage | Provenance |
|---|---|---|
| `.codex/skills/ponytail/` | Modification minimale complète | Copie du skill personnel fourni par l'utilisateur ; frontmatter MIT |
| `.codex/skills/caveman/` | Chat concis, sens et erreurs conservés | JuliusBrussee/caveman, commit `1f951fb4ad40ba63302c6e7f987147e034d599d9` |
| `.codex/skills/i-have-adhd/` | Suivi explicite, étapes courtes et prochaine action | Copie du skill personnel explicitement invoqué par l'utilisateur ; frontmatter MIT |
| `.codex/skills/graphify/` | Relations entre fichiers et architecture | Graphify-Labs/graphify, copie locale version `0.9.92` |
| `.codex/skills/brainstorming/` | Cadrage d'un besoin | Superpowers, copie locale commit `bb92a77741419a4ab5f06e711a283343f1ada0c3` |
| `.codex/skills/writing-plans/` | Plan d'implémentation | Même provenance Superpowers |
| `.codex/skills/executing-plans/` | Exécution séquentielle approuvée | Même provenance Superpowers |
| `.codex/skills/systematic-debugging/` | Diagnostic avant correctif | Même provenance Superpowers |
| `.codex/skills/verification-before-completion/` | Preuve avant conclusion | Même provenance Superpowers |
| `.codex/skills/test-driven-development/` | Cycle de tests si autorisé | Même provenance Superpowers |

Les copies locales peuvent différer de leur upstream ; les fichiers vendoriés font
foi pour cette révision. Les licences Superpowers, Caveman et Graphify sont
conservées à côté des skills. La licence du code futur reste une décision distincte.

## Politique effective

`AGENTS.md` prime sur les workflows : pas de sous-agent sans demande explicite,
pas de test unitaire/fonctionnel sans demande, pas de contrôle du bureau sans
réautorisation. Une recommandation de reviewer ou de TDD dans un skill ne lève
aucune de ces restrictions. La revue se fait dans le contexte courant.

Ponytail guide le code. Caveman raccourcit le chat, sans rendre les documents
télégraphiques. Superpowers structure la tâche utile, sans reposer des questions
déjà tranchées. Les skills sont lus à la demande ; aucun chargement global ajouté.

Graphify est préparé, mais son moteur n'est pas installé et aucun graphe n'a été
construit lors de la préparation documentaire. Le socle local M1 existe
désormais ; son premier index reste à générer avec le moteur. Utiliser un index
structurel ciblé, sans secrets ni dépendances. Lire le skill avant sa génération.
Pour une requête, budget initial de 1 200 tokens ; élargir seulement si insuffisant.
Ne pas utiliser `--watch`, hook post-commit ou extraction sémantique globale par défaut.

RTK est utilisé s'il est disponible ; aucune installation globale ou configuration
de la machine n'est nécessaire au dépôt. Les recherches `rg` et sorties bornées
réduisent le contexte lu ; Caveman agit surtout sur la longueur du chat.
Aucun pourcentage d'économie global n'est garanti.

## Installation réalisée

Le helper skill-installer a été essayé ; les interpréteurs Python de la machine
n'étaient pas utilisables. Caveman a donc été copié par téléchargement HTTPS
du `SKILL.md` et de sa licence MIT depuis le commit épinglé ci-dessus.
Aucun installateur distant, hook, proxy ou moteur Caveman n'a été exécuté.

## Sources

- [Caveman épinglé](https://github.com/JuliusBrussee/caveman/tree/1f951fb4ad40ba63302c6e7f987147e034d599d9/skills/caveman)
- [Superpowers](https://github.com/obra/superpowers)
- [Graphify](https://github.com/Graphify-Labs/graphify)
