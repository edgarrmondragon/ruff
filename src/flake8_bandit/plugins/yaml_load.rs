use rustc_hash::{FxHashMap, FxHashSet};
use rustpython_ast::{Expr, Keyword};

use crate::ast::helpers::{match_module_member, SimpleCallArgs};
use crate::ast::types::Range;
use crate::registry::{Check, CheckKind};

/// S506
pub fn yaml_load(
    func: &Expr,
    args: &Vec<Expr>,
    keywords: &Vec<Keyword>,
    from_imports: &FxHashMap<&str, FxHashSet<&str>>,
    import_aliases: &FxHashMap<&str, &str>,
) -> Option<Check> {
    let mut is_unsafe_load = false;
    if match_module_member(func, "yaml", "load", from_imports, import_aliases) {
        let call_args = SimpleCallArgs::new(args, keywords);
        if let Some(loader_arg) = call_args.get_argument("Loader", Some(1)) {
            if !match_module_member(
                loader_arg,
                "yaml",
                "SafeLoader",
                from_imports,
                import_aliases,
            ) && !match_module_member(
                loader_arg,
                "yaml",
                "CSafeLoader",
                from_imports,
                import_aliases,
            ) {
                is_unsafe_load = true;
            }
        } else {
            is_unsafe_load = true;
        }
    }

    if is_unsafe_load {
        Some(Check::new(
            CheckKind::UseOfYamlLoad,
            Range::from_located(func),
        ))
    } else {
        None
    }
}
