use crate::models::Action;
use crate::services::ConfigService;
use crate::services::group_service::GroupService;

#[derive(Default)]
pub struct ActionService {
    group_service: GroupService,
}

impl ActionService {
    pub fn execute(&mut self, config_service: &ConfigService, action: &Action) {
        match action {
            Action::OpenGroup { group_id } => self.group_service.open(config_service, *group_id),
        }
    }
}
