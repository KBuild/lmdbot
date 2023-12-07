use std::collections::HashMap;

use serenity::model::prelude::RoleId;

impl serenity::prelude::TypeMapKey for RoleMatcher {
    type Value = RoleMatcher;
}

pub struct RoleMatcher(HashMap<String, RoleId>);

impl RoleMatcher {
    pub fn new(file_path: &str) -> Self {
        let contents = std::fs::read_to_string(file_path)
            .expect("Should have been able to read the file");

        let role_map = contents.lines()
            .filter(|str| !str.is_empty())
            .map(|s| s.split(',').collect::<Vec<&str>>())
            .map(|v|
                (
                    v[0].to_string(),
                    match v[1].parse::<u64>() {
                        Ok(id) => RoleId::new(id),
                        Err(_) => panic!("Invalid role id"),
                    }
                )
            )
            .collect::<HashMap<String, RoleId>>();
        
        #[cfg(debug_assertions)]
        println!("Now I got a map of roles: {:?}", role_map);

        RoleMatcher(role_map)
    }

    pub fn get_all_roles(&self) -> &HashMap<String, RoleId> {
        &self.0
    }

    pub fn get_role_id(&self, role_name: &str) -> Option<&RoleId> {
        self.0.get(role_name)
    }
}
