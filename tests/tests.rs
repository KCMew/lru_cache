use lru_cache::Cache;

/// Test du cache LRU.
/// 
/// Ce test vérifie le comportement complet du cache avec :
/// - L'ajout initial d'éléments
/// - La vérification des valeurs
/// - Le dépassement de capacité
/// - L'ordre LRU
#[test]
fn test_cache_detailed() {
    println!("\n=== Début du test détaillé ===");
    
    let mut cache = Cache::new(3);
    println!("Cache créé avec une capacité de 3");

    // Test 1: Ajout initial
    println!("\n1. Test d'ajout initial:");
    cache.put("A", String::from("value_a"));
    println!("  Ajouté: A -> value_a");
    cache.put("B", String::from("value_b"));
    println!("  Ajouté: B -> value_b");
    cache.put("C", String::from("value_c"));
    println!("  Ajouté: C -> value_c");

    // Test 2: Vérification des valeurs
    println!("\n2. Vérification des valeurs initiales:");
    assert_eq!(cache.get(&"A"), Some(&String::from("value_a")));
    println!("  ✓ A est présent avec value_a");
    assert_eq!(cache.get(&"B"), Some(&String::from("value_b")));
    println!("  ✓ B est présent avec value_b");
    assert_eq!(cache.get(&"C"), Some(&String::from("value_c")));
    println!("  ✓ C est présent avec value_c");

    // Test 3: Dépassement de capacité
    println!("\n3. Test de dépassement de capacité:");
    cache.put("D", String::from("value_d"));
    println!("  Ajouté: D -> value_d (devrait évincer A)");
    
    assert_eq!(cache.get(&"A"), None);
    println!("  ✓ A a bien été évincé");
    assert_eq!(cache.get(&"D"), Some(&String::from("value_d")));
    println!("  ✓ D est bien présent");

    // Test 4: Test de l'ordre LRU
    println!("\n4. Test de l'ordre LRU:");
    let _ = cache.get(&"B");
    println!("  Accès à B (devrait le déplacer en fin de liste)");
    cache.put("E", String::from("value_e"));
    println!("  Ajouté: E -> value_e (devrait évincer C)");

    assert_eq!(cache.get(&"C"), None);
    println!("  ✓ C a bien été évincé");
    assert_eq!(cache.get(&"B"), Some(&String::from("value_b")));
    println!("  ✓ B est toujours présent car récemment utilisé");

    println!("\n=== Fin du test détaillé ===");
}

/// Test de compatibilité avec différents types de données.
/// 
/// Vérifie que le cache fonctionne correctement avec :
/// - Des entiers
/// - Des nombres flottants
/// - D'autres types de données
#[test]
fn test_different_types() {
    println!("\n=== Test avec différents types ===");
    
    // Test avec des entiers
    let mut cache_int = Cache::new(2);
    println!("\n1. Test avec des entiers:");
    cache_int.put("nombre1", 42);
    println!("  Ajouté: nombre1 -> 42");
    cache_int.put("nombre2", 84);
    println!("  Ajouté: nombre2 -> 84");
    
    assert_eq!(cache_int.get(&"nombre1"), Some(&42));
    println!("  ✓ nombre1 retrouvé correctement");

    // Test avec des flottants
    let mut cache_float = Cache::new(2);
    println!("\n2. Test avec des flottants:");
    cache_float.put("pi", 3.14);
    println!("  Ajouté: pi -> 3.14");
    cache_float.put("e", 2.71);
    println!("  Ajouté: e -> 2.71");
    
    assert_eq!(cache_float.get(&"pi"), Some(&3.14));
    println!("  ✓ pi retrouvé correctement");

    println!("\n=== Fin du test avec différents types ===");
}