use std::{collections::HashMap, path::PathBuf};

struct Data {
    pub projector: HashMap<PathBuf, HashMap<String, String>>,
}