; Exported function
(export_statement
  declaration: (function_declaration)) @show

; Exported class
(export_statement
  declaration: (class_declaration)) @show

; Exported variable/const
(export_statement
  declaration: (lexical_declaration)) @show

; Exported interface
(export_statement
  declaration: (interface_declaration)) @show

; Exported type alias
(export_statement
  declaration: (type_alias_declaration)) @show

; Exported enum
(export_statement
  declaration: (enum_declaration)) @show

; Export default
(export_statement
  (function_declaration)) @show

; Named exports
(export_statement) @show

; Class methods and fields
(method_definition) @show.indent
(public_field_definition) @show.indent

; Interface members
(property_signature) @show.indent.noloc
(method_signature) @show.indent

; Enum members
(enum_assignment) @show.indent.noloc
