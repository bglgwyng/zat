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
              (template_string) (class)] @hide))) @show

; Exported variable/const (fallback - keeps literal values)
(export_statement
  "export" @strip
  declaration: (lexical_declaration)) @show

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
            (template_string) (class)] @hide)) @show_if_ref

(lexical_declaration
  (variable_declarator
    name: (identifier) @name)) @show_if_ref

(function_declaration
  name: (identifier) @name
  body: (statement_block) @hide) @show_if_ref

(class_declaration
  name: (identifier) @name
  body: (class_body) @hide) @show_if_ref

; Class methods (public, identified by property_identifier name)
(method_definition
  name: (property_identifier)
  body: (statement_block) @hide) @show.indent

; Class fields (public, identified by property_identifier)
(field_definition
  property: (property_identifier)) @show.indent.noloc
