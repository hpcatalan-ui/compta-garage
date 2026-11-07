# Garage Compta - Guide de Test

## Prérequis

1. Avoir construit l'API:
   ```bash
   cd garage-compta
   cargo build -p garage-compta-api
   ```

2. Démarrer le serveur:
   ```bash
   cargo run -p garage-compta-api
   ```

Le serveur démarrera sur `http://localhost:3000`

---

## Commandes cURL pour Tester l'API

### 1. Vérification de Santé

```bash
# Health check
curl -v http://localhost:3000/health

# Readiness check
curl -v http://localhost:3000/ready
```

**Réponse attendue**:
```json
{
  "status": "healthy",
  "timestamp": "2024-12-19T10:30:00Z"
}
```

---

### 2. Authentification

#### Login (nécessite un utilisateur existant)

Si aucun utilisateur n'existe, il faudra en créer un via la CLI ou directement en base.

```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "username": "admin",
    "password": "admin123"
  }'
```

**Réponse attendue**:
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 86400
}
```

**Stocker le token pour les requêtes suivantes**:
```bash
TOKEN="eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

---

### 3. Gestion des Clients

#### Créer un client

```bash
curl -X POST http://localhost:3000/api/clients \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jean Dupont",
    "email": "jean.dupont@example.com",
    "phone": "+33123456789",
    "address": {
      "street": "123 Rue de Paris",
      "postal_code": "75001",
      "city": "Paris",
      "country": "France",
      "additional": "Appartement 4B"
    },
    "client_type": "Particulier",
    "payment_terms": "Comptant"
  }'
```

**Réponse attendue**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "Jean Dupont",
  "email": "jean.dupont@example.com",
  "phone": "+33123456789",
  "address": {
    "street": "123 Rue de Paris",
    "postal_code": "75001",
    "city": "Paris",
    "country": "France",
    "additional": "Appartement 4B"
  },
  "client_type": "Particulier",
  "payment_terms": "Comptant",
  "balance": "0.00",
  "created_at": "2024-12-19T10:30:00Z",
  "updated_at": "2024-12-19T10:30:00Z"
}
```

#### Lister tous les clients

```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/api/clients
```

#### Obtenir un client spécifique

```bash
CLIENT_ID="550e8400-e29b-41d4-a716-446655440000"
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/api/clients/$CLIENT_ID
```

#### Mettre à jour un client

```bash
curl -X PUT http://localhost:3000/api/clients/$CLIENT_ID \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "Jean Dupont (Mis à jour)",
    "email": "jean.dupont.updated@example.com",
    "phone": "+33123456789"
  }'
```

#### Supprimer un client

```bash
curl -X DELETE http://localhost:3000/api/clients/$CLIENT_ID \
  -H "Authorization: Bearer $TOKEN"
```

---

### 4. Gestion des Véhicules

#### Créer un véhicule

```bash
curl -X POST http://localhost:3000/api/vehicles \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "client_id": "$CLIENT_ID",
    "registration": "AB-123-CD",
    "make": "Peugeot",
    "model": "308",
    "year": 2020,
    "vin": "VF37AMFUE88123456",
    "fuel_type": "Essence",
    "transmission": "Manuelle",
    "category": "Voiture",
    "mileage": 45000,
    "purchase_date": "2020-01-15",
    "purchase_price": 25000.00,
    "notes": "Véhicule bien entretenu"
  }'
```

**Réponse attendue**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440001",
  "client_id": "550e8400-e29b-41d4-a716-446655440000",
  "registration": "AB-123-CD",
  "make": "Peugeot",
  "model": "308",
  "year": 2020,
  "fuel_type": "Essence",
  "transmission": "Manuelle",
  "category": "Voiture",
  "mileage": 45000,
  "purchase_price": "25000.00",
  "created_at": "2024-12-19T10:30:00Z",
  "updated_at": "2024-12-19T10:30:00Z"
}
```

#### Changer le propriétaire d'un véhicule

```bash
VEHICLE_ID="550e8400-e29b-41d4-a716-446655440001"
NEW_OWNER_ID="550e8400-e29b-41d4-a716-446655440002"

curl -X POST http://localhost:3000/api/vehicles/$VEHICLE_ID/change-owner \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "new_owner_id": "$NEW_OWNER_ID",
    "purchase_price": 22000.00,
    "change_date": "2024-01-15",
    "notes": "Vente à nouveau propriétaire"
  }'
```

#### Lister tous les véhicules

```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/api/vehicles
```

---

### 5. Gestion des Fournisseurs

#### Créer un fournisseur

```bash
curl -X POST http://localhost:3000/api/suppliers \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "name": "AutoPièces SA",
    "contact": "Pierre Martin",
    "email": "contact@autopieces.fr",
    "phone": "+33234567890",
    "address": {
      "street": "456 Rue Industrielle",
      "postal_code": "69000",
      "city": "Lyon",
      "country": "France"
    },
    "siret": "12345678901234",
    "vat_number": "FR123456789",
    "payment_terms": "Net30",
    "default_margin": 30.0,
    "notes": "Fournisseur principal"
  }'
```

**Réponse attendue**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440003",
  "name": "AutoPièces SA",
  "email": "contact@autopieces.fr",
  "phone": "+33234567890",
  "address": {
    "street": "456 Rue Industrielle",
    "postal_code": "69000",
    "city": "Lyon",
    "country": "France"
  },
  "siret": "12345678901234",
  "vat_number": "FR123456789",
  "payment_terms": "Net30",
  "default_margin": 30.0,
  "created_at": "2024-12-19T10:30:00Z",
  "updated_at": "2024-12-19T10:30:00Z"
}
```

#### Importer un catalogue fournisseur (CSV)

Créer un fichier `catalogue.csv`:
```csv
Référence,Nom,Prix d'achat,Description,Catégorie
REF001,Bougie,5.99,Bougie d'allumage,Pièce
REF002,Filtre à huile,12.50,Filtre à huile,Consommable
REF003,Main d'œuvre,60.00,Heure de main d'œuvre,Service
```

Puis importer:
```bash
SUPPLIER_ID="550e8400-e29b-41d4-a716-446655440003"

curl -X POST http://localhost:3000/api/suppliers/$SUPPLIER_ID/import-prices \
  -H "Authorization: Bearer $TOKEN" \
  -F "file=@catalogue.csv"
```

**Réponse attendue**:
```json
{
  "imported_count": 3,
  "parts": [
    {
      "reference": "REF001",
      "name": "Bougie",
      "purchase_price": "5.99",
      "selling_price": "7.79",  // 5.99 * 1.30
      "supplier_id": "550e8400-e29b-41d4-a716-446655440003"
    },
    // ... autres pièces
  ]
}
```

---

### 6. Gestion des Pièces

#### Créer une pièce manuellement

```bash
curl -X POST http://localhost:3000/api/parts \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "reference": "REF004",
    "name": "Plaquettes de frein",
    "description": "Plaquettes de frein avant",
    "category": "Piece",
    "unit": "Unite",
    "purchase_price": 45.50,
    "selling_price": 69.99,
    "tax_rate": 20.0,
    "stock_quantity": 10,
    "minimum_stock": 2,
    "supplier_id": "$SUPPLIER_ID",
    "weight": 1.5
  }'
```

#### Lister toutes les pièces

```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/api/parts
```

---

### 7. Gestion des Factures

#### Créer une facture

```bash
curl -X POST http://localhost:3000/api/invoices \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "client_id": "$CLIENT_ID",
    "vehicle_id": "$VEHICLE_ID",
    "invoice_type": "Facture",
    "date": "2024-01-20",
    "due_date": "2024-02-19",
    "payment_method": "Virement",
    "lines": [
      {
        "part_id": "550e8400-e29b-41d4-a716-446655440004",
        "description": "Vidange complète",
        "quantity": 1,
        "unit_price": 89.99,
        "tax_rate": 20.0,
        "discount": 0
      },
      {
        "description": "Main d\'œuvre - Vidange",
        "quantity": 0.5,
        "unit_price": 60.00,
        "tax_rate": 20.0,
        "discount": 0
      }
    ],
    "discount": 0
  }'
```

**Réponse attendue**:
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440005",
  "invoice_number": "FACT-2024-0001",
  "client_id": "550e8400-e29b-41d4-a716-446655440000",
  "vehicle_id": "550e8400-e29b-41d4-a716-446655440001",
  "invoice_type": "Facture",
  "status": "Valide",
  "date": "2024-01-20",
  "due_date": "2024-02-19",
  "subtotal": "119.99",
  "tax_amount": "24.00",
  "total": "143.99",
  "discount": "0.00",
  "lines": [...],
  "created_at": "2024-12-19T10:30:00Z",
  "updated_at": "2024-12-19T10:30:00Z"
}
```

#### Lister toutes les factures

```bash
curl -H "Authorization: Bearer $TOKEN" \
  http://localhost:3000/api/invoices
```

#### Enregistrer un paiement

```bash
INVOICE_ID="550e8400-e29b-41d4-a716-446655440005"

curl -X POST http://localhost:3000/api/invoices/$INVOICE_ID/pay \
  -H "Authorization: Bearer $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "amount": 143.99,
    "payment_method": "Virement",
    "payment_date": "2024-01-21",
    "reference": "VIR-2024-001",
    "notes": "Paiement complet"
  }'
```

---

### 8. Export FEC

```bash
curl -H "Authorization: Bearer $TOKEN" \
  "http://localhost:3000/api/reports/fec?start_date=2024-01-01&end_date=2024-12-31" \
  --output fec_export.csv
```

**Fichier généré**: `fec_export.csv` au format compatible Sage 500/Ciel

---

## Script de Test Complet

Créer un script `test_api.sh`:

```bash
#!/bin/bash

# Configuration
API_URL="http://localhost:3000"
TOKEN=""

# Fonction pour faire une requête avec le token
api_request() {
  local method=$1
  local endpoint=$2
  local data=$3
  
  if [ -z "$data" ]; then
    curl -s -w "\nHTTP_CODE:%{http_code}\n" \
      -H "Authorization: Bearer $TOKEN" \
      "$API_URL$endpoint"
  else
    curl -s -w "\nHTTP_CODE:%{http_code}\n" \
      -H "Authorization: Bearer $TOKEN" \
      -H "Content-Type: application/json" \
      -d "$data" \
      -X $method "$API_URL$endpoint"
  fi
}

# Test 1: Health check
echo "=== Test 1: Health Check ==="
response=$(curl -s -w "\nHTTP_CODE:%{http_code}\n" $API_URL/health)
http_code=$(echo "$response" | grep "HTTP_CODE" | cut -d: -f2)
if [ "$http_code" = "200" ]; then
  echo "✅ PASS: Health check OK"
else
  echo "❌ FAIL: Health check failed with code $http_code"
fi

# Test 2: Login (nécessite un utilisateur existant)
echo ""
echo "=== Test 2: Login ==="
login_response=$(curl -s -w "\nHTTP_CODE:%{http_code}\n" \
  -X POST $API_URL/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}')
http_code=$(echo "$login_response" | grep "HTTP_CODE" | cut -d: -f2)
body=$(echo "$login_response" | grep -v "HTTP_CODE")

if [ "$http_code" = "200" ]; then
  TOKEN=$(echo "$body" | jq -r '.access_token')
  echo "✅ PASS: Login successful, token obtained"
else
  echo "⚠️  SKIP: Login failed (no admin user?), code: $http_code"
  echo "Body: $body"
  exit 1
fi

# Test 3: Créer un client
echo ""
echo "=== Test 3: Create Client ==="
client_data='{
  "name": "Test Client",
  "email": "test@example.com",
  "phone": "+33123456789",
  "address": {
    "street": "123 Test Street",
    "postal_code": "75001",
    "city": "Paris",
    "country": "France"
  },
  "client_type": "Particulier",
  "payment_terms": "Comptant"
}'

response=$(api_request POST "/api/clients" "$client_data")
http_code=$(echo "$response" | grep "HTTP_CODE" | cut -d: -f2)
body=$(echo "$response" | grep -v "HTTP_CODE")

if [ "$http_code" = "200" ] || [ "$http_code" = "201" ]; then
  CLIENT_ID=$(echo "$body" | jq -r '.id')
  echo "✅ PASS: Client created with ID: $CLIENT_ID"
else
  echo "❌ FAIL: Create client failed with code $http_code"
  echo "Body: $body"
fi

# Test 4: Lister les clients
echo ""
echo "=== Test 4: List Clients ==="
response=$(api_request GET "/api/clients" "")
http_code=$(echo "$response" | grep "HTTP_CODE" | cut -d: -f2)

if [ "$http_code" = "200" ]; then
  echo "✅ PASS: List clients successful"
else
  echo "❌ FAIL: List clients failed with code $http_code"
fi

# Test 5: Créer un véhicule
echo ""
echo "=== Test 5: Create Vehicle ==="
vehicle_data='{
  "client_id": "'$CLIENT_ID'",
  "registration": "TEST-123",
  "make": "TestMake",
  "model": "TestModel",
  "year": 2020,
  "fuel_type": "Essence",
  "transmission": "Manuelle",
  "category": "Voiture",
  "mileage": 50000,
  "purchase_price": 20000.00
}'

response=$(api_request POST "/api/vehicles" "$vehicle_data")
http_code=$(echo "$response" | grep "HTTP_CODE" | cut -d: -f2)
body=$(echo "$response" | grep -v "HTTP_CODE")

if [ "$http_code" = "200" ] || [ "$http_code" = "201" ]; then
  VEHICLE_ID=$(echo "$body" | jq -r '.id')
  echo "✅ PASS: Vehicle created with ID: $VEHICLE_ID"
else
  echo "❌ FAIL: Create vehicle failed with code $http_code"
  echo "Body: $body"
fi

# Test 6: Créer une facture
echo ""
echo "=== Test 6: Create Invoice ==="
invoice_data='{
  "client_id": "'$CLIENT_ID'",
  "vehicle_id": "'$VEHICLE_ID'",
  "invoice_type": "Facture",
  "date": "2024-12-19",
  "due_date": "2024-12-29",
  "payment_method": "Comptant",
  "lines": [
    {
      "description": "Test Service",
      "quantity": 1,
      "unit_price": 100.00,
      "tax_rate": 20.0,
      "discount": 0
    }
  ],
  "discount": 0
}'

response=$(api_request POST "/api/invoices" "$invoice_data")
http_code=$(echo "$response" | grep "HTTP_CODE" | cut -d: -f2)
body=$(echo "$response" | grep -v "HTTP_CODE")

if [ "$http_code" = "200" ] || [ "$http_code" = "201" ]; then
  INVOICE_ID=$(echo "$body" | jq -r '.id')
  INVOICE_NUMBER=$(echo "$body" | jq -r '.invoice_number')
  echo "✅ PASS: Invoice created with number: $INVOICE_NUMBER"
else
  echo "❌ FAIL: Create invoice failed with code $http_code"
  echo "Body: $body"
fi

# Test 7: Lister les factures
echo ""
echo "=== Test 7: List Invoices ==="
response=$(api_request GET "/api/invoices" "")
http_code=$(echo "$response" | grep "HTTP_CODE" | cut -d: -f2)

if [ "$http_code" = "200" ]; then
  echo "✅ PASS: List invoices successful"
else
  echo "❌ FAIL: List invoices failed with code $http_code"
fi

echo ""
echo "=== Tests terminés ==="
```

Pour exécuter le script:
```bash
chmod +x test_api.sh
./test_api.sh
```

---

## Dépannage

### Erreur: "Connection refused"
**Solution**: Vérifier que le serveur est démarré:
```bash
cargo run -p garage-compta-api
```

### Erreur: "Unauthorized"
**Solution**: Vérifier que le token est valide et non expiré. Reconnectez-vous:
```bash
curl -X POST http://localhost:3000/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "admin", "password": "admin123"}'
```

### Erreur: "Not Found"
**Solution**: Vérifier que l'ID utilisé existe. Lister les ressources pour voir les IDs valides.

### Erreur de validation
**Solution**: Vérifier que les données envoyées correspondent au schéma attendu (types, champs requis, etc.).

---

## Notes

- Les endpoints nécessitent une authentification (sauf `/health` et `/ready`)
- Les IDs sont des UUIDs au format string
- Les montants décimaux sont au format string (ex: "100.00")
- Les dates sont au format ISO 8601 (YYYY-MM-DD)
- Les enums doivent correspondre exactement aux valeurs définies (ex: "Particulier", "Essence")

---

*Guide de test - Garage Compta v0.1.0*
*Dernière mise à jour: 2024-12-19*
