/// Module de tests de performance comparant les accès avec et sans cache.
/// 
/// Ce benchmark permet de mesurer et comparer :
/// - Le temps d'accès direct au service API
/// - Le temps d'accès via le cache LRU
use lru_cache::{Cache, api_service::ApiService};
use criterion::{black_box, criterion_group, criterion_main, Criterion};

/// Fonction principale de benchmark comparant les performances.
/// 
/// # Structure du test
/// 
/// - Initialisation d'un cache de taille 3
/// - Pré-remplissage avec des données test
/// - Mesure des temps d'accès dans les deux scénarios
/// 
/// # Métriques mesurées
/// 
/// - Temps d'accès direct à l'API (avec latence simulée)
/// - Temps d'accès via le cache (lecture en mémoire)
fn bench_cache(c: &mut Criterion) {
    let mut cache = Cache::new(3);
    let api = ApiService::new();
    
    // Pré-remplissage du cache
    cache.put("B", api.fetch_data("B").unwrap());
    cache.put("C", api.fetch_data("C").unwrap());
    cache.put("D", api.fetch_data("D").unwrap());

    c.bench_function("direct_api_access", |b| {
        b.iter(|| {
            black_box(api.fetch_data("B"));
        })
    });

    c.bench_function("cached_access", |b| {
        b.iter(|| {
            black_box(cache.get(&"B"));
        })
    });
}

criterion_group!(benches, bench_cache);
criterion_main!(benches);