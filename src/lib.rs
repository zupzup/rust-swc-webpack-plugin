use swc_core::common::DUMMY_SP;
use swc_core::ecma::{
    ast::*,
    transforms::testing::test,
    utils::quote_ident,
    visit::{as_folder, noop_visit_mut_type, FoldWith, VisitMut, VisitMutWith},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

pub struct TransformVisitor;

const CONSOLE: &str = "console";
const DEBUG: &str = "debug";
const LOG: &str = "log";

impl VisitMut for TransformVisitor {
    noop_visit_mut_type!();

    fn visit_mut_call_expr(&mut self, call_expr: &mut CallExpr) {
        call_expr.visit_mut_children_with(self);

        if let Callee::Expr(callee) = &mut call_expr.callee {
            if let Expr::Member(member) = &**callee {
                if let (Expr::Ident(obj), MemberProp::Ident(prop)) = (&*member.obj, &member.prop) {
                    if &obj.sym == CONSOLE && &prop.sym == LOG {
                        *callee = Box::new(Expr::Member(MemberExpr {
                            span: DUMMY_SP,
                            obj: member.obj.to_owned(),
                            prop: MemberProp::Ident(quote_ident!(DEBUG)),
                        }));
                    }
                }
            }
        }
    }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
    program.fold_with(&mut as_folder(TransformVisitor))
}

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    log_to_debug,
    // Input codes
    r#"console.log("hello, world");"#,
    // Output codes after transformed with plugin
    r#"console.debug("hello, world");"#
);

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    debug_stays_debug,
    // Input codes
    r#"console.debug("hello, world");"#,
    // Output codes after transformed with plugin
    r#"console.debug("hello, world");"#
);

test!(
    Default::default(),
    |_| as_folder(TransformVisitor),
    not_interested_in_args,
    // Input codes
    r#"console.debug("log");"#,
    // Output codes after transformed with plugin
    r#"console.debug("log");"#
);
