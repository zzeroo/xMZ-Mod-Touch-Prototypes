//! # xMZ-Mod-Touch-Prototypes
//! Sammlung verschiedenster Git Repos mit Prototypen die bei der Entwicklung der xMZ-Mod-Touch Plattform helfen sollen.
//!
//!
//! ## Structuren
//! ### Definition
//! * ein [Server](struct.Server.html) kann viele [Exception](struct.Exception.html) enthalten
//! * eine [Exception](struct.Exception.html) kann viele [Action](struct.Action.html) enthalten
//! * eine [Action](struct.Action.html) besteht aus einer mut Referenz zu einem [ShiftRegister](struct.ShiftRegister.html)
//! * eine [Action](struct.Action.html) besteht aus einer [ShiftRegister](struct.ShiftRegister.html) Funktion (set, clear)
//! * eine [Action](struct.Action.html) besteht aus einer Pin Nummer (u32)
//! * ein [Server](struct.Server.html) kann viele [Zone](struct.Zone.html) enthalten
//! * eine [Zone](struct.Zone.html) kann viele Kombisensor enthalten
//! * ein Kombisensor kann viele [Exception](struct.Exception.html) enthalten
//! * ein Kombisensor kann viele Sensor enthalten
//! * ein Sensor kann viele [Exception](struct.Exception.html) enthalten
//!
//! ### Initalisierung
//!
//! * der [Server](struct.Server.html) wird erzeugt
//! * eine [Exception](struct.Exception.html) Wartungsintervall erzeugt
//! * eine [Action](struct.Action.html) LED::set(3) wird erzeugt
//! * eine [Action](struct.Action.html) RELAIS::clear(1) wird erzeugt
//! * der [Exception](struct.Exception.html) Wartungsintervall wird die [Action](struct.Action.html) LED::set(3) hinzugefügt
//! * der [Exception](struct.Exception.html) Wartungsintervall wird die [Action](struct.Action.html) RELAIS::clear(1) hinzugefügt
//! * die [Exception](struct.Exception.html) Wartungsintervall wird dem [Server](struct.Server.html) hinzugefügt
//! * im [Server](struct.Server.html) wird eine [Zone](struct.Zone.html) (Zone1) erzeugt
//! * ein Kombisensor Kombisensor1 wird erzeugt
//! * eine [Exception](struct.Exception.html) Kabelbruch wird erzeugt
//! * eine [Action](struct.Action.html) LED::set(2) wird erzeugt
//! * eine [Action](struct.Action.html) RELAIS::clear(1) wird erzeugt
//! * der [Exception](struct.Exception.html) Kabelbruch wird die [Action](struct.Action.html) LED::set(2) hinzugefügt
//! * der [Exception](struct.Exception.html) Kabelbruch wird die [Action](struct.Action.html) RELAIS::clear(1) hinzugefügt
//! * die [Exception](struct.Exception.html) Kabelbruch wird dem Kombisensor hinzugefügt
//! * ein Sensor NO2 wird erzeugt
//! * ein Sensor CO wird erzeugt
//! * Kombisensor1 wird Sensor NO2 hinzugefügt
//! * Kombisensor1 wird Sensor CO hinzugefügt
//! * Zone1 wird Kombisensor1 hinzugefügt
//!
//! ```
//! Server {
//!   Exceptions [
//!     Exception::WartungsInterval {
//!       Actions [
//!         (&ShiftRegister::<led>, ShiftRegister::set(), 3),
//!         (&ShiftRegister::<relais>, ShiftRegister::clear(), 1)
//!       ]
//!     },
//!   ]
//!
//!   Zones [
//!     Zone {
//!       Kombisensors [
//!         Kombisensor {
//!           Exceptions [
//!             Exception::Kabelbruch {
//!               Actions [
//!                 (&ShiftRegister::<led>, ShiftRegister::set(), 2),
//!                 (&ShiftRegister::<relais>, ShiftRegister::clear(), 1)
//!               ]
//!             },
//!           ]
//!           Sensors {
//!
//!           }
//!         }
//!       }
//!     }
//!   ]
//! ```
extern crate time;

mod exception;
mod kombisensor;
mod sensor;
mod server;
mod shift_register;

pub use self::exception::{Exception, ExceptionType};
pub use self::kombisensor::Kombisensor;
pub use self::sensor::Sensor;
pub use self::server::Server;
pub use self::shift_register::{ShiftRegister, ShiftRegisterType};
