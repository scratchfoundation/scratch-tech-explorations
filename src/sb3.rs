use std::io;

use bevy::prelude::*;
use zip::ZipArchive;

use crate::{project::ProjectLoadError, project_bundle::ProjectBundle};

pub struct SB3;

impl SB3 {
    pub fn from_reader<R>(sb3_reader: R) -> Result<ProjectBundle, ProjectLoadError>
    where
        R: io::Read + std::io::Seek,
    {
        // this will open the ZIP and read the central directory
        let mut sb3_zip = ZipArchive::new(sb3_reader)?;

        let project_json_reader = sb3_zip.by_name("project.json")?;

        // let project_json: serde_json::Value = serde_json::from_reader(project_json_reader)?;
        // info!("Project loaded data: {:#?}", project_json);
        // Ok(ProjectBundle {
        //     title: "hi".to_string(),
        //     extensions: vec![],
        //     meta: serde_json::Value::Null,
        //     monitors: vec![],
        //     targets: vec![],
        // })

        let project_bundle: ProjectBundle = serde_json::from_reader(project_json_reader)?;

        Ok(project_bundle)
    }
}
