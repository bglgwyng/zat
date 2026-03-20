; Function definitions
(function_definition
  body: (compound_statement) @hide) @show

; Function declarations (prototypes)
(declaration
  declarator: (function_declarator)) @show

; Struct with fields
(struct_specifier
  name: (_)
  body: (field_declaration_list) @hide) @show

; Struct forward declaration
(struct_specifier
  name: (_)) @show

; Struct fields
(field_declaration) @show.indent.noloc

; Enum
(enum_specifier
  name: (_)
  body: (enumerator_list) @hide) @show

; Enum forward declaration
(enum_specifier
  name: (_)) @show

; Enum values
(enumerator) @show.indent.noloc

; Typedef
(type_definition) @show
