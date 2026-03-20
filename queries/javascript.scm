; Exported function
(export_statement
  declaration: (function_declaration)) @show

; Exported class
(export_statement
  declaration: (class_declaration)) @show

; Exported variable/const
(export_statement
  declaration: (lexical_declaration)) @show

; Export default
(export_statement
  (function_declaration)) @show

; Named exports
(export_statement) @show

; Class methods (public, identified by property_identifier name)
(method_definition
  name: (property_identifier)) @show.indent

; Class fields (public, identified by property_identifier)
(field_definition
  property: (property_identifier)) @show.indent.noloc
