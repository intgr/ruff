use rustpython_parser::ast::{Alias, AliasData, Located, Stmt, StmtKind};

use ruff_diagnostics::{AutofixKind, Diagnostic, Edit, Fix, Violation};
use ruff_macros::{derive_message_formats, violation};
use ruff_python_ast::helpers::{create_stmt, unparse_stmt};

use crate::checkers::ast::Checker;
use crate::registry::AsRule;

#[violation]
pub struct ManualFromImport {
    module: String,
    name: String,
}

impl Violation for ManualFromImport {
    const AUTOFIX: AutofixKind = AutofixKind::Sometimes;

    #[derive_message_formats]
    fn message(&self) -> String {
        let ManualFromImport { module, name } = self;
        format!("Use `from {module} import {name}` in lieu of alias")
    }

    fn autofix_title(&self) -> Option<String> {
        let ManualFromImport { module, name } = self;
        Some(format!("Replace with `from {module} import {name}`"))
    }
}

/// PLR0402
pub fn manual_from_import(checker: &mut Checker, stmt: &Stmt, alias: &Alias, names: &[Alias]) {
    let Some(asname) = &alias.node.asname else {
        return;
    };
    let Some((module, name)) = alias.node.name.rsplit_once('.') else {
        return;
    };
    if name != asname {
        return;
    }

    let fixable = names.len() == 1;
    let mut diagnostic = Diagnostic::new(
        ManualFromImport {
            module: module.to_string(),
            name: name.to_string(),
        },
        alias.range(),
    );
    if fixable && checker.patch(diagnostic.kind.rule()) {
        diagnostic.set_fix(Fix::unspecified(Edit::range_replacement(
            unparse_stmt(
                &create_stmt(StmtKind::ImportFrom {
                    module: Some(module.to_string()),
                    names: vec![Located::with_range(
                        AliasData {
                            name: asname.into(),
                            asname: None,
                        },
                        stmt.range(),
                    )],
                    level: Some(0),
                }),
                checker.stylist,
            ),
            stmt.range(),
        )));
    }
    checker.diagnostics.push(diagnostic);
}
