; Import
(import_declaration) @show

; Class, struct, enum, actor, extension (all use class_declaration)
(class_declaration
  body: (class_body) @hide) @show

; Enum uses enum_class_body
(class_declaration
  body: (enum_class_body) @hide) @show

; Protocol
(protocol_declaration
  body: (protocol_body) @hide) @show

; Top-level functions
(function_declaration
  body: (function_body) @hide) @show

; Top-level properties
(property_declaration) @show

; Top-level typealias
(typealias_declaration) @show

; Class/struct members: functions (exclude private)
((class_body
  (function_declaration
    body: (function_body) @hide) @show.indented)
  (#not-match? @show.indented "private"))

; Class/struct members: init (exclude private)
((class_body
  (init_declaration
    body: (function_body) @hide) @show.indented)
  (#not-match? @show.indented "private"))

; Class/struct members: properties (exclude private)
((class_body
  (property_declaration) @show.indented.noloc)
  (#not-match? @show.indented.noloc "private"))

; Class/struct members: typealias
(class_body
  (typealias_declaration) @show.indented)

; Enum members: functions (exclude private)
((enum_class_body
  (function_declaration
    body: (function_body) @hide) @show.indented)
  (#not-match? @show.indented "private"))

; Enum members: init (exclude private)
((enum_class_body
  (init_declaration
    body: (function_body) @hide) @show.indented)
  (#not-match? @show.indented "private"))

; Enum entries
(enum_class_body
  (enum_entry) @show.indented.noloc)

; Protocol members: function declarations
(protocol_body
  (protocol_function_declaration) @show.indented)

; Protocol members: property declarations
(protocol_body
  (protocol_property_declaration) @show.indented.noloc)
