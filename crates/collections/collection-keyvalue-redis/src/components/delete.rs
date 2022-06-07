use wasmflow_interface_keyvalue::delete::*;

use crate::components::generated::delete::*;

pub(crate) type State = ();

#[async_trait::async_trait]
impl wasmflow_sdk::sdk::stateful::BatchedComponent for Component {
  type Context = crate::Context;
  type State = State;
  async fn job(
    input: Self::Inputs,
    output: Self::Outputs,
    context: Self::Context,
    state: Option<Self::State>,
    _config: Option<Self::Config>,
  ) -> Result<Option<Self::State>, Box<dyn std::error::Error + Send + Sync>> {
    trace!(?input, "delete");
    let mut cmd = redis::Cmd::del(&input.keys);
    let num: u32 = context.run_cmd(&mut cmd).await?;
    output.num.done(num)?;
    Ok(state)
  }
}
