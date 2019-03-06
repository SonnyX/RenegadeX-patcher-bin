extern crate renegadex_patcher;
extern crate ini;

use renegadex_patcher::{Downloader, Update};
use ini::Ini;

fn main() {
  let conf = match Ini::load_from_file("RenegadeX.ini") {
    Ok(conf) => conf,
    Err(_e) => {
      let mut conf = Ini::new();
      conf.with_section(Some("RenX_Patcher"))
        .set("GameLocation", "C:/Program Files (x86)/Renegade X/")
        .set("VersionUrl", "https://static.renegade-x.com/launcher_data/version/release.json");
      conf.write_to_file("RenegadeX.ini").unwrap();
      conf
    }
  };
  let section = conf.section(Some("RenX_Patcher".to_owned())).unwrap();
  let game_location = section.get("GameLocation").unwrap();
  let version_url = section.get("VersionUrl").unwrap();
  let mut patcher : Downloader = Downloader::new();
  patcher.set_location(game_location.to_string());
  patcher.set_version_url(version_url.to_string());
  patcher.retrieve_mirrors().unwrap();
  match patcher.update_available().unwrap() {
    Update::UpToDate => {
      println!("Game up to date, verifying game integrity!");
      patcher.poll_progress();
      patcher.download().unwrap();
    },
    Update::Resume | Update::Delta | Update::Full => {
      println!("Update available!");
      patcher.poll_progress();
      patcher.download().unwrap();
    }
  }
  assert!(true);
}
