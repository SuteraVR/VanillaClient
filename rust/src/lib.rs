pub mod world;
pub mod avatar;
pub mod error;
pub mod transform;
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
#[class(base=Button)]
struct StartButton{
    base: Base<Button>,
}

#[godot_api]
impl IButton for StartButton{
    fn init(base: Base<Button>) -> Self {
        Self {
            base,
        }
    }
    fn pressed(&mut self){
        godot_print!("==== Start SuteraVR! ====");
        let Some(mut scene_tree) = self.base_mut().get_tree() else{
            godot_print!("failed to get scene_tree.");
            return;
        };
        scene_tree.change_scene_to_file("res://sutera_vr.tscn".into());
    }
}

#[derive(GodotClass)]
#[class(base=Node3D)]
struct SuteraVR{
    base: Base<Node3D>,
}

#[godot_api]
impl INode3D for SuteraVR{
    fn init(base: Base<Node3D>) -> Self {
        Self {
            base,
        }
    }

    fn enter_tree(&mut self) {
        //------------tracing_subscriberの初期化処理-------------
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
        //------------初期化処理終了-------------
        let xr_server = XrServer::singleton();
        let Some(mut xr_interface) = xr_server.find_interface("OpenXR".into()) else{
            godot_print!("failed to get xr_interface.");
            return;
        };
        let _ = xr_interface.initialize();

        if xr_interface.is_initialized(){
            godot_print!("xr_interface is initialized.");
            let Some(mut viewport) = self.base_mut().get_viewport() else{
                godot_print!("failed to get viewport.");
                return;
            };
            viewport.set_use_xr(true);
        }
    }
}
