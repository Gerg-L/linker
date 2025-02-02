use derivative::Derivative;
use log::{
    error,
    info,
};
use serde::{
    Deserialize,
    Deserializer,
    Serialize,
};
use std::{
    cmp::Ordering,
    fs::{
        self,
        Metadata,
    },
    path::{
        Path,
        PathBuf,
    },
};

pub const VERSION: u16 = 1;

#[derive(Serialize, Deserialize)]
pub struct Manifest {
    pub files: Vec<File>,
    pub clobber_by_default: bool,
    pub version: u16,
}

fn deserialize_octal<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<u32>, D::Error> {
    if let Some(str) = Option::<String>::deserialize(deserializer)? {
        match u32::from_str_radix(&str, 8) {
            Ok(x) => Ok(Some(x)),
            Err(e) => Err(serde::de::Error::custom(e)),
        }
    } else {
        Ok(None)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Derivative)]
#[derivative(PartialEq, Eq)]
pub struct File {
    pub source: Option<PathBuf>,
    pub target: PathBuf,
    #[serde(rename = "type")]
    pub kind: FileKind,
    pub clobber: Option<bool>,
    #[serde(default, deserialize_with = "deserialize_octal")]
    pub permissions: Option<u32>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,

    #[serde(skip)]
    #[derivative(PartialEq = "ignore")]
    pub metadata: Option<Metadata>,
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum FileKind {
    Directory,
    RecursiveSymlink,
    File,
    Symlink,
    Modify,
    Delete,
}

impl Ord for FileKind {
    fn cmp(&self, other: &Self) -> Ordering {
        fn value(kind: &FileKind) -> u8 {
            match kind {
                FileKind::Directory => 1,
                FileKind::RecursiveSymlink => 2,
                FileKind::File => 3,
                FileKind::Symlink => 4,
                FileKind::Modify => 5,
                FileKind::Delete => 7,
            }
        }
        value(self).cmp(&value(other))
    }
}

impl PartialOrd for FileKind {
    fn partial_cmp(&self, other: &FileKind) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Manifest {
    pub fn read(manifest_path: &Path) -> Manifest {
        let read_manifest = fs::read_to_string(manifest_path).expect("Failed to read manifest");
        let deserialized_manifest: Manifest =
            serde_json::from_str(&read_manifest).expect("Failed to read manifest");

        info!("Deserialized manifest: '{}'", manifest_path.display());
        info!("Manifest version: '{}'", deserialized_manifest.version);
        info!("Program version: '{}'", VERSION);

        if deserialized_manifest.version != VERSION {
            panic!("Version mismatch!\n Program and manifest version must be the same");
        };
        deserialized_manifest
    }

    pub fn activate(&mut self, prefix: &str) {
        self.files.sort_by_key(|f| f.kind);

        self.files.iter_mut().for_each(|file| {
            if let Err(e) = file.activate(self.clobber_by_default, prefix) {
                error!(
                    "Failed to activate file: '{}'\n Reason: '{}'",
                    file.target.display(),
                    e
                );
            }
        })
    }

    pub fn deactivate(&mut self) {
        self.files.sort_by_key(|f| f.kind);

        self.files.iter_mut().rev().for_each(|file| {
            if let Err(e) = file.deactivate() {
                error!(
                    "Failed to deactivate file: '{}'\n Reason: '{}'",
                    file.target.display(),
                    e
                );
            }
        });
    }

    pub fn diff(mut self, mut old_manifest: Self, prefix: &str) {
        let mut intersection: Vec<File> = vec![];

        old_manifest.files.retain(|f| {
            let contains = self.files.contains(f);
            if contains {
                intersection.push(f.clone());
            }
            !contains
        });

        self.files.retain(|f| !old_manifest.files.contains(f));

        old_manifest.deactivate();
        self.activate(prefix);
    }
}
