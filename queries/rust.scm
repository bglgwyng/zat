; Public function
(function_item) @signature

; Struct with fields
(struct_item
  body: (field_declaration_list) @body) @signature

; Struct (unit/tuple)
(struct_item) @signature

; Enum with variants
(enum_item
  body: (enum_variant_list) @body) @signature

; Trait with body
(trait_item
  body: (declaration_list) @body) @signature

; Impl block with body
(impl_item
  body: (declaration_list) @body) @signature

; Type alias
(type_item) @signature

; Const
(const_item) @signature

; Static
(static_item) @signature

; Mod
(mod_item) @signature

; Use (pub)
(use_declaration) @signature
