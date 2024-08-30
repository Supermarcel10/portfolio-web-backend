use super::skill::Skill;
use crate::models::base::entity::{Base, BaseEntity};

pub trait AssetEntity: BaseEntity {
	fn title(&self) -> &str;
	fn skills(&self) -> &[Skill];
	fn related_assets(&self) -> &[Box<dyn AssetEntity>];
}

pub struct Asset {
	pub base: Base,
	pub title: String,
	pub skills: Vec<Skill>,
	pub related_assets: Vec<Box<dyn AssetEntity>>,
}

#[macro_export]
macro_rules! impl_asset {
	($type:ty) => {
		impl $crate::models::structs::asset::AssetEntity for $type {
			fn title(&self) -> &str {
				todo!()
			}

			fn skills(&self) -> &[$crate::models::structs::skill::Skill] {
				todo!()
			}

			fn related_assets(&self) -> &[Box<dyn $crate::models::structs::asset::AssetEntity>] {
				todo!()
			}
		}

		$crate::impl_base!($type);
	}
}

pub use impl_asset;
