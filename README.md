Slab Allocator en Rust (no_std)
Objectif

Implémenter un Slab Allocator simple en Rust sans la bibliothèque standard (no_std). Ce projet met en œuvre une gestion mémoire efficace avec un allocateur slab, permettant des opérations rapides et prévisibles dans un environnement minimaliste.

Pourquoi utiliser le premier ajustement (First Fit) ?

L’algorithme de premier ajustement (First Fit) est une méthode efficace pour gérer la mémoire. Il recherche le premier bloc libre suffisamment grand pour satisfaire une demande d’allocation. Les avantages incluent :

- Simplicité et rapidité : Le premier ajustement évite de parcourir toute la liste des blocs libres, réduisant le temps de recherche.
- Réduction de la fragmentation interne : Les petits blocs restants sont réutilisés rapidement, limitant les pertes de mémoire inutilisées.
- Compatibilité avec les scénarios à forte allocation/désallocation : L’algorithme est particulièrement adapté aux environnements où les allocations sont fréquentes et où la mémoire disponible change régulièrement.

Structure du Projet

    src/allocator.rs : Contient l’implémentation de l’allocateur, y compris la logique d’allocation, de désallocation, et de gestion de la mémoire libre.
    src/lib.rs : Fichier de configuration principal. Définit l’utilisation de no_std et contient les tests unitaires.
    REPORT.md : Document détaillant les choix techniques, les étapes de développement, et les considérations de conception.
    REPORT_TEST.md : Fichier listant les résultats des tests unitaires pour chaque étape du développement, avec une description des cas d’utilisation vérifiés.

Fonctionnalités

    Allocation simple de blocs mémoire fixes.
    Désallocation des blocs pour leur réutilisation.
    Gestion des cas de saturation et refus d’allocation quand la mémoire est pleine.
    Suivi du nombre de blocs libres restants pour valider les performances et la cohérence.
    Stabilité garantie, même avec des scénarios complexes d’allocations et de désallocations.
    Détection des erreurs de double désallocation et gestion des utilisations après libération
    Gestion des alignements incorrects et des tailles d’allocation excessives pour éviter des erreurs de comportement.
    Tests de régression garantissant que les nouvelles optimisations n’altèrent pas les fonctionnalités existantes..
    Optimisation des performances grâce à l'implémentation d'un algorithme de premier ajustement (First Fit) pour l’allocation des blocs libres.
    Mesures de performance pour évaluer l’efficacité de l’allocation et de la désallocation.


Prérequis

Avant de compiler ou de tester le projet, assurez-vous d’avoir installé les sources de Rust nécessaires :

    
bash
    rustup component add rust-src --toolchain nightly
    rustup component add miri
    rustup default nightly


Compilation

Pour compiler le projet, exécutez la commande suivante :
    
bash
    cargo build


Exécution des Tests:
Pour lancer les tests unitaires et vérifier le bon fonctionnement de l’allocateur, utilisez la commande suivante :
    
bash
    cargo test


Pour des analyses plus poussées et la détection d’erreurs de comportement indéfini, utilisez Miri 
Pour activer les tests Miri avec une capture détaillée des erreurs :
    
bash
    MIRIFLAGS="-Zmiri-backtrace=full" cargo miri test -- --nocapture


Nettoyage du Projet

Pour nettoyer les fichiers générés par la compilation :
    
    cargo clean

Résultats des Tests

Des tests unitaires sont fournis pour vérifier les cas suivants :

    Allocation de blocs mémoire.
    Libération et réutilisation de blocs.
    Gestion de la saturation mémoire.
    Vérification des compteurs de blocs libres.
    Allocation jusqu'à saturation.
    Refus d'allocation en cas de saturation.
    Suivi dynamique des blocs libres.
    Double désallocation.
    Utilisation après libération.
    Performance mesurée pour l’allocation de 128 blocs : environ 4 µs.
    Performance mesurée pour l’allocation et désallocation partielle de 128 blocs : environ 5 µs.
    Vérification avec Miri : aucune erreur de mémoire ou comportement indéfini détecté.

Tous ces tests sont documentés dans le fichier REPORT_TEST.md et sont conçus pour garantir la fiabilité de l’allocateur dans divers scénarios.