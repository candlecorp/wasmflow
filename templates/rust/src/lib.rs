mod wick {
  wick_component::wick_import!();
}
use wick::*;

// Operations that follow common patterns can have their boilerplate generated
// via the #[wick_component::operation] attribute like so:
#[wick_component::operation(binary_interleaved_pairs)]
fn add(left: i64, right: i64, _ctx: Context<add::Config>) -> Result<i64, anyhow::Error> {
  Ok(left + right)
}

// Operations where you need more control over the incoming and outgoing streams
// can be defined manually like so:
#[async_trait::async_trait(?Send)]
impl greet::Operation for Component {
  type Error = anyhow::Error;
  type Inputs = greet::Inputs;
  type Outputs = greet::Outputs;
  type Config = greet::Config;

  async fn greet(
    mut inputs: Self::Inputs,
    mut outputs: Self::Outputs,
    _ctx: Context<Self::Config>,
  ) -> Result<(), Self::Error> {
    while let Some(packet) = inputs.input.next().await {
      // "Signals" are special packets that are used to indicate that a stream
      // has ended, has opened a substream, or has closed a substream.
      if packet.is_signal() {
        // This example propagates all signals to the output stream, resetting
        // the port name to our output port.
        outputs.output.send_raw_payload(packet.into());
        continue;
      }
      let name = propagate_if_error!(packet.decode(), outputs, continue);

      outputs.output.send(&format!("Hello, {}", name));
    }
    outputs.output.done();
    Ok(())
  }
}
