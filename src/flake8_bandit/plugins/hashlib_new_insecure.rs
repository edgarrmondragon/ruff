use rustc_hash::{FxHashMap, FxHashSet};
use rustpython_ast::{Constant, Expr, ExprKind, Keyword};

use crate::ast::helpers::{match_module_member, SimpleCallArgs};
use crate::ast::types::Range;
use crate::registry::{Check, CheckKind};

const INSECURE_ALGOS: [&str; 4] = ["md4", "md5", "sha", "sha1"];

fn get_string_value(expr: &Expr) -> Option<&str> {
    match &expr.node {
        ExprKind::Constant {
            value: Constant::Str(value),
            ..
        } => Some(value),
        _ => None,
    }
}

fn get_bool_value(expr: &Expr) -> Option<bool> {
    match &expr.node {
        ExprKind::Constant {
            value: Constant::Bool(value),
            ..
        } => Some(*value),
        _ => None,
    }
}

/// S324
pub fn hashlib_new_insecure(
    func: &Expr,
    args: &Vec<Expr>,
    keywords: &Vec<Keyword>,
    from_imports: &FxHashMap<&str, FxHashSet<&str>>,
    import_aliases: &FxHashMap<&str, &str>,
    at_least_py39: bool,
) -> Option<Check> {
    let mut used_insecure_hash = false;
    let mut used_hash: Option<String> = None;

    if match_module_member(func, "hashlib", "new", from_imports, import_aliases) {
        let call_args = SimpleCallArgs::new(args, keywords);
        if let Some(name_arg) = call_args.get_argument("name", Some(0)) {
            if let Some(name_value) = get_string_value(name_arg) {
                if INSECURE_ALGOS.contains(&name_value.to_lowercase().as_str()) {
                    used_hash = Some(name_value.to_uppercase());
                    used_insecure_hash = call_args
                        .get_argument("usedforsecurity", None)
                        .and_then(get_bool_value)
                        .unwrap_or(true);
                }
            }
        }
    } else {
        for hash_func in INSECURE_ALGOS {
            if match_module_member(func, "hashlib", hash_func, from_imports, import_aliases) {
                let call_args = SimpleCallArgs::new(args, keywords);
                used_hash = Some(hash_func.to_uppercase());
                used_insecure_hash = call_args
                    .get_argument("usedforsecurity", None)
                    .and_then(get_bool_value)
                    .unwrap_or(true);
                break;
            }
        }
    }

    if used_insecure_hash {
        Some(Check::new(
            CheckKind::UseOfInsecureHash(used_hash.unwrap_or("UNKNOWN".to_string()), at_least_py39),
            Range::from_located(func),
        ))
    } else {
        None
    }
}
