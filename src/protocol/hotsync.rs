use super::slp::{SLPTransportTrait, SLP};
use super::padp::PADP;
use super::cmp_dlp::{CMPDLP, DLPVersion};
use super::usb::USB;

pub struct Hotsync<'a> {
    transport: Option<&'a dyn SLPTransportTrait>,
    slp_handler: Option<&'a SLP>,
    padp_handler: Option<&'a PADP<'a>>,
    cmp_dlp_handler: Option<&'a CMPDLP<'a>>,
    usb_handler: Option<&'a USB>
}

impl<'a> Hotsync<'a> {
    const DLP_VERSION: DLPVersion = DLPVersion {
        major_version: 1,
        minor_version: 3
    };
}

/// Construct a new `HotSync` object with the specified `CMPDLP` object.
/// 
/// # Parameters
/// 
/// * `cmp_dlp`: The `CMPDLP` handler to use for I/O.
///
/// # Return
/// 
/// The `Hotsync` struct with `CMPDLP`
pub fn new_hotsync_with_cmpdlp<'a>(cmp_dlp: &'a CMPDLP<'a>) -> Hotsync {
    Hotsync {
        transport: None,
        slp_handler: None,
        padp_handler: None,
        cmp_dlp_handler: Some(cmp_dlp),
        usb_handler: None
    }
}

/// Construct a new `HotSync` object using the provided `SLPTransportTrait`-typed object.
/// 
/// This function will create all of the necessary protocol objects, and connect them
/// so they're ready for a synchronization session.
/// 
/// # Parameters
/// 
/// * `transport`: The `SLPTransportTrait`-typed object to use for synchronization.
///
/// # Return
/// 
/// The `Hotsync` struct with transport class
pub fn new_hotsync_with_transport<'a>(transport: &'a dyn SLPTransportTrait) -> Hotsync {
    // TODO: Create transport class -- Serial
    // TODO: Create transport class -- USB

    Hotsync {
        transport: Some(transport),
        slp_handler: None,
        padp_handler: None,
        cmp_dlp_handler: None,
        usb_handler: None
    }
}