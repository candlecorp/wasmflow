/**********************************************
***** This file is generated, do not edit *****
***********************************************/

pub use vino_provider::prelude::*;

pub mod __multi__;
pub mod error; // error
pub mod test_component; // test-component

#[derive(Debug)]
pub(crate) struct Dispatcher {}
#[async_trait]
impl Dispatch for Dispatcher {
  type Context = crate::Context;
  async fn dispatch(
    op: &str,
    context: Self::Context,
    data: TransportMap,
  ) -> Result<TransportStream, Box<NativeComponentError>> {
    let result = match op {
      "error" => {
        self::generated::error::Component::default()
          .execute(context, data)
          .await
      }
      "test-component" => {
        self::generated::test_component::Component::default()
          .execute(context, data)
          .await
      }
      "__multi__" => {
        self::generated::__multi__::Component::default()
          .execute(context, data)
          .await
      }
      _ => Err(Box::new(NativeComponentError::new(format!(
        "Component not found on this provider: {}",
        op
      )))),
    }?;
    Ok(result)
  }
}

pub fn get_signature() -> ProviderSignature {
  let mut components = std::collections::HashMap::new();

  components.insert("error", generated::error::signature());
  components.insert("test-component", generated::test_component::signature());

  ProviderSignature {
    name: Some("test-vino-provider".to_owned()),
    types: std::collections::HashMap::from([]).into(),
    components: components.into(),
  }
}

pub mod types {
  // no additional types
}

pub mod generated {

  // start namespace
  // Component name : error
  pub mod error {
    use async_trait::async_trait;

    pub use vino_provider::prelude::*;

    pub fn signature() -> ComponentSignature {
      ComponentSignature {
        name: "error".to_owned(),
        inputs: inputs_list().into(),
        outputs: outputs_list().into(),
      }
    }

    #[derive(Default, Copy, Clone, Debug)]
    pub struct Component {}

    #[async_trait]
    impl NativeComponent for Component {
      type Context = crate::Context;
      async fn execute(
        &self,
        context: Self::Context,
        data: TransportMap,
      ) -> Result<TransportStream, Box<NativeComponentError>> {
        let inputs = populate_inputs(data).map_err(|e| NativeComponentError::new(e.to_string()))?;
        let (outputs, stream) = get_outputs();
        let result = tokio::spawn(crate::components::error::job(inputs, outputs, context))
          .await
          .map_err(|e| Box::new(NativeComponentError::new(format!("Component error: {}", e))))?;
        match result {
          Ok(_) => Ok(stream),
          Err(e) => Err(Box::new(NativeComponentError::new(e.to_string()))),
        }
      }
    }

    #[cfg(any(feature = "native", feature = "wasm"))]
    pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
      Ok(Inputs {
        input: payload.consume("input")?,
      })
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
    pub struct Inputs {
      #[serde(rename = "input")]
      pub input: String,
    }

    #[cfg(any(feature = "native", feature = "wasm"))]
    impl From<Inputs> for TransportMap {
      fn from(inputs: Inputs) -> TransportMap {
        let mut map = TransportMap::new();
        map.insert("input", MessageTransport::success(&inputs.input));

        map
      }
    }

    #[must_use]
    #[cfg(any(feature = "native", feature = "wasm"))]
    pub fn inputs_list() -> std::collections::HashMap<String, TypeSignature> {
      let mut map = std::collections::HashMap::new();
      map.insert("input".to_owned(), TypeSignature::String);
      map
    }
    #[derive(Debug, Default)]
    #[cfg(feature = "provider")]
    pub struct OutputPorts {
      pub output: OutputPortSender,
    }

    #[must_use]
    #[cfg(any(feature = "native", feature = "wasm"))]
    pub fn outputs_list() -> std::collections::HashMap<String, TypeSignature> {
      let mut map = std::collections::HashMap::new();
      map.insert("output".to_owned(), TypeSignature::String);
      map
    }

    #[derive(Debug)]
    #[cfg(feature = "provider")]
    pub struct OutputPortSender {
      port: PortChannel,
    }

    #[cfg(feature = "provider")]
    impl Default for OutputPortSender {
      fn default() -> Self {
        Self {
          port: PortChannel::new("output"),
        }
      }
    }

    #[cfg(feature = "provider")]
    impl PortSender for OutputPortSender {
      fn get_port(&self) -> Result<&PortChannel, ProviderError> {
        if self.port.is_closed() {
          Err(ProviderError::SendChannelClosed)
        } else {
          Ok(&self.port)
        }
      }

      fn get_port_name(&self) -> &str {
        &self.port.name
      }
    }

    #[must_use]
    #[cfg(feature = "provider")]
    pub fn get_outputs() -> (OutputPorts, TransportStream) {
      let mut outputs = OutputPorts::default();
      let mut ports = vec![&mut outputs.output.port];
      let stream = PortChannel::merge_all(&mut ports);
      (outputs, stream)
    }

    #[cfg(all(feature = "guest"))]
    #[allow(missing_debug_implementations)]
    pub struct Outputs {
      packets: ProviderOutput,
    }

    #[cfg(all(feature = "native", feature = "guest"))]
    impl Outputs {
      pub async fn output(&mut self) -> Result<PortOutput<String>, ProviderError> {
        let packets = self.packets.take("output").await;
        Ok(PortOutput::new("output".to_owned(), packets))
      }
    }

    #[cfg(all(feature = "wasm", feature = "guest"))]
    impl Outputs {
      pub fn output(&mut self) -> Result<PortOutput, WasmError> {
        let packets = self
          .packets
          .take("output")
          .ok_or_else(|| WasmError::ResponseMissing("output".to_owned()))?;
        Ok(PortOutput::new("output".to_owned(), packets))
      }
    }

    #[cfg(all(feature = "wasm", feature = "guest"))]
    impl From<ProviderOutput> for Outputs {
      fn from(packets: ProviderOutput) -> Self {
        Self { packets }
      }
    }

    #[cfg(all(feature = "native", feature = "guest"))]
    impl From<ProviderOutput> for Outputs {
      fn from(output: ProviderOutput) -> Self {
        Self { packets: output }
      }
    }

    #[cfg(all(feature = "native", feature = "guest"))]
    impl From<BoxedTransportStream> for Outputs {
      fn from(stream: BoxedTransportStream) -> Self {
        Self {
          packets: ProviderOutput::new(stream),
        }
      }
    }
  }
  // Component name : test-component
  pub mod test_component {
    use async_trait::async_trait;

    pub use vino_provider::prelude::*;

    pub fn signature() -> ComponentSignature {
      ComponentSignature {
        name: "test-component".to_owned(),
        inputs: inputs_list().into(),
        outputs: outputs_list().into(),
      }
    }

    #[derive(Default, Copy, Clone, Debug)]
    pub struct Component {}

    #[async_trait]
    impl NativeComponent for Component {
      type Context = crate::Context;
      async fn execute(
        &self,
        context: Self::Context,
        data: TransportMap,
      ) -> Result<TransportStream, Box<NativeComponentError>> {
        let inputs = populate_inputs(data).map_err(|e| NativeComponentError::new(e.to_string()))?;
        let (outputs, stream) = get_outputs();
        let result = tokio::spawn(crate::components::test_component::job(inputs, outputs, context))
          .await
          .map_err(|e| Box::new(NativeComponentError::new(format!("Component error: {}", e))))?;
        match result {
          Ok(_) => Ok(stream),
          Err(e) => Err(Box::new(NativeComponentError::new(e.to_string()))),
        }
      }
    }

    #[cfg(any(feature = "native", feature = "wasm"))]
    pub fn populate_inputs(mut payload: TransportMap) -> Result<Inputs, TransportError> {
      Ok(Inputs {
        input: payload.consume("input")?,
      })
    }

    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
    pub struct Inputs {
      #[serde(rename = "input")]
      pub input: String,
    }

    #[cfg(any(feature = "native", feature = "wasm"))]
    impl From<Inputs> for TransportMap {
      fn from(inputs: Inputs) -> TransportMap {
        let mut map = TransportMap::new();
        map.insert("input", MessageTransport::success(&inputs.input));

        map
      }
    }

    #[must_use]
    #[cfg(any(feature = "native", feature = "wasm"))]
    pub fn inputs_list() -> std::collections::HashMap<String, TypeSignature> {
      let mut map = std::collections::HashMap::new();
      map.insert("input".to_owned(), TypeSignature::String);
      map
    }
    #[derive(Debug, Default)]
    #[cfg(feature = "provider")]
    pub struct OutputPorts {
      pub output: OutputPortSender,
    }

    #[must_use]
    #[cfg(any(feature = "native", feature = "wasm"))]
    pub fn outputs_list() -> std::collections::HashMap<String, TypeSignature> {
      let mut map = std::collections::HashMap::new();
      map.insert("output".to_owned(), TypeSignature::String);
      map
    }

    #[derive(Debug)]
    #[cfg(feature = "provider")]
    pub struct OutputPortSender {
      port: PortChannel,
    }

    #[cfg(feature = "provider")]
    impl Default for OutputPortSender {
      fn default() -> Self {
        Self {
          port: PortChannel::new("output"),
        }
      }
    }

    #[cfg(feature = "provider")]
    impl PortSender for OutputPortSender {
      fn get_port(&self) -> Result<&PortChannel, ProviderError> {
        if self.port.is_closed() {
          Err(ProviderError::SendChannelClosed)
        } else {
          Ok(&self.port)
        }
      }

      fn get_port_name(&self) -> &str {
        &self.port.name
      }
    }

    #[must_use]
    #[cfg(feature = "provider")]
    pub fn get_outputs() -> (OutputPorts, TransportStream) {
      let mut outputs = OutputPorts::default();
      let mut ports = vec![&mut outputs.output.port];
      let stream = PortChannel::merge_all(&mut ports);
      (outputs, stream)
    }

    #[cfg(all(feature = "guest"))]
    #[allow(missing_debug_implementations)]
    pub struct Outputs {
      packets: ProviderOutput,
    }

    #[cfg(all(feature = "native", feature = "guest"))]
    impl Outputs {
      pub async fn output(&mut self) -> Result<PortOutput<String>, ProviderError> {
        let packets = self.packets.take("output").await;
        Ok(PortOutput::new("output".to_owned(), packets))
      }
    }

    #[cfg(all(feature = "wasm", feature = "guest"))]
    impl Outputs {
      pub fn output(&mut self) -> Result<PortOutput, WasmError> {
        let packets = self
          .packets
          .take("output")
          .ok_or_else(|| WasmError::ResponseMissing("output".to_owned()))?;
        Ok(PortOutput::new("output".to_owned(), packets))
      }
    }

    #[cfg(all(feature = "wasm", feature = "guest"))]
    impl From<ProviderOutput> for Outputs {
      fn from(packets: ProviderOutput) -> Self {
        Self { packets }
      }
    }

    #[cfg(all(feature = "native", feature = "guest"))]
    impl From<ProviderOutput> for Outputs {
      fn from(output: ProviderOutput) -> Self {
        Self { packets: output }
      }
    }

    #[cfg(all(feature = "native", feature = "guest"))]
    impl From<BoxedTransportStream> for Outputs {
      fn from(stream: BoxedTransportStream) -> Self {
        Self {
          packets: ProviderOutput::new(stream),
        }
      }
    }
  }

  pub mod __multi__ {

    #[derive(Debug, serde::Deserialize, serde::Serialize, Clone)]
    pub enum ComponentInputs {
      Error(super::error::Inputs),
      TestComponent(super::test_component::Inputs),
    }

    #[cfg(all(feature = "guest"))]
    #[allow(missing_debug_implementations)]
    pub enum ComponentOutputs {
      Error(super::error::Outputs),
      TestComponent(super::test_component::Outputs),
    }
    #[cfg(any(feature = "native"))]
    pub use vino_provider::native::prelude::*;
    #[cfg(any(feature = "wasm"))]
    pub use vino_provider::wasm::prelude::*;

    #[derive(Debug, Default)]
    #[cfg(feature = "provider")]
    pub struct OutputPorts {
      pub result: ResultPortSender,
    }

    #[must_use]
    #[cfg(any(feature = "native", feature = "wasm"))]
    pub fn outputs_list() -> std::collections::HashMap<String, TypeSignature> {
      let mut map = std::collections::HashMap::new();
      map.insert("result".to_owned(), TypeSignature::Bool);
      map
    }

    #[derive(Debug)]
    #[cfg(feature = "provider")]
    pub struct ResultPortSender {
      port: PortChannel,
    }

    #[cfg(feature = "provider")]
    impl Default for ResultPortSender {
      fn default() -> Self {
        Self {
          port: PortChannel::new("result"),
        }
      }
    }

    #[cfg(feature = "provider")]
    impl PortSender for ResultPortSender {
      fn get_port(&self) -> Result<&PortChannel, ProviderError> {
        if self.port.is_closed() {
          Err(ProviderError::SendChannelClosed)
        } else {
          Ok(&self.port)
        }
      }

      fn get_port_name(&self) -> &str {
        &self.port.name
      }
    }

    #[must_use]
    #[cfg(feature = "provider")]
    pub fn get_outputs() -> (OutputPorts, TransportStream) {
      let mut outputs = OutputPorts::default();
      let mut ports = vec![&mut outputs.result.port];
      let stream = PortChannel::merge_all(&mut ports);
      (outputs, stream)
    }

    #[cfg(all(feature = "guest"))]
    #[allow(missing_debug_implementations)]
    pub struct Outputs {
      packets: ProviderOutput,
    }

    #[cfg(all(feature = "native", feature = "guest"))]
    impl Outputs {
      pub async fn result(&mut self) -> Result<PortOutput<bool>, ProviderError> {
        let packets = self.packets.take("result").await;
        Ok(PortOutput::new("result".to_owned(), packets))
      }
    }

    #[cfg(all(feature = "wasm", feature = "guest"))]
    impl Outputs {
      pub fn result(&mut self) -> Result<PortOutput, WasmError> {
        let packets = self
          .packets
          .take("result")
          .ok_or_else(|| WasmError::ResponseMissing("result".to_owned()))?;
        Ok(PortOutput::new("result".to_owned(), packets))
      }
    }

    #[cfg(all(feature = "wasm", feature = "guest"))]
    impl From<ProviderOutput> for Outputs {
      fn from(packets: ProviderOutput) -> Self {
        Self { packets }
      }
    }

    #[cfg(all(feature = "native", feature = "guest"))]
    impl From<ProviderOutput> for Outputs {
      fn from(output: ProviderOutput) -> Self {
        Self { packets: output }
      }
    }

    #[cfg(all(feature = "native", feature = "guest"))]
    impl From<BoxedTransportStream> for Outputs {
      fn from(stream: BoxedTransportStream) -> Self {
        Self {
          packets: ProviderOutput::new(stream),
        }
      }
    }
    use async_trait::async_trait;

    pub use vino_provider::prelude::*;

    #[derive(Default, Copy, Clone, Debug)]
    pub struct Component {}

    #[async_trait]
    impl NativeComponent for Component {
      type Context = crate::Context;
      async fn execute(
        &self,
        context: Self::Context,
        data: TransportMap,
      ) -> Result<TransportStream, Box<NativeComponentError>> {
        let inputs = populate_inputs(data).map_err(|e| NativeComponentError::new(e.to_string()))?;
        let (outputs, stream) = get_outputs();
        let result = tokio::spawn(crate::components::__multi__::job(inputs, outputs, context))
          .await
          .map_err(|e| Box::new(NativeComponentError::new(format!("Component error: {}", e))))?;
        match result {
          Ok(_) => Ok(stream),
          Err(e) => Err(Box::new(NativeComponentError::new(e.to_string()))),
        }
      }
    }

    pub fn populate_inputs(mut payload: TransportMap) -> Result<Vec<ComponentInputs>, TransportError> {
      payload.consume::<Vec<ComponentInputs>>("inputs")
    }
  }
}
