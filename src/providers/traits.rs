use anyhow::Result;

use crate::providers::types::{
    Project,
    ProjectType,
    ProjectVersion,
};

trait ContentProvider {
    fn search_projects(
        &self,
        query: Option<&str>,
        project_type: ProjectType,
    ) -> Result<Vec<Project>>;

    fn get_project(
        &self,
        project_id: &str,
    ) -> Result<Project>;

    fn get_project_versions(
        &self,
        project_id: &str,
        game_version: Option<&str>,
        loader: Option<&str>,
    ) -> Result<Vec<ProjectVersion>>;

    fn download_version(
        &self,
        version: &ProjectVersion,
        destination: &str,
    ) -> Result<()>;
}