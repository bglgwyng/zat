; Public function
(function_item
  (visibility_modifier) @hide
  body: (block) @hide) @show

; Public struct (with fields)
(struct_item
  (visibility_modifier) @hide
  body: (field_declaration_list) @hide) @show

; Public struct (tuple/unit)
(struct_item
  (visibility_modifier) @hide) @show

; Public enum
(enum_item
  (visibility_modifier) @hide
  body: (enum_variant_list) @hide) @show

; Struct fields (pub)
(field_declaration_list
  (field_declaration
    (visibility_modifier) @hide) @show.noloc)

; Enum variants
(enum_variant) @show.noloc

; Public trait
(trait_item
  (visibility_modifier) @hide
  body: (declaration_list) @hide) @show

; Trait method signatures
(declaration_list
  (function_signature_item) @show)

; Public impl block
(impl_item
  body: (declaration_list) @hide) @show

; Impl methods (pub)
(impl_item
  body: (declaration_list
    (function_item
      (visibility_modifier) @hide
      body: (block) @hide) @show))

; Public type alias
(type_item
  (visibility_modifier) @hide) @show

; Public const
(const_item
  (visibility_modifier) @hide) @show

; Public static
(static_item
  (visibility_modifier) @hide) @show

; Public mod (with body)
(mod_item
  (visibility_modifier) @hide
  body: (declaration_list) @hide) @show

; Public mod (declaration)
(mod_item
  (visibility_modifier) @hide) @show
