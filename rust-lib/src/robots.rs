const DEFAULT_HEAD_META: &str = "noindex, noarchive, noimageindex, nofollow";

pub fn generate_robots(noindex: bool, content_type: &str) -> &str {
  if content_type != "article" {
    return DEFAULT_HEAD_META;
  }

  if noindex {
    return DEFAULT_HEAD_META;
  }
  "noarchive, noimageindex"
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate_robots() {
    assert_eq!(
      generate_robots(true, "article"),
      "noindex, noarchive, noimageindex, nofollow"
    );
    assert_eq!(generate_robots(false, "article"), "noarchive, noimageindex");
    assert_eq!(
      generate_robots(true, "page"),
      "noindex, noarchive, noimageindex, nofollow"
    );
    assert_eq!(
      generate_robots(false, "page"),
      "noindex, noarchive, noimageindex, nofollow"
    );
  }
}
