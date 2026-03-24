pub mod models;
pub mod sdk_code;
pub mod sdk_data;

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[ignore]
  fn test_gen() {
    sdk_data::gen_sdk_data("../../schemas", "../../sdk_data").unwrap();

    sdk_code::gen_sdk_code("../../sdk_data", "../ofdsdk/src").unwrap();
  }
}
