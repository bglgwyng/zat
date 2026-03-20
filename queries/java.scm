; Class
(class_declaration
  body: (class_body) @hide) @show

; Interface
(interface_declaration
  body: (interface_body) @hide) @show

; Enum
(enum_declaration
  body: (enum_body) @hide) @show

; Methods (exclude private, strip "public")
((method_declaration
  body: (block) @hide) @show.indented
  (#not-match? @show.indented "^private"))

; Methods (abstract, no body — exclude private)
((method_declaration) @show.indented
  (#not-match? @show.indented "^private"))
(method_declaration
  ((modifiers) @strip
    (#eq? @strip "public")))

; Constructors (exclude private, strip "public")
((constructor_declaration
  body: (constructor_body) @hide) @show.indented
  (#not-match? @show.indented "^private"))
(constructor_declaration
  ((modifiers) @strip
    (#eq? @strip "public")))

; Fields (exclude private, strip "public")
((field_declaration) @show.indented.noloc
  (#not-match? @show.indented.noloc "^private"))
(field_declaration
  ((modifiers) @strip
    (#eq? @strip "public")))

; Enum constants
(enum_constant) @show.indented.noloc
