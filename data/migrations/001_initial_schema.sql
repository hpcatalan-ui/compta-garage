-- Migration 1: Initial schema for Garage Compta

-- Clients
CREATE TABLE IF NOT EXISTS clients (
    id TEXT PRIMARY KEY,
    client_type TEXT NOT NULL DEFAULT 'Individual',
    name TEXT NOT NULL,
    first_name TEXT,
    company TEXT,
    siret TEXT,
    tvat TEXT,
    street TEXT NOT NULL,
    postal_code TEXT NOT NULL,
    city TEXT NOT NULL,
    country TEXT NOT NULL DEFAULT 'France',
    additional TEXT,
    phone TEXT NOT NULL,
    email TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    balance TEXT NOT NULL DEFAULT '0',
    payment_terms TEXT NOT NULL DEFAULT 'Net30',
    notes TEXT,
    code TEXT
);

-- Vehicles
CREATE TABLE IF NOT EXISTS vehicles (
    id TEXT PRIMARY KEY,
    registration TEXT NOT NULL UNIQUE,
    vin TEXT NOT NULL UNIQUE,
    make TEXT NOT NULL,
    model TEXT NOT NULL,
    version TEXT,
    year INTEGER NOT NULL,
    mileage INTEGER NOT NULL DEFAULT 0,
    fuel_type TEXT NOT NULL DEFAULT 'Petrol',
    transmission TEXT NOT NULL DEFAULT 'Manual',
    engine_capacity INTEGER,
    power INTEGER,
    color TEXT,
    current_owner_id TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'Car',
    next_inspection_date TEXT,
    next_service_date TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    is_active BOOLEAN NOT NULL DEFAULT true,
    code TEXT,
    FOREIGN KEY (current_owner_id) REFERENCES clients(id)
);

-- Vehicle Owner History
CREATE TABLE IF NOT EXISTS vehicle_owner_history (
    id TEXT PRIMARY KEY,
    vehicle_id TEXT NOT NULL,
    client_id TEXT NOT NULL,
    start_date TEXT NOT NULL,
    end_date TEXT,
    purchase_price TEXT,
    notes TEXT,
    FOREIGN KEY (vehicle_id) REFERENCES vehicles(id),
    FOREIGN KEY (client_id) REFERENCES clients(id)
);

-- Suppliers
CREATE TABLE IF NOT EXISTS suppliers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    code TEXT,
    street TEXT,
    postal_code TEXT,
    city TEXT,
    country TEXT DEFAULT 'France',
    additional TEXT,
    phone TEXT,
    email TEXT,
    website TEXT,
    payment_terms TEXT NOT NULL DEFAULT 'Net30',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    notes TEXT,
    tvat TEXT,
    siret TEXT,
    contact_name TEXT,
    contact_email TEXT,
    contact_phone TEXT
);

-- Parts
CREATE TABLE IF NOT EXISTS parts (
    id TEXT PRIMARY KEY,
    reference TEXT NOT NULL UNIQUE,
    supplier_reference TEXT,
    name TEXT NOT NULL,
    description TEXT,
    category TEXT NOT NULL DEFAULT 'Maintenance',
    supplier_id TEXT,
    purchase_price TEXT NOT NULL,
    selling_price TEXT NOT NULL,
    tax_rate TEXT NOT NULL DEFAULT '0.20',
    stock_quantity TEXT NOT NULL DEFAULT '0',
    minimum_stock TEXT NOT NULL DEFAULT '5',
    location TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    barcode TEXT,
    unit TEXT NOT NULL DEFAULT 'Unit',
    weight TEXT,
    dimensions TEXT,
    notes TEXT,
    compatible_vehicles TEXT,
    compatible_fuel_types TEXT,
    FOREIGN KEY (supplier_id) REFERENCES suppliers(id)
);

-- Invoices
CREATE TABLE IF NOT EXISTS invoices (
    id TEXT PRIMARY KEY,
    invoice_number TEXT NOT NULL UNIQUE,
    invoice_type TEXT NOT NULL DEFAULT 'Invoice',
    client_id TEXT NOT NULL,
    vehicle_id TEXT,
    date TEXT NOT NULL,
    due_date TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Draft',
    payment_method TEXT,
    payment_date TEXT,
    subtotal TEXT NOT NULL DEFAULT '0',
    tax_amount TEXT NOT NULL DEFAULT '0',
    total TEXT NOT NULL DEFAULT '0',
    discount TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    created_by TEXT NOT NULL,
    is_exported BOOLEAN NOT NULL DEFAULT false,
    exported_at TEXT,
    external_reference TEXT,
    FOREIGN KEY (client_id) REFERENCES clients(id),
    FOREIGN KEY (vehicle_id) REFERENCES vehicles(id),
    FOREIGN KEY (created_by) REFERENCES users(id)
);

-- Invoice Lines
CREATE TABLE IF NOT EXISTS invoice_lines (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL,
    line_number INTEGER NOT NULL,
    description TEXT NOT NULL,
    quantity TEXT NOT NULL,
    unit_price TEXT NOT NULL,
    tax_rate TEXT NOT NULL DEFAULT '0.20',
    discount TEXT,
    total_ht TEXT NOT NULL,
    total_ttc TEXT NOT NULL,
    is_labor BOOLEAN NOT NULL DEFAULT false,
    part_id TEXT,
    labor_rate_id TEXT,
    FOREIGN KEY (invoice_id) REFERENCES invoices(id),
    FOREIGN KEY (part_id) REFERENCES parts(id)
);

-- Payments
CREATE TABLE IF NOT EXISTS payments (
    id TEXT PRIMARY KEY,
    invoice_id TEXT NOT NULL,
    amount TEXT NOT NULL,
    method TEXT NOT NULL DEFAULT 'Cash',
    date TEXT NOT NULL,
    reference TEXT,
    notes TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    status TEXT NOT NULL DEFAULT 'Pending',
    FOREIGN KEY (invoice_id) REFERENCES invoices(id)
);

-- Users
CREATE TABLE IF NOT EXISTS users (
    id TEXT PRIMARY KEY,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    role TEXT NOT NULL DEFAULT 'Viewer',
    first_name TEXT,
    last_name TEXT,
    email TEXT,
    phone TEXT,
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    last_login TEXT,
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    is_locked BOOLEAN NOT NULL DEFAULT false,
    locked_at TEXT,
    notes TEXT
);

-- Labor Rates
CREATE TABLE IF NOT EXISTS labor_rates (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    hourly_rate TEXT NOT NULL,
    tax_rate TEXT NOT NULL DEFAULT '0.20',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL
);

-- Settings
CREATE TABLE IF NOT EXISTS settings (
    id TEXT PRIMARY KEY,
    key TEXT NOT NULL UNIQUE,
    value TEXT NOT NULL,
    description TEXT,
    updated_at TEXT NOT NULL
);

-- Invoice numbering
CREATE TABLE IF NOT EXISTS invoice_numbering (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    year INTEGER NOT NULL,
    last_number INTEGER NOT NULL DEFAULT 0,
    updated_at TEXT NOT NULL
);

-- Migration tracking
CREATE TABLE IF NOT EXISTS _migrations (
    version TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    applied_at TEXT NOT NULL DEFAULT datetime('now')
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_clients_name ON clients(name);
CREATE INDEX IF NOT EXISTS idx_clients_code ON clients(code);
CREATE INDEX IF NOT EXISTS idx_vehicles_registration ON vehicles(registration);
CREATE INDEX IF NOT EXISTS idx_vehicles_current_owner ON vehicles(current_owner_id);
CREATE INDEX IF NOT EXISTS idx_parts_reference ON parts(reference);
CREATE INDEX IF NOT EXISTS idx_parts_category ON parts(category);
CREATE INDEX IF NOT EXISTS idx_invoices_number ON invoices(invoice_number);
CREATE INDEX IF NOT EXISTS idx_invoices_client ON invoices(client_id);
CREATE INDEX IF NOT EXISTS idx_invoices_date ON invoices(date);
CREATE INDEX IF NOT EXISTS idx_invoices_status ON invoices(status);
CREATE INDEX IF NOT EXISTS idx_invoice_lines_invoice ON invoice_lines(invoice_id);
CREATE INDEX IF NOT EXISTS idx_payments_invoice ON payments(invoice_id);
CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_suppliers_name ON suppliers(name);
