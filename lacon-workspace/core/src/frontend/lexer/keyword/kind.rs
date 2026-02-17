#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeywordKind {
	// ─────────────────────────────────────────────
	// Логические операторы
	// ─────────────────────────────────────────────
	And, // and \\ LogicalAnd (keyword)
	Or,  // or  \\ LogicalOr (keyword)
	Not, // not \\ LogicalNot (keyword)

	// ─────────────────────────────────────────────
	// Управление потоком
	// ─────────────────────────────────────────────
	If,        // if   \\ Conditional
	Else,      // else \\ AlternativeBranch
	Elif,      // elif \\ ElseIf
	Match,     // match\\ PatternMatch
	Case,      // case \\ MatchArm
	Default,   // default \\ FallbackCase
	Switch,    // switch \\ SwitchStatement
	For,       // for  \\ LoopFor
	While,     // while\\ LoopWhile
	Loop,      // loop \\ InfiniteLoop
	Until,     // until \\ LoopUntil
	Spread,    // spread \\ ExpansionDirective
	Generate,  // generate \\ GeneratorBlock
	Combine,   // combine \\ Combine
	Enumerate, // enumerate \\ Enumeration
	Filter,    // filter \\ Filter
	Flatten,   // flatten \\ Flatten
	Repeat,    // repeat \\ Repeat
	Transform, // transform \\ Transform
	Transpose, // transpose \\ Transpose
	Break,     // break \\ LoopBreak
	Continue,  // continue \\ LoopContinue
	Return,    // return \\ FunctionReturn
	Yield,     // yield \\ GeneratorYield
	Exit,      // exit \\ ProgramExit
	Cancel,    // cancel \\ AbortExecution
	Try,       // try  \\ ExceptionBlock
	Catch,     // catch\\ ExceptionHandler
	Finally,   // finally \\ CleanupBlock
	Throw,     // throw \\ RaiseException
	Await,     // await \\ AsyncAwait
	Async,     // async \\ AsyncContext
	Coroutine, // coroutine \\ CoroutineDecl
	Defer,     // defer \\ DeferredExecution

	// ─────────────────────────────────────────────
	// Объявления и структура программы
	// ─────────────────────────────────────────────
	Declare,   // declare \\ Declaration
	Class,     // class \\ ClassDecl
	Interface, // interface \\ InterfaceDecl
	Enum,      // enum \\ EnumDecl
	Container, // container \\ Namespace / Module
	Callable,  // callable \\ Callable (заменить функции и процедуры на callable)
	Function,  // function \\ FunctionDecl
	Procedure, // procedure \\ ProcedureDecl
	Event,     // event \\ Event
	Variable,  // var  \\ VariableDecl
	Constant,  // const\\ ConstantDecl
	Entry,     // entry \\ Entry
	Structure, // struct \\ StructureDecl
	Import,    // import \\ ImportModule
	Export,    // export \\ ExportSymbol
	From,      // from \\ ImportSource
	Include,   // include \\ IncludeFile
	Provide,   // provide \\ ProvideDataToInclude
	New,       // new \\ NewInstance
	Use,       // use \\ Use
	Schema,    // \\ Data schema for Data Output

	Sanction,   // sanction \\ Sanction
	Be,         // be \\ Be
	Only,       // only \\ Only
	Context,    // context \\ Context
	Condition,  // condition \\ Condition
	Action,     // action \\ Action
	Capability, // capability \\ Capability
	May,        // may \\ May

	//
	Set,
	Get,
	Trigger,
	On,
	Unset,
	Untrigger,
	UnsetAll,

	// ─────────────────────────────────────────────
	// Типовая система
	// ─────────────────────────────────────────────
	Type,           // type \\ TypeDecl
	AutoValue,      // auto \\ AutoValue
	Alias,          // alias\\ TypeAlias
	Generic,        // <T>  \\ GenericParam
	UndefinedValue, // undefined \\ UndefinedValue
	NoneValue,      // none \\ NoneValue
	NilValue,       // nil  \\ NilValue
	Boolean,        // bool \\ BooleanType
	As,             // as   \\ TypeCast
	Is,             // is   \\ TypeCheck
	Extends,        // extends \\ Inheritance
	Implements,     // implements \\ InterfaceImpl
	In,             // in   \\ Membership
	Of,             // of   \\ Association
	Where,          // where \\ TypeConstraint
	When,           // when \\ ConditionalGuard
	Contains,       // contains \\ CollectionContains
	With,           // with \\ Composition

	// ─────────────────────────────────────────────
	// Контекст объекта
	// ─────────────────────────────────────────────
	This,      // this \\ CurrentInstance
	SelfScope, // self \\ IntrospectiveScope
	Origin,    // origin \\ OriginSource, позволяет ссылаться на исходный объект (текущий статик)
	Super,     // super\\ BaseInstance (родительский статик)
	Root,      // root \\ ObjectRoot
	Parent,    // parent \\ CurrentParent
	Here,      // here \\ CurrentLocation

	// ─────────────────────────────────────────────
	// Модификаторы доступа и ОО
	// ─────────────────────────────────────────────
	Public,    // public \\ PublicAccess
	Private,   // private \\ PrivateAccess
	Protected, // protected \\ ProtectedAccess
	Internal,  // internal \\ ModuleAccess
	External,  // external \\ ExternalLinkage
	Global,    // global \\ GlobalAccess
	Local,     // local \\ LocalAccess
	Static,    // static \\ StaticMember
	Virtual,   // virtual \\ Overridable
	Abstract,  // abstract \\ AbstractMember
	Override,  // override \\ OverrideBase
	Final,     // final \\ NonOverridable
	Strict,    // strict \\ StrictMode

	// ─────────────────────────────────────────────
	// Метапрограммирование / атрибуты
	// ─────────────────────────────────────────────
	Meta,      // meta \\ MetaContext
	Reflect,   // reflect \\ Reflection
	Attribute, // attribute \\ Annotation

	NumberInfinity,
	Delta,
	Xor,
	Bitwise,
	SectionMaker,
	Marker,
}
