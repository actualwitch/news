

pub fn pluralize<'a>(count: i32, singular: &'a str, plural: &'a str) -> &'a str {
  if count == 1 {
      return singular
  }
  plural
}