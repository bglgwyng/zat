; Top-level functions
(module
  (function_definition) @show)

; Top-level decorated definitions (show the function, not the decorator)
(module
  (decorated_definition
    definition: (function_definition) @show))

; Top-level assignments
(module
  (expression_statement
    (assignment)) @show)

; Class
(class_definition) @show

; Class methods
(class_definition
  body: (block
    (function_definition) @show.indented))

; Class decorated methods (show the function, not the decorator)
(class_definition
  body: (block
    (decorated_definition
      definition: (function_definition) @show.indented)))

; Class attributes
(class_definition
  body: (block
    (expression_statement) @show.indented.noloc))
