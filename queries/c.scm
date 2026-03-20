; Function definitions
(function_definition) @show

; Function declarations (prototypes)
(declaration
  declarator: (function_declarator)) @show

; Struct with fields
(struct_specifier
  name: (_)) @show

; Struct fields
(field_declaration) @show.indent

; Enum
(enum_specifier
  name: (_)) @show

; Enum values
(enumerator) @show.indent

; Typedef
(type_definition) @show
