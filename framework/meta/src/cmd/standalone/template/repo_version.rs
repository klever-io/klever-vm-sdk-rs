pub enum RepoVersion {
    Master,
    Tag(String),
}

impl RepoVersion {
    pub fn url(&self) -> String {
        match self {
            RepoVersion::Master => {
                "https://github.com/klever-io/klever-vm-sdk-rs/archive/refs/heads/master.zip".to_string()
            },
            RepoVersion::Tag(tag) => {
                format!("https://github.com/klever-io/klever-vm-sdk-rs/archive/refs/tags/v{tag}.zip")
            },
        }
    }

    pub fn temp_dir_name(&self) -> String {
        match self {
            RepoVersion::Master => "klever-vm-sdk-rs-master".to_string(),
            RepoVersion::Tag(tag) => {
                format!("klever-vm-sdk-rs-{tag}")
            },
        }
    }
}
