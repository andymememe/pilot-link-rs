use super::slp::{SLPTransportTrait, SLP};
use super::padp::PADP;
use super::cmp_dlp::{CMPDLP, DLPVersion, new_dlp_version_with_settings};
use super::usb::USB;

pub struct Hotsync<'a> {
    transport: Option<&'a dyn SLPTransportTrait>,
    slp_handler: Option<SLP>,
    padp_handler: Option<PADP<'a>>,
    cmp_dlp_handler: Option<CMPDLP<'a>>,
    usb_handler: Option<USB>
}

impl<'a> Hotsync<'a> {
    const DLP_VERSION: DLPVersion = DLPVersion {
        major_version: 1,
        minor_version: 3
    };
}


/// Construct a new HotSync object with the specified `CMPDLP` object.
/// # Parameters
/// 
/// * `cmp_dlp`: The CMP / DLP handler to use for I/O.
/// 
/// # Return
///
/// A `Hotsync` instance with CMP / DLP
pub fn new_hotsync_with_cmp_dlp(cmp_dlp: CMPDLP) -> Hotsync {
    Hotsync {
        transport: None,
        padp_handler: None,
        cmp_dlp_handler: Some(cmp_dlp),
        slp_handler: None,
        usb_handler: None,
    }
}


/// Construct a new HotSync object using the prov
/// 
/// This construct will create all of the necessary protocol objects, 
/// and connect them so they're ready for a synchronization session.
/// 
/// # Parameters
/// 
/// * `transport`: The transport class to use for synchronization.
/// 
/// # Return
///
/// A `Hotsync` instance with Tramsport
pub fn new_hotsync_with_transport<'a> (transport: &'a dyn SLPTransportTrait) -> Hotsync {
    Hotsync {
        transport: Some(transport),
        padp_handler: None,
        cmp_dlp_handler: None,
        slp_handler: None,
        usb_handler: None,
    }
}