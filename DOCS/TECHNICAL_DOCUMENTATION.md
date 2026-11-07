# Garage Compta - Documentation Technique

## Résumé

Ce document décrit l'architecture technique, les règles de gestion et les implémentations du projet Garage Compta, un logiciel de comptabilité pour garages automobiles.

---

## 1. Architecture Technique

### Stack Technique
- **Langage**: Rust 2021 Edition
- **Framework Web**: Axum 0.7.x
- **UI**: Leptos (mode CSR)
- **Base de données**: SQLite avec SQLx 0.7
- **Runtime**: Tokio
- **Décimaux**: rust_decimal
- **Authentification**: JWT

### Structure du Workspace
```
garage-compta/
├── core/          # Logique métier et modèles
├── data/          # Couche persistance (SQLx)
├── api/           # API REST (Axum)
├── ui/            # Interface Web (Leptos)
└── cli/           # Interface CLI
```

---

## 2. Règles de Gestion

### Facturation
1. **Numérotation**: Format `FACT-YYYY-NNNN` (continue, sans trou)
2. **TVA**: 20% standard, calcul automatique
3. **Statuts**: Brouillon → Validé → Payé/Annulé
4. **Échéance**: Date facture + conditions de paiement

### Véhicules
1. **Propriété**: Chaque véhicule a un propriétaire actuel
2. **Historique**: `VehicleOwnerHistory` trace tous les changements
3. **Immatriculation**: Unique dans la base

### Pièces
1. **Stock**: Suivi quantité + alerte seuil minimum
2. **Prix**: Prix d'achat + marge = prix de vente
3. **Catégories**: Pièce, Consommable, Service, Main d'œuvre

### Clients
1. **Solde**: Total factures non payées - total paiements
2. **Types**: Particulier, Professionnel, Entreprise
3. **Conditions**: Comptant, Net30, Net60, Net90

### Fournisseurs
1. **Import**: CSV/XLS avec calcul automatique prix de vente
2. **Marge**: Configurable par fournisseur (défaut: 30%)

### Comptabilité
1. **FEC Export**: Format compatible Sage 500/Ciel
2. **TVA**: Collectée (44571) et déductible (44566)

---

## 3. Implémentations Clés

### SQLx avec Decimal
```rust
// Storage: Decimal as TEXT in SQLite
// Retrieval: Use Text<Decimal> wrapper
row.get::<Text<Decimal>, _>("price").map(|t| t.0)?
```

### Axum Router avec État
```rust
// Problème: Router<AppState> ne peut pas être utilisé directement avec axum::serve
// Solution: Convertir en Router<()> avec .with_state()
let app: Router<AppState> = create_router(state.clone());
let app_with_state: Router<()> = app.with_state(state);
axum::serve(listener, app_with_state).await?;
```

### Numérotation Factures
```rust
// File-based locking pour éviter les conflits multi-post
pub struct InvoiceNumberingService {
    lock_file: PathBuf,
}

impl InvoiceNumberingService {
    pub async fn next_number(&self, year: i32) -> Result<String, CoreError> {
        // Verrouillage fichier
        // Lecture dernier numéro
        // Incrémentation
        // Sauvegarde
        Ok(format!("FACT-{}-{:04}", year, next_num))
    }
}
```

---

## 4. Commandes de Test

### Démarrer l'API
```bash
cargo run -p garage-compta-api
```

### Tester avec curl
```bash
# Health check
curl http://localhost:3000/health

# Liste des clients (nécessite authentification)
curl -H "Authorization: Bearer <token>" http://localhost:3000/api/clients
```

---

## 5. Prochaines Étapes

1. ✅ **API compile et démarre**
2. ⏳ Nettoyer les warnings dans `garage-compta-data`
3. ⏳ Tester tous les endpoints avec curl
4. ⏳ Connecter l'UI au backend
5. ⏳ Implémenter les repositories SQLx
6. ⏳ Tester les migrations
7. ⏳ Valider l'import CSV/XLS
8. ⏳ Tester l'export FEC

---

*Documentation technique - Garage Compta v0.1.0*
