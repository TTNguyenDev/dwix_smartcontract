use crate::*;

#[near_bindgen]
impl DwixContract {
    pub fn new_project(&mut self, name: String, category: String, domain: String, description: String, data: String) {
        let block_timestamp = env::block_timestamp() / 1_000_000_000;
        let project_id: ProjectId =
            block_timestamp.to_string() + "_" + &env::predecessor_account_id();

        let project = Project {
            name,
            category,
            description,
            data,
            domain,
        };
        self.websities.insert(&project_id, &project);

        let mut projects = self
            .projects_owner
            .get(&env::predecessor_account_id())
            .unwrap_or(UnorderedSet::new(StorageKey::ProjectsOwnerInner {
                owner: env::predecessor_account_id(),
            }));

        projects.insert(&project_id);
        self.projects_owner
            .insert(&env::predecessor_account_id(), &projects);
    }

    pub fn update_data(&mut self, project_id: ProjectId, data: String) {
        let mut project = self.websities.get(&project_id).expect("Website not found");
        project.update_data(data);
        self.websities.insert(&project_id, &project);
    }
}
