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

; Class methods and fields
(method_definition) @show.indent
(field_definition) @show.indent.noloc
