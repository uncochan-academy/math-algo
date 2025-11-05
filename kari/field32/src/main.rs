mod calculator;
use calculator::Calculator;

mod field32;
use field32::*;

//ウィンドウやイベント処理に使うものをインポート
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

//描画に使うものをインポート
use glium::{Display, Surface};

//UI部品に使うものをインポート
use imgui::{Context, Ui};

//imguiとgliumをつなぐためのモジュール
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};

//時間を扱うためのモジュール
use std::time::Instant;

fn main() {
    //ここに初期設定とイベントループのコードを書く
    let event_loop = EventLoop::new();
    let size = winit::dpi::LogicalSize::new(480.0, 320.0);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true);
    let builder = WindowBuilder::new()
        .with_title("Field32 calculator")
        .with_inner_size(size);
    let display = Display::new(builder, context, &event_loop)
        .expect("Failed to initialize display");
    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), display.gl_window().window(), HiDpiMode::Default);

    // 上付き文字を含むグリフ範囲を指定
    let glyph_ranges = imgui::FontGlyphRanges::from_slice(&[
        0x0020, 0x00FF, // 基本ラテン文字
        0x2070, 0x209F, // 上付き・下付き文字 (⁰¹²³⁴⁵⁶⁷⁸⁹など)
        0,
    ]);

    // フォントファイルを読み込んで追加
    let font_path = "C:\\Windows\\Fonts\\arial.ttf";
    match std::fs::read(font_path) {
        Ok(font_data) => {
            let font_data_static: &'static [u8] = Box::leak(font_data.into_boxed_slice());
            imgui.fonts().add_font(&[
                imgui::FontSource::TtfData {
                    data: font_data_static,
                    size_pixels: 16.0,
                    config: Some(imgui::FontConfig {
                        glyph_ranges,
                        ..Default::default()
                    }),
                }
            ]);
        }
        Err(_) => {
            // フォントファイルが見つからない場合はデフォルトフォント
            imgui.fonts().add_font(&[imgui::FontSource::DefaultFontData { config: None }]);
        }
    }
    let mut renderer = imgui_glium_renderer::Renderer::init(&mut imgui, &display)
        .expect("Failed to initialize renderer");
    let mut last_frame = Instant::now();
    let mut calculator = Calculator::new();
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested, ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                let ui = imgui.frame();
                //ui.show_demo_window(&mut true);

                calculator.render(&ui);

                let mut target = display.draw();
                target.clear_color_srgb(0.1, 0.1, 0.1, 1.0);
                
                

                
                
                platform.prepare_render(ui, display.gl_window().window());
                
                
                let draw_data = imgui.render();
                renderer.render(&mut target, draw_data)
                    .expect("Rendering failed");
                target.finish()
                    .expect("Failed to swap buffers");
            }


            Event::NewEvents(_) => {
                platform.prepare_frame(imgui.io_mut(), display.gl_window().window())
                    .expect("Failed to prepare frame");
                let now = Instant::now();
                imgui.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                display.gl_window().window().request_redraw();
            }
            event => {
                platform.handle_event(imgui.io_mut(), display.gl_window().window(), &event);
            }    
        }
    });

}