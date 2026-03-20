; Exported function
(export_statement
  "export" @strip
  declaration: (function_declaration
    body: (statement_block) @hide)) @show

; Exported class
(export_statement
  "export" @strip
  declaration: (class_declaration
    body: (class_body) @hide)) @show

; Exported variable/const (typed, hide = value)
(export_statement
  "export" @strip
  declaration: (lexical_declaration
    (variable_declarator
      type: (type_annotation)
      "=" @hide
      value: (_) @hide))) @show

; Exported variable/const (arrow function, hide body)
(export_statement
  "export" @strip
  declaration: (lexical_declaration
    (variable_declarator
      value: (arrow_function
        body: (statement_block) @hide)))) @show

; Exported variable/const (function expression, hide body)
(export_statement
  "export" @strip
  declaration: (lexical_declaration
    (variable_declarator
      value: (function_expression
        body: (statement_block) @hide)))) @show

; Exported variable/const (fallback)
(export_statement
  "export" @strip
  declaration: (lexical_declaration)) @show

; Exported interface
(export_statement
  "export" @strip
  declaration: (interface_declaration
    body: (interface_body) @hide)) @show

; Exported type alias
(export_statement
  "export" @strip
  declaration: (type_alias_declaration)) @show

; Exported enum
(export_statement
  "export" @strip
  declaration: (enum_declaration
    body: (enum_body) @hide)) @show

; Export default
(export_statement
  "export" @strip
  (function_declaration
    body: (statement_block) @hide)) @show

; Named exports: resolve references
(export_statement
  (export_clause
    (export_specifier
      name: (identifier) @ref)))

; Non-exported declarations (for @ref resolution)
(lexical_declaration
  (variable_declarator
    name: (identifier) @name)) @show_if_ref

(function_declaration
  name: (identifier) @name
  body: (statement_block) @hide) @show_if_ref

(class_declaration
  name: (type_identifier) @name
  body: (class_body) @hide) @show_if_ref

(interface_declaration
  name: (type_identifier) @name
  body: (interface_body) @hide) @show_if_ref

(type_alias_declaration
  name: (type_identifier) @name) @show_if_ref

(enum_declaration
  name: (identifier) @name
  body: (enum_body) @hide) @show_if_ref

; Class methods (exclude private, strip "public")
((method_definition
    body: (statement_block) @hide) @show.indent
  (#not-match? @show.indent "^private"))
(method_definition
  ((accessibility_modifier) @strip
    (#eq? @strip "public")))

; Class fields (exclude private, strip "public")
((public_field_definition) @show.indent
  (#not-match? @show.indent "^private"))
(public_field_definition
  ((accessibility_modifier) @strip
    (#eq? @strip "public")))

; Interface members
(property_signature) @show.indent.noloc
(method_signature) @show.indent

; Enum members
(enum_assignment) @show.indent.noloc
