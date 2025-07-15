#[cfg(test)]
mod tests {
    use super::super::roles::McpRolesManager;

    #[test]
    fn test_role_assignment_and_permissions() {
        let mut manager = McpRolesManager::new();

        // Test assigning a valid role
        manager.assign_role("test_user_1", "developer");
        let user_role = manager.get_user_role("test_user_1");
        assert!(user_role.is_some());
        assert_eq!(user_role.unwrap().name, "developer");
        assert!(manager.has_permission("test_user_1", "read_code"));
        assert!(!manager.has_permission("test_user_1", "delete_data"));

        // Test assigning an invalid role
        manager.assign_role("test_user_2", "non_existent_role");
        assert!(manager.get_user_role("test_user_2").is_none());

        // Test admin role
        manager.assign_role("admin_user", "admin");
        assert!(manager.has_permission("admin_user", "any_permission_imaginable"));
        assert!(manager.has_permission("admin_user", "read_only"));
    }
}
