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
  body: (block) @hide) @show.indent

; Methods (abstract, no body)
(method_declaration) @show.indent

; Constructors
(constructor_declaration
  body: (constructor_body) @hide) @show.indent

; Fields
(field_declaration) @show.indent.noloc

; Enum constants
(enum_constant) @show.indent.noloc
