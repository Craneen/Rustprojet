Résultats des Tests - Slab Allocator

Résultats des Tests - Jour 1

Tests réalisés

    Compilation avec no_std :
    Réussi → Validation de la configuration en mode no_std avec une liste chaînée initialisée.
    Vérification de l’initialisation des blocs libres :
    Réussi → Tous les blocs libres ont été correctement initialisés et enchaînés dans la liste.

Conclusion
Le projet est correctement configuré en mode no_std, et les blocs libres sont prêts pour les étapes suivantes.

Résultats des Tests - Jour 2

Tests réalisés

    Allocation simple :
    Réussi → Un bloc de mémoire peut être alloué correctement.
    Désallocation d'un bloc :
    Réussi → Un bloc alloué peut être libéré et remis dans la liste chaînée.
    Réutilisation d'un bloc libéré :
    Réussi → Un bloc libéré est réutilisé lors d'une nouvelle allocation.
    Allocation jusqu'à saturation :
    Réussi → L'allocateur peut allouer jusqu'à la limite de la mémoire disponible.
    Refus d'allocation en cas de saturation :
    Réussi → L'allocateur renvoie un pointeur nul lorsque toute la mémoire est saturée.

Conclusion
Tous les tests valident le bon fonctionnement de l'allocateur en termes de gestion de l’allocation et de la libération. Les cas simples et les erreurs sont correctement gérés.

Résultats des Tests - Jour 3

Tests réalisés

    Vérification des compteurs de blocs libres :
        Validation que le nombre de blocs libres diminue après chaque allocation réussie.
        Validation que le nombre de blocs libres augmente après chaque libération.
    Test de la mise à jour des structures internes :
        Vérification que la liste des blocs libres reste cohérente et correctement chaînée après plusieurs allocations et libérations successives.
    Simulation de charges complexes :
        Enchaînement d’opérations d’allocation et de désallocation dans des ordres variés pour confirmer la stabilité de l’allocateur.

Conclusion
Les tests du Jour 3 confirment la robustesse des mécanismes internes de l'allocateur. Les structures de données restent fiables sous des conditions d’utilisation plus complexes, et les compteurs de blocs libres reflètent fidèlement l’état de la mémoire.