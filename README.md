Slab Allocator en Rust (no_std)
Objectif

Implémenter un Slab Allocator simple en Rust sans la bibliothèque standard (no_std). Ce projet met en œuvre une gestion mémoire efficace avec un allocateur slab, permettant des opérations rapides et prévisibles dans un environnement minimaliste.
Structure du Projet

    src/allocator.rs : Contient l’implémentation de l’allocateur, y compris la logique d’allocation, de désallocation, et de gestion de la mémoire libre.
    src/lib.rs : Fichier de configuration principal. Définit l’utilisation de no_std et contient les tests unitaires.
    REPORT.md : Document détaillant les choix techniques, les étapes de développement et les considérations de conception.
    REPORT_TEST.md : Fichier listant les résultats des tests unitaires pour chaque étape du développement, avec une description des cas d’utilisation vérifiés.

Fonctionnalités

    Allocation simple de blocs mémoire fixes.
    Désallocation des blocs pour leur réutilisation.
    Gestion des cas de saturation et refus d’allocation quand la mémoire est pleine.
    Suivi du nombre de blocs libres restants pour valider les performances et la cohérence.

Prérequis

Avant de compiler ou de tester le projet, assurez-vous d’avoir installé les sources de Rust nécessaires, notamment si vous travaillez avec une configuration no_std.


Compilation

Pour compiler le projet, exécutez la commande suivante :

```bash
cargo build

Exécution des Tests

Pour lancer les tests unitaires et vérifier le bon fonctionnement de l’allocateur, utilisez la commande suivante :

cargo test

Résultats des Tests

Des tests unitaires sont fournis pour vérifier les cas suivants :

    Allocation de blocs mémoire.
    Libération et réutilisation de blocs.
    Gestion de la saturation mémoire.
    Vérification des compteurs de blocs libres.

Tous ces tests sont documentés dans le fichier REPORT_TEST.md et sont conçus pour garantir la fiabilité de l’allocateur dans divers scénarios.
