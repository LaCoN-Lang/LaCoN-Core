#[derive(Debug, Clone)]
pub enum ErrorType {
	Unknown,
	Lexical(LexicalError),
	Syntax(SyntaxError),
	Semantic(SemanticError),
	Runtime,
	Expected,
}

#[derive(Debug, Clone)]
pub enum LexicalError {
	InvalidCharacter(char),
	UnterminatedString,
	UnterminatedBlockComment,
	InvalidIndent,
	InvalidEscapeSequence,
	InvalidToken,
}

#[derive(Debug, Clone)]
pub enum SyntaxError {
	UnexpectedToken,
	MissingToken,
	InvalidExpression,
	UnsupportedEscapeSequence,
	InvalidStatement,
	InvalidDeclaration,
	InvalidType,
	InvalidFunctionDefinition,
	InvalidParameter,
	InvalidArgument,
	InvalidOperatorUsage,
	InvalidControlFlow,
}

#[derive(Debug, Clone)]
pub enum SemanticError {
	TypeMismatch,
	UndefinedVariable,
	Redefinition,
	InvalidOperation,
	ConstAssignment,
	FunctionArityMismatch,
	InvalidReturn,
	InvalidBreakContinue,
	InvalidImport,
	ModuleNotFound,
	AccessViolation,
	InvalidFieldAccess,
	InvalidIndexAccess,
	InvalidMethodAccess,
	InvalidOperator,
}
