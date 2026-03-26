; Imports
(import_declaration) @show

; Package
(package_declaration) @show

; Class
(class_declaration
  body: (class_body) @hide) @show

; Class braces
(class_declaration
  body: (class_body
    "{" @show.noloc))
(class_declaration
  body: (class_body
    "}" @show.noloc))

; Interface
(interface_declaration
  body: (interface_body) @hide) @show

; Interface braces
(interface_declaration
  body: (interface_body
    "{" @show.noloc))
(interface_declaration
  body: (interface_body
    "}" @show.noloc))

; Enum
(enum_declaration
  body: (enum_body) @hide) @show

; Enum braces
(enum_declaration
  body: (enum_body
    "{" @show.noloc))
(enum_declaration
  body: (enum_body
    "}" @show.noloc))

; Methods (exclude private, strip "public")
((method_declaration
  body: (block) @hide) @show
  (#not-match? @show "^private"))

; Methods (abstract, no body — exclude private)
((method_declaration) @show
  (#not-match? @show "^private"))
(method_declaration
  ((modifiers) @hide
    (#eq? @hide "public")))

; Constructors (exclude private, strip "public")
((constructor_declaration
  body: (constructor_body) @hide) @show
  (#not-match? @show "^private"))
(constructor_declaration
  ((modifiers) @hide
    (#eq? @hide "public")))

; Fields (exclude private, strip "public")
((field_declaration) @show.noloc
  (#not-match? @show.noloc "^private"))
(field_declaration
  ((modifiers) @hide
    (#eq? @hide "public")))

; Enum constants
(enum_constant) @show.noloc
