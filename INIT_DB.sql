-- Initial Database Setup for Garage Compta
-- This script creates the initial tables and inserts sample data

-- Enable foreign keys
PRAGMA foreign_keys = ON;

-- Create tables

-- Clients
CREATE TABLE IF NOT EXISTS clients (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    email TEXT,
    phone TEXT,
    address_street TEXT NOT NULL,
    address_postal_code TEXT NOT NULL,
    address_city TEXT NOT NULL,
    address_country TEXT NOT NULL,
    address_additional TEXT,
    client_type TEXT NOT NULL,
    payment_terms TEXT NOT NULL,
    balance TEXT NOT NULL DEFAULT '0',
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Vehicles
CREATE TABLE IF NOT EXISTS vehicles (
    id TEXT PRIMARY KEY,
    client_id TEXT NOT NULL,
    registration TEXT UNIQUE NOT NULL,
    make TEXT NOT NULL,
    model TEXT NOT NULL,
    year INTEGER,
    vin TEXT,
    fuel_type TEXT NOT NULL,
    transmission TEXT NOT NULL,
    category TEXT NOT NULL,
    mileage INTEGER,
    next_inspection TEXT,
    purchase_date TEXT,
    purchase_price TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (client_id) REFERENCES clients(id)
);

-- Vehicle Owner History
CREATE TABLE IF NOT EXISTS vehicle_owner_history (
    id TEXT PRIMARY KEY,
    vehicle_id TEXT NOT NULL,
    previous_owner_id TEXT,
    new_owner_id TEXT NOT NULL,
    change_date TEXT NOT NULL,
    purchase_price TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL,
    FOREIGN KEY (vehicle_id) REFERENCES vehicles(id),
    FOREIGN KEY (previous_owner_id) REFERENCES clients(id),
    FOREIGN KEY (new_owner_id) REFERENCES clients(id)
);

-- Suppliers
CREATE TABLE IF NOT EXISTS suppliers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    contact TEXT,
    email TEXT,
    phone TEXT,
    address_street TEXT NOT NULL,
    address_postal_code TEXT NOT NULL,
    address_city TEXT NOT NULL,
    address_country TEXT NOT NULL,
    address_additional TEXT,
    siret TEXT,
    vat_number TEXT,
    payment_terms TEXT NOT NULL,
    default_margin TEXT NOT NULL,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Parts
CREATE TABLE IF NOT EXISTS parts (
    id TEXT PRIMARY KEY,
    reference TEXT UNIQUE NOT NULL,
    supplier_reference TEXT,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL,
    unit TEXT NOT NULL,
    purchase_price TEXT NOT NULL,
    selling_price TEXT NOT NULL,
    tax_rate TEXT NOT NULL,
    stock_quantity TEXT NOT NULL,
    minimum_stock TEXT NOT NULL,
    supplier_id TEXT,
    weight TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (supplier_id) REFERENCES suppliers(id)
);

-- Invoices
CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    invoice_number TEXT UNIQUE NOT NULL,
    client_id TEXT NOT NULL,
    vehicle_id TEXT,
    invoice_type TEXT NOT NULL,
    status TEXT NOT NULL,
    date TEXT NOT NULL,
    due_date TEXT NOT NULL,
    payment_method TEXT NOT NULL,
    subtotal TEXT NOT NULL,
    tax_amount TEXT NOT NULL,
    total TEXT NOT NULL,
    discount TEXT NOT NULL DEFAULT '0',
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (client_id) REFERENCES clients(id),
    FOREIGN KEY (vehicle_id) REFERENCES vehicles(id)
);

-- Invoice Lines
CREATE TABLE IF NOT EXISTS invoice_lines (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL,
    part_id TEXT,
    description TEXT NOT NULL,
    quantity TEXT NOT NULL,
    unit_price TEXT NOT NULL,
    tax_rate TEXT NOT NULL,
    discount TEXT NOT NULL DEFAULT '0',
    total_ht TEXT NOT NULL,
    total_ttc TEXT NOT NULL,
    position INTEGER NOT NULL,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id),
    FOREIGN KEY (part_id) REFERENCES parts(id)
);

-- Payments
CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    payment_method TEXT NOT NULL,
    payment_date TEXT NOT NULL,
    status TEXT NOT NULL,
    reference TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id)
);

-- Users
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    role TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT 1,
    last_login TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Invoice Numbering
CREATE TABLE IF NOT EXISTS invoice_numbering (
    id INTEGER PRIMARY KEY,
    year INTEGER NOT NULL,
    last_number INTEGER NOT NULL,
    lock_file TEXT,
    updated_at TEXT NOT NULL
);

-- Migrations tracking
CREATE TABLE IF NOT EXISTS _migrations (
    version TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    applied_at TEXT NOT NULL DEFAULT datetime('now')
);

-- Sample Data

-- Admin User (password: admin123 - hashed with Argon2)
-- To generate the hash, run: cargo run -p garage-compta-cli -- hash-password admin123
INSERT OR IGNORE INTO users (id, username, email, password_hash, first_name, last_name, role, is_active, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440000', 'admin', 'admin@garage.com', '$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJC1OeqncgJ2almHUf9KY', 'Admin', 'User', 'Admin', 1, datetime('now'), datetime('now'));

-- Technician User (password: tech123)
INSERT OR IGNORE INTO users (id, username, email, password_hash, first_name, last_name, role, is_active, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440001', 'tech', 'tech@garage.com', '$argon2id$v=19$m=65536,t=3,p=4$c29tZXNhbHQ$RdescudvJC1OeqncgJ2almHUf9KY', 'John', 'Doe', 'Technicien', 1, datetime('now'), datetime('now'));

-- Sample Client
INSERT OR IGNORE INTO clients (id, name, email, phone, address_street, address_postal_code, address_city, address_country, client_type, payment_terms, balance, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440010', 'Jean Dupont', 'jean.dupont@example.com', '+33123456789', '123 Rue de Paris', '75001', 'Paris', 'France', 'Particulier', 'Comptant', '0.00', datetime('now'), datetime('now'));

-- Another Client
INSERT OR IGNORE INTO clients (id, name, email, phone, address_street, address_postal_code, address_city, address_country, client_type, payment_terms, balance, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440011', 'Marie Martin', 'marie.martin@example.com', '+33234567890', '456 Avenue des Champs', '75008', 'Paris', 'France', 'Particulier', 'Net30', '0.00', datetime('now'), datetime('now'));

-- Sample Vehicle
INSERT OR IGNORE INTO vehicles (id, client_id, registration, make, model, year, fuel_type, transmission, category, mileage, purchase_date, purchase_price, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440020', '550e8400-e29b-41d4-a716-446655440010', 'AB-123-CD', 'Peugeot', '308', 2020, 'Essence', 'Manuelle', 'Voiture', 45000, '2020-01-15', '25000.00', datetime('now'), datetime('now'));

-- Sample Supplier
INSERT OR IGNORE INTO suppliers (id, name, contact, email, phone, address_street, address_postal_code, address_city, address_country, siret, vat_number, payment_terms, default_margin, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440030', 'AutoPièces SA', 'Pierre Martin', 'contact@autopieces.fr', '+33234567890', '456 Rue Industrielle', '69000', 'Lyon', 'France', '12345678901234', 'FR123456789', 'Net30', '30.00', datetime('now'), datetime('now'));

-- Sample Parts
INSERT OR IGNORE INTO parts (id, reference, name, description, category, unit, purchase_price, selling_price, tax_rate, stock_quantity, minimum_stock, supplier_id, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440040', 'REF001', 'Bougie', 'Bougie d''allumage', 'Piece', 'Unite', '5.99', '7.79', '20.00', '100', '10', '550e8400-e29b-41d4-a716-446655440030', datetime('now'), datetime('now'));

INSERT OR IGNORE INTO parts (id, reference, name, description, category, unit, purchase_price, selling_price, tax_rate, stock_quantity, minimum_stock, supplier_id, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440041', 'REF002', 'Filtre à huile', 'Filtre à huile', 'Consommable', 'Unite', '12.50', '16.25', '20.00', '50', '5', '550e8400-e29b-41d4-a716-446655440030', datetime('now'), datetime('now'));

INSERT OR IGNORE INTO parts (id, reference, name, description, category, unit, purchase_price, selling_price, tax_rate, stock_quantity, minimum_stock, supplier_id, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440042', 'REF003', 'Main d''œuvre', 'Heure de main d''œuvre', 'Service', 'Heure', '60.00', '60.00', '20.00', '0', '0', NULL, datetime('now'), datetime('now'));

-- Sample Invoice
INSERT OR IGNORE INTO invoices (id, invoice_number, client_id, vehicle_id, invoice_type, status, date, due_date, payment_method, subtotal, tax_amount, total, discount, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440050', 'FACT-2024-0001', '550e8400-e29b-41d4-a716-446655440010', '550e8400-e29b-41d4-a716-446655440020', 'Facture', 'Paye', '2024-01-15', '2024-01-15', 'Comptant', '149.98', '30.00', '179.98', '0.00', datetime('now'), datetime('now'));

-- Invoice Lines
INSERT OR IGNORE INTO invoice_lines (id, invoice_id, part_id, description, quantity, unit_price, tax_rate, discount, total_ht, total_ttc, position) 
VALUES ('550e8400-e29b-41d4-a716-446655440100', '550e8400-e29b-41d4-a716-446655440050', '550e8400-e29b-41d4-a716-446655440040', 'Bougie x4', '4', '5.99', '20.00', '0.00', '23.96', '28.75', 1);

INSERT OR IGNORE INTO invoice_lines (id, invoice_id, part_id, description, quantity, unit_price, tax_rate, discount, total_ht, total_ttc, position) 
VALUES ('550e8400-e29b-41d4-a716-446655440101', '550e8400-e29b-41d4-a716-446655440050', '550e8400-e29b-41d4-a716-446655440041', 'Filtre à huile', '1', '12.50', '20.00', '0.00', '12.50', '15.00', 2);

INSERT OR IGNORE INTO invoice_lines (id, invoice_id, description, quantity, unit_price, tax_rate, discount, total_ht, total_ttc, position) 
VALUES ('550e8400-e29b-41d4-a716-446655440102', '550e8400-e29b-41d4-a716-446655440050', 'Main d''œuvre - Vidange', '1', '60.00', '20.00', '0.00', '60.00', '72.00', 3);

-- Payment for the invoice
INSERT OR IGNORE INTO payments (id, invoice_id, amount, payment_method, payment_date, status, reference, created_at, updated_at) 
VALUES ('550e8400-e29b-41d4-a716-446655440200', '550e8400-e29b-41d4-a716-446655440050', '179.98', 'Comptant', '2024-01-15', 'Valide', 'PAY-2024-0001', datetime('now'), datetime('now'));

-- Invoice Numbering
INSERT OR IGNORE INTO invoice_numbering (id, year, last_number, updated_at) 
VALUES (1, 2024, 1, datetime('now'));

-- Indexes
CREATE INDEX IF NOT EXISTS idx_clients_name ON clients(name);
CREATE INDEX IF NOT EXISTS idx_vehicles_registration ON vehicles(registration);
CREATE INDEX IF NOT EXISTS idx_vehicles_client_id ON vehicles(client_id);
CREATE INDEX IF NOT EXISTS idx_invoices_number ON invoices(invoice_number);
CREATE INDEX IF NOT EXISTS idx_invoices_client_id ON invoices(client_id);
CREATE INDEX IF NOT EXISTS idx_invoices_date ON invoices(date);
CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
CREATE INDEX IF NOT EXISTS idx_parts_reference ON parts(reference);
CREATE INDEX IF NOT EXISTS idx_parts_supplier_id ON parts(supplier_id);
CREATE INDEX IF NOT EXISTS idx_payments_invoice_id ON payments(invoice_id);
CREATE INDEX IF NOT EXISTS idx_payments_date ON payments(payment_date);

-- Views

-- View for client balance
CREATE VIEW IF NOT EXISTS client_balances AS
SELECT 
    c.id,
    c.name,
    c.balance,
    COALESCE(SUM(i.total), 0) as total_invoiced,
    COALESCE(SUM(p.amount), 0) as total_paid
FROM clients c
LEFT JOIN invoices i ON c.id = i.client_id AND i.status != 'Annule'
LEFT JOIN payments p ON i.id = p.invoice_id AND p.status = 'Valide'
GROUP BY c.id, c.name, c.balance;

-- View for vehicle history
CREATE VIEW IF NOT EXISTS vehicle_histories AS
SELECT 
    v.id as vehicle_id,
    v.registration,
    v.make,
    v.model,
    v.year,
    c1.name as current_owner_name,
    c1.id as current_owner_id,
    voh.change_date as last_owner_change,
    c2.name as previous_owner_name,
    c2.id as previous_owner_id
FROM vehicles v
JOIN clients c1 ON v.client_id = c1.id
LEFT JOIN vehicle_owner_history voh ON v.id = voh.vehicle_id AND voh.new_owner_id = v.client_id
LEFT JOIN clients c2 ON voh.previous_owner_id = c2.id;

-- View for invoice summary
CREATE VIEW IF NOT EXISTS invoice_summary AS
SELECT 
    i.id,
    i.invoice_number,
    i.date,
    i.due_date,
    i.status,
    i.total,
    c.name as client_name,
    c.id as client_id,
    SUM(p.amount) as paid_amount,
    (i.total - COALESCE(SUM(p.amount), 0)) as remaining_amount
FROM invoices i
JOIN clients c ON i.client_id = c.id
LEFT JOIN payments p ON i.id = p.invoice_id AND p.status = 'Valide'
GROUP BY i.id, i.invoice_number, i.date, i.due_date, i.status, i.total, c.name, c.id;

-- Trigger for updating client balance on invoice creation
CREATE TRIGGER IF NOT EXISTS update_client_balance_on_invoice_create 
AFTER INSERT ON invoices
FOR EACH ROW
WHEN NEW.status != 'Annule'
BEGIN
    UPDATE clients 
    SET balance = balance + NEW.total
    WHERE id = NEW.client_id;
END;

-- Trigger for updating client balance on invoice deletion
CREATE TRIGGER IF NOT EXISTS update_client_balance_on_invoice_delete 
AFTER DELETE ON invoices
FOR EACH ROW
WHEN OLD.status != 'Annule'
BEGIN
    UPDATE clients 
    SET balance = balance - OLD.total
    WHERE id = OLD.client_id;
END;

-- Trigger for updating client balance on payment
CREATE TRIGGER IF NOT EXISTS update_client_balance_on_payment 
AFTER INSERT ON payments
FOR EACH ROW
WHEN NEW.status = 'Valide'
BEGIN
    UPDATE clients 
    SET balance = balance - NEW.amount
    WHERE id = (SELECT client_id FROM invoices WHERE id = NEW.invoice_id);
END;

-- Mark migrations as applied
INSERT OR IGNORE INTO _migrations (version, name) VALUES ('0.1.0', 'Initial schema and sample data');

-- Output success message
SELECT 'Database initialized successfully!' as message;
