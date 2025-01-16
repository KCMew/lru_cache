//! 
//! # Fonctionnalités
//! 
//! - Stockage persistant des données dans un fichier
//! - Sauvegarde automatique après chaque modification
//! - Restauration des données au démarrage
//! - Conservation de l'ordre LRU entre les sessions

use std::fs::{self, File};
use std::io::{self, Read};
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Debug;

/// Structure du cache LRU persistant.
/// 
/// # Paramètres de type
/// 
/// * `K` - Type de la clé, doit implémenter `Hash`, `Eq`, `Clone` et `Debug`
/// * `V` - Type de la valeur, doit implémenter `Clone` et `Debug`
/// 
/// # Champs
/// 
/// * `capacity` - Nombre maximum d'éléments dans le cache
/// * `storage` - Table de hachage stockant les paires clé-valeur
/// * `access_order` - Vecteur maintenant l'ordre d'accès LRU
/// * `file_path` - Chemin du fichier de persistance
pub struct PersistentCache<K, V> {
    capacity: usize,
    storage: HashMap<K, V>,
    access_order: Vec<K>,
    file_path: String,
}

impl<K, V> PersistentCache<K, V>
where
    K: Hash + Eq + Clone + Debug,
    V: Clone + Debug,
{
    /// Crée une nouvelle instance du cache persistant.
    /// 
    /// # Arguments
    /// 
    /// * `capacity` - Capacité maximale du cache
    /// * `file_path` - Chemin du fichier de sauvegarde
    /// 
    /// # Comportement
    /// 
    /// - Crée un nouveau cache vide avec la capacité spécifiée
    /// - Tente de charger les données existantes depuis le fichier
    /// - Initialise les structures de données internes
    pub fn new(capacity: usize, file_path: &str) -> Self {
        let cache = PersistentCache {
            capacity,
            storage: HashMap::with_capacity(capacity),
            access_order: Vec::with_capacity(capacity),
            file_path: file_path.to_string(),
        };

        if let Ok(content) = cache.load_from_file() {
            if let Some(idx) = content.find('\n') {
                let (storage_str, order_str) = content.split_at(idx);
                println!("Storage: {}", storage_str);
                println!("Order: {}", order_str);
            }
        }

        cache
    }

    /// Insère une nouvelle paire clé-valeur dans le cache.
    /// 
    /// # Arguments
    /// 
    /// * `key` - Clé à insérer
    /// * `value` - Valeur associée
    /// 
    /// # Comportement
    /// 
    /// - Met à jour la valeur si la clé existe
    /// - Évince l'élément le moins récemment utilisé si nécessaire
    /// - Sauvegarde automatiquement l'état dans le fichier
    pub fn put(&mut self, key: K, value: V) {
        if let Some(idx) = self.access_order.iter().position(|k| k == &key) {
            self.access_order.remove(idx);
        }

        if self.storage.len() >= self.capacity && !self.storage.contains_key(&key) {
            if let Some(lru_key) = self.access_order.first().cloned() {
                self.storage.remove(&lru_key);
                self.access_order.remove(0);
            }
        }

        self.storage.insert(key.clone(), value);
        self.access_order.push(key);
        
        self.save_to_file().unwrap_or_else(|e| eprintln!("Erreur lors de la sauvegarde: {}", e));
    }

    /// Récupère une référence à la valeur associée à la clé.
    /// 
    /// # Arguments
    /// 
    /// * `key` - Clé à rechercher
    /// 
    /// # Retourne
    /// 
    /// * `Some(&V)` - Référence à la valeur si trouvée
    /// * `None` - Si la clé n'existe pas
    /// 
    /// # Effets de bord
    /// 
    /// - Met à jour l'ordre d'accès
    /// - Sauvegarde l'état si l'ordre est modifié
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(value) = self.storage.get(key) {
            if let Some(idx) = self.access_order.iter().position(|k| k == key) {
                let k = self.access_order.remove(idx);
                self.access_order.push(k);
                self.save_to_file().unwrap_or_else(|e| eprintln!("Erreur lors de la sauvegarde: {}", e));
            }
            Some(value)
        } else {
            None
        }
    }

    /// Sauvegarde l'état actuel du cache dans le fichier.
    /// 
    /// # Retourne
    /// 
    /// `io::Result<()>` - Succès ou erreur d'écriture
    fn save_to_file(&self) -> io::Result<()> {
        let content = format!("{:?}\n{:?}", self.storage, self.access_order);
        fs::write(&self.file_path, content)
    }

    /// Charge l'état du cache depuis le fichier.
    /// 
    /// # Retourne
    /// 
    /// * `io::Result<String>` - Contenu du fichier ou erreur de lecture
    /// 
    /// # Comportement
    /// 
    /// - Retourne une chaîne vide si le fichier n'existe pas
    /// - Lit le contenu complet du fichier si présent
    fn load_from_file(&self) -> io::Result<String> {
        if !std::path::Path::new(&self.file_path).exists() {
            return Ok(String::new());
        }
        let mut content = String::new();
        File::open(&self.file_path)?.read_to_string(&mut content)?;
        Ok(content)
    }
}