pub fn init_hooks(name: &str) -> String {
  let hooks_file = format!(
    "hook:
  namespace: \"{name}\"
  source_path: \"src\"
  depends: []
  actions:
config:
  example_api_url: \"\"
"
  );

  return hooks_file;
}

pub fn init_cogs(name: &str) -> String {
  let cog_file = format!(
    "name: \"{name}\"
namespace: \"{name}\"
"
  );

  return cog_file;
}
