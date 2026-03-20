; Exported function
(export_statement
  declaration: (function_declaration)) @signature

; Exported class with body
(export_statement
  declaration: (class_declaration
    body: (class_body) @body)) @signature

; Exported variable/const
(export_statement
  declaration: (lexical_declaration)) @signature

; Exported interface with body
(export_statement
  declaration: (interface_declaration
    body: (interface_body) @body)) @signature

; Exported type alias
(export_statement
  declaration: (type_alias_declaration)) @signature

; Exported enum with body
(export_statement
  declaration: (enum_declaration
    body: (enum_body) @body)) @signature

; Export default
(export_statement
  (function_declaration)) @signature

; Named exports
(export_statement) @signature
