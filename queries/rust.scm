; Public function
(function_item
  (visibility_modifier) @strip) @show

; Public struct
(struct_item
  (visibility_modifier) @strip) @show

; Public enum
(enum_item
  (visibility_modifier) @strip) @show

; Enum variants
(enum_variant) @show.indent.noloc

; Public trait
(trait_item
  (visibility_modifier) @strip) @show

; Trait method signatures
(declaration_list
  (function_signature_item) @show.indent)

; Public impl block
(impl_item) @show

; Impl methods (pub)
(impl_item
  body: (declaration_list
    (function_item
      (visibility_modifier) @strip) @show.indent))

; Public type alias
(type_item
  (visibility_modifier) @strip) @show

; Public const
(const_item
  (visibility_modifier) @strip) @show

; Public static
(static_item
  (visibility_modifier) @strip) @show

; Public mod
(mod_item
  (visibility_modifier) @strip) @show
