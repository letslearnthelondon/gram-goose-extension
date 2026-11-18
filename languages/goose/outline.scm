; Symbols in the outline panel / breadcrumbs. Items nest by syntax containment,
; so a function inside a namespace, or a variant inside an enum, becomes a
; child of it.

(function_declaration name: (_) @name) @item
(struct_declaration name: (_) @name) @item
(enum_declaration name: (_) @name) @item
(type_alias_declaration name: (_) @name) @item
(namespace_declaration name: (_) @name) @item
(enum_variant name: (_) @name) @item
(field_declaration name: (_) @name) @item
(extern_member (_) @name) @item

; Top-level globals only; locals inside function bodies would be noise.
(source_file (variable_declaration name: (_) @name) @item)
