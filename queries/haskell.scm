; Module header
(haskell
  (header) @show.noloc)

; Type signatures (top-level)
(declarations
  (signature) @show)

; data type declarations
(data_type) @show

; newtype declarations
(newtype) @show

; type synonym
(type_synomym) @show

; class declaration (hide method bodies, show header)
(class
  declarations: (class_declarations) @hide) @show

; Class method signatures (indented)
(class_declarations
  (signature) @show.indented)

; instance declaration (hide method bodies, show header)
(instance
  declarations: (instance_declarations) @hide) @show

; fixity declarations
(fixity) @show
