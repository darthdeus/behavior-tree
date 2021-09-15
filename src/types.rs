#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Status {
    Success,
    Failure,
    Running,
    // Initialized,
}

impl core::default::Default for Status {
    fn default() -> Self {
        Self::Running
    }
}

#[derive(Clone, Debug)]
pub struct TreeRepr {
    pub name: &'static str,
    pub status: Status,
    pub detail: String,
    pub children: Vec<TreeRepr>,
}

impl TreeRepr {
    pub fn new(name: &'static str, children: Vec<TreeRepr>) -> Self {
        Self {
            name,
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

    pub cursor: Cursor,
}

impl DebugRepr {
    pub fn new(name: &str, cursor: Cursor, status: Status) -> Self {
        Self {
            name: name.to_string(),
            cursor,
            status,
            params: None,
        }
    }
}
