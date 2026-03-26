; Require statements
(program
  ((call
    method: (identifier) @_method) @show
    (#match? @_method "^require")))

; Class
(class
  (body_statement) @hide) @show

; Module
(module
  (body_statement) @hide) @show

; Instance methods
(body_statement
  (method
    (body_statement) @hide
    "end" @hide) @show)

; Singleton methods (class methods like def self.foo)
(body_statement
  (singleton_method
    (body_statement) @hide
    "end" @hide) @show)

; Private keyword hides subsequent methods
(body_statement
  ((identifier) @hide_after
    (#eq? @hide_after "private")))

; Top-level methods
(program
  (method
    (body_statement) @hide
    "end" @hide) @show)

; Top-level assignments (constants)
(program
  (assignment) @show)
