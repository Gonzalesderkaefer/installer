// Used modules
use super::{display::DspServer, distro::Distro, transfer::Transfer};
use std::io;




pub struct System<'a> {
    distro: Distro<'a>,
    display: DspServer,
    transfer: Transfer
}

impl<'a> System <'a> {
    pub fn get() -> io::Result<Self> {
        // Store distro
        let dist = Distro::get_distro();

        // Get the dsp server return an error if it
        // returns an error
        let dsp = {
            match DspServer::get(&dist) {
                Ok(dsp) => dsp,
                Err(e) => {
                    return Err(e);
                }
            }
        };


        // Get The transfermethod

        let tr = {
            match Transfer::get_transfer() {
                Ok(value) => value,
                Err(e) => {
                    return Err(e);
                }
            }
        };

        return Ok(System {
            distro: dist,
            display: dsp,
            transfer: tr,
        });
    }
}
