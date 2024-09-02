pub mod world;
use godot::classes::*;
use godot::prelude::*;
use tracing::instrument;
use tracing_error::ErrorLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

//create godot entrypoint
struct SuteraExtension;
#[gdextension]
unsafe impl ExtensionLibrary for SuteraExtension {}

//this node is interface among rust and Godot.
#[derive(GodotClass)]
#[class(base=Node)]
struct SuteraWorldLoader {
    once: bool,
    init_once: bool,
    base: Base<Node>,
}

#[godot_api]
impl INode for SuteraWorldLoader {
    #[instrument(skip_all, name = "init", level = "info")]
    fn init(base: Base<Node>) -> Self {
        let once = true;
        let init_once = true;
        godot_print!("test");
        Self {
            once,
            init_once,
            base,
        }
    }

    #[instrument(skip_all, name = "process", level = "info")]
    #[allow(unused_variables)]
    fn process(&mut self, delta: f64) {
        if self.init_once {
            //initが2回実行されてしまうため、初期化処理などはここで処理
            tracing_subscriber::Registry::default()
                .with(tracing_subscriber::fmt::layer()  //エラーメッセージを文字列に整形
                .with_file(true)    //ファイル名の表示有無
                .with_level(true)   //severityレベルの表示有無
                .with_line_number(true) //イベント発生行の表示有無
                .with_target(false) //イベント発生targetの表示有無
                .with_filter(tracing_subscriber::filter::LevelFilter::INFO))    //severityで表示するレベルを限定
                .with(ErrorLayer::default())
                .try_init()
                .expect("failed to initialize tracing_subscriber.");
            self.init_once = false;
        }
    }

    #[instrument(skip_all, name = "input", level = "info")]
    fn input(&mut self, event: Gd<InputEvent>) {
        if event.get_class() == GString::from("InputEventKey") && self.once {
            tracing::info!("received eventkey!");
            let yaml_path = String::from("../godot/models/world/world.yaml");
            match world::yaml_loader::load_world(yaml_path, &mut self.base_mut()) {
                Ok(()) => (),
                Err(e) => {
                    tracing::error!("{}", e.error);
                }
            }
            self.once = false;
        }
    }
}
