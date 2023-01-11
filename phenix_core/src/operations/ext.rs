pub trait OperationExt {
  type EvalResult;

  fn eval(&self) -> Self::EvalResult;
}
