└── root (kind: SyntaxFile)
    ├── items (kind: ItemList)
    │   └── child #0 (kind: FunctionWithBody)
    │       ├── attributes (kind: AttributeList) []
    │       ├── declaration (kind: FunctionDeclaration)
    │       │   ├── function_kw (kind: TerminalFunction)
    │       │   │   ├── leading_trivia (kind: Trivia)
    │       │   │   │   ├── child #0 (kind: TokenSingleLineComment): '/// 1 unused parameter: b'
    │       │   │   │   └── child #1 (kind: TokenNewline).
    │       │   │   ├── token (kind: TokenFunction): 'fn'
    │       │   │   └── trailing_trivia (kind: Trivia)
    │       │   │       └── child #0 (kind: TokenWhitespace).
    │       │   ├── name (kind: TerminalIdentifier)
    │       │   │   ├── leading_trivia (kind: Trivia) []
    │       │   │   ├── token (kind: TokenIdentifier): 'foo'
    │       │   │   └── trailing_trivia (kind: Trivia) []
    │       │   ├── generic_params (kind: OptionWrappedGenericParamListEmpty) []
    │       │   └── signature (kind: FunctionSignature)
    │       │       ├── lparen (kind: TerminalLParen)
    │       │       │   ├── leading_trivia (kind: Trivia) []
    │       │       │   ├── token (kind: TokenLParen): '('
    │       │       │   └── trailing_trivia (kind: Trivia) []
    │       │       ├── parameters (kind: ParamList)
    │       │       │   ├── item #0 (kind: Param)
    │       │       │   │   ├── modifiers (kind: ModifierList) []
    │       │       │   │   ├── name (kind: TerminalIdentifier)
    │       │       │   │   │   ├── leading_trivia (kind: Trivia) []
    │       │       │   │   │   ├── token (kind: TokenIdentifier): 'a'
    │       │       │   │   │   └── trailing_trivia (kind: Trivia) []
    │       │       │   │   └── type_clause (kind: TypeClause)
    │       │       │   │       ├── colon (kind: TerminalColon)
    │       │       │   │       │   ├── leading_trivia (kind: Trivia) []
    │       │       │   │       │   ├── token (kind: TokenColon): ':'
    │       │       │   │       │   └── trailing_trivia (kind: Trivia)
    │       │       │   │       │       └── child #0 (kind: TokenWhitespace).
    │       │       │   │       └── ty (kind: ExprPath)
    │       │       │   │           └── item #0 (kind: PathSegmentSimple)
    │       │       │   │               └── ident (kind: TerminalIdentifier)
    │       │       │   │                   ├── leading_trivia (kind: Trivia) []
    │       │       │   │                   ├── token (kind: TokenIdentifier): 'int'
    │       │       │   │                   └── trailing_trivia (kind: Trivia) []
    │       │       │   ├── separator #0 (kind: TerminalComma)
    │       │       │   │   ├── leading_trivia (kind: Trivia) []
    │       │       │   │   ├── token (kind: TokenComma): ','
    │       │       │   │   └── trailing_trivia (kind: Trivia)
    │       │       │   │       └── child #0 (kind: TokenWhitespace).
    │       │       │   └── item #1 (kind: Param)
    │       │       │       ├── modifiers (kind: ModifierList) []
    │       │       │       ├── name (kind: TerminalIdentifier)
    │       │       │       │   ├── leading_trivia (kind: Trivia) []
    │       │       │       │   ├── token (kind: TokenIdentifier): 'b'
    │       │       │       │   └── trailing_trivia (kind: Trivia) []
    │       │       │       └── type_clause (kind: TypeClause)
    │       │       │           ├── colon (kind: TerminalColon)
    │       │       │           │   ├── leading_trivia (kind: Trivia) []
    │       │       │           │   ├── token (kind: TokenColon): ':'
    │       │       │           │   └── trailing_trivia (kind: Trivia)
    │       │       │           │       └── child #0 (kind: TokenWhitespace).
    │       │       │           └── ty (kind: ExprPath)
    │       │       │               └── item #0 (kind: PathSegmentSimple)
    │       │       │                   └── ident (kind: TerminalIdentifier)
    │       │       │                       ├── leading_trivia (kind: Trivia) []
    │       │       │                       ├── token (kind: TokenIdentifier): 'int'
    │       │       │                       └── trailing_trivia (kind: Trivia) []
    │       │       ├── rparen (kind: TerminalRParen)
    │       │       │   ├── leading_trivia (kind: Trivia) []
    │       │       │   ├── token (kind: TokenRParen): ')'
    │       │       │   └── trailing_trivia (kind: Trivia)
    │       │       │       └── child #0 (kind: TokenWhitespace).
    │       │       ├── ret_ty (kind: ReturnTypeClause)
    │       │       │   ├── arrow (kind: TerminalArrow)
    │       │       │   │   ├── leading_trivia (kind: Trivia) []
    │       │       │   │   ├── token (kind: TokenArrow): '->'
    │       │       │   │   └── trailing_trivia (kind: Trivia)
    │       │       │   │       └── child #0 (kind: TokenWhitespace).
    │       │       │   └── ty (kind: ExprPath)
    │       │       │       └── item #0 (kind: PathSegmentSimple)
    │       │       │           └── ident (kind: TerminalIdentifier)
    │       │       │               ├── leading_trivia (kind: Trivia) []
    │       │       │               ├── token (kind: TokenIdentifier): 'int'
    │       │       │               └── trailing_trivia (kind: Trivia)
    │       │       │                   └── child #0 (kind: TokenWhitespace).
    │       │       ├── implicits_clause (kind: OptionImplicitsClauseEmpty) []
    │       │       └── optional_no_panic (kind: OptionTerminalNoPanicEmpty) []
    │       └── body (kind: ExprBlock)
    │           ├── lbrace (kind: TerminalLBrace)
    │           │   ├── leading_trivia (kind: Trivia) []
    │           │   ├── token (kind: TokenLBrace): '{'
    │           │   └── trailing_trivia (kind: Trivia)
    │           │       └── child #0 (kind: TokenNewline).
    │           ├── statements (kind: StatementList)
    │           │   └── child #0 (kind: StatementReturn)
    │           │       ├── return_kw (kind: TerminalReturn)
    │           │       │   ├── leading_trivia (kind: Trivia)
    │           │       │   │   └── child #0 (kind: TokenWhitespace).
    │           │       │   ├── token (kind: TokenReturn): 'return'
    │           │       │   └── trailing_trivia (kind: Trivia)
    │           │       │       └── child #0 (kind: TokenWhitespace).
    │           │       ├── expr_clause (kind: ExprClause)
    │           │       │   └── expr (kind: ExprBinary)
    │           │       │       ├── lhs (kind: ExprPath)
    │           │       │       │   └── item #0 (kind: PathSegmentSimple)
    │           │       │       │       └── ident (kind: TerminalIdentifier)
    │           │       │       │           ├── leading_trivia (kind: Trivia) []
    │           │       │       │           ├── token (kind: TokenIdentifier): 'a'
    │           │       │       │           └── trailing_trivia (kind: Trivia)
    │           │       │       │               └── child #0 (kind: TokenWhitespace).
    │           │       │       ├── op (kind: TerminalPlus)
    │           │       │       │   ├── leading_trivia (kind: Trivia) []
    │           │       │       │   ├── token (kind: TokenPlus): '+'
    │           │       │       │   └── trailing_trivia (kind: Trivia)
    │           │       │       │       └── child #0 (kind: TokenWhitespace).
    │           │       │       └── rhs (kind: ExprPath)
    │           │       │           └── item #0 (kind: PathSegmentSimple)
    │           │       │               └── ident (kind: TerminalIdentifier)
    │           │       │                   ├── leading_trivia (kind: Trivia) []
    │           │       │                   ├── token (kind: TokenIdentifier): 'a'
    │           │       │                   └── trailing_trivia (kind: Trivia) []
    │           │       └── semicolon (kind: TerminalSemicolon)
    │           │           ├── leading_trivia (kind: Trivia) []
    │           │           ├── token (kind: TokenSemicolon): ';'
    │           │           └── trailing_trivia (kind: Trivia)
    │           │               └── child #0 (kind: TokenNewline).
    │           └── rbrace (kind: TerminalRBrace)
    │               ├── leading_trivia (kind: Trivia) []
    │               ├── token (kind: TokenRBrace): '}'
    │               └── trailing_trivia (kind: Trivia)
    │                   └── child #0 (kind: TokenNewline).
    └── eof (kind: TerminalEndOfFile)
        ├── leading_trivia (kind: Trivia)
        │   ├── child #0 (kind: TokenNewline).
        │   ├── child #1 (kind: TokenSingleLineComment): '/// Total unused parameters in this file: 1'
        │   └── child #2 (kind: TokenNewline).
        ├── token (kind: TokenEndOfFile).
        └── trailing_trivia (kind: Trivia) []
