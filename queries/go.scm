; Functions
(function_declaration
  body: (block) @hide) @show

; Methods
(method_declaration
  body: (block) @hide) @show

; Struct type declaration
(type_declaration
  (type_spec
    type: (struct_type
      (field_declaration_list) @hide))) @show

; Interface type declaration
(type_declaration
  (type_spec
    type: (interface_type) @hide)) @show

; Other type declarations
(type_declaration) @show

; Struct fields
(field_declaration) @show.indent.noloc

; Interface methods
(method_elem) @show.indent.noloc
