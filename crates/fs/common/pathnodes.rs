//--------------------------------------------------------------------------------------------------
// Type Definitions
//--------------------------------------------------------------------------------------------------

/// Represents the directory nodes along a path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PathNodes<T> {
    pub path: Vec<(T, String)>,
    pub tail: T,
}

/// The kinds of outcome from getting a `PathNodes`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathNodesResult<T> {
    /// The complete path exists.
    Complete(PathNodes<T>),

    /// The path does not exist.
    MissingLink(PathNodes<T>, String),

    /// Encountered a node that is not a directory.
    NotADirectory(PathNodes<T>, String),
}

//--------------------------------------------------------------------------------------------------
// Implementations
//--------------------------------------------------------------------------------------------------

impl<T> PathNodes<T> {
    /// Returns the length of the path nodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PathNodes, public::PublicDirectory};
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// let nodes = PathNodes::<Rc<PublicDirectory>> {
    ///     path: vec![
    ///         (Rc::new(PublicDirectory::new(Utc::now())), "music".to_string()),
    ///         (Rc::new(PublicDirectory::new(Utc::now())), "rock".to_string()),
    ///     ],
    ///     tail: Rc::new(PublicDirectory::new(Utc::now())),
    /// };
    ///
    /// assert_eq!(nodes.len(), 2);
    /// ```
    pub fn len(&self) -> usize {
        self.path.len()
    }

    /// Checks if the path nodes are empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use wnfs::{PathNodes, public::PublicDirectory};
    /// use std::rc::Rc;
    /// use chrono::Utc;
    ///
    /// let nodes = PathNodes::<Rc<PublicDirectory>> {
    ///     path: vec![
    ///         (Rc::new(PublicDirectory::new(Utc::now())), "music".to_string()),
    ///         (Rc::new(PublicDirectory::new(Utc::now())), "rock".to_string()),
    ///     ],
    ///     tail: Rc::new(PublicDirectory::new(Utc::now())),
    /// };
    ///
    /// assert!(!nodes.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.path.is_empty()
    }
}
