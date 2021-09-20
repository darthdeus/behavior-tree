#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Initialized,
    Success,
    Failure,
    Running,
}

impl core::default::Default for Status {
    fn default() -> Self {
        Self::Initialized
    }
}

#[derive(Clone, Debug)]
pub struct TreeRepr {
    pub name: String,
    pub status: Status,
    pub detail: String,
    pub children: Vec<TreeRepr>,
}

impl TreeRepr {
    pub fn new<T: AsRef<str>>(name: T, children: Vec<TreeRepr>) -> Self {
        Self {
            name: name.as_ref().to_owned(),
            status: Status::Running,
            detail: "".to_owned(),
            children,
        }
    }

    pub fn with_detail(self, detail: String) -> Self {
        Self { detail, ..self }
    }

    pub fn with_status(self, status: Status) -> Self {
        Self { status, ..self }
    }
}

#[derive(Debug)]
pub enum Cursor {
    // Condition(bool),
    Index(usize, Box<DebugRepr>),
    Leaf,
}

impl core::default::Default for Cursor {
    fn default() -> Self {
        Self::Leaf
    }
}

impl Cursor {
    pub fn index(&self) -> usize {
        match self {
            Cursor::Index(i, _) => *i,
            Cursor::Leaf => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
pub struct DebugRepr {
    pub name: String,
    pub params: Option<String>,
    pub status: Status,
    pub bool_override: Option<bool>,
    pub cursor: Cursor,
}

impl DebugRepr {
    pub fn new(name: &str, cursor: Cursor, status: Status) -> Self {
        Self {
            name: name.to_string(),
            cursor,
            status,
            bool_override: None,
            params: None,
        }
    }

    pub fn with_override(self, bool_override: bool) -> Self {
        Self {
            bool_override: Some(bool_override),
            ..self
        }
    }
}
