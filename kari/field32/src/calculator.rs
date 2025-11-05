pub struct Calculator {
    pub opened: bool,
    display: Option<String>,
    error_message: Option<String>,
}

impl Calculator {
    pub fn new() -> Self {
         Calculator {
            opened: true,
            display: None,
            error_message: None,
        }
    }

    

    pub fn render(&mut self, ui: &imgui::Ui) {
        ui.window("Calculator")
            .size([1000.0, 800.0], imgui::Condition::FirstUseEver)
            .opened(&mut self.opened)
            .build(|| {
                const BTN_SIZE: [f32; 2] = [80.0, 60.0];

                //表示エリアを追加
                let mut display_text = self.display.clone().unwrap_or_else(|| String::from("0"));
                ui.input_text_multiline("##display", &mut display_text, [400.0, 100.0])
                    .read_only(true)
                    .build();

                //ディスプレイの下にエラーメッセージを表示
                if let Some(ref error) = self.error_message {
                    ui.text_colored([1.0, 0.0, 0.0, 1.0], error);
                }



                //ボタンを作成
                if ui.button_with_size("7", BTN_SIZE) {
                    //7ボタンが押されたときの処理
                    println!("7ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("8", BTN_SIZE) {
                    //8ボタンが押されたときの処理
                    println!("8ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("9", BTN_SIZE) {
                    //9ボタンが押されたときの処理
                    println!("9ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("a^-1", BTN_SIZE) {
                    //逆元ボタンが押されたときの処理
                    println!("逆元ボタンが押されました！");
                }
                if ui.button_with_size("4", BTN_SIZE) {
                    //4ボタンが押されたときの処理
                    println!("4ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("5", BTN_SIZE) {
                    //5ボタンが押されたときの処理
                    println!("5ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("6", BTN_SIZE) {
                    //6ボタンが押されたときの処理
                    println!("6ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("*", BTN_SIZE) {
                    //乗算ボタンが押されたときの処理
                    println!("乗算ボタンが押されました！");
                }
                if ui.button_with_size("1", BTN_SIZE) {
                    //1ボタンが押されたときの処理
                    println!("1ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("2", BTN_SIZE) {
                    //2ボタンが押されたときの処理
                    println!("2ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("3", BTN_SIZE) {
                    //3ボタンが押されたときの処理
                    println!("3ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("-", BTN_SIZE) {
                    //減算ボタンが押されたときの処理
                    println!("減算ボタンが押されました！");
                }
                if ui.button_with_size("C", BTN_SIZE) {
                    //クリアボタンが押されたときの処理
                    println!("クリアボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("0", BTN_SIZE) {
                    //0ボタンが押されたときの処理
                    println!("0ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("=", BTN_SIZE) {
                    //イコールボタンが押されたときの処理
                    println!("イコールボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("+", BTN_SIZE) {
                    //加算ボタンが押されたときの処理
                    println!("加算ボタンが押されました！");
                }
            }
        );
    }
}

