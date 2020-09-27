use ::std::error::Error;
use ::std::fmt::{Debug, Display, Formatter, Result as FormatResult};

use ::url::Url;

#[derive(Debug, Clone, Default)]
pub struct MaximumAttemptExceeded;

impl Display for MaximumAttemptExceeded {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Maximum retrieving count exceeded.");
  }
}

impl Error for MaximumAttemptExceeded {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

unsafe impl Send for MaximumAttemptExceeded {}

#[derive(Debug, Clone)]
pub struct StatusFailure {
  pub url: Url,
  pub code: u16,
  pub text: String,
}

impl Display for StatusFailure {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Status Failure: {}", self);
  }
}
impl Error for StatusFailure {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

unsafe impl Send for StatusFailure {}

#[derive(Debug, Clone)]
pub struct EmptyError {
  pub field: String,
}

impl Display for EmptyError {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "Field {} is required, but it's empty", self.field);
  }
}

impl Error for EmptyError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

unsafe impl Send for EmptyError {}

#[derive(Debug, Clone)]
pub struct GenericError<'t> {
  msg: &'t str,
}

impl<'t> GenericError<'t> {
  pub fn new(msg: &'t str) -> Self {
    return Self { msg };
  }
}

impl<'t> Display for GenericError<'t> {
  fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
    return write!(f, "{}", self.msg);
  }
}

impl<'t> Error for GenericError<'t> {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    None
  }
}

unsafe impl<'t> Send for GenericError<'t> {}
