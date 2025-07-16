impl PackageRegistry {
    /// Exposes envs for CLI usage
    pub fn envs(&self) -> &std::collections::HashMap<String, Vec<iota_interaction::types::base_types::ObjectID>> {
        &self.envs
    }
}