use phenix_core::{ComplexCreationBuilder, RuntimeBuilder};
use phenix_std::RuntimeBuilderStdExt;
use phenix_svelte::PhenixSvelteExtension;

fn main() {
  let runtime = RuntimeBuilder::default().with_std().with_svelte().build();

  let creation = ComplexCreationBuilder::new("svelte:project:create")
    .with("svelte:project$name", "my_test_project")
    .into();

  let result = runtime.evaluate(&creation);

  match result {
    Ok(value) => println!("{:?}", value),
    Err(error) => println!("{}", error),
  }
}
