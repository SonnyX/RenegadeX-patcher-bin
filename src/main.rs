extern crate renegadex_patcher;
extern crate ini;

use std::error::Error;
use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming};
use renegadex_patcher::{Downloader, Update};
use ini::Ini;

fn main() -> Result<(), std::boxed::Box<dyn Error>> {
  Logger::try_with_env_or_str("info")?
    .format(flexi_logger::opt_format)
    .log_to_file(FileSpec::default().directory("logs"))
    .duplicate_to_stderr(Duplicate::Warn)
    .rotate(Criterion::Age(Age::Day), Naming::Numbers, Cleanup::KeepLogFiles(5))
    .print_message()
    .start()
    .unwrap_or_else(|e| panic!("Logger initialization failed with {}", e));

  let conf = match Ini::load_from_file("RenegadeX.ini") {
    Ok(conf) => conf,
    Err(_e) => {
      let mut conf = Ini::new();
      conf.with_section(Some("RenX_Patcher"))
        .set("GameLocation", "../")
        .set("VersionUrl", "https://static.ren-x.com/launcher_data/version/release.json");
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
  patcher.rank_mirrors().expect(concat!(file!(),":",line!()));
  match patcher.update_available().unwrap() {
    Update::UpToDate => {
      println!("Game up to date!");
      //patcher.poll_progress();
      //patcher.download().unwrap();
    },
    Update::Resume | Update::Delta | Update::Full | Update::Unknown => {
      println!("Update available!");
      patcher.poll_progress();
      patcher.download().unwrap();
    },
    _ => {

    }
  }
  Ok(())
}
