use crate::page::*;
use crate::*;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Project {
    pub pages: UnorderedSet<Page>,
    pub domain: String,
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedProject {
    pub pages: Vec<Page>,
    pub domain: String,
}

impl WrappedProject {
    fn from(project: Project) -> Self {
        WrappedProject {
            domain: project.domain,
            pages: project.pages.to_vec(),
        }
    }
}

impl Project {
    pub(crate) fn add_pages(&mut self, pages: Vec<Page>) {
        for page in pages {
            self.pages.insert(&page);
        }
    }

    pub(crate) fn delete_page(&mut self, page: Page) {
        self.pages.remove(&page);
    }
}

#[near_bindgen]
impl DwixContract {
    //NOTE: Define view functions
    pub fn get_lastest_projects(&self, from_index: u64, limit: u64) -> Vec<WrappedProject> {
        let projects = self.websities.keys_as_vector();
        (from_index..std::cmp::min(from_index + limit, projects.len()))
            .map(|index| {
                let project_id = projects.get(index).unwrap();
                WrappedProject::from(self.websities.get(&project_id).unwrap())
            })
            .collect()
    }
    pub fn get_project(&self, project_id: ProjectId) -> WrappedProject {
        let project = self.websities.get(&project_id).expect("Website not found");
        WrappedProject::from(project)
    }

    pub fn get_user_websites(&self, account_id: AccountId) -> Vec<WrappedProject> {
        if let Some(projects) = self.projects_owner.get(&account_id) {
            projects
                .iter()
                .map(|id| {
                    let project = self.websities.get(&id).expect("Website not found");
                    WrappedProject::from(project)
                })
                .collect()
        } else {
            vec![]
        }
    }
}
