; ─── Top-level declarations ───────────────────────────────────────────────────

; Classes/interfaces with a regular body
(source_file
  (class_declaration
    (class_body) @hide) @show)

; Classes/enums with an enum class body
(source_file
  (class_declaration
    (enum_class_body) @hide) @show)

; Classes with no body (data class, abstract class header, etc.)
(source_file
  (class_declaration) @show)

; Top-level object declarations
(source_file
  (object_declaration
    (class_body) @hide) @show)

; Top-level type aliases
(source_file
  (type_alias) @show)

; Top-level functions (with body)
(source_file
  (function_declaration
    (function_body) @hide) @show)

; Top-level functions (no body)
(source_file
  (function_declaration) @show)

; Top-level properties
(source_file
  (property_declaration) @show.noloc)

; ─── Class body members ───────────────────────────────────────────────────────

; Member functions (with body, exclude private)
((class_declaration
  (class_body
    (function_declaration
      (function_body) @hide) @show.indented))
 (#not-match? @show.indented "^private"))

; Member functions (no body — interface/abstract, exclude private)
((class_declaration
  (class_body
    (function_declaration) @show.indented))
 (#not-match? @show.indented "^private"))

; Member properties (exclude private)
((class_declaration
  (class_body
    (property_declaration) @show.indented.noloc))
 (#not-match? @show.indented.noloc "^private"))

; Nested class declarations (exclude private)
((class_declaration
  (class_body
    (class_declaration) @show.indented))
 (#not-match? @show.indented "^private"))

; Secondary constructors (exclude private)
((class_declaration
  (class_body
    (secondary_constructor) @show.indented))
 (#not-match? @show.indented "^private"))

; Companion object — show collapsed (body hidden)
(class_declaration
  (class_body
    (companion_object
      (class_body) @hide) @show.indented))

; ─── Object declaration body members ─────────────────────────────────────────

; Object member functions (with body, exclude private)
((object_declaration
  (class_body
    (function_declaration
      (function_body) @hide) @show.indented))
 (#not-match? @show.indented "^private"))

; Object member functions (no body, exclude private)
((object_declaration
  (class_body
    (function_declaration) @show.indented))
 (#not-match? @show.indented "^private"))

; Object member properties (exclude private)
((object_declaration
  (class_body
    (property_declaration) @show.indented.noloc))
 (#not-match? @show.indented.noloc "^private"))

; ─── Enum body members ────────────────────────────────────────────────────────

; Enum entries
(enum_class_body
  (enum_entry) @show.indented.noloc)

; Enum member functions (with body, exclude private)
((class_declaration
  (enum_class_body
    (function_declaration
      (function_body) @hide) @show.indented))
 (#not-match? @show.indented "^private"))

; Enum member functions (no body, exclude private)
((class_declaration
  (enum_class_body
    (function_declaration) @show.indented))
 (#not-match? @show.indented "^private"))
