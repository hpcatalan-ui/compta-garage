//! User model

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User role
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT", rename_all = "lowercase")]
pub enum UserRole {
    /// View-only access
    Viewer,
    /// Can create and edit data
    Editor,
    /// Can manage users and settings
    Manager,
    /// Full access
    Admin,
}

impl Default for UserRole {
    fn default() -> Self {
        Self::Viewer
    }
}

impl From<UserRole> for &str {
    fn from(val: UserRole) -> Self {
        match val {
            UserRole::Viewer => "Viewer",
            UserRole::Editor => "Editor",
            UserRole::Manager => "Manager",
            UserRole::Admin => "Admin",
        }
    }
}

impl std::str::FromStr for UserRole {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Viewer" => Ok(Self::Viewer),
            "Editor" => Ok(Self::Editor),
            "Manager" => Ok(Self::Manager),
            "Admin" => Ok(Self::Admin),
            _ => Err(format!("Invalid UserRole: {}", s)),
        }
    }
}

/// User permissions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Permission {
    /// View clients
    ViewClients,
    /// Edit clients
    EditClients,
    /// View vehicles
    ViewVehicles,
    /// Edit vehicles
    EditVehicles,
    /// View invoices
    ViewInvoices,
    /// Edit invoices
    EditInvoices,
    /// View parts
    ViewParts,
    /// Edit parts
    EditParts,
    /// View suppliers
    ViewSuppliers,
    /// Edit suppliers
    EditSuppliers,
    /// View payments
    ViewPayments,
    /// Edit payments
    EditPayments,
    /// Manage users
    ManageUsers,
    /// Manage settings
    ManageSettings,
    /// Export data
    ExportData,
    /// All permissions
    All,
}

/// User entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// Unique identifier
    pub id: Uuid,
    /// Username
    pub username: String,
    /// Password hash
    pub password_hash: String,
    /// Role
    pub role: UserRole,
    /// First name
    pub first_name: Option<String>,
    /// Last name
    pub last_name: Option<String>,
    /// Email
    pub email: Option<String>,
    /// Phone
    pub phone: Option<String>,
    /// Whether the user is active
    pub is_active: bool,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Last login
    pub last_login: Option<DateTime<Utc>>,
    /// Failed login attempts
    pub failed_login_attempts: i32,
    /// Whether the account is locked
    pub is_locked: bool,
    /// Locked at timestamp
    pub locked_at: Option<DateTime<Utc>>,
    /// Notes
    pub notes: Option<String>,
}

impl User {
    /// Creates a new user
    pub fn new(username: String, password_hash: String, role: UserRole) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            username,
            password_hash,
            role,
            first_name: None,
            last_name: None,
            email: None,
            phone: None,
            is_active: true,
            created_at: now,
            updated_at: now,
            last_login: None,
            failed_login_attempts: 0,
            is_locked: false,
            locked_at: None,
            notes: None,
        }
    }

    /// Full name
    pub fn full_name(&self) -> String {
        match (&self.first_name, &self.last_name) {
            (Some(first), Some(last)) => format!("{} {}", first, last),
            (Some(first), None) => first.clone(),
            (None, Some(last)) => last.clone(),
            (None, None) => self.username.clone(),
        }
    }

    /// Checks if user has a specific permission
    pub fn has_permission(&self, permission: Permission) -> bool {
        match self.role {
            UserRole::Admin => true,
            UserRole::Manager => {
                match permission {
                    Permission::ManageUsers | Permission::ManageSettings => false,
                    Permission::All => false,
                    _ => true,
                }
            }
            UserRole::Editor => {
                match permission {
                    Permission::ViewClients | Permission::EditClients => true,
                    Permission::ViewVehicles | Permission::EditVehicles => true,
                    Permission::ViewInvoices | Permission::EditInvoices => true,
                    Permission::ViewParts | Permission::EditParts => true,
                    Permission::ViewSuppliers | Permission::EditSuppliers => true,
                    Permission::ViewPayments | Permission::EditPayments => true,
                    Permission::ExportData => true,
                    _ => false,
                }
            }
            UserRole::Viewer => {
                match permission {
                    Permission::ViewClients | Permission::ViewVehicles => true,
                    Permission::ViewInvoices | Permission::ViewParts => true,
                    Permission::ViewSuppliers | Permission::ViewPayments => true,
                    _ => false,
                }
            }
        }
    }

    /// Locks the user account
    pub fn lock(&mut self) {
        self.is_locked = true;
        self.locked_at = Some(Utc::now());
        self.failed_login_attempts = 0;
    }

    /// Unlocks the user account
    pub fn unlock(&mut self) {
        self.is_locked = false;
        self.locked_at = None;
        self.failed_login_attempts = 0;
    }

    /// Increments failed login attempts
    pub fn increment_failed_attempts(&mut self) {
        self.failed_login_attempts += 1;
    }

    /// Resets failed login attempts
    pub fn reset_failed_attempts(&mut self) {
        self.failed_login_attempts = 0;
    }

    /// Sets last login
    pub fn set_last_login(&mut self, when: DateTime<Utc>) {
        self.last_login = Some(when);
        self.reset_failed_attempts();
    }
}
