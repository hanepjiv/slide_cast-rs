// -*- mode:rust; coding:utf-8-unix; -*-

//! lib.rs

//  Copyright 2016 hanepjiv
//  @author hanepjiv <hanepjiv@gmail.com>
//  @copyright The MIT License (MIT) / Apache License Version 2.0
//  @since 2016/03/01
//  @date 2017/07/21

// ////////////////////////////////////////////////////////////////////////////
// attribute  =================================================================
#![deny(
    anonymous_parameters,
    box_pointers,
    fat_ptr_transmutes,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences,
    const_err,
    dead_code,
    deprecated,
    deprecated_attr,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    late_bound_lifetime_arguments,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_in_public,
    private_no_mangle_fns,
    private_no_mangle_statics,
    renamed_and_removed_lints,
    stable_features,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_lints,
    unreachable_code,
    unreachable_patterns,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comment,
    unused_features,
    unused_imports,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_unsafe,
    unused_variables,
    while_true,
    exceeding_bitshifts,
    extra_requirement_in_impl,
    invalid_type_param_default,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    legacy_imports,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    parenthesized_params_in_types_and_modules,
    pub_use_of_private_extern_crate,
    resolve_trait_on_defaulted_unit,
    safe_extern_statics,
    unknown_crate_types,
)]
#![warn(
    dead_code,
)]
#![allow(
    box_pointers,
    unsafe_code,
    trivial_casts,
    trivial_numeric_casts,
)]
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[macro_export]
macro_rules! offset_of {
    ($container:ty : $field:ident) => (unsafe {
        &(*(0usize as *const $container)).$field as *const _ as isize
    });
}
// ============================================================================
#[macro_export]
macro_rules! slide_cast {
    ($arg:ident, $container:ty : $field:ident) => (unsafe {
        transmute::<_, &mut $container>(transmute::<_, isize>($arg) -
                                        offset_of!($container, $field))
    });
}
// ////////////////////////////////////////////////////////////////////////////
// ============================================================================
#[cfg(test)]
mod tests {
    // ========================================================================
    #[derive(Debug, Clone, )]
    struct Container {
        _first:                 i64,
        _second:                i32,
        _third:                 i32,
    }
    impl Drop for Container {
        fn drop(&mut self) { println!("Container::drop") }
    }
    // ========================================================================
    #[test]
    fn it_works_offset_of() {
        assert_eq!( 0, offset_of!(Container : _first));
        assert_eq!( 8, offset_of!(Container : _second));
        assert_eq!(12, offset_of!(Container : _third));
    }
}
