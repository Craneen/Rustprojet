Commandes Nécessaires pour Compiler et Tester

    Installer les sources de Rust :

rustup component add rust-src --toolchain nightly

    Nettoyer le projet (optionnel) :

cargo clean

    Compiler le projet en no_std :

cargo build -Z build-std=core,alloc --target x86_64-unknown-linux-gnu

    Exécuter les tests unitaires :

cargo test -Z build-std=core,alloc --target x86_64-unknown-linux-gnu

---

### **2️ Complément du `markdown.txt` pour le Jour 2**

```markdown
---

### **Bilan de la Journée 1**

**Résultats attendus :**  
- Le projet est configuré en mode `no_std`.  
- La structure de l’allocateur est prête avec l’initialisation des blocs libres.  
- Le projet compile sans erreur.  

**Prochaine étape (Jour 2)** : Implémenter les fonctions **`alloc()`** et **`dealloc()`**.

---

### **Bilan de la Journée 2**

Modifications et Ajouts Réalisés

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
        Gestion de l’erreur en cas de saturation mémoire

**Prochaine étape (Jour 3)** : Optimisation des performances et gestion des cas limites.



