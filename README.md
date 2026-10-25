# Sillage

Un client YouTube Music natif, rapide et discret, conçu pour Windows.

**Statut : conception et roadmap. Aucun lecteur exécutable n'est encore livré.**

Sillage vise une interface personnalisée en **Slint**, une logique en **Rust**
et une lecture audio locale avec **Symphonia + rodio**, sans navigateur embarqué.
La priorité porte sur la réactivité, le soin du rendu et la consommation pendant
une longue écoute, particulièrement quand la fenêtre est masquée.

## Commencer ici

- [Instructions pour les agents](AGENTS.md)
- [Design et décisions techniques](docs/superpowers/specs/2026-10-25-sillage-design.md)
- [Roadmap et plan d'implémentation](docs/superpowers/plans/2026-10-25-sillage.md)
- [Architecture cible](docs/ARCHITECTURE.md)
- [Skills et économie de tokens](docs/TOOLING.md)
- [État de reprise](docs/STATUS.md)

Le périmètre initial est Windows 11 x64, choix de cadrage à confirmer avant
l'implémentation. Sillage est un projet distinct d'UltraMusic : aucun profil,
cookie, installateur ou identifiant d'application ne sera réutilisé implicitement.

## Ce qui doit être démontré

La recherche, l'accès aux flux et la connexion au compte doivent être vérifiés
avant d'annoncer une compatibilité YouTube Music. Une interface native ne garantit
ni l'accès à tous les formats, ni la qualité Premium, ni la stabilité du service.
Les objectifs de performance du design sont des cibles, pas des mesures acquises.

## Inspirations et statut

[TubeFast](https://github.com/yigitbozyaka/tubefast) est une référence technique
pour un client Rust sans moteur web. Sillage ne constitue pas un fork de TubeFast.
Toute reprise de code devra citer son origine et conserver sa licence.

Sillage est un projet indépendant, sans affiliation à Google ou YouTube.
Les marques et contenus appartiennent à leurs titulaires.
La licence du futur code applicatif sera arrêtée avant sa publication ; les skills
vendoriés conservent leurs licences propres.
