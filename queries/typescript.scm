; Imports
(import_statement) @show

; Exported function
(export_statement
  declaration: (function_declaration
    body: (statement_block) @hide) @show)

; Exported class
(export_statement
  declaration: (class_declaration
    body: (class_body) @hide) @show)

; Exported variable/const (non-literal value, hide = value)
(export_statement
  declaration: (lexical_declaration
    (variable_declarator
      "=" @hide
      value: [(arrow_function) (function_expression) (object) (array)
              (call_expression) (new_expression) (await_expression)
              (member_expression) (subscript_expression) (identifier)
              (binary_expression) (unary_expression) (parenthesized_expression)
              (template_string) (class) (satisfies_expression)
              (as_expression) (non_null_expression)] @hide)
    ";" @hide) @show)

; Exported variable/const (typed, hide = value)
(export_statement
  declaration: (lexical_declaration
    (variable_declarator
      type: (type_annotation)
      "=" @hide
      value: (_) @hide)
    ";" @hide) @show)

; Exported variable/const (fallback - keeps literal values)
(export_statement
  declaration: (lexical_declaration) @show)

; Exported interface
(export_statement
  declaration: (interface_declaration
    body: (interface_body) @hide) @show)

; Exported type alias (object type, show members)
(export_statement
  declaration: (type_alias_declaration
    value: (object_type) @hide) @show)

; Exported type alias (other)
(export_statement
  declaration: (type_alias_declaration) @show)

; Exported enum
(export_statement
  declaration: (enum_declaration
    body: (enum_body) @hide) @show)

; Export default
(export_statement
  (function_declaration
    body: (statement_block) @hide) @show)

; Re-exports
(export_statement
  "export" @hide
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
  name: (type_identifier) @name
  value: (object_type) @hide) @show_if_ref

(type_alias_declaration
  name: (type_identifier) @name) @show_if_ref

(enum_declaration
  name: (identifier) @name
  body: (enum_body) @hide) @show_if_ref

; Class body braces
(class_body "{" @show.noloc)
(class_body "}" @show.noloc)

; Class methods (exclude private, hide "public")
((method_definition
    body: (statement_block) @hide) @show
  (#not-match? @show "^private"))
(method_definition
  ((accessibility_modifier) @hide
    (#eq? @hide "public")))

; Class fields (exclude private, hide "public", hide initializer)
((public_field_definition
    "=" @hide
    value: (_) @hide) @show
  (#not-match? @show "^private"))
((public_field_definition) @show
  (#not-match? @show "^private"))
(public_field_definition
  ((accessibility_modifier) @hide
    (#eq? @hide "public")))

; Interface members
(interface_body
  "{" @show.noloc)
(interface_body
  "}" @show.noloc)
(interface_body
  (property_signature) @show.noloc)
(interface_body
  (method_signature) @show)

; Object type members (for type aliases)
(object_type
  "{" @show.noloc)
(object_type
  "}" @show.noloc)
(object_type
  (property_signature) @show.noloc)
(object_type
  (method_signature) @show)

; Enum braces
(enum_body "{" @show.noloc)
(enum_body "}" @show.noloc)

; Enum members
(enum_assignment) @show.noloc
