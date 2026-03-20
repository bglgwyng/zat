; Struct with fields
(type_declaration
  (type_spec
    name: (type_identifier)
    type: (struct_type (field_declaration_list) @body)) @signature)

; Interface with methods
(type_declaration
  (type_spec
    name: (type_identifier)
    type: (interface_type) @body) @signature)

; Simple type declarations (type alias, empty struct, etc.)
(type_declaration
  (type_spec) @signature)

; Functions
(function_declaration) @signature

; Methods
(method_declaration) @signature
