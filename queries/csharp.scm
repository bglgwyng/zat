; Using directives
(using_directive) @show

; Namespace
(namespace_declaration
  body: (declaration_list) @hide) @show

; Namespace braces
(namespace_declaration
  body: (declaration_list
    "{" @show.noloc))
(namespace_declaration
  body: (declaration_list
    "}" @show.noloc))

; Namespace members
(namespace_declaration
  body: (declaration_list
    (_) @show))

; Class
(class_declaration
  body: (declaration_list) @hide) @show

; Interface
(interface_declaration
  body: (declaration_list) @hide) @show

; Struct
(struct_declaration
  body: (declaration_list) @hide) @show

; Enum
(enum_declaration
  body: (enum_member_declaration_list) @hide) @show

; Record (with optional body)
(record_declaration
  body: (declaration_list) @hide) @show
(record_declaration) @show

; Class braces
(class_declaration
  body: (declaration_list
    "{" @show.noloc))
(class_declaration
  body: (declaration_list
    "}" @show.noloc))

; Interface braces
(interface_declaration
  body: (declaration_list
    "{" @show.noloc))
(interface_declaration
  body: (declaration_list
    "}" @show.noloc))

; Struct braces
(struct_declaration
  body: (declaration_list
    "{" @show.noloc))
(struct_declaration
  body: (declaration_list
    "}" @show.noloc))

; Enum braces
(enum_declaration
  body: (enum_member_declaration_list
    "{" @show.noloc))
(enum_declaration
  body: (enum_member_declaration_list
    "}" @show.noloc))

; Strip visibility/other modifiers from type declarations
(class_declaration
  ((modifier) @hide
    (#match? @hide "^(public|internal|private|protected|static|abstract|sealed|partial)$")))
(interface_declaration
  ((modifier) @hide
    (#match? @hide "^(public|internal|private|protected|partial)$")))
(struct_declaration
  ((modifier) @hide
    (#match? @hide "^(public|internal|private|protected|static|partial)$")))
(enum_declaration
  ((modifier) @hide
    (#match? @hide "^(public|internal|private|protected)$")))
(record_declaration
  ((modifier) @hide
    (#match? @hide "^(public|internal|private|protected|partial)$")))

; Methods (exclude private, strip "public")
((method_declaration
  body: (block) @hide) @show
  (#not-match? @show "^private"))
((method_declaration) @show
  (#not-match? @show "^private"))
(method_declaration
  ((modifier) @hide
    (#eq? @hide "public")))

; Constructors (exclude private, strip "public")
((constructor_declaration
  body: (block) @hide) @show
  (#not-match? @show "^private"))
(constructor_declaration
  ((modifier) @hide
    (#eq? @hide "public")))

; Properties (exclude private, strip "public", hide accessor bodies)
((property_declaration
  accessors: (accessor_list) @hide) @show
  (#not-match? @show "^private"))
((property_declaration) @show
  (#not-match? @show "^private"))
(property_declaration
  ((modifier) @hide
    (#eq? @hide "public")))

; Fields (exclude private, strip "public")
((field_declaration) @show.noloc
  (#not-match? @show.noloc "^private"))
(field_declaration
  ((modifier) @hide
    (#eq? @hide "public")))

; Enum members
(enum_member_declaration) @show.noloc
