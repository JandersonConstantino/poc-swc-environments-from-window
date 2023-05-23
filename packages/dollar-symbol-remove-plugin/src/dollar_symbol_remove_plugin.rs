use swc_core::ecma::{
    ast::*,
    transforms::testing::test,
    visit::{as_folder, FoldWith, VisitMut},
};
use swc_core::plugin::{plugin_transform, proxies::TransformPluginProgramMetadata};

use crate::util;

pub struct TransformVisitor;

fn transform_key_value(node: &mut Prop, prefix_name: String) {
  match node {
    Prop::KeyValue(KeyValueProp { key, value, .. }) => {
      let mut has_updated_key_name = false;
      let mut updated_key = String::new();

      if let PropName::Ident(ident) = key {
        has_updated_key_name = ident.sym.starts_with("$");

        updated_key = ident.sym.replace("$", "");
        ident.sym = updated_key.clone().into();
      }

      let transformed_key_name = util::transform_key_name(
        prefix_name.clone(),
        updated_key.clone()
      );

      if has_updated_key_name && util::is_production_environment() && value.is_lit() {
        *value = Lit::Str(Str {
          value: "".into(),
          span: swc_core::common::DUMMY_SP,
          raw: Some(
            format!("window.{}", transformed_key_name).into()
          ),
        }).into();
      }

      if let Expr::Object(i) = &mut **value {
        for ele in &mut i.props {
          if let PropOrSpread::Prop(p) = ele {
            transform_key_value(p, transformed_key_name.clone());
          }
        }
      }
    }
    _ => {}
  }
}

impl VisitMut for TransformVisitor {
  fn visit_mut_prop(&mut self, node: &mut Prop) {
    transform_key_value(node, String::new());
  }
}

#[plugin_transform]
pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
  program.fold_with(&mut as_folder(TransformVisitor))
}

test!(
  Default::default(),
  |_| as_folder(TransformVisitor),
  must_change_only_prop_name_when_is_not_production,
  r#"
    const obj = {
      $prop1: 10,
      foo: {
          $baar: 'localhost:3030'
      }
    };
  "#,
  r#"
    const obj = {
      prop1: 10,
      foo: {
          baar: 'localhost:3030'
      }
    };
  "#
);