use std::path::{Path, PathBuf};

/// A `.gml` source file and it's corresponding `.gmlpp` and helper files if required
#[derive(Clone, Debug)]
pub struct Source(PathBuf);

impl Source {
    /// The path to the `.gml` file for this source
    pub fn gml(&self) -> PathBuf {
        self.0.with_extension("gml")
    }

    /// The path to the `.gmlpp` file for this source
    pub fn gmlpp(&self) -> PathBuf {
        self.0.with_extension("gmlpp")
    }

    /// The path to the helper `.gml` file for this source
    pub fn helper(&self) -> PathBuf { 
        // TODO: this is the wrong path
        self.0.with_extension("gml")
    }

    /// Determines the corresponding source files for a gml or gmlpp file
    pub fn from<P: AsRef<Path>>(path: P) -> Self {
        Source(path.as_ref().to_owned())
    }

    /// Adjusts all the paths of this source to the provided base path
    pub fn resolved_to(self, mut path: PathBuf) -> Self {
        path.push(self.0);
        Source(path)
    }
}


