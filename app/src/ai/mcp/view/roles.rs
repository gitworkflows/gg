use std::collections::HashMap;
use log::info;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct McpRole {
    pub id: Uuid,
    pub name: String,
    pub permissions: Vec<String>,
}

pub struct McpRolesManager {
    roles: HashMap<String, McpRole>,
    user_roles: HashMap<String, String>, // user_id -> role_name
}

impl McpRolesManager {
    pub fn new() -> Self {
        let mut roles = HashMap::new();
        roles.insert("admin".to_string(), McpRole {
            id: Uuid::new_v4(),
            name: "admin".to_string(),
            permissions: vec!["all".to_string()],
        });
        roles.insert("developer".to_string(), McpRole {
            id: Uuid::new_v4(),
            name: "developer".to_string(),
            permissions: vec!["read_code".to_string(), "write_code".to_string()],
        });
        roles.insert("viewer".to_string(), McpRole {
            id: Uuid::new_v4(),
            name: "viewer".to_string(),
            permissions: vec!["read_only".to_string()],
        });

        McpRolesManager {
            roles,
            user_roles: HashMap::new(),
        }
    }

    pub fn assign_role(&mut self, user_id: &str, role_name: &str) {
        if self.roles.contains_key(role_name) {
            self.user_roles.insert(user_id.to_string(), role_name.to_string());
            info!("Assigned role '{}' to user '{}'", role_name, user_id);
        } else {
            info!("Role '{}' does not exist.", role_name);
        }
    }

    pub fn get_user_role(&self, user_id: &str) -> Option<&McpRole> {
        self.user_roles.get(user_id)
            .and_then(|role_name| self.roles.get(role_name))
    }

    pub fn has_permission(&self, user_id: &str, permission: &str) -> bool {
        if let Some(role) = self.get_user_role(user_id) {
            role.permissions.contains(&permission.to_string()) || role.permissions.contains(&"all".to_string())
        } else {
            false
        }
    }
}
