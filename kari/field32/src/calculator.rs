pub struct Calculator {
    pub opened: bool,
    display: String,
    current_input: String,
    stored_value: Option<u32>,
    stored_display: Option<String>,
    operator: Option<char>,
    error_message: Option<String>,
}

fn parse_input(input: &str) -> Result<u32, std::num::ParseIntError> {
    let trimmed = input.trim_start_matches('-');
    trimmed.parse::<u32>()
}



impl Calculator {
    pub fn new() -> Self {
         Calculator {
            opened: true,
            display: String::from("0"),
            current_input: String::new(),
            stored_value: None,
            stored_display: None,
            operator: None,
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
                ui.input_text_multiline("##display", &mut self.display, [400.0, 100.0])
                    .read_only(true)
                    .build();

                //ディスプレイの下にエラーメッセージを表示
                if let Some(ref error) = self.error_message {
                    ui.text_colored([1.0, 0.0, 0.0, 1.0], error);
                }



                //ボタンを作成
                if ui.button_with_size("7", BTN_SIZE) {
                    self.current_input.push_str("7");
                    self.display = self.current_input.clone();
                    //7ボタンが押されたときの処理
                    println!("7ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("8", BTN_SIZE) {
                    //8ボタンが押されたときの処理
                    self.current_input.push_str("8");
                    self.display = self.current_input.clone();                    
                    println!("8ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("9", BTN_SIZE) {
                    //9ボタンが押されたときの処理
                    self.current_input.push_str("9");
                    self.display = self.current_input.clone();
                    println!("9ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("a^-1", BTN_SIZE) {
                    //逆元ボタンが押されたときの処理
                    println!("逆元ボタンが押されました！");
                }
                if ui.button_with_size("4", BTN_SIZE) {
                    //4ボタンが押されたときの処理
                    self.current_input.push_str("4");
                    self.display = self.current_input.clone();
                    println!("4ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("5", BTN_SIZE) {
                    //5ボタンが押されたときの処理
                    self.current_input.push_str("5");
                    self.display = self.current_input.clone();
                    println!("5ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("6", BTN_SIZE) {
                    //6ボタンが押されたときの処理
                    self.current_input.push_str("6");
                    self.display = self.current_input.clone();
                    println!("6ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("*", BTN_SIZE) {
                    //乗算ボタンが押されたときの処理
                    println!("乗算ボタンが押されました！");
                }
                if ui.button_with_size("1", BTN_SIZE) {
                    //1ボタンが押されたときの処理
                    self.current_input.push_str("1");
                    self.display = self.current_input.clone();
                    println!("1ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("2", BTN_SIZE) {
                    //2ボタンが押されたときの処理
                    self.current_input.push_str("2");
                    self.display = self.current_input.clone();
                    println!("2ボタンが押されました！");
                }
                ui.same_line();
                if ui.button_with_size("3", BTN_SIZE) {
                    //3ボタンが押されたときの処理
                    self.current_input.push_str("3");
                    self.display = self.current_input.clone();
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
                    self.current_input.push_str("0");
                    self.display = self.current_input.clone();
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

