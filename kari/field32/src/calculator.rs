use crate::field32::*;
use imgui::Ui;

pub struct Calculator {
    input: String,
    result: String,
    history: Vec<String>,
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            input: String::new(),
            result: String::from("0"),
            history: Vec::new(),
        }
    }

    pub fn render(&mut self, ui: &Ui) {
        // メインウィンドウ
        ui.window("Field32 Calculator")
            .size([460.0, 300.0], imgui::Condition::FirstUseEver)
            .position([10.0, 10.0], imgui::Condition::FirstUseEver)
            .build(|| {
                // ディスプレイエリア
                ui.text("Input:");
                ui.same_line();
                ui.text(&self.input);

                ui.separator();

                ui.text("Result:");
                ui.same_line();
                ui.text(&self.result);

                ui.separator();

                // ボタンレイアウト
                let button_size = [50.0, 40.0];

                // 数字ボタン (1-9)
                for row in 0..3 {
                    for col in 0..3 {
                        let num = row * 3 + col + 1;
                        if ui.button_with_size(&format!("{}", num), button_size) {
                            self.input.push_str(&format!("{}", num));
                        }
                        if col < 2 {
                            ui.same_line();
                        }
                    }
                }

                // 0ボタン
                if ui.button_with_size("0", button_size) {
                    self.input.push_str("0");
                }

                ui.separator();

                // 演算子ボタン
                if ui.button_with_size("+", button_size) {
                    self.input.push_str(" + ");
                }
                ui.same_line();
                if ui.button_with_size("*", button_size) {
                    self.input.push_str(" * ");
                }
                ui.same_line();
                if ui.button_with_size("^-1", button_size) {
                    self.input.push_str("^-1");
                }

                ui.separator();

                // 制御ボタン
                if ui.button_with_size("Clear", [100.0, 40.0]) {
                    self.input.clear();
                    self.result = String::from("0");
                }
                ui.same_line();
                if ui.button_with_size("Calculate", [100.0, 40.0]) {
                    self.calculate();
                }

                ui.separator();

                // 履歴表示
                ui.text("History:");
                for h in self.history.iter().rev().take(5) {
                    ui.text(h);
                }
            });
    }

    fn calculate(&mut self) {
        match self.parse_and_evaluate_with_steps(&self.input) {
            Ok((result, steps)) => {
                let poly = takousiki(result);
                self.result = format!("{} (0x{:08X}, {})", result.value, result.value, poly);

                // 入力式を履歴に追加
                self.history.push(format!("Input: {}", self.input));

                // 途中式を履歴に追加
                for step in steps {
                    self.history.push(format!("  {}", step));
                }

                // 最終結果を履歴に追加
                self.history.push(format!("Result: {}", self.result));
                self.history.push(String::new()); // 空行で区切り
            }
            Err(e) => {
                self.result = format!("Error: {}", e);
            }
        }
    }

    fn parse_and_evaluate_with_steps(&self, expr: &str) -> Result<(MyNumber, Vec<String>), String> {
        let expr = expr.trim();
        let mut steps = Vec::new();

        // 逆元演算子をチェック
        if expr.ends_with("^-1") {
            let value_str = expr.trim_end_matches("^-1").trim();
            let value = self.parse_hex(value_str)?;
            let num = create_number(value);

            steps.push(format!("{} = {}", value, takousiki(num)));
            let result = inverse(num)?;
            steps.push(format!("{}^-1 = {}", takousiki(num), takousiki(result)));

            return Ok((result, steps));
        }

        // 乗算をチェック
        if expr.contains('*') {
            let parts: Vec<&str> = expr.split('*').collect();
            if parts.len() != 2 {
                return Err("Invalid multiplication expression".to_string());
            }
            let a_val = self.parse_hex(parts[0].trim())?;
            let b_val = self.parse_hex(parts[1].trim())?;
            let a = create_number(a_val);
            let b = create_number(b_val);

            steps.push(format!("{} = {}", a_val, takousiki(a)));
            steps.push(format!("{} = {}", b_val, takousiki(b)));

            let result = a * b;
            steps.push(format!("({}) * ({}) = {}", takousiki(a), takousiki(b), takousiki(result)));

            return Ok((result, steps));
        }

        // 加算をチェック (XOR)
        if expr.contains('+') {
            let parts: Vec<&str> = expr.split('+').collect();
            if parts.len() != 2 {
                return Err("Invalid addition expression".to_string());
            }
            let a_val = self.parse_hex(parts[0].trim())?;
            let b_val = self.parse_hex(parts[1].trim())?;
            let a = create_number(a_val);
            let b = create_number(b_val);

            steps.push(format!("{} = {}", a_val, takousiki(a)));
            steps.push(format!("{} = {}", b_val, takousiki(b)));

            let result = create_number(a_val ^ b_val);
            steps.push(format!("({}) + ({}) = {}", takousiki(a), takousiki(b), takousiki(result)));

            return Ok((result, steps));
        }

        // 単一の値
        let value = self.parse_hex(expr)?;
        let num = create_number(value);
        steps.push(format!("{} = {}", value, takousiki(num)));
        Ok((num, steps))
    }

    fn parse_hex(&self, s: &str) -> Result<u32, String> {
        let s = s.trim();
        if s.is_empty() {
            return Err("Empty value".to_string());
        }

        s.parse::<u32>()
            .map_err(|_| format!("Invalid decimal value: {}", s))
    }
}
