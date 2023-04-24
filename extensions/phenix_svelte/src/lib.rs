pub use {
  component::{new_svelte_component, new_svelte_component_content, new_svelte_component_script},
  ext::PhenixSvelteExtension,
  phenix_identifiers::{
    SVELTE_COMPONENT, SVELTE_COMPONENT_CONTENT, SVELTE_COMPONENT_SCRIPT__CONTENT,
    SVELTE_COMPONENT__BODY, SVELTE_PROJECT, SVELTE_PROJECT_INIT, SVELTE_PROJECT__NAME,
  },
  project::{new_svelte_project_init_operation, new_svelte_project_operation},
};

mod component;
mod ext;
mod phenix_identifiers;
mod project;
