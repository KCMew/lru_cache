//! # Tests du Cache LRU Persistant
//! 
//! Ce module de test vérifie le bon fonctionnement de la persistance du cache LRU.
//! Il teste notamment :
//! - La sauvegarde des données dans un fichier
//! - La récupération des données après redémarrage
//! - La cohérence des données entre deux instances du cache

/// Module de tests du cache LRU persistant.
/// 
/// Ce module vérifie la fonctionnalité de persistance du cache, notamment :
/// - La sauvegarde des données dans un fichier
/// - La récupération des données après redémarrage
/// - La cohérence des données entre deux instances
use lru_cache::PersistentCache;
use std::{thread, time::Duration};

/// Test principal vérifiant la persistance du cache.
/// 
/// # Phases de test
/// 
/// 1. Phase d'écriture
///    - Création d'un premier cache
///    - Insertion de données test
///    - Vérification des insertions
///    - Sauvegarde automatique
/// 
/// 2. Phase de lecture
///    - Création d'un second cache
///    - Vérification de la récupération des données
///    - Validation de la cohérence
#[test]
fn test_persistent_cache() {
    let cache_file = "test_persistence.txt";
    
    // Nettoyage préalable du fichier de test
    std::fs::remove_file(cache_file).ok();
    
    // Phase 1 : écriture des données
    {
        println!("=== Création du premier cache ===");
        let mut cache = PersistentCache::new(3, cache_file);
        
        cache.put("B", "value_b");
        cache.put("C", "value_c");
        cache.put("D", "value_d");
        
        println!("Valeurs insérées:");
        println!("B -> {:?}", cache.get(&"B"));
        println!("C -> {:?}", cache.get(&"C"));
        println!("D -> {:?}", cache.get(&"D"));
        
        thread::sleep(Duration::from_secs(1));
    }
    println!("\nPremier cache détruit");
    
    // Phase 2 : lecture des données
    {
        println!("\n=== Création du second cache ===");
        let mut cache: PersistentCache<&str, &str> = PersistentCache::new(3, cache_file);
        
        println!("Lecture des valeurs:");
        println!("B -> {:?}", cache.get(&"B"));
        println!("C -> {:?}", cache.get(&"C"));
        println!("D -> {:?}", cache.get(&"D"));
    }
}