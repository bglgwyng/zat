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

; Exported variable/const (non-literal value, hide = value)
(export_statement
  "export" @strip
  declaration: (lexical_declaration
    (variable_declarator
      "=" @hide
      value: [(arrow_function) (function_expression) (object) (array)
              (call_expression) (new_expression) (await_expression)
              (member_expression) (subscript_expression) (identifier)
              (binary_expression) (unary_expression) (parenthesized_expression)
              (template_string) (class) (satisfies_expression)
              (as_expression) (non_null_expression)] @hide))) @show

; Exported variable/const (typed, hide = value)
(export_statement
  "export" @strip
  declaration: (lexical_declaration
    (variable_declarator
      type: (type_annotation)
      "=" @hide
      value: (_) @hide))) @show

; Exported variable/const (fallback - keeps literal values)
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

; Re-exports
(export_statement
  "export" @strip
  source: (string)) @show

; Named exports: resolve references
(export_statement
  (export_clause
    (export_specifier
      name: (identifier) @ref)))

; Non-exported declarations (for @ref resolution)
(lexical_declaration
  (variable_declarator
    name: (identifier) @name
    "=" @hide
    value: [(arrow_function) (function_expression) (object) (array)
            (call_expression) (new_expression) (await_expression)
            (member_expression) (subscript_expression) (identifier)
            (binary_expression) (unary_expression) (parenthesized_expression)
            (template_string) (class) (satisfies_expression)
            (as_expression) (non_null_expression)] @hide)) @show_if_ref

(lexical_declaration
  (variable_declarator
    name: (identifier) @name
    type: (type_annotation)
    "=" @hide
    value: (_) @hide)) @show_if_ref

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
    body: (statement_block) @hide) @show.indented
  (#not-match? @show.indented "^private"))
(method_definition
  ((accessibility_modifier) @strip
    (#eq? @strip "public")))

; Class fields (exclude private, strip "public")
((public_field_definition) @show.indented
  (#not-match? @show.indented "^private"))
(public_field_definition
  ((accessibility_modifier) @strip
    (#eq? @strip "public")))

; Interface members
(interface_body
  (property_signature) @show.indented.noloc)
(interface_body
  (method_signature) @show.indented)

; Enum members
(enum_assignment) @show.indented.noloc
