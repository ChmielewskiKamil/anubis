└── root (kind: SyntaxFile)
    ├── items (kind: ItemList)
    │   ├── child #0 (kind: ItemUse)
    │   │   ├── attributes (kind: AttributeList) []
    │   │   ├── use_kw (kind: TokenUse): 'use'
    │   │   ├── use_path (kind: UsePathSingle)
    │   │   │   ├── ident (kind: PathSegmentSimple)
    │   │   │   │   └── ident (kind: TokenIdentifier): 'std'
    │   │   │   ├── colon_colon (kind: TokenColonColon): '::'
    │   │   │   └── use_path (kind: UsePathLeaf)
    │   │   │       ├── ident (kind: PathSegmentSimple)
    │   │   │       │   └── ident (kind: TokenIdentifier): 'format'
    │   │   │       └── alias_clause (kind: OptionAliasClauseEmpty) []
    │   │   └── semicolon (kind: TokenSemicolon): ';'
    │   └── child #1 (kind: FunctionWithBody)
    │       ├── attributes (kind: AttributeList) []
    │       ├── declaration (kind: FunctionDeclaration)
    │       │   ├── function_kw (kind: TokenFunction): 'fn'
    │       │   ├── name (kind: TokenIdentifier): 'foo'
    │       │   ├── generic_params (kind: OptionWrappedGenericParamListEmpty) []
    │       │   └── signature (kind: FunctionSignature)
    │       │       ├── lparen (kind: TokenLParen): '('
    │       │       ├── parameters (kind: ParamList)
    │       │       │   └── item #0 (kind: Param)
    │       │       │       ├── modifiers (kind: ModifierList) []
    │       │       │       ├── name (kind: TokenIdentifier): 'a'
    │       │       │       └── type_clause (kind: TypeClause)
    │       │       │           ├── colon (kind: TokenColon): ':'
    │       │       │           └── ty (kind: ExprPath)
    │       │       │               └── item #0 (kind: PathSegmentSimple)
    │       │       │                   └── ident (kind: TokenIdentifier): 'int'
    │       │       ├── rparen (kind: TokenRParen): ')'
    │       │       ├── ret_ty (kind: ReturnTypeClause)
    │       │       │   ├── arrow (kind: TokenArrow): '->'
    │       │       │   └── ty (kind: ExprPath)
    │       │       │       └── item #0 (kind: PathSegmentSimple)
    │       │       │           └── ident (kind: TokenIdentifier): 'felt252'
    │       │       ├── implicits_clause (kind: OptionImplicitsClauseEmpty) []
    │       │       └── optional_no_panic (kind: OptionTerminalNoPanicEmpty) []
    │       └── body (kind: ExprBlock)
    │           ├── lbrace (kind: TokenLBrace): '{'
    │           ├── statements (kind: StatementList)
    │           │   └── child #0 (kind: StatementReturn)
    │           │       ├── return_kw (kind: TokenReturn): 'return'
    │           │       ├── expr_clause (kind: ExprClause)
    │           │       │   └── expr (kind: ExprPath)
    │           │       │       └── item #0 (kind: PathSegmentSimple)
    │           │       │           └── ident (kind: TokenIdentifier): 'a'
    │           │       └── semicolon (kind: TokenSemicolon): ';'
    │           └── rbrace (kind: TokenRBrace): '}'
    └── eof (kind: TokenEndOfFile).
