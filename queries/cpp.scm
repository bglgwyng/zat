; Function definitions
(function_definition) @show

; Function declarations
(declaration
  declarator: (function_declarator)) @show

; Class
(class_specifier
  name: (_)) @show

; Struct
(struct_specifier
  name: (_)) @show

; Fields and methods inside class/struct
(field_declaration) @show.indent

; Declarations inside class/struct (constructors, etc.)
(field_declaration_list
  (declaration) @show.indent)

; Access specifiers
(access_specifier) @show.indent

; Enum
(enum_specifier
  name: (_)) @show

; Enum values
(enumerator) @show.indent

; Namespace
(namespace_definition
  name: (_)) @show

; Declarations inside namespace
(namespace_definition
  body: (declaration_list
    (_) @show.indent))

; Typedef
(type_definition) @show

; Template declaration
(template_declaration) @show
