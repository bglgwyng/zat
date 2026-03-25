; Top-level functions
(module
  (function_definition
    body: (block) @hide) @show)

; Top-level decorated definitions (show the function, not the decorator)
(module
  (decorated_definition
    definition: (function_definition
      body: (block) @hide) @show))

; Top-level assignments
(module
  (expression_statement
    (assignment)) @show)

; Class
(class_definition
  body: (block) @hide) @show

; Class methods
(class_definition
  body: (block
    (function_definition
      body: (block) @hide) @show))

; Class decorated methods (show the function, not the decorator)
(class_definition
  body: (block
    (decorated_definition
      definition: (function_definition
        body: (block) @hide) @show)))

; Class attributes
(class_definition
  body: (block
    (expression_statement) @show.noloc))
