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

; Fields inside class/struct
(field_declaration) @show.indent.noloc

; Declarations inside class/struct (constructors, etc.)
(field_declaration_list
  (declaration) @show.indent)

; Public/protected access specifier: show label and enable members after
(field_declaration_list
  ((access_specifier) @show.indent.noindent.noloc.show_after
    (#match? @show.indent.noindent.noloc.show_after "^(public|protected)$"))
  . ":" @append)

; Private: hide members after
(field_declaration_list
  ((access_specifier) @hide_after
    (#match? @hide_after "^private$")))

; Enum
(enum_specifier
  name: (_)) @show

; Enum values
(enumerator) @show.indent.noloc

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
