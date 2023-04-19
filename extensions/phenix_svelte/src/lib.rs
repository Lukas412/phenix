pub use {
  ext::PhenixSvelteExtension,
  project::{
    new_svelte_project_init_operation, new_svelte_project_operation, SVELTE_PROJECT,
    SVELTE_PROJECT_INIT, SVELTE_PROJECT__NAME,
  },
};

mod ext;
mod project;
