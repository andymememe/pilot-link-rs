use super::slp::{SLPTransportTrait, SLP};
use super::padp::PADP;
use super::cmp_dlp::{CMPDLP, DLPVersion};
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