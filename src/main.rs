extern crate lib_renegade_x_patcher;

use lib_renegade_x_patcher::Downloader;

fn main() {
/*
  let matches = App::new("RenegadeX downloader/patcher")
    .author("Author: Randy von der Weide")
    .arg(Arg::with_name("check")
      .short("c")
      .long("check")
      .help("Checks if game is installed or if an update is available")
    )
    .arg(Arg::with_name("update")
      .short("u")
      .long("update")
      .help("Downloads and installs update if available")
    )
    .arg(Arg::with_name("RENX_PATH")
      .help("The location where RenegadeX is installed or should be installed")
      .required(true)
      .index(1))
    .get_matches();
*/
  let mut patcher : Downloader = Downloader::new();
  patcher.RenegadeX_location = Some("C:/Program Files (x86)/Renegade X/".to_string());
  patcher.update();
}
