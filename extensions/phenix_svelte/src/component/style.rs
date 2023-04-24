pub fn new_svelte_component_script() -> TextOperation {
  new_html_element_style_with(
    new_std_text_empty(),
    GetArgumentOperation::new(SVELTE_COMPONENT_SCRIPT__CONTENT),
  )
}
