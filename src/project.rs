use crate::page::*;
use crate::*;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
#[serde(tag = "type")]
pub enum ProjectCategory {
    Standard,
    Ecom
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Project {
    pub name: String,
    pub category: ProjectCategory,
    pub description: String,
    pub data: String,
    pub domain: String,
    pub owner: AccountId
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct WrappedProject {
    pub name: String,
    pub category: ProjectCategory,
    pub description: String,
    pub data: String,
    pub domain: String,
    pub project_id: ProjectId,
    pub owner: AccountId
}

impl WrappedProject {
    fn from(project: Project, project_id: ProjectId) -> Self {
        WrappedProject {
            name: project.name,
            category: project.category,
            description: project.description,
            data: project.data,
            domain: project.domain,
            project_id,
            owner: project.owner,
        }
    }
}

impl Project {
    pub(crate) fn update_data(&mut self, data: String) {
        self.data = data;
    }
}

#[near_bindgen]
impl DwixContract {
    //NOTE: Define view functions
    pub fn get_lastest_projects(&self, from_index: u64, limit: u64) -> Vec<WrappedProject> {
        let projects = self.websities.keys_as_vector();

        let from = if projects.len() > (limit + from_index) {
            projects.len() - limit - from_index
        } else {
            0
        };

        let to = if projects.len() > from_index {
            projects.len() - from_index
        } else {
            0
        };
        (from..to)
            .map(|index| {
                let project_id = projects.get(index).unwrap(); 
                WrappedProject::from(self.websities.get(&project_id).unwrap(), project_id)
            })
            .rev()
            .collect()
    }
    pub fn get_project(&self, project_id: ProjectId) -> WrappedProject {
        let project = self.websities.get(&project_id).expect("Website not found");
        WrappedProject::from(project, project_id)
    }

    pub fn available_domain(&self, domain: String) -> bool {
        !self.used_domains.contains(&domain)
    }

    pub fn get_user_websites(&self, account_id: AccountId) -> Vec<WrappedProject> {
        if let Some(projects) = self.projects_owner.get(&account_id) {
            projects
                .iter()
                .map(|id| {
                    let project = self.websities.get(&id).expect("Website not found");
                    WrappedProject::from(project, id)
                })
                .collect()
        } else {
            vec![]
        }
    }

    pub fn check_deploy_queue(&self, project_id: ProjectId) -> Option<Timestamp> {
        self.deploy_queue.get(&project_id)
    }
}
