use std::collections::HashMap;
use std::time::Duration;

/// Service API simulé pour les tests et benchmarks.
/// 
/// # Champs
/// 
/// * `data` - Données pré-configurées pour simulation
/// 
/// # Exemple
/// 
/// ```rust
/// use lru_cache::api_service::ApiService;
/// 
/// let api = ApiService::new();
/// let value = api.fetch_data("A");
/// ```
pub struct ApiService {
    data: HashMap<String, String>,
}

impl ApiService {
    /// Crée une nouvelle instance du service API avec des données pré-configurées.
    pub fn new() -> Self {
        let mut data = HashMap::new();
        data.insert("A".to_string(), "value_a".to_string());
        data.insert("B".to_string(), "value_b".to_string());
        data.insert("C".to_string(), "value_c".to_string());
        ApiService { data }
    }

    /// Simule une requête API avec un délai artificiel.
    /// 
    /// # Arguments
    /// 
    /// * `key` - Clé de la donnée à récupérer
    /// 
    /// # Retourne
    /// 
    /// * `Some(String)` - Donnée trouvée
    /// * `None` - Donnée non trouvée
    pub fn fetch_data(&self, key: &str) -> Option<String> {
        std::thread::sleep(Duration::from_millis(100));
        self.data.get(key).cloned()
    }
}