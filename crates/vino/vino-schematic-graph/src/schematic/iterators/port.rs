use super::*;
use crate::component::ComponentPort;
use crate::port::PortDirection;
use crate::{ComponentIndex, PortReference};

#[derive(Debug, Clone)]
#[must_use]
pub struct Port<'graph, DATA> {
  schematic: &'graph Schematic<DATA>,
  pub(super) port: PortReference,
}

impl<'graph, DATA> Port<'graph, DATA>
where
  DATA: Clone,
{
  pub fn new(schematic: &'graph Schematic<DATA>, port: PortReference) -> Self {
    Self { schematic, port }
  }

  pub fn component(&self) -> ComponentHop<DATA> {
    ComponentHop::new(self.schematic, self.port.component_index)
  }

  pub fn connections(&self) -> Connections<DATA> {
    get_port_connections(self.schematic, &self.port)
  }

  #[must_use]
  pub fn name(&self) -> &str {
    get_port_name(self.schematic, &self.port)
  }

  pub fn inner(&self) -> &ComponentPort {
    get_ports_component(self.schematic, &self.port)
  }

  pub fn direction(&self) -> PortDirection {
    self.port.direction
  }
}

impl<'graph, DATA> AsRef<PortReference> for Port<'graph, DATA> {
  fn as_ref(&self) -> &PortReference {
    &self.port
  }
}

impl<'graph, DATA> std::fmt::Display for Port<'graph, DATA>
where
  DATA: Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", display(self.schematic, &self.port))
  }
}

#[derive(Debug, Clone)]
#[must_use]
pub struct Ports<'graph, DATA> {
  pub(super) direction: Option<PortDirection>,
  pub(super) ports: Vec<PortReference>,
  pub(super) cur_index: usize,
  pub(super) component_index: ComponentIndex,
  schematic: &'graph Schematic<DATA>,
}

impl<'graph, DATA> Ports<'graph, DATA> {
  pub(crate) fn new(
    schematic: &'graph Schematic<DATA>,
    component_index: ComponentIndex,
    ports: Vec<PortReference>,
  ) -> Self {
    Self {
      direction: ports.get(0).map(|p| *p.direction()),
      ports,
      cur_index: 0,
      component_index,
      schematic,
    }
  }

  #[must_use]
  pub fn len(&self) -> usize {
    self.ports.len()
  }

  #[must_use]
  pub fn is_empty(&self) -> bool {
    self.ports.is_empty()
  }
}

impl<'graph, DATA> Iterator for Ports<'graph, DATA>
where
  DATA: Clone,
{
  type Item = Port<'graph, DATA>;

  fn next(&mut self) -> Option<Port<'graph, DATA>> {
    let result = self
      .ports
      .get(self.cur_index)
      .map(|index| Port::new(self.schematic, *index));
    self.cur_index += 1;
    result
  }
}

fn display<DATA>(schematic: &Schematic<DATA>, port: &PortReference) -> String
where
  DATA: Clone,
{
  let component = &schematic.components[port.component_index];
  let (port, dir) = match port.direction {
    PortDirection::In => (&component.inputs()[port.port_index], "IN"),
    PortDirection::Out => (&component.outputs()[port.port_index], "OUT"),
  };
  format!("{}.{}.{}", component, dir, port)
}

impl<'graph, DATA> std::fmt::Display for Ports<'graph, DATA>
where
  DATA: Clone,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    for (index, port) in self.ports.iter().enumerate() {
      let comma = if index < (self.ports.len() - 1) { ", " } else { "" };
      if index == self.cur_index {
        write!(f, ">>>{}<<<{}", display(self.schematic, port), comma)?;
      } else {
        write!(f, "{}{}", display(self.schematic, port), comma)?;
      }
    }
    if self.cur_index >= self.ports.len() {
      write!(f, ", >>>DONE<<<")?;
    }
    Ok(())
  }
}
