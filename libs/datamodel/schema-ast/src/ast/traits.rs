use super::{Attribute, Comment, Identifier, Span};

/// An AST node with a span.
pub trait WithSpan {
    /// The span of the node.
    fn span(&self) -> &Span;
}

/// An AST node with a name (from the identifier).
pub trait WithName {
    /// The name of the item.
    fn name(&self) -> &str;
}

/// An AST node with an identifier.
pub trait WithIdentifier {
    /// The identifier.
    fn identifier(&self) -> &Identifier;
}

/// An AST node with attributes.
pub trait WithAttributes {
    /// The attributes.
    fn attributes(&self) -> &[Attribute];
}

/// An AST node with documentation.
pub trait WithDocumentation {
    /// The documentation string, if defined.
    fn documentation(&self) -> &Option<Comment>;

    /// Is this node commented out?
    fn is_commented_out(&self) -> bool;
}

impl<T> WithName for T
where
    T: WithIdentifier,
{
    fn name(&self) -> &str {
        &self.identifier().name
    }
}
