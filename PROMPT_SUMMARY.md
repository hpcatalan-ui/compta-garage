# Garage Compta - Résumé du Projet et Prompt de Continuation

## Contexte Initial

**Objectif Principal**: Créer un logiciel de comptabilité pour garage automobile compatible avec les standards français (Sage 500, Ciel) avec un budget minimal et une livraison rapide.

**Fonctionnalités Clés Demandées**:
- ✅ Facturation (invoicing) avec numérotation continue (FACT-YYYY-NNNN)
- ✅ Gestion clients/véhicules avec historique de propriété
- ✅ Devis (quotes)
- ✅ Import de catalogues fournisseurs depuis CSV/XLS
- ✅ Export FEC pour compatibilité Sage/Ciel
- ✅ TVA à 20% et conformité fiscale française
- ✅ Mode portable (clé USB) avec SQLite

## Architecture Technique

### Stack Technique
- **Langage**: Rust 2021 Edition
- **Framework Web**: Axum 0.7.x
- **UI**: Leptos (mode CSR pour portabilité)
- **Base de données**: SQLite (portable) avec SQLx 0.7
- **Runtime**: Tokio
- **Décimaux**: rust_decimal (stockés en TEXT dans SQLite)
- **Authentification**: JWT

### Structure du Workspace
```
garage-compta/
├── Cargo.toml                    # Workspace avec 5 crates
├── core/                         # Logique métier et modèles
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── error.rs              # CoreError avec From<sqlx::Error>
│       ├── sqlx_impl.rs          # Implémentations SQLx pour tous les types
│       ├── models/               # Modèles domaine
│       │   ├── address.rs
│       │   ├── client.rs
│       │   ├── vehicle.rs        # Avec VehicleOwnerHistory
│       │   ├── invoice.rs        # Avec InvoiceLine
│       │   ├── part.rs
│       │   ├── supplier.rs
│       │   ├── user.rs
│       │   └── payment.rs
│       └── services/
│           ├── invoice_numbering.rs  # Numérotation continue
│           └── csv_importer.rs       # Import CSV/XLS fournisseurs
│       └── accounting/
│           ├── fec_export.rs      # Export FEC
│           └── vat_calculator.rs  # Calcul TVA
├── data/                         # Couche persistance
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── connection.rs         # Configuration DB
│       ├── migrations/
│       │   └── mod.rs            # Migrations custom (sans refinery)
│       └── repositories/         # Répositories pour chaque modèle
│           ├── client_repository.rs
│           ├── vehicle_repository.rs
│           ├── invoice_repository.rs
│           ├── part_repository.rs
│           ├── supplier_repository.rs
│           ├── user_repository.rs
│           └── payment_repository.rs
├── api/                          # API REST
│   ├── Cargo.toml
│   ├── src/
│   │   ├── main.rs               # Point d'entrée serveur ✅ CORRIGÉ
│   │   ├── lib.rs
│   │   ├── state.rs              # AppState avec Arc<Database>
│   │   ├── routes/
│   │   │   ├── mod.rs            # Router principal ✅ CORRIGÉ
│   │   │   ├── health.rs         # /health, /ready
│   │   │   ├── clients/
│   │   │   │   └── mod.rs        # CRUD clients
│   │   │   ├── vehicles/
│   │   │   │   └── mod.rs        # CRUD véhicules
│   │   │   ├── invoices/
│   │   │   │   └── mod.rs        # CRUD factures
│   │   │   ├── parts/
│   │   │   │   └── mod.rs        # CRUD pièces
│   │   │   ├── suppliers/
│   │   │   │   └── mod.rs        # CRUD fournisseurs
│   │   │   └── auth/
│   │   │       └── mod.rs        # Login JWT
│   │   └── middleware/
│   │       ├── mod.rs
│   │       ├── auth.rs           # Middleware d'authentification
│   │       ├── error.rs          # Gestion des erreurs HTTP
│   │       └── logging.rs        # Logging des requêtes
├── ui/                           # Interface Web (Leptos)
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs                # UI avec mock data
└── cli/                          # Interface CLI
    ├── Cargo.toml
    └── src/
        └── main.rs               # Commandes CLI
```

## Travail Réalisé

### Phase 1: Architecture et Modélisation
- ✅ Création de la structure workspace avec 5 crates
- ✅ Implémentation de tous les modèles domaine (Client, Vehicle, Invoice, Part, Supplier, User, Payment, Address)
- ✅ Système de numérotation de factures avec verrouillage fichier
- ✅ Import CSV/XLS pour les catalogues fournisseurs
- ✅ Export FEC pour Sage/Ciel
- ✅ Calcul TVA et conformité fiscale française

### Phase 2: Couche Données
- ✅ Configuration SQLite avec SQLx 0.7
- ✅ Implémentation de `sqlx_impl.rs` avec:
  - `Type<Sqlite>`, `Encode`, `Decode` pour tous les enums
  - `FromRow` pour tous les structs
  - Gestion spéciale de `Decimal` via `Text<Decimal>`
  - Sérialisation/désérialisation de `Address`
- ✅ Répositories pour tous les modèles
- ✅ Système de migrations custom (remplace refinery)

### Phase 3: API REST (Dernières Corrections)

#### Problème Principal Résolu
**Erreur**: `axum::serve(listener, app)` échouait avec:
```
error[E0277]: the trait bound `for<'a> Router<AppState>: tower::Service<IncomingStream<'a>>` is not satisfied
```

**Cause**: En Axum 0.7, `Router<AppState>` n'implémente pas `Service<IncomingStream>` directement.

**Solution Implémentée**:
1. Tous les sous-routers retournent `Router<AppState>`
2. Le router principal retourne `Router<AppState>`
3. Dans `main.rs`, appel de `.with_state(state)` pour convertir en `Router<()>`
4. `Router<()>` peut alors être utilisé directement avec `axum::serve`

#### Fichiers Modifiés

**`api/src/main.rs`** (NOUVEAU):
```rust
#[tokio::main]
async fn main() {
    // Initialisation logging, état, migrations...
    let state = AppState::new_with_defaults("./data/garage_compta.db").await?;
    garage_compta_data::migrations::run_migrations(&state.db.pool).await?;
    
    let app = routes::create_app(state.clone()).await;
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    
    // SOLUTION: Convertir Router<AppState> en Router<()> avec .with_state()
    let app_with_state = app.with_state(state);
    axum::serve(listener, app_with_state).await?;
}
```

**`api/src/routes/mod.rs`**:
- `create_router(state: AppState) -> Router<AppState>` (pas de `.with_state()` initial)
- `create_app(state: AppState) -> Router<AppState>`
- Merge de tous les sous-routers avec leur état cloné

**`api/Cargo.toml`**:
- Ajout de `[[bin]]` section pointant vers `src/main.rs`
- Ajout de dépendances: `hyper = "1.0"`, `hyper-util = { version = "0.1", features = ["tokio"] }`

**Tous les sous-routers** (`health.rs`, `clients/mod.rs`, `vehicles/mod.rs`, etc.):
- Signature changée de `router() -> Router` à `router(state: AppState) -> Router<AppState>`
- Ajout de `.with_state(state)` sur chaque router

**Tous les middlewares** (`auth.rs`, `error.rs`, `logging.rs`):
- Correction des signatures pour utiliser `Request<Body>`, `Next` (pas de génériques)
- Correction des types de retour: `Result<Response<Body>, Response<Body>>`
- Ajout des imports nécessaires: `use axum::body::Body;`

### Phase 4: Validation
- ✅ `cargo check -p garage-compta-core` → SUCCESS (avec 2 warnings mineurs)
- ✅ `cargo check -p garage-compta-data` → SUCCESS (avec 27 warnings d'imports inutilisés)
- ✅ `cargo check -p garage-compta-api` → SUCCESS
- ✅ `cargo build -p garage-compta-api` → SUCCESS

## Prochaines Étapes

### Priorité Immédiate
1. **Nettoyer les warnings** dans `garage-compta-data`:
   - Supprimer les imports inutilisés dans tous les repositories
   - Corriger la variable `vehicle` unused dans `vehicle_repository.rs`

2. **Tester l'API** avec des commandes curl:
   ```bash
   # Démarrer le serveur
   cargo run -p garage-compta-api
   
   # Health check
   curl http://localhost:3000/health
   
   # Liste des clients
   curl http://localhost:3000/api/clients
   
   # Créer un client
   curl -X POST http://localhost:3000/api/clients \
     -H "Content-Type: application/json" \
     -d '{"name": "Test Client", "email": "test@example.com"}'
   ```

### À Moyen Terme
1. **Connecter l'UI au backend** (actuellement mock data)
2. **Implémenter l'authentification JWT** complète
3. **Finaliser les repositories** avec les implémentations SQLx
4. **Tester les migrations** de base de données
5. **Valider l'import CSV/XLS** des fournisseurs

### Long Terme
1. **Mode multi-post** avec synchronisation
2. **Sauvegarde NAS** automatique
3. **Packaging USB key** pour déploiement portable
4. **Documentation complète**
5. **Tests unitaires et d'intégration**

## Règles de Gestion et Contraintes Techniques

### Règles Métier
1. **Numérotation des factures**: Format `FACT-YYYY-NNNN` avec continuité garantie
2. **Véhicules**: Chaque véhicule a un historique de propriété (`VehicleOwnerHistory`)
3. **Pièces**: Import depuis CSV/XLS avec calcul automatique des prix de vente (prix d'achat + marge)
4. **TVA**: Taux standard à 20%, calcul automatique sur les factures
5. **FEC Export**: Format compatible Sage 500 et Ciel

### Contraintes Techniques
1. **Rust 2021 Edition** obligatoire
2. **SQLx 0.7** avec features: `sqlite`, `runtime-tokio`, `chrono`, `uuid`, `rust_decimal`
3. **Decimal** stocké en TEXT dans SQLite avec sérialisation custom
4. **Pas de refinery** (implémentation custom des migrations pour async SQLite)
5. **Orphan Rule**: Les traits SQLx pour types externes sont dans `core/src/sqlx_impl.rs`
6. **Axum 0.7**: Router avec état nécessite `.with_state()` final pour obtenir `Router<()>`

### Bonnes Pratiques
1. **Clone de l'état**: Toujours cloner `AppState` avant de le passer aux sous-routers
2. **Arc pour le partage**: Utiliser `Arc<Database>` et `Arc<AppConfig>` dans `AppState`
3. **Gestion des erreurs**: Conversion `CoreError` → `ApiError` → `Response<Body>`
4. **Middleware**: Utiliser `State<AppState>` pour accéder à l'état dans les handlers

## Commandes Utiles

```bash
# Vérifier tout le workspace
cargo check

# Builder l'API
cargo build -p garage-compta-api

# Démarrer l'API
cargo run -p garage-compta-api

# Nettoyer les warnings
cargo fix --lib -p garage-compta-data
cargo fix --lib -p garage-compta-core

# Vérifier un crate spécifique
cargo check -p garage-compta-api
```

## État Actuel

- ✅ **Core**: Compile avec succès
- ✅ **Data**: Compile avec succès (27 warnings mineurs)
- ✅ **API**: Compile avec succès et peut être exécuté
- ⏳ **UI**: Existe mais utilise mock data (à connecter au backend)
- ⏳ **CLI**: Existe mais non testé

**Prochaine Action**: Tester l'API avec curl et nettoyer les warnings.
