use phenix_core::{AddOperation, ComplexCreationBuilder, GetArgumentOperation, RuntimeBuilder};
use phenix_std::RuntimeBuilderStdExt;
use phenix_svelte::PhenixSvelteExtension;

fn main() {
  let runtime = RuntimeBuilder::default().with_std().with_svelte().build();

  let creation = ComplexCreationBuilder::new("test:number")
    .with("test:number$a", 100)
    .with("test:number$b", 567654)
    .into();

  let result = runtime.evaluate(&creation);

  match result {
    Ok(value) => println!("{:?}", value),
    Err(error) => println!("{}", error),
  }
}
