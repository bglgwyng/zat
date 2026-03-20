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
  name: (identifier) @name) @show_if_ref

; Class methods (public, identified by property_identifier name)
(method_definition
  name: (property_identifier)) @show.indent

; Class fields (public, identified by property_identifier)
(field_definition
  property: (property_identifier)) @show.indent.noloc
