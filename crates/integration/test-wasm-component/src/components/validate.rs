use std::error::Error;
use std::fmt::Display;
use std::usize;

pub use crate::components::generated::validate::*;

#[derive(Debug)]
enum LengthError {
  TooShort,
  TooLong,
}

impl Display for LengthError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "{}",
      match self {
        LengthError::TooShort => format!("Needs to be longer than {} characters", MINIMUM_LENGTH),
        LengthError::TooLong => format!("Needs to be shorter than {} characters", MAXIMUM_LENGTH),
      }
    )
  }
}

impl Error for LengthError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    Some(self)
  }
}

static MINIMUM_LENGTH: usize = 8;
static MAXIMUM_LENGTH: usize = 512;

pub(crate) type State = ();

#[async_trait::async_trait]
impl wasmflow_sdk::sdk::ephemeral::BatchedComponent for Component {
  type State = State;
  async fn job(
    input: Self::Inputs,
    output: Self::Outputs,
    state: Option<Self::State>,
    _config: Option<Self::Config>,
  ) -> Result<Option<Self::State>, Box<dyn std::error::Error + Send + Sync>> {
    let password = input.input;
    if password.len() < MINIMUM_LENGTH {
      output.output.done_exception(LengthError::TooShort.to_string())?;
      return Ok(state);
    }
    if password.len() > MAXIMUM_LENGTH {
      output.output.done_exception(LengthError::TooLong.to_string())?;
      return Ok(state);
    }
    output.output.done(password)?;

    Ok(state)
  }
}
