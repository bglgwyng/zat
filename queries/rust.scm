; Public function
(function_item
  (visibility_modifier) @strip
  body: (block) @hide) @show

; Public struct (with fields)
(struct_item
  (visibility_modifier) @strip
  body: (field_declaration_list) @hide) @show

; Public struct (tuple/unit)
(struct_item
  (visibility_modifier) @strip) @show

; Public enum
(enum_item
  (visibility_modifier) @strip
  body: (enum_variant_list) @hide) @show

; Enum variants
(enum_variant) @show.indent.noloc

; Public trait
(trait_item
  (visibility_modifier) @strip
  body: (declaration_list) @hide) @show

; Trait method signatures
(declaration_list
  (function_signature_item) @show.indent)

; Public impl block
(impl_item
  body: (declaration_list) @hide) @show

; Impl methods (pub)
(impl_item
  body: (declaration_list
    (function_item
      (visibility_modifier) @strip
      body: (block) @hide) @show.indent))

; Public type alias
(type_item
  (visibility_modifier) @strip) @show

; Public const
(const_item
  (visibility_modifier) @strip) @show

; Public static
(static_item
  (visibility_modifier) @strip) @show

; Public mod (with body)
(mod_item
  (visibility_modifier) @strip
  body: (declaration_list) @hide) @show

; Public mod (declaration)
(mod_item
  (visibility_modifier) @strip) @show
