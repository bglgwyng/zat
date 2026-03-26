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

; Struct braces
(struct_specifier
  body: (field_declaration_list
    "{" @show.noloc))
(struct_specifier
  body: (field_declaration_list
    "}" @show.noloc))

; Struct fields
(field_declaration) @show.noloc

; Enum
(enum_specifier
  name: (_)
  body: (enumerator_list) @hide) @show

; Enum forward declaration
(enum_specifier
  name: (_)) @show

; Enum braces
(enum_specifier
  body: (enumerator_list
    "{" @show.noloc))
(enum_specifier
  body: (enumerator_list
    "}" @show.noloc))

; Enum values
(enumerator) @show.noloc

; Typedef
(type_definition) @show
