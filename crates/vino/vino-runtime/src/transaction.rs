use std::time::Duration;

use vino_transport::TransportMap;

use self::executor::SchematicOutput;
use self::ports::PortStatuses;
use crate::dev::prelude::*;
use crate::schematic_service::input_message::InputMessage;
type Result<T> = std::result::Result<T, TransactionError>;

pub(crate) mod executor;
pub(crate) mod ports;

#[derive(Clone, Debug)]
pub struct ComponentPayload {
  pub tx_id: String,
  pub instance: String,
  pub payload_map: TransportMap,
}

#[derive(Debug)]
pub enum TransactionUpdate {
  NoOp,
  Drained,
  Error(String),
  Timeout(Duration),
  Transition(ConnectionTargetDefinition),
  Execute(ComponentPayload),
  Result(SchematicOutput),
  Done(String),
  Update(InputMessage),
}

impl std::fmt::Display for TransactionUpdate {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let name = match self {
      TransactionUpdate::Drained => "drained",
      TransactionUpdate::Error(_) => "error",
      TransactionUpdate::Timeout(_) => "timeout",
      TransactionUpdate::Transition(_) => "transition",
      TransactionUpdate::Execute(_) => "execute",
      TransactionUpdate::Result(_) => "result",
      TransactionUpdate::Done(_) => "done",
      TransactionUpdate::Update(_) => "update",
      TransactionUpdate::NoOp => "noop",
    };
    f.write_str(name)
  }
}

#[derive(Debug)]
struct Transaction {
  tx_id: String,
  ports: PortStatuses,
  output_ports: Vec<ConnectionTargetDefinition>,
  schematic_name: String,
}

impl Transaction {
  fn new<T: AsRef<str>>(tx_id: T, model: &SharedModel) -> Self {
    let ports = PortStatuses::new(&tx_id, model);
    let readable = model.read();

    let schematic_name = readable.get_name();
    let output_ports = readable.get_schematic_outputs().cloned().collect();
    drop(readable);
    Self {
      tx_id: tx_id.as_ref().to_owned(),
      ports,
      output_ports,
      schematic_name,
    }
  }

  pub(crate) fn log_prefix(&self) -> String {
    format!("TX:{}({}):", self.tx_id, self.schematic_name)
  }

  fn is_done(&self) -> bool {
    for port in &self.output_ports {
      if !self.ports.is_closed(port) {
        return false;
      }
    }
    true
  }
}

#[cfg(test)]
mod tests {
  use std::sync::Arc;
  use std::time::Duration;

  use parking_lot::RwLock;
  use vino_packet::packet::v0::Payload;
  use vino_packet::Packet;

  use super::*;
  use crate::schematic_service::input_message::InputMessage;
  #[allow(unused_imports)]
  use crate::test::prelude::{assert_eq, *};
  use crate::transaction::executor::TransactionExecutor;

  static REF_ID: &str = "REF_ID_LOGGER";

  fn make_model() -> TestResult<Arc<RwLock<SchematicModel>>> {
    let schematic_name = "Test";
    let mut schematic_def = new_schematic(schematic_name);
    schematic_def.providers.push("test-namespace".to_owned());
    schematic_def
      .instances
      .insert(REF_ID.to_owned(), ComponentDefinition::new("test-namespace", "log"));
    schematic_def
      .connections
      .push(ConnectionDefinition::from_v0_str(&format!("<>=>{}[input]", REF_ID))?);
    schematic_def
      .connections
      .push(ConnectionDefinition::from_v0_str(&format!("{}[output]=><>", REF_ID))?);
    Ok(Arc::new(RwLock::new(SchematicModel::try_from(schematic_def)?)))
  }

  #[test_logger::test]
  fn test_transaction() -> TestResult<()> {
    let tx_id = "some tx";
    let model = make_model()?;

    let mut transaction = Transaction::new(tx_id, &model);
    let from = ConnectionTargetDefinition::new(SCHEMATIC_INPUT, "input");
    let to = ConnectionTargetDefinition::new(REF_ID, "input");

    println!("pushing to port");
    let connection = ConnectionDefinition::new(from, to.clone());
    transaction
      .ports
      .receive(&connection, Packet::V0(Payload::MessagePack(vec![])).into());
    assert!(transaction.ports.is_port_ready(&to));
    println!("taking from port");
    let output = transaction.ports.take_from_port(&to);
    assert_eq!(output, Some(MessageTransport::Success(Success::MessagePack(vec![]))));
    transaction
      .ports
      .receive(&connection, Packet::V0(Payload::Exception("!!".into())).into());
    let output = transaction.ports.take_from_port(&to);
    assert!(matches!(output, Some(MessageTransport::Failure(Failure::Exception(_)))));

    Ok(())
  }

  fn conn(from_name: &str, from_port: &str, to_name: &str, to_port: &str) -> ConnectionDefinition {
    ConnectionDefinition {
      from: ConnectionTargetDefinition::new(from_name, from_port),
      to: ConnectionTargetDefinition::new(to_name, to_port),
      default: None,
    }
  }

  #[test_logger::test(tokio::test)]
  async fn test_transaction_map() -> TestResult<()> {
    let model = make_model()?;

    let mut map = TransactionExecutor::new(model, Duration::from_millis(100));
    let tx_id = "some tx".to_owned();
    let (mut ready_rx, tx) = map.new_transaction(&tx_id);

    // First message sends from the schematic input to the component
    tx.send(TransactionUpdate::Update(InputMessage {
      connection: conn(SCHEMATIC_INPUT, "input", REF_ID, "input"),
      payload: MessageTransport::success(&"input payload"),
      tx_id: tx_id.clone(),
    }))?;

    // Second closes the schematic input
    tx.send(TransactionUpdate::Update(InputMessage {
      connection: conn(SCHEMATIC_INPUT, "input", REF_ID, "input"),
      payload: MessageTransport::Signal(MessageSignal::Done),
      tx_id: tx_id.clone(),
    }))?;

    // Third simulates output from the component
    tx.send(TransactionUpdate::Update(InputMessage {
      connection: conn(REF_ID, "output", SCHEMATIC_OUTPUT, "output"),
      payload: MessageTransport::success(&"output payload"),
      tx_id: tx_id.clone(),
    }))?;

    // Second closes the schematic input
    tx.send(TransactionUpdate::Update(InputMessage {
      connection: conn(REF_ID, "output", SCHEMATIC_OUTPUT, "output"),
      payload: MessageTransport::Signal(MessageSignal::Done),
      tx_id: tx_id.clone(),
    }))?;

    // Transaction should close automatically after this because the schematic
    // is complete

    let handle = tokio::spawn(async move {
      let mut msgs = vec![];
      while let Some(payloadmsg) = ready_rx.recv().await {
        println!("Got message : {:?}", payloadmsg);
        msgs.push(payloadmsg);
      }
      msgs
    });
    let msgs = handle.await?;
    println!("Transaction Updates {:#?}", msgs);

    // 1 execute the component
    assert!(matches!(msgs[0], TransactionUpdate::Execute(_)));
    // 2 get result for schematic
    assert!(matches!(msgs[1], TransactionUpdate::Result(_)));
    // 3 get done signal for schematic port
    assert!(matches!(msgs[2], TransactionUpdate::Result(_)));
    // 4 get done update for schematic transaction
    assert!(matches!(msgs[3], TransactionUpdate::Done(_)));
    assert_eq!(msgs.len(), 4);

    Ok(())
  }

  // TODO: Bad test: either delete or figure out what it really needs
  // to test
  //
  // #[test_logger::test(tokio::test)]
  // async fn test_invalid_message() -> TestResult<()> {
  //   let model = make_model()?;

  //   let mut map = TransactionExecutor::new(model, Duration::from_millis(100));
  //   let tx_id = get_uuid();
  //   let (mut ready_rx, tx) = map.new_transaction(tx_id.clone());

  //   // First message sends from the schematic input to the component
  //   tx.send(TransactionUpdate::Update(InputMessage {
  //     connection: conn(SCHEMATIC_INPUT, "input", REF_ID, "input"),
  //     payload: MessageTransport::Failure(Failure::Invalid),
  //     tx_id: tx_id.clone(),
  //   }))?;

  //   // Second closes the schematic input
  //   tx.send(TransactionUpdate::Update(InputMessage {
  //     connection: conn(SCHEMATIC_INPUT, "input", REF_ID, "input"),
  //     payload: MessageTransport::Signal(MessageSignal::Done),
  //     tx_id: tx_id.clone(),
  //   }))?;

  //   // Transaction should close automatically after this because the schematic
  //   // is complete

  //   let handle = tokio::spawn(async move {
  //     let mut msgs = vec![];
  //     while let Some(payloadmsg) = ready_rx.recv().await {
  //       println!("Got message : {:?}", payloadmsg);
  //       msgs.push(payloadmsg);
  //     }
  //     msgs
  //   });
  //   let msgs = handle.await?;
  //   println!("Transaction Updates {:#?}", msgs);

  //   // 1 execute the component
  //   assert!(matches!(msgs[0], TransactionUpdate::Execute(_)));
  //   // 2 get result for schematic
  //   assert!(matches!(msgs[1], TransactionUpdate::Result(_)));
  //   // 3 get done signal for schematic port
  //   assert!(matches!(msgs[2], TransactionUpdate::Result(_)));
  //   // 4 get done update for schematic transaction
  //   assert!(matches!(msgs[3], TransactionUpdate::Done(_)));
  //   assert_eq!(msgs.len(), 4);

  //   Ok(())
  // }
}
