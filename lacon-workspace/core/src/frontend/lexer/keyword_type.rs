pub enum KeywordType {
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
}

// TODO: Отделить кейворды от типов токенов, ввести в типы токены единый тип Keyword
