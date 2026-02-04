use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub enum ErrorType {
	#[error("Unknown error")]
	Unknown,
	#[error("Lexical error: {0}")]
	Lexical(LexicalError),
	#[error("Syntax error: {0}")]
	Syntax(SyntaxError),
	#[error("Semantic error: {0}")]
	Semantic(SemanticError),
	#[error("Runtime error")]
	Runtime,
}

#[derive(Debug, Clone, Error)]
pub enum LexicalError {
	#[error("Invalid character: {0}")]
	InvalidCharacter(char),
	#[error("Unterminated string literal")]
	UnterminatedString,
	#[error("Unterminated block comment")]
	UnterminatedBlockComment,
	#[error("Invalid indentation")]
	InvalidIndent,
	#[error("Invalid escape sequence: {0}")]
	InvalidEscapeSequence(String),
	#[error("Invalid token")]
	InvalidToken,
}

#[derive(Debug, Clone, Error)]
pub enum SyntaxError {
	#[error("Expected {expected}, found {found:?}")]
	Expected { expected: String, found: Vec<String> },
	// UnexpectedToken,
	// MissingToken,
	// InvalidExpression,
	// UnsupportedEscapeSequence,
	// InvalidStatement,
	// InvalidDeclaration,
	// InvalidType,
	// InvalidFunctionDefinition,
	// InvalidParameter,
	// InvalidArgument,
	// InvalidOperatorUsage,
	// InvalidControlFlow,
}

#[derive(Debug, Clone, Error)]
pub enum SemanticError {
	#[error("Expected type {expected}, found {found}")]
	TypeMismatch { expected: String, found: String },
	// UndefinedVariable,
	// Redefinition,
	// InvalidOperation,
	// ConstAssignment,
	// FunctionArityMismatch,
	// InvalidReturn,
	// InvalidBreakContinue,
	// InvalidImport,
	// ModuleNotFound,
	// AccessViolation,
	// InvalidFieldAccess,
	// InvalidIndexAccess,
	// InvalidMethodAccess,
	// InvalidOperator,
}
