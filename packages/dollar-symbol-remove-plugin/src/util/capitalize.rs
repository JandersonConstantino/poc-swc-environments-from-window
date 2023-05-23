pub fn capitalize(value: String) -> String {
  let mut chars = value.as_str().chars();

  match chars.next() {
    None => String::new(),
    Some(char) => {
      let first_letter = char.to_uppercase().collect::<String>();
      let rest_of_string = chars.collect::<String>();

      return String::from(format!("{}{}", first_letter, rest_of_string));
    }
  }
}
