use std::io;

use crate::sb2::*;

use zip::{ZipArchive, result::ZipError};

pub type ProjectLoadResult = Result<Project, ProjectLoadError>;

#[derive(Debug)]
pub enum ProjectLoadError {
    Io(std::io::Error),
    Parse(serde_json::Error),
    Zip(ZipError),
}

impl From<std::io::Error> for ProjectLoadError {
    fn from(err: std::io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<serde_json::Error> for ProjectLoadError {
    fn from(err: serde_json::Error) -> Self {
        Self::Parse(err)
    }
}

impl From<ZipError> for ProjectLoadError {
    fn from(err: ZipError) -> Self {
        Self::Zip(err)
    }
}

impl std::fmt::Display for ProjectLoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Io(err) => write!(f, "{}", err),
            Self::Parse(err) => write!(f, "{}", err),
            Self::Zip(err) => write!(f, "{}", err),
        }
    }
}

impl Project {
    pub fn from_reader<R>(sb2_reader: R) -> Result<Project, ProjectLoadError>
    where
        R: io::Read + std::io::Seek,
    {
        // this will open the ZIP and read the central directory
        let mut sb2_zip = ZipArchive::new(sb2_reader)?;

        let project_json_reader = sb2_zip.by_name("project.json")?;

        // let project_json: serde_json::Value = serde_json::from_reader(project_json_reader)?;
        // info!("Project loaded data: {:#?}", project_json);
        // Ok(ProjectBundle {
        //     title: "hi".to_string(),
        //     extensions: vec![],
        //     meta: serde_json::Value::Null,
        //     monitors: vec![],
        //     targets: vec![],
        // })

        let project_bundle: Project = serde_json::from_reader(project_json_reader)?;

        Ok(project_bundle)
    }
}
