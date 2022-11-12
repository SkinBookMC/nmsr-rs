use std::collections::HashMap;
use std::path::Path;

use strum::IntoEnumIterator;
use strum::{Display, EnumCount, EnumIter, EnumString};

use nmsr_lib::parts::manager::PartsManager;
use nmsr_lib::vfs::{PhysicalFS, VfsPath};

use crate::utils::errors::NMSRaaSError::MissingPartManager;
use crate::utils::Result;

#[derive(Debug, Clone, PartialEq, Eq, Hash, EnumString, EnumIter, EnumCount, Display)]
#[strum(serialize_all = "lowercase")]
pub(crate) enum RenderMode {
    FullBody,
    FrontFull,
    FullBodyIso,
    Head,
    HeadIso,
    Face,
}

#[derive(Debug, Clone)]
pub(crate) struct NMSRaaSManager {
    part_managers: HashMap<RenderMode, PartsManager>,
}

impl NMSRaaSManager {
    pub(crate) fn get_manager(&self, render_type: &RenderMode) -> Result<&PartsManager> {
        self.part_managers
            .get(render_type)
            .ok_or_else(|| MissingPartManager(render_type.clone()))
    }

    pub(crate) fn new(part_root: impl AsRef<Path>) -> Result<NMSRaaSManager> {
        let part_root: VfsPath = PhysicalFS::new(part_root).into();
        let mut map = HashMap::with_capacity(RenderMode::COUNT);

        for render_type in RenderMode::iter() {
            let path = part_root.join(render_type.to_string())?;

            let part_manager = PartsManager::new(&path)?;
            map.insert(render_type, part_manager);
        }

        Ok(NMSRaaSManager { part_managers: map })
    }
}
