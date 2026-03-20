; Function
(function_item) @show

; Struct
(struct_item) @show

; Struct fields
(field_declaration) @show.indent.noloc

; Enum
(enum_item) @show

; Enum variants
(enum_variant) @show.indent.noloc

; Trait
(trait_item) @show

; Trait method signatures
(declaration_list
  (function_signature_item) @show.indent)

; Impl block
(impl_item) @show

; Impl methods
(impl_item
  body: (declaration_list
    (function_item) @show.indent))

; Type alias
(type_item) @show

; Const
(const_item) @show

; Static
(static_item) @show

; Mod
(mod_item) @show

; Use
(use_declaration) @show
