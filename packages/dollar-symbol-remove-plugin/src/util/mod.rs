use std::env;

mod capitalize;

pub fn is_production_environment() -> bool {
  if let Ok(node_env) = env::var("NODE_ENV") {
    return node_env == "production";
  }

  return false;
}

pub fn transform_key_name(prefix_name: String, key: String) -> String {
  if prefix_name == "" {
    return String::from(key);
  }

  return format!("{}{}", prefix_name, capitalize::capitalize(key));
}
