use std::path::PathBuf;

use phenix_core::{ComplexCreationBuilder, RuntimeBuilder};

fn main() {
  let runtime = RuntimeBuilder::default()
    .with_value("test::pack::bool".into(), true.into())
    .with_value("test::pack::number".into(), 42.into())
    .with_value("test::pack::path".into(), PathBuf::from("test/path").into())
    .with_value("test::pack::string".into(), "test-string".into())
    .build();

  let creation = ComplexCreationBuilder::new("test::pack::number".into()).build();

  let result = runtime.eval(creation);

  println!("{:?}", result);
}
