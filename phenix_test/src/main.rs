use phenix_core::{AddOperation, ComplexCreationBuilder, GetArgumentOperation, RuntimeBuilder};
use phenix_std::RuntimeBuilderStdExt;

fn main() {
  let runtime = RuntimeBuilder::default()
    .with_std()
    .with_number(
      "test:number",
      AddOperation::new(
        GetArgumentOperation::new("test:number$a"),
        GetArgumentOperation::new("test:number$b"),
      ),
    )
    .build();

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
