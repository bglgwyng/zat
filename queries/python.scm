; Top-level functions
(module
  (function_definition) @show)

; Top-level decorated definitions
(module
  (decorated_definition) @show)

; Class
(class_definition) @show

; Class methods and attributes
(class_definition
  body: (block
    (function_definition) @show.indent))

(class_definition
  body: (block
    (decorated_definition) @show.indent))

(class_definition
  body: (block
    (expression_statement) @show.indent))
