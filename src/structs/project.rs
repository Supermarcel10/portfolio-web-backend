use crate::{
	impl_asset, impl_attachable, impl_describable, impl_duration_dateable, impl_featurable,
};

pub struct Project {
	pub status: ProjectStatus,
	pub challenges: Vec<String>,
	pub outcomes: Vec<String>,
	pub category: Option<String>,
	pub github_url: Option<String>,
	pub deployment_url: Option<String>,
}

pub enum ProjectStatus {
	Planning,
	Designing,
	Developing,
	Released {
		version: String,
	},
	Updating {
		current_version: String,
		target_version: String,
	},
}

impl_asset!(Project);
impl_attachable!(Project);
impl_describable!(Project);
impl_duration_dateable!(Project);
impl_featurable!(Project);
