/// Module principal de la bibliothèque de cache LRU.
/// 
/// # Fonctionnalités principales
/// 
/// - Cache LRU en mémoire
/// - Cache LRU persistant
/// - Service API simulé
/// 
/// # Exemple d'utilisation
/// 
/// ```rust
/// use lru_cache::Cache;
/// 
/// let mut cache = Cache::new(3);
/// cache.put("clé1", "valeur1");
/// assert_eq!(cache.get(&"clé1"), Some(&"valeur1"));
/// ```
pub mod cache;

/// Module contenant le service API simulé pour les tests.
pub mod api_service;

/// Module implémentant le cache persistant.
pub mod persistent_cache;

pub use cache::Cache;
pub use persistent_cache::PersistentCache;

#[cfg(test)]
mod tests {
    use super::Cache;

    #[test]
    fn test_lru_cache() {
        let mut cache = Cache::new(3);
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));

        assert_eq!(cache.get(&"A"), None);
        assert_eq!(cache.get(&"D"), Some(&String::from("value_d")));
        assert_eq!(cache.get(&"B"), Some(&String::from("value_b")));
    }
}