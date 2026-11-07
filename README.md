# Garage Compta

> **Logiciel de comptabilité pour garages automobiles** - Compatible Sage 500 & Ciel

---

## 📋 Description

Garage Compta est une solution de gestion comptable spécialement conçue pour les garages automobiles. Elle permet de gérer les clients, véhicules, factures, devis, pièces, fournisseurs et bien plus, avec une compatibilité totale avec les logiciels de comptabilité français Sage 500 et Ciel via l'export FEC.

---

## ✨ Fonctionnalités

### 🎯 Principales
- ✅ **Facturation**: Numérotation continue (FACT-YYYY-NNNN), gestion des statuts, calcul automatique TVA
- ✅ **Gestion Clients/Véhicules**: Chaque client peut avoir plusieurs véhicules avec historique de propriété
- ✅ **Devis**: Création, conversion en facture, suivi
- ✅ **Catalogue Pièces**: Import depuis CSV/XLS, calcul automatique des prix de vente avec marge
- ✅ **Fournisseurs**: Gestion complète avec conditions de paiement
- ✅ **Export FEC**: Format compatible Sage 500 et Ciel

### 🔧 Techniques
- **Portable**: Fonctionne sur clé USB avec SQLite
- **Multi-post**: Synchronisation entre plusieurs postes
- **Sauvegarde NAS**: Sauvegarde automatique
- **TVA**: Conformité fiscale française (20%)

---

## 🚀 Installation

### Prérequis
- Rust 1.70+
- SQLite 3
- Tokio runtime

### Étapes

```bash
# Cloner le dépôt
git clone <repository-url>
cd garage-compta

# Builder le projet
cargo build --release

# Le binaire est dans target/release/garage-compta-api
```

### Mode Développement

```bash
# Builder et démarrer l'API
cargo run -p garage-compta-api

# L'API sera accessible sur http://localhost:3000
```

---

## 🏗️ Architecture

```
garage-compta/
├── core/          # Logique métier et modèles
│   ├── src/
│   │   ├── models/       # Entités (Client, Vehicle, Invoice, etc.)
│   │   ├── services/     # Services (numérotation, import CSV, etc.)
│   │   ├── accounting/   # Comptabilité (FEC, TVA)
│   │   └── sqlx_impl.rs  # Implémentations SQLx
├── data/          # Couche persistance
│   ├── src/
│   │   ├── repositories/ # Répositories SQLx
│   │   └── migrations/   # Migrations de base de données
├── api/           # API REST (Axum)
│   ├── src/
│   │   ├── routes/       # Routes API
│   │   ├── middleware/   # Middlewares (auth, logging)
│   │   └── state.rs      # État de l'application
├── ui/            # Interface Web (Leptos)
├── cli/           # Interface CLI
└── INIT_DB.sql    # Script d'initialisation
```

### Stack Technique
- **Langage**: Rust 2021 Edition
- **Framework Web**: Axum 0.7.x
- **UI**: Leptos (mode CSR)
- **Base de données**: SQLite
- **ORM**: SQLx 0.7
- **Runtime**: Tokio
- **Décimaux**: rust_decimal
- **Authentification**: JWT

---

## 📡 API REST

### Endpoints

| Méthode | Endpoint | Description |
|---------|----------|-------------|
| GET | `/health` | Vérification santé |
| GET | `/ready` | Vérification disponibilité |
| POST | `/api/auth/login` | Connexion (JWT) |
| GET/POST/PUT/DELETE | `/api/clients` | Gestion des clients |
| GET/POST/PUT/DELETE | `/api/vehicles` | Gestion des véhicules |
| POST | `/api/vehicles/:id/change-owner` | Changer propriétaire |
| GET/POST/PUT/DELETE | `/api/invoices` | Gestion des factures |
| POST | `/api/invoices/:id/pay` | Enregistrer un paiement |
| GET/POST/PUT/DELETE | `/api/parts` | Gestion des pièces |
| GET/POST/PUT/DELETE | `/api/suppliers` | Gestion des fournisseurs |
| POST | `/api/suppliers/:id/import-prices` | Importer catalogue |
| GET | `/api/reports/fec` | Export FEC |

### Authentification

```bash
# Login
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}'

# Utiliser le token
curl -H "Authorization: Bearer <token>" \
  http://localhost:3000/api/clients
```

---

## 📦 Déploiement

### Mode Portable (Clé USB)

```
garage-compta/
├── garage-compta-api    # Binaire compilé
├── data/                # Base de données
│   └── garage_compta.db
├── config/              # Configuration
└── start.bat            # Script de démarrage
```

### Script de démarrage (Windows)

```bat
@echo off
set PORT=3000
set DATABASE_PATH=./data/garage_compta.db
set JWT_SECRET=votre-secret-secure

start garage-compta-api
```

### Script de démarrage (Linux/Mac)

```bash
#!/bin/bash
export PORT=3000
export DATABASE_PATH=./data/garage_compta.db
export JWT_SECRET=votre-secret

./garage-compta-api
```

---

## 🔧 Configuration

Voir `api/src/state.rs` pour la configuration par défaut:

```rust
AppConfig {
    app_name: "Garage Compta",
    app_version: "0.1.0",
    environment: "development",
    port: 3000,
    jwt_secret: "votre-secret-secure",
    jwt_expiry: 86400,  // 24 heures
    database_path: "./data/garage_compta.db",
    enable_cors: true,
    cors_origins: vec!["http://localhost:3000"],
}
```

---

## 📚 Documentation

- [Documentation Technique](DOCS/TECHNICAL_DOCUMENTATION.md)
- [Guide de Test](TESTING.md)
- [Résumé du Projet](PROMPT_SUMMARY.md)
- [État Actuel](CURRENT_STATUS.md)

---

## 🧪 Tests

### Tester avec cURL

```bash
# Health check
curl http://localhost:3000/health

# Créer un client
curl -X POST http://localhost:3000/api/clients \
  -H "Authorization: Bearer <token>" \
  -H "Content-Type: application/json" \
  -d '{"name": "Jean Dupont", "email": "jean@example.com"}'

# Voir tous les clients
curl -H "Authorization: Bearer <token>" \
  http://localhost:3000/api/clients
```

### Script de test complet

Voir [TESTING.md](TESTING.md) pour un guide complet.

---

## 📖 Modèle de Données

### Entités Principales

- **Client**: Nom, email, téléphone, adresse, type, conditions de paiement, solde
- **Vehicle**: Immatriculation, marque, modèle, année, type de carburant, propriétaire
- **VehicleOwnerHistory**: Historique des changements de propriétaire
- **Invoice**: Numéro, client, véhicule, type, statut, date, lignes, total
- **InvoiceLine**: Description, quantité, prix unitaire, TVA, total
- **Part**: Référence, nom, prix d'achat, prix de vente, stock, fournisseur
- **Supplier**: Nom, contact, adresse, SIRET, TVA, conditions de paiement
- **Payment**: Montant, méthode, date, statut, référence
- **User**: Nom d'utilisateur, email, mot de passe, rôle

---

## 🎨 Interface Utilisateur

L'interface web est développée avec **Leptos** en mode CSR (Client-Side Rendering) pour une portabilité maximale.

### Pages
- Dashboard
- Clients
- Véhicules
- Factures
- Devis
- Pièces
- Fournisseurs
- Rapports
- Paramètres

---

## 🔐 Sécurité

- **Authentification**: JWT avec expiration configurable
- **Hachage**: Argon2 pour les mots de passe
- **Rôles**: Admin, Manager, Technicien, Comptable, Vendeur
- **Permissions**: Contrôle d'accès par rôle

---

## 📊 Comptabilité

### TVA
- Taux standard: 20%
- Taux réduits: 10%, 5.5%, 2.1% (optionnels)
- Calcul automatique sur les factures

### Export FEC
- Format compatible Sage 500 et Ciel
- Inclut: factures, paiements, avoirs
- Période configurable

### Plan Comptable
- 707: Ventes de marchandises
- 44571: TVA collectée
- 44566: TVA déductible
- 512: Banque
- 411: Clients
- 401: Fournisseurs

---

## 💼 Intégration Fournisseurs

### Import CSV/XLS

Format CSV attendu:
```csv
Référence,Nom,Prix d'achat,Description,Catégorie
REF001,Bougie,5.99,Bougie d'allumage,Pièce
REF002,Filtre à huile,12.50,Filtre à huile,Consommable
```

Commande:
```bash
curl -X POST http://localhost:3000/api/suppliers/<id>/import-prices \
  -H "Authorization: Bearer <token>" \
  -F "file=@catalogue.csv"
```

### Calcul des Prix
- Prix de vente = Prix d'achat × (1 + marge/100)
- Marge par défaut: configurable par fournisseur

---

## 📈 Synchronisation Multi-Post

- Synchronisation via fichier partagé (NAS, clé USB)
- Détection des conflits
- Fusion des bases de données
- Fréquence: automatique ou manuelle

---

## 💾 Sauvegarde

### Sauvegarde NAS

```bash
#!/bin/bash
BACKUP_DIR="/mnt/nas/garage-compta-backups"
DATE=$(date +%Y-%m-%d_%H-%M-%S)

sqlite3 ./data/garage_compta.db ".backup ${BACKUP_DIR}/garage_compta_${DATE}.db"

# Supprimer les anciennes sauvegardes (plus de 30 jours)
find ${BACKUP_DIR} -name "garage_compta_*.db" -mtime +30 -delete
```

---

## 🤝 Contribuer

1. Forker le projet
2. Créer une branche (`git checkout -b feature/nouvelle-fonctionnalité`)
3. Commiter vos changements (`git commit -m 'Ajout nouvelle fonctionnalité'`)
4. Pousser vers la branche (`git push origin feature/nouvelle-fonctionnalité`)
5. Ouvrir une Pull Request

---

## 📜 Licence

Ce projet est sous licence **Propriétaire** - Tous droits réservés.

---

## 📞 Support

Pour toute question ou problème, consulter:
- [Documentation Technique](DOCS/TECHNICAL_DOCUMENTATION.md)
- [Guide de Test](TESTING.md)
- [État Actuel](CURRENT_STATUS.md)

---

*Version: 0.1.0*
*Dernière mise à jour: 2024-12-19*
