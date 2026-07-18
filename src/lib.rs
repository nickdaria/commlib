#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]

/// Direction of travel relative to the host
pub enum Direction {
    /// No associated direction
    None,

    /// Transmit
    Tx,

    /// Receive
    Rx,
}

/// Shared behavior for raw byte-backed protocol elements.
pub trait Raw<'a> {
    /// Direction relative to the host
    fn direction(&self) -> Direction;

    /// The data contained within
    fn as_bytes(&self) -> &'a [u8];

    /// The size in bytes
    fn size(&self) -> usize;
}

/// An individual packet within a message
pub trait Packet<'a>: Raw<'a> {}

/// A complete message that can be broken down into packets
pub trait Message<'a>: Raw<'a> {}

/// State machine that handles bidirectional packet exchange to send/receive messages
pub trait TransportProtocol<'a> {
    type Packet: Packet<'a>;
    type Message: Message<'a>;
    type Error;
    type State: Copy + core::fmt::Debug;

    fn packet_rx(&mut self, packet: Self::Packet) -> Result<(), Self::Error>;
    fn packet_tx(&mut self) -> Option<Self::Packet>;
    
    fn write(&mut self, message: Self::Message) -> Result<(), Self::Error>;
    fn poll(&mut self) -> Self::State;
}