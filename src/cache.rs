//! 
//! # Fonctionnalités
//! 
//! - Stockage en mémoire avec capacité fixe
//! - Politique d'éviction LRU automatique
//! - Support de types génériques pour les clés et valeurs
//! - Mise à jour automatique de l'ordre d'accès

use std::collections::HashMap;
use std::hash::Hash;

/// Structure principale du cache LRU.
/// 
/// # Paramètres de type
/// 
/// * `K` - Type de la clé, doit implémenter `Hash`, `Eq` et `Clone`
/// * `V` - Type de la valeur, doit implémenter `Clone`
/// 
/// # Champs
/// 
/// * `capacity` - Nombre maximum d'éléments que le cache peut contenir
/// * `storage` - Table de hachage stockant les paires clé-valeur
/// * `access_order` - Vecteur maintenant l'ordre d'accès aux éléments
pub struct Cache<K, V> {
    capacity: usize,
    storage: HashMap<K, V>,
    access_order: Vec<K>,
}

impl<K, V> Cache<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Crée un nouveau cache avec la capacité spécifiée.
    /// 
    /// # Arguments
    /// 
    /// * `capacity` - Capacité maximale du cache
    /// 
    /// # Retourne
    /// 
    /// Une nouvelle instance de `Cache<K, V>` vide
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            storage: HashMap::with_capacity(capacity),
            access_order: Vec::with_capacity(capacity),
        }
    }

    /// Insère une nouvelle paire clé-valeur dans le cache.
    /// 
    /// # Comportement
    /// 
    /// - Si la clé existe déjà, la valeur est mise à jour
    /// - Si le cache est plein, l'élément le moins récemment utilisé est supprimé
    /// - La clé est placée en fin de liste des accès
    /// 
    /// # Arguments
    /// 
    /// * `key` - Clé à insérer
    /// * `value` - Valeur associée à la clé
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
    }

    /// Récupère une référence à la valeur associée à la clé.
    /// 
    /// # Comportement
    /// 
    /// - Met à jour l'ordre d'accès en plaçant la clé en fin de liste
    /// - Ne modifie pas la valeur stockée
    /// 
    /// # Arguments
    /// 
    /// * `key` - Clé à rechercher
    /// 
    /// # Retourne
    /// 
    /// * `Some(&V)` - Référence à la valeur si la clé existe
    /// * `None` - Si la clé n'existe pas dans le cache
    pub fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(value) = self.storage.get(key) {
            if let Some(idx) = self.access_order.iter().position(|k| k == key) {
                let k = self.access_order.remove(idx);
                self.access_order.push(k);
            }
            Some(value)
        } else {
            None
        }
    }
}