use phenix_core::{ComplexCreationBuilder, RuntimeBuilder};
use phenix_std::RuntimeBuilderStdExt;
use phenix_svelte::{PhenixSvelteExtension, SVELTE_PROJECT, SVELTE_PROJECT__NAME};

fn main() {
  let runtime = RuntimeBuilder::default().with_std().with_svelte().build();

  let creation = ComplexCreationBuilder::new(SVELTE_PROJECT)
    .with(SVELTE_PROJECT__NAME, "my_test_project")
    .into();

  let result = runtime.evaluate(&creation);

  match result {
    Ok(value) => println!("{:#?}", value),
    Err(error) => println!("{}", error),
  }
}
