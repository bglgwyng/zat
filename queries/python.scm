; Top-level functions
(module
  (function_definition) @signature)

; Top-level decorated definitions (functions/classes with decorators)
(module
  (decorated_definition) @signature)

; Class with body (matches both decorated and plain)
(class_definition
  body: (block) @body) @signature
