; Exported function
(export_statement
  "export" @strip
  declaration: (function_declaration)) @show

; Exported class
(export_statement
  "export" @strip
  declaration: (class_declaration)) @show

; Exported variable/const
(export_statement
  "export" @strip
  declaration: (lexical_declaration)) @show

; Exported interface
(export_statement
  "export" @strip
  declaration: (interface_declaration)) @show

; Exported type alias
(export_statement
  "export" @strip
  declaration: (type_alias_declaration)) @show

; Exported enum
(export_statement
  "export" @strip
  declaration: (enum_declaration)) @show

; Export default
(export_statement
  "export" @strip
  (function_declaration)) @show

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
  name: (identifier) @name) @show_if_ref

(class_declaration
  name: (type_identifier) @name) @show_if_ref

(interface_declaration
  name: (type_identifier) @name) @show_if_ref

(type_alias_declaration
  name: (type_identifier) @name) @show_if_ref

(enum_declaration
  name: (identifier) @name) @show_if_ref

; Class methods (exclude private, strip "public")
((method_definition) @show.indent
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
