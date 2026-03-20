; Function definitions
(function_definition) @signature

; Function declarations (prototypes)
(declaration
  declarator: (function_declarator)) @signature

; Typedef struct with fields
(type_definition
  type: (struct_specifier
    body: (field_declaration_list) @body)
  declarator: (type_identifier) @name) @signature

; Named struct with fields
(struct_specifier
  name: (_)
  body: (field_declaration_list) @body) @signature

; Enum with values
(enum_specifier
  name: (_)
  body: (enumerator_list) @body) @signature

; Typedef (simple, function pointers, etc.)
(type_definition) @signature
