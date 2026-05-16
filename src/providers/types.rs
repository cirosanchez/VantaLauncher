pub(crate) struct Project {
    id: String,
    display_name: String,
    description: String,
    icon_url: Option<String>,
    downloads: u64,
    versions: Vec<String>,
    project_type: ProjectType,
    provider: Provider
}

pub(crate) enum ProjectType {
    Mod,
    Modpack,
    Shader,
    ResourcePack,
    DataPack
}

pub(crate) struct ProjectVersion {
    id: String,
    cdn_uri: String,
    loaders: Vec<Loader>,
    game_versions: Vec<String>,
    project_type: ProjectType,
}


enum Loader {
    Fabric,
    Forge,
    NeoForge,
    Quilt
}

enum Provider {
    Modrinth,
    Curseforge
}