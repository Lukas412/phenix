pub use {
  ext::PhenixSvelteExtension,
  phenix_identifiers::{SVELTE_PROJECT, SVELTE_PROJECT_INIT, SVELTE_PROJECT__NAME},
  project::{new_svelte_project_init_operation, new_svelte_project_operation},
};

mod ext;
mod page;
mod phenix_identifiers;
mod project;
