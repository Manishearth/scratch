#![feature(plugin_registrar, box_syntax)]
#![feature(rustc_private, collections)]
#![allow(unused_imports, unknown_lints)]

#[macro_use]
extern crate syntax;
#[macro_use]
extern crate rustc;

// Only for the compile time checking of paths
extern crate collections;

use rustc::plugin::Registry;
use rustc::lint::LintPassObject;
use syntax::ptr::P;
use syntax::ast;
use syntax::ast::*;
use rustc::middle::ty;
use rustc::lint::{Context, LintPass, LintArray, Lint, Level};
use syntax::codemap::{ExpnInfo, Span};

#[allow(missing_copy_implementations)]
pub struct LetPass;

declare_lint!(pub LET_UNIT_VALUE, Warn,
              "creating a let binding to a value of unit type, which usually can't be used afterwards");


fn check_let_unit(cx: &Context, decl: &Decl) {
    if let DeclLocal(ref local) = decl.node {
        let bindtype = &cx.tcx.pat_ty(&*local.pat).sty;
        if *bindtype == ty::TyTuple(vec![]) {
            cx.span_lint(LET_UNIT_VALUE, decl.span,
                "this let-binding has unit value.");
        }
    }
}

impl LintPass for LetPass {
    fn get_lints(&self) -> LintArray {
        lint_array!(LET_UNIT_VALUE)
    }

    fn check_decl(&mut self, cx: &Context, decl: &Decl) {
        check_let_unit(cx, decl)
    }
}



#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_lint_pass(box LetPass as LintPassObject);
}
