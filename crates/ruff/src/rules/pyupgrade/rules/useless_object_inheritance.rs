use rustpython_parser::ast::{self, Expr, Ranged, Stmt};

use ruff_diagnostics::{AlwaysAutofixableViolation, Diagnostic, Fix};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::helpers::identifier_range;

use crate::autofix::edits::remove_argument;
use crate::checkers::ast::Checker;
use crate::registry::AsRule;

#[violation]
pub struct UselessObjectInheritance {
    name: String,
}

impl AlwaysAutofixableViolation for UselessObjectInheritance {
    #[derive_message_formats]
    fn message(&self) -> String {
        let UselessObjectInheritance { name } = self;
        format!("Class `{name}` inherits from `object`")
    }

    fn autofix_title(&self) -> String {
        "Remove `object` inheritance".to_string()
    }
}

/// UP004
pub(crate) fn useless_object_inheritance(
    checker: &mut Checker,
    class_def: &ast::StmtClassDef,
    stmt: &Stmt,
) {
    for expr in &class_def.bases {
        let Expr::Name(ast::ExprName { id, .. }) = expr else {
            continue;
        };
        if id != "object" {
            continue;
        }
        if !checker.semantic_model().is_builtin("object") {
            continue;
        }

        let mut diagnostic = Diagnostic::new(
            UselessObjectInheritance {
                name: class_def.name.to_string(),
            },
            expr.range(),
        );
        if checker.patch(diagnostic.kind.rule()) {
            diagnostic.try_set_fix(|| {
                let edit = remove_argument(
                    checker.locator,
                    identifier_range(stmt, checker.locator).start(),
                    expr.range(),
                    &class_def.bases,
                    &class_def.keywords,
                    true,
                )?;
                Ok(Fix::automatic(edit))
            });
        }
        checker.diagnostics.push(diagnostic);
    }
}
