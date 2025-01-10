Bilan de la Journée 1

Résultats attendus :

    Le projet est configuré en mode no_std.
    La structure de l’allocateur est prête avec l’initialisation des blocs libres.
    Le projet compile sans erreur.

Prochaine étape (Jour 2) : Implémenter les fonctions alloc() et dealloc().
Bilan de la Journée 2

Modifications et Ajouts Réalisés :

    Implémentation des Fonctions Essentielles :
        alloc() : Gestion de l’allocation de blocs mémoire.
        dealloc() : Gestion de la libération de blocs mémoire.

    Fichiers Modifiés et Créés :
        src/allocator.rs : Implémentation complète des fonctions d’allocation et de désallocation.
        src/lib.rs : Import du module allocator.
        tests/allocator_test.rs : Ajout de tests unitaires pour valider les fonctionnalités.
        REPORT_TEST.md : Résultats détaillés des tests.

    Tests Unitaires Ajoutés :
        Allocation d’un bloc mémoire.
        Libération et réutilisation d’un bloc.
        Allocation jusqu’à saturation.
        Gestion de l’erreur en cas de saturation.

    Résultats des tests :
        Tous les tests passent avec cargo test.

Résultat Final :

    Les fonctions alloc() et dealloc() sont entièrement fonctionnelles.
    Tests couvrant les cas simples et les erreurs ajoutés et validés.

Bilan de la Journée 3

Modifications et Ajouts Réalisés :

    Ajout d’un compteur de blocs libres :
        Suivi précis du nombre de blocs alloués et disponibles.
        Mise à jour automatique lors des appels à alloc() et dealloc().

    Tests supplémentaires :
        Vérification de l’évolution du compteur après chaque allocation et désallocation.
        Validation que le compteur ne descend pas en dessous de zéro ni ne dépasse le nombre total de blocs.

    Fichiers Modifiés :
        src/allocator.rs : Ajout du compteur free_count et des mises à jour dans les fonctions existantes.
        src/lib.rs : Ajout de tests unitaires pour valider le comportement du compteur.

    Résultats des tests :
        Tous les tests passent avec cargo test, y compris les nouveaux tests liés au compteur.

Résultat Final :

    L’allocateur est maintenant capable de suivre l’état précis de ses blocs libres.
    Tous les tests confirment le bon fonctionnement de l’allocateur avec la nouvelle gestion du compteur.

Prochaine Étape : Jour 4

Objectifs :

    Refactoriser le code pour améliorer sa clarté et sa maintenabilité.
    Ajouter des commentaires et une documentation complète.
    Étendre les tests pour couvrir des cas de fragmentation mémoire.