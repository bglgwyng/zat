; Class
(class_declaration
  body: (class_body) @hide) @show

; Interface
(interface_declaration
  body: (interface_body) @hide) @show

; Enum
(enum_declaration
  body: (enum_body) @hide) @show

; Methods
(method_declaration
  body: (block) @hide) @show.indented

; Methods (abstract, no body)
(method_declaration) @show.indented

; Constructors
(constructor_declaration
  body: (constructor_body) @hide) @show.indented

; Fields
(field_declaration) @show.indented.noloc

; Enum constants
(enum_constant) @show.indented.noloc
