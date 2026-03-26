; Includes
(preproc_include) @show

; Function definitions
(function_definition
  body: (compound_statement) @hide) @show

; Function declarations
(declaration
  declarator: (function_declarator)) @show

; Class
(class_specifier
  name: (_)
  body: (field_declaration_list) @hide) @show

; Class forward declaration
(class_specifier
  name: (_)) @show

; Struct
(struct_specifier
  name: (_)
  body: (field_declaration_list) @hide) @show

; Struct forward declaration
(struct_specifier
  name: (_)) @show

; Class braces
(class_specifier
  body: (field_declaration_list
    "{" @show.noloc))
(class_specifier
  body: (field_declaration_list
    "}" @show.noloc))

; Struct braces
(struct_specifier
  body: (field_declaration_list
    "{" @show.noloc))
(struct_specifier
  body: (field_declaration_list
    "}" @show.noloc))

; Fields inside class/struct
(field_declaration) @show.noloc

; Declarations inside class/struct (constructors, etc.)
(field_declaration_list
  (declaration) @show)

; Public/protected access specifier: show label and enable members after
(field_declaration_list
  ((access_specifier) @show.noloc.show_after
    (#match? @show.noloc.show_after "^(public|protected)$"))
  . ":" @show.noloc)

; Private: hide members after
(field_declaration_list
  ((access_specifier) @hide_after
    (#match? @hide_after "^private$")))

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

; Namespace
(namespace_definition
  name: (_)
  body: (declaration_list) @hide) @show

; Namespace braces
(namespace_definition
  body: (declaration_list
    "{" @show.noloc))
(namespace_definition
  body: (declaration_list
    "}" @show.noloc))

; Declarations inside namespace
(namespace_definition
  body: (declaration_list
    (_) @show))

; Typedef
(type_definition) @show

; Template declaration
(template_declaration) @show
