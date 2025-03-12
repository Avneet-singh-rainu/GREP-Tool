use grep::{MyGrep};
use FireFile::firefile;
mod grep;
mod FireFile;

fn main(){
    // let grep = MyGrep{};
    // grep.run();
    let firefiles = firefile{};
    firefiles.findfiles();

}
