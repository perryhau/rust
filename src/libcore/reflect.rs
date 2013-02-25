// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/*!

Runtime type reflection

*/

use sys;
#[cfg(stage0)]
use intrinsic::{TyDesc, get_tydesc};

#[cfg(stage1)]
#[cfg(stage2)]
#[cfg(stage3)]
use sys::{TyDesc, get_tydesc};

use libc::c_void;
use vec;

#[cfg(stage0)]
use intrinsic::{TyVisitor};

#[cfg(stage1)]
#[cfg(stage2)]
#[cfg(stage3)]
#[abi = "rust-intrinsic"]
extern mod rusti {
    fn visit_tydesc(++td: *TyDesc, &&tv: TyVisitor);
}

#[cfg(stage1)]
#[cfg(stage2)]
#[cfg(stage3)]
/// Invoke `tv.visit_*` for the type described by `td`
pub fn visit_tydesc(++td: *TyDesc, &&tv: TyVisitor) {
    unsafe { rusti::visit_tydesc(td, tv) }
}

/**
 * Primary reflection callback interface. The compiler generates
 * code for this; callers can use it to reflect on a type via
 * `sys::get_tydesc` and `visit_tydesc` in this module.
 *
 * This interface is quite low-level and most callers will want
 * to use a higher-level interface such as `MovePtrAdaptor`.
 */
#[cfg(stage1)]
#[cfg(stage2)]
#[cfg(stage3)]
#[lang="tyvisitor"]
pub trait TyVisitor {
    fn visit_bot(&self) -> bool;
    fn visit_nil(&self) -> bool;
    fn visit_bool(&self) -> bool;

    fn visit_int(&self) -> bool;
    fn visit_i8(&self) -> bool;
    fn visit_i16(&self) -> bool;
    fn visit_i32(&self) -> bool;
    fn visit_i64(&self) -> bool;

    fn visit_uint(&self) -> bool;
    fn visit_u8(&self) -> bool;
    fn visit_u16(&self) -> bool;
    fn visit_u32(&self) -> bool;
    fn visit_u64(&self) -> bool;

    fn visit_float(&self) -> bool;
    fn visit_f32(&self) -> bool;
    fn visit_f64(&self) -> bool;

    fn visit_char(&self) -> bool;
    fn visit_str(&self) -> bool;

    fn visit_estr_box(&self) -> bool;
    fn visit_estr_uniq(&self) -> bool;
    fn visit_estr_slice(&self) -> bool;
    fn visit_estr_fixed(&self, n: uint, sz: uint, align: uint) -> bool;

    fn visit_box(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_uniq(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_ptr(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_rptr(&self, mtbl: uint, inner: *TyDesc) -> bool;

    fn visit_vec(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_unboxed_vec(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_box(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_uniq(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_slice(&self, mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_evec_fixed(&self, n: uint, sz: uint, align: uint,
                        mtbl: uint, inner: *TyDesc) -> bool;

    fn visit_enter_rec(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;
    fn visit_rec_field(&self, i: uint, name: &str,
                       mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_leave_rec(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;

    fn visit_enter_class(&self, n_fields: uint,
                         sz: uint, align: uint) -> bool;
    fn visit_class_field(&self, i: uint, name: &str,
                         mtbl: uint, inner: *TyDesc) -> bool;
    fn visit_leave_class(&self, n_fields: uint,
                         sz: uint, align: uint) -> bool;

    fn visit_enter_tup(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;
    fn visit_tup_field(&self, i: uint, inner: *TyDesc) -> bool;
    fn visit_leave_tup(&self, n_fields: uint,
                       sz: uint, align: uint) -> bool;

    fn visit_enter_enum(&self, n_variants: uint,
                        sz: uint, align: uint) -> bool;
    fn visit_enter_enum_variant(&self, variant: uint,
                                disr_val: int,
                                n_fields: uint,
                                name: &str) -> bool;
    fn visit_enum_variant_field(&self, i: uint, inner: *TyDesc) -> bool;
    fn visit_leave_enum_variant(&self, variant: uint,
                                disr_val: int,
                                n_fields: uint,
                                name: &str) -> bool;
    fn visit_leave_enum(&self, n_variants: uint,
                        sz: uint, align: uint) -> bool;

    fn visit_enter_fn(&self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool;
    fn visit_fn_input(&self, i: uint, mode: uint, inner: *TyDesc) -> bool;
    fn visit_fn_output(&self, retstyle: uint, inner: *TyDesc) -> bool;
    fn visit_leave_fn(&self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool;

    fn visit_trait(&self) -> bool;
    fn visit_var(&self) -> bool;
    fn visit_var_integral(&self) -> bool;
    fn visit_param(&self, i: uint) -> bool;
    fn visit_self(&self) -> bool;
    fn visit_opaque_box(&self) -> bool;
    fn visit_closure_ptr(&self, ck: uint) -> bool;
}

/**
 * Trait for visitor that wishes to reflect on data. To use this, create a
 * struct that encapsulates the set of pointers you wish to walk through a
 * data structure, and implement both `MovePtr` for it as well as `TyVisitor`;
 * then build a MovePtrAdaptor wrapped around your struct.
 */
pub trait MovePtr {
    fn move_ptr(adjustment: fn(*c_void) -> *c_void);
    fn push_ptr();
    fn pop_ptr();
}

/// Helper function for alignment calculation.
#[inline(always)]
pub fn align(size: uint, align: uint) -> uint {
    ((size + align) - 1u) & !(align - 1u)
}

/// Adaptor to wrap around visitors implementing MovePtr.
pub struct MovePtrAdaptor<V> {
    inner: V
}
pub fn MovePtrAdaptor<V:TyVisitor + MovePtr>(v: V) -> MovePtrAdaptor<V> {
    MovePtrAdaptor { inner: v }
}

impl<V:TyVisitor + MovePtr> MovePtrAdaptor<V> {
    #[inline(always)]
    fn bump(sz: uint) {
      do self.inner.move_ptr() |p| {
            ((p as uint) + sz) as *c_void
      };
    }

    #[inline(always)]
    fn align(a: uint) {
      do self.inner.move_ptr() |p| {
            align(p as uint, a) as *c_void
      };
    }

    #[inline(always)]
    fn align_to<T>() {
        self.align(sys::min_align_of::<T>());
    }

    #[inline(always)]
    fn bump_past<T>() {
        self.bump(sys::size_of::<T>());
    }
}

/// Abstract type-directed pointer-movement using the MovePtr trait
impl<V:TyVisitor + MovePtr> TyVisitor for MovePtrAdaptor<V> {
    fn visit_bot(&self) -> bool {
        self.align_to::<()>();
        if ! self.inner.visit_bot() { return false; }
        self.bump_past::<()>();
        true
    }

    fn visit_nil(&self) -> bool {
        self.align_to::<()>();
        if ! self.inner.visit_nil() { return false; }
        self.bump_past::<()>();
        true
    }

    fn visit_bool(&self) -> bool {
        self.align_to::<bool>();
        if ! self.inner.visit_bool() { return false; }
        self.bump_past::<bool>();
        true
    }

    fn visit_int(&self) -> bool {
        self.align_to::<int>();
        if ! self.inner.visit_int() { return false; }
        self.bump_past::<int>();
        true
    }

    fn visit_i8(&self) -> bool {
        self.align_to::<i8>();
        if ! self.inner.visit_i8() { return false; }
        self.bump_past::<i8>();
        true
    }

    fn visit_i16(&self) -> bool {
        self.align_to::<i16>();
        if ! self.inner.visit_i16() { return false; }
        self.bump_past::<i16>();
        true
    }

    fn visit_i32(&self) -> bool {
        self.align_to::<i32>();
        if ! self.inner.visit_i32() { return false; }
        self.bump_past::<i32>();
        true
    }

    fn visit_i64(&self) -> bool {
        self.align_to::<i64>();
        if ! self.inner.visit_i64() { return false; }
        self.bump_past::<i64>();
        true
    }

    fn visit_uint(&self) -> bool {
        self.align_to::<uint>();
        if ! self.inner.visit_uint() { return false; }
        self.bump_past::<uint>();
        true
    }

    fn visit_u8(&self) -> bool {
        self.align_to::<u8>();
        if ! self.inner.visit_u8() { return false; }
        self.bump_past::<u8>();
        true
    }

    fn visit_u16(&self) -> bool {
        self.align_to::<u16>();
        if ! self.inner.visit_u16() { return false; }
        self.bump_past::<u16>();
        true
    }

    fn visit_u32(&self) -> bool {
        self.align_to::<u32>();
        if ! self.inner.visit_u32() { return false; }
        self.bump_past::<u32>();
        true
    }

    fn visit_u64(&self) -> bool {
        self.align_to::<u64>();
        if ! self.inner.visit_u64() { return false; }
        self.bump_past::<u64>();
        true
    }

    fn visit_float(&self) -> bool {
        self.align_to::<float>();
        if ! self.inner.visit_float() { return false; }
        self.bump_past::<float>();
        true
    }

    fn visit_f32(&self) -> bool {
        self.align_to::<f32>();
        if ! self.inner.visit_f32() { return false; }
        self.bump_past::<f32>();
        true
    }

    fn visit_f64(&self) -> bool {
        self.align_to::<f64>();
        if ! self.inner.visit_f64() { return false; }
        self.bump_past::<f64>();
        true
    }

    fn visit_char(&self) -> bool {
        self.align_to::<char>();
        if ! self.inner.visit_char() { return false; }
        self.bump_past::<char>();
        true
    }

    fn visit_str(&self) -> bool {
        self.align_to::<~str>();
        if ! self.inner.visit_str() { return false; }
        self.bump_past::<~str>();
        true
    }

    fn visit_estr_box(&self) -> bool {
        self.align_to::<@str>();
        if ! self.inner.visit_estr_box() { return false; }
        self.bump_past::<@str>();
        true
    }

    fn visit_estr_uniq(&self) -> bool {
        self.align_to::<~str>();
        if ! self.inner.visit_estr_uniq() { return false; }
        self.bump_past::<~str>();
        true
    }

    fn visit_estr_slice(&self) -> bool {
        self.align_to::<&static/str>();
        if ! self.inner.visit_estr_slice() { return false; }
        self.bump_past::<&static/str>();
        true
    }

    fn visit_estr_fixed(&self, n: uint,
                        sz: uint,
                        align: uint) -> bool {
        self.align(align);
        if ! self.inner.visit_estr_fixed(n, sz, align) { return false; }
        self.bump(sz);
        true
    }

    fn visit_box(&self, mtbl: uint, inner: *TyDesc) -> bool {
        self.align_to::<@u8>();
        if ! self.inner.visit_box(mtbl, inner) { return false; }
        self.bump_past::<@u8>();
        true
    }

    fn visit_uniq(&self, mtbl: uint, inner: *TyDesc) -> bool {
        self.align_to::<~u8>();
        if ! self.inner.visit_uniq(mtbl, inner) { return false; }
        self.bump_past::<~u8>();
        true
    }

    fn visit_ptr(&self, mtbl: uint, inner: *TyDesc) -> bool {
        self.align_to::<*u8>();
        if ! self.inner.visit_ptr(mtbl, inner) { return false; }
        self.bump_past::<*u8>();
        true
    }

    fn visit_rptr(&self, mtbl: uint, inner: *TyDesc) -> bool {
        self.align_to::<&static/u8>();
        if ! self.inner.visit_rptr(mtbl, inner) { return false; }
        self.bump_past::<&static/u8>();
        true
    }

    fn visit_unboxed_vec(&self, mtbl: uint, inner: *TyDesc) -> bool {
        self.align_to::<vec::UnboxedVecRepr>();
        if ! self.inner.visit_vec(mtbl, inner) { return false; }
        true
    }

    fn visit_vec(&self, mtbl: uint, inner: *TyDesc) -> bool {
        self.align_to::<~[u8]>();
        if ! self.inner.visit_vec(mtbl, inner) { return false; }
        self.bump_past::<~[u8]>();
        true
    }

    fn visit_evec_box(&self, mtbl: uint, inner: *TyDesc) -> bool {
        self.align_to::<@[u8]>();
        if ! self.inner.visit_evec_box(mtbl, inner) { return false; }
        self.bump_past::<@[u8]>();
        true
    }

    fn visit_evec_uniq(&self, mtbl: uint, inner: *TyDesc) -> bool {
        self.align_to::<~[u8]>();
        if ! self.inner.visit_evec_uniq(mtbl, inner) { return false; }
        self.bump_past::<~[u8]>();
        true
    }

    fn visit_evec_slice(&self, mtbl: uint, inner: *TyDesc) -> bool {
        self.align_to::<&static/[u8]>();
        if ! self.inner.visit_evec_slice(mtbl, inner) { return false; }
        self.bump_past::<&static/[u8]>();
        true
    }

    fn visit_evec_fixed(&self, n: uint, sz: uint, align: uint,
                        mtbl: uint, inner: *TyDesc) -> bool {
        self.align(align);
        if ! self.inner.visit_evec_fixed(n, sz, align, mtbl, inner) {
            return false;
        }
        self.bump(sz);
        true
    }

    fn visit_enter_rec(&self, n_fields: uint, sz: uint, align: uint) -> bool {
        self.align(align);
        if ! self.inner.visit_enter_rec(n_fields, sz, align) { return false; }
        true
    }

    fn visit_rec_field(&self, i: uint, name: &str,
                       mtbl: uint, inner: *TyDesc) -> bool {
        unsafe { self.align((*inner).align); }
        if ! self.inner.visit_rec_field(i, name, mtbl, inner) {
            return false;
        }
        unsafe { self.bump((*inner).size); }
        true
    }

    fn visit_leave_rec(&self, n_fields: uint, sz: uint, align: uint) -> bool {
        if ! self.inner.visit_leave_rec(n_fields, sz, align) { return false; }
        true
    }

    fn visit_enter_class(&self, n_fields: uint, sz: uint, align: uint)
                      -> bool {
        self.align(align);
        if ! self.inner.visit_enter_class(n_fields, sz, align) {
            return false;
        }
        true
    }

    fn visit_class_field(&self, i: uint, name: &str,
                         mtbl: uint, inner: *TyDesc) -> bool {
        unsafe { self.align((*inner).align); }
        if ! self.inner.visit_class_field(i, name, mtbl, inner) {
            return false;
        }
        unsafe { self.bump((*inner).size); }
        true
    }

    fn visit_leave_class(&self, n_fields: uint, sz: uint, align: uint)
                      -> bool {
        if ! self.inner.visit_leave_class(n_fields, sz, align) {
            return false;
        }
        true
    }

    fn visit_enter_tup(&self, n_fields: uint, sz: uint, align: uint) -> bool {
        self.align(align);
        if ! self.inner.visit_enter_tup(n_fields, sz, align) { return false; }
        true
    }

    fn visit_tup_field(&self, i: uint, inner: *TyDesc) -> bool {
        unsafe { self.align((*inner).align); }
        if ! self.inner.visit_tup_field(i, inner) { return false; }
        unsafe { self.bump((*inner).size); }
        true
    }

    fn visit_leave_tup(&self, n_fields: uint, sz: uint, align: uint) -> bool {
        if ! self.inner.visit_leave_tup(n_fields, sz, align) { return false; }
        true
    }

    fn visit_enter_fn(&self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool {
        if ! self.inner.visit_enter_fn(purity, proto, n_inputs, retstyle) {
            return false
        }
        true
    }

    fn visit_fn_input(&self, i: uint, mode: uint, inner: *TyDesc) -> bool {
        if ! self.inner.visit_fn_input(i, mode, inner) { return false; }
        true
    }

    fn visit_fn_output(&self, retstyle: uint, inner: *TyDesc) -> bool {
        if ! self.inner.visit_fn_output(retstyle, inner) { return false; }
        true
    }

    fn visit_leave_fn(&self, purity: uint, proto: uint,
                      n_inputs: uint, retstyle: uint) -> bool {
        if ! self.inner.visit_leave_fn(purity, proto, n_inputs, retstyle) {
            return false;
        }
        true
    }

    fn visit_enter_enum(&self, n_variants: uint, sz: uint, align: uint)
                     -> bool {
        self.align(align);
        if ! self.inner.visit_enter_enum(n_variants, sz, align) {
            return false;
        }
        true
    }

    fn visit_enter_enum_variant(&self, variant: uint,
                                disr_val: int,
                                n_fields: uint,
                                name: &str) -> bool {
        self.inner.push_ptr();
        if ! self.inner.visit_enter_enum_variant(variant, disr_val,
                                                 n_fields, name) {
            return false;
        }
        true
    }

    fn visit_enum_variant_field(&self, i: uint, inner: *TyDesc) -> bool {
        unsafe { self.align((*inner).align); }
        if ! self.inner.visit_enum_variant_field(i, inner) { return false; }
        unsafe { self.bump((*inner).size); }
        true
    }

    fn visit_leave_enum_variant(&self, variant: uint,
                                disr_val: int,
                                n_fields: uint,
                                name: &str) -> bool {
        if ! self.inner.visit_leave_enum_variant(variant, disr_val,
                                                 n_fields, name) {
            return false;
        }
        self.inner.pop_ptr();
        true
    }

    fn visit_leave_enum(&self, n_variants: uint, sz: uint, align: uint)
                     -> bool {
        if ! self.inner.visit_leave_enum(n_variants, sz, align) {
            return false;
        }
        self.bump(sz);
        true
    }

    fn visit_trait(&self) -> bool {
        self.align_to::<TyVisitor>();
        if ! self.inner.visit_trait() { return false; }
        self.bump_past::<TyVisitor>();
        true
    }

    fn visit_var(&self) -> bool {
        if ! self.inner.visit_var() { return false; }
        true
    }

    fn visit_var_integral(&self) -> bool {
        if ! self.inner.visit_var_integral() { return false; }
        true
    }

    fn visit_param(&self, i: uint) -> bool {
        if ! self.inner.visit_param(i) { return false; }
        true
    }

    fn visit_self(&self) -> bool {
        self.align_to::<&static/u8>();
        if ! self.inner.visit_self() { return false; }
        self.align_to::<&static/u8>();
        true
    }

    #[cfg(stage0)]
    fn visit_type(&self) -> bool {
        if ! self.inner.visit_type() { return false; }
        true
    }

    fn visit_opaque_box(&self) -> bool {
        self.align_to::<@u8>();
        if ! self.inner.visit_opaque_box() { return false; }
        self.bump_past::<@u8>();
        true
    }

    #[cfg(stage0)]
    fn visit_constr(&self, inner: *TyDesc) -> bool {
        if ! self.inner.visit_constr(inner) { return false; }
        true
    }

    fn visit_closure_ptr(&self, ck: uint) -> bool {
        self.align_to::<fn@()>();
        if ! self.inner.visit_closure_ptr(ck) { return false; }
        self.bump_past::<fn@()>();
        true
    }
}
