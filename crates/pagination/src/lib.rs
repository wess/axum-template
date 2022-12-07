use serde::Deserialize;

const DEFAULT_LIMIT: usize = 500;

/// Query parameters used to paginate API
#[derive(Debug, Deserialize, PartialEq, Eq)]
pub struct Paginate {
  pub page: Option<usize>,
  pub limit: Option<usize>,
  pub offset: Option<usize>,
}

impl Paginate {
  /// Check and update with correct values
  pub fn build(&mut self) {
    // Page
    if let Some(page) = self.page {
      if page < 1 {
        self.page = Some(1);
      }
    } else {
      self.page = Some(1);
    }

    // Limit
    if let Some(limit) = self.limit {
      if !(1..=DEFAULT_LIMIT).contains(&limit) {
        self.limit = Some(DEFAULT_LIMIT);
      }
    } else {
      self.limit = Some(DEFAULT_LIMIT);
    }

    // Offset
    self.offset = match (self.page, self.limit) {
      (Some(page), Some(limit)) => Some((page - 1) * limit),
      _ => Some(0),
    };
  }
}