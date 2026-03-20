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

; Export default
(export_statement
  (function_declaration)) @signature

; Named exports
(export_statement) @signature
