//! 7. Macros — declarative macro_rules!, procedural macros.

#[macro_export]
macro_rules! update_cursor_fields {
    ( $struct_to_change:ident, $new_value:expr, $field_name:ident) => {
        $struct_to_change.$field_name = $new_value;
    };
}

// Custom Derive macros
// Procedural macros
// 1. Derive
// 2. Attribute like
// 3. Function like
