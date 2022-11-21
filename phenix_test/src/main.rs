use phenix_core::{ComplexCreationBuilder, RuntimeBuilder};

fn main() {
  let runtime = RuntimeBuilder::default()
    .with_value("test::pack1".into(), "test1".into())
    .with_value("test::pack2".into(), "test2".into())
    .build();

  let creation = ComplexCreationBuilder::new("test::pack1".into()).build();

  let result = runtime.eval(creation);

  println!("{:?}", result);
}
