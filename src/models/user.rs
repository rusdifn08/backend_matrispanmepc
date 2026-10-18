use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserRole {
    SuperAdmin,
    ExportManager,
    LogisticsOfficer,
    SourcingOfficer,
    FinanceOfficer,
    ExportStaff,
}

impl std::fmt::Display for UserRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserRole::SuperAdmin => write!(f, "SUPER_ADMIN"),
            UserRole::ExportManager => write!(f, "EXPORT_MANAGER"),
            UserRole::LogisticsOfficer => write!(f, "LOGISTICS_OFFICER"),
            UserRole::SourcingOfficer => write!(f, "SOURCING_OFFICER"),
            UserRole::FinanceOfficer => write!(f, "FINANCE_OFFICER"),
            UserRole::ExportStaff => write!(f, "EXPORT_STAFF"),
        }
    }
}

impl From<&str> for UserRole {
    fn from(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "SUPER_ADMIN" | "SUPERADMIN" | "ADMIN" => UserRole::SuperAdmin,
            "EXPORT_MANAGER" => UserRole::ExportManager,
            "LOGISTICS_OFFICER" | "LOGISTICS" => UserRole::LogisticsOfficer,
            "SOURCING_OFFICER" | "SOURCING" => UserRole::SourcingOfficer,
            "FINANCE_OFFICER" | "FINANCE" => UserRole::FinanceOfficer,
            _ => UserRole::ExportStaff,
        }
    }
}

impl UserRole {
    pub fn permissions(&self) -> Vec<String> {
        match self {
            UserRole::SuperAdmin => vec![
                "all:access".to_string(),
                "users:manage".to_string(),
                "orders:approve".to_string(),
                "orders:edit_price".to_string(),
                "suppliers:approve".to_string(),
                "invoices:generate".to_string(),
                "products:edit".to_string(),
                "messages:reply".to_string(),
                "audit:view".to_string(),
                "security:test".to_string(),
            ],
            UserRole::ExportManager => vec![
                "orders:view".to_string(),
                "orders:approve".to_string(),
                "orders:edit_price".to_string(),
                "invoices:generate".to_string(),
                "invoices:view".to_string(),
                "messages:reply".to_string(),
                "products:edit".to_string(),
            ],
            UserRole::LogisticsOfficer => vec![
                "orders:view".to_string(),
                "logistics:belawan".to_string(),
                "invoices:view".to_string(),
                "messages:view".to_string(),
            ],
            UserRole::SourcingOfficer => vec![
                "suppliers:view".to_string(),
                "suppliers:approve".to_string(),
                "products:view".to_string(),
                "messages:view".to_string(),
                "messages:reply".to_string(),
            ],
            UserRole::FinanceOfficer => vec![
                "invoices:view".to_string(),
                "invoices:generate".to_string(),
                "orders:view".to_string(),
                "orders:edit_price".to_string(),
                "finance:rates".to_string(),
            ],
            UserRole::ExportStaff => vec![
                "orders:view".to_string(),
                "suppliers:view".to_string(),
                "products:view".to_string(),
                "messages:view".to_string(),
            ],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub nik: String,
    pub name: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: UserRole,
    pub department: String,
    pub phone: Option<String>,
    pub is_active: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfileDto {
    pub id: Uuid,
    pub nik: String,
    pub name: String,
    pub email: String,
    pub role: UserRole,
    pub department: String,
    pub phone: Option<String>,
    pub is_active: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub permissions: Vec<String>,
}

impl User {
    pub fn from_row(row: &sqlx::postgres::PgRow) -> Self {
        use sqlx::Row;
        Self {
            id: Uuid::parse_str(row.get("id")).unwrap_or_else(|_| Uuid::new_v4()),
            nik: row.get("nik"),
            name: row.get("name"),
            email: row.get("email"),
            password_hash: row.get("password_hash"),
            role: row.get::<&str, _>("role").into(),
            department: row.get("department"),
            phone: row.get("phone"),
            is_active: row.get("is_active"),
            last_login_at: row.get("last_login_at"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        }
    }
}

impl From<&User> for UserProfileDto {
    fn from(u: &User) -> Self {
        Self {
            id: u.id,
            nik: u.nik.clone(),
            name: u.name.clone(),
            email: u.email.clone(),
            role: u.role.clone(),
            department: u.department.clone(),
            phone: u.phone.clone(),
            is_active: u.is_active,
            last_login_at: u.last_login_at,
            created_at: u.created_at,
            permissions: u.role.permissions(),
        }
    }
}

impl From<User> for UserProfileDto {
    fn from(u: User) -> Self {
        (&u).into()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginPayload {
    pub identifier: String, // NIK, Email, or "admin"
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterPayload {
    pub nik: String,
    pub name: String,
    pub email: String,
    pub password: String,
    pub department: String,
    pub role: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthResponse {
    pub success: bool,
    pub message: String,
    pub token: String,
    pub user: UserProfileDto,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAuditLog {
    pub id: Uuid,
    pub user_nik: Option<String>,
    pub user_email: Option<String>,
    pub action: String,
    pub severity: String,
    pub ip_address: Option<String>,
    pub user_agent: Option<String>,
    pub details: Option<serde_json::Value>,
    pub created_at: DateTime<Utc>,
}
