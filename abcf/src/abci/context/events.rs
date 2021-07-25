use crate::module::Event;
use alloc::vec::Vec;
use tm_protos::abci;

pub struct EventContext<'a> {
    events: &'a mut Vec<abci::Event>,
}

impl<'a> EventContext<'a> {
    pub fn new(events: &'a mut Vec<abci::Event>) -> Self {
        EventContext { events }
    }

    pub fn emmit(&mut self, _event: impl Event) {
        self.events.push(abci::Event::default())
    }
}

pub struct EventContextImpl {
    pub begin_block_events: Vec<abci::Event>,
    pub check_tx_events: Vec<abci::Event>,
    pub deliver_tx_events: Vec<abci::Event>,
    pub end_block_events: Vec<abci::Event>,
}

impl Default for EventContextImpl {
    fn default() -> Self {
        Self {
            begin_block_events: Vec::new(),
            check_tx_events: Vec::new(),
            deliver_tx_events: Vec::new(),
            end_block_events: Vec::new(),
        }
    }
}