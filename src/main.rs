extern crate renegadex_patcher;
extern crate ini;
extern crate tracing_subscriber;

use std::{error::Error, sync::atomic::Ordering};
use tracing::{info, error};
use ini::Ini;
use tracing_subscriber::{prelude::__tracing_subscriber_SubscriberExt, util::SubscriberInitExt, Layer};

mod as_string;
mod version_information;

fn main() -> Result<(), Box<dyn Error + Send + Sync + 'static>> {

  let runtime : tokio::runtime::Runtime = tokio::runtime::Builder::new_multi_thread().enable_all().thread_stack_size(2_usize.pow(22)).build().expect("");
  let console_layer = console_subscriber::ConsoleLayer::builder().with_default_env().spawn();

  tracing_subscriber::registry()
    .with(console_layer)
    .with(tracing_subscriber::fmt::layer().with_filter(tracing_subscriber::filter::LevelFilter::from_level(tracing::Level::DEBUG)))
    .init();

  runtime.block_on(async move {
    let conf = match Ini::load_from_file("RenegadeX.ini") {
      Ok(conf) => conf,
      Err(_e) => {
        let mut conf = Ini::new();
        conf.with_section(Some("RenX_Patcher"))
          .set("GameLocation", "D:/Renegade X/")
          .set("VersionUrl", "https://static.ren-x.com/launcher_data/version/release.json");
        conf.write_to_file("RenegadeX.ini").unwrap();
        conf
      }
    };
    let section = conf.section(Some("RenX_Patcher".to_owned())).unwrap();
    let game_location = section.get("GameLocation").unwrap();
    let version_url = section.get("VersionUrl").unwrap();
  
    let mut patcher_builder = renegadex_patcher::PatcherBuilder::new();
    patcher_builder.set_software_location(game_location.to_string());
    let version_information = crate::version_information::VersionInformation::retrieve(version_url).await?;
    patcher_builder.set_software_information(version_information.software.mirrors, version_information.software.version, version_information.software.instructions_hash);
  
    patcher_builder.set_success_callback(Box::new(move || {
      info!("Calling download done");
    }));
    patcher_builder.set_failure_callback(Box::new(move |error| {
      error!("failure_callback {:#?}", &error);
    }));
    patcher_builder.set_progress_callback(Box::new(move |progress| {
      let json = format!(
        "{{\"action\": \"{}\",\"hash\": {{\"value\":{}, \"maximum\":{}}},\"download\": {{\"bytes\": {{\"value\":{}.0, \"maximum\":{}.0}}, \"files\": {{\"value\":{}, \"maximum\":{}}} }},\"patch\": {{\"value\":{}, \"ready\": {}, \"maximum\":{}}}}}",
        progress.get_current_action().unwrap(),
        progress.processed_instructions.0.load(Ordering::Relaxed),
        progress.processed_instructions.1.load(Ordering::Relaxed),
        0,
        progress.downloaded_bytes.1.load(Ordering::Relaxed),
        progress.downloaded_files.0.load(Ordering::Relaxed),
        progress.downloaded_files.1.load(Ordering::Relaxed),
        progress.patched_files.0.load(Ordering::Relaxed),
        progress.patched_files.1.load(Ordering::Relaxed),
        progress.patched_files.2.load(Ordering::Relaxed)
      );
      info!("{}", json);
    }));
  
    let mut patcher = patcher_builder.build()?;
  
    patcher.start_patching().await;

    tokio::signal::ctrl_c().await?;
    Ok::<(), Box<dyn Error + Send + Sync + 'static>>(())
  })
}
