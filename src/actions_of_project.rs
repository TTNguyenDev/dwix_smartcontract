use crate::*;

#[near_bindgen]
impl DwixContract {
    pub fn new_project(&mut self, domain: String, pages: Vec<Page>) {
        let block_timestamp = env::block_timestamp() / 1_000_000_000;
        let project_id: ProjectId =
            block_timestamp.to_string() + "_" + &env::predecessor_account_id();

        let mut project = Project {
            domain,
            pages: UnorderedSet::new(StorageKey::Pages {
                project_id: project_id.clone(),
            }),
        };
        project.add_pages(pages);
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

    pub fn add_pages(&mut self, project_id: ProjectId, pages: Vec<Page>) {
        let mut project = self.websities.get(&project_id).expect("Website not found");
        project.add_pages(pages);
        self.websities.insert(&project_id, &project);
    }

    pub fn delete_page(&mut self, project_id: ProjectId, page: Page) {
        let mut project = self.websities.get(&project_id).expect("Website not found");
        project.delete_page(page);
        self.websities.insert(&project_id, &project);
    }
}
