//距離系
const int trigPin = 9;
const int echoPin = 2;
const int buzzerPin = 6;

//表示系
const int latchPin = 10;
const int clockPin = 13;
const int dataPin = 11;
const int digitPins[] = {8, 3, 4, 5}; // 12, 9, 8, 6番ピン


const byte numPatterns[] = {
  0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x07, 0x7F, 0x6F, 0x00
};

volatile unsigned long echoStartTime = 0;
volatile unsigned long echoDuration = 0;
volatile bool newDistanceAvailable = false;



unsigned long lastBuzzerStartTime = 0; 

unsigned long lastSensorTriggerTime = 0;
// unsigned long lastSensorTime = 0;
const int sensorInterval = 150; // 計測を0.15秒おきにして安定させる

int currentDistance = 0;
bool isBuzzerOn = false;

void setup() {
  pinMode(latchPin, OUTPUT);
  pinMode(clockPin, OUTPUT);
  pinMode(dataPin, OUTPUT);
  for (int i = 0; i < 4; i++) {
    pinMode(digitPins[i], OUTPUT);
    digitalWrite(digitPins[i], HIGH); // 消灯
  }






  pinMode(trigPin, OUTPUT);
  pinMode(echoPin, INPUT);
  pinMode(buzzerPin, OUTPUT);
  Serial.begin(9600);
  
  // 起動時にブザーを一瞬鳴らしてテスト
  digitalWrite(buzzerPin, HIGH);
  delay(20);
  digitalWrite(buzzerPin, LOW);

  // 【重要】割り込みの設定
  // echoPin(2番)の電圧が変わるたび(CHANGE)、echoISRという関数を自動実行する
  attachInterrupt(digitalPinToInterrupt(echoPin), echoISR, CHANGE);
  
  Serial.println("System Ready");
}

void loop() {
  unsigned long currentMillis = millis();

  if (currentMillis - lastSensorTriggerTime >= sensorInterval) {
    lastSensorTriggerTime = currentMillis;
    
    // 超音波を発射（これは一瞬なので処理を止めない）
    digitalWrite(trigPin, LOW);
    delayMicroseconds(2);
    digitalWrite(trigPin, HIGH);
    delayMicroseconds(10);
    digitalWrite(trigPin, LOW);
    
    // ※ここで「受信待ち」はしません！すぐに下の処理へ進みます。
    // 受信の計算は、一番下の echoISR 関数が勝手にやってくれます。
  }

  // --- 2. 割り込みで計算された距離を取り込む ---
  if (newDistanceAvailable) {
    noInterrupts(); // データ読み取り中に割り込みが入らないよう一時停止
    unsigned long d = echoDuration;
    newDistanceAvailable = false;
    interrupts(); // 再開
    
    // 計算
    int dist = d * 0.034 / 2;
    if (dist > 400 || dist <= 0) dist = 999; // 範囲外処理
    currentDistance = dist;
  }

  /*
  // --- 1. センサー計測セクション ---
  // ブザーが鳴っていない時だけ計測し、干渉を防ぐ
  if (!isBuzzerOn && (currentMillis - lastSensorTime >= sensorInterval)) {
    lastSensorTime = currentMillis;

    // 確実にリセットをかける
    digitalWrite(trigPin, LOW);
    delayMicroseconds(5); 
    digitalWrite(trigPin, HIGH);
    delayMicroseconds(10);
    digitalWrite(trigPin, LOW);

    // タイムアウトを少し長めに(30ms)設定して遠距離に対応
    long duration = pulseIn(echoPin, HIGH, 30000); 

    if (duration > 0) {
      currentDistance = duration * 0.034 / 2;
      // 異常な値（0や大きすぎる値）をフィルタリング
      if (currentDistance > 400) currentDistance = 999;
    } else {
      currentDistance = 999; // 反射がないときは「遠くにいる」とみなす
    }

    

    // デバッグ確認
    Serial.print("D: ");
    Serial.print(currentDistance);
    Serial.println(" cm");
  }
*/



  // --- 2. ブザー制御セクション (線形的インターバル) ---
  // 100cm以内に入ったら鳴らし始める
  if (currentDistance > 0 && currentDistance < 100) {
    
    // 距離2cm〜100cmに対して、間隔を50ms〜800msへ線形変化
    int interval = map(currentDistance, 2, 100, 50, 800);

    if (!isBuzzerOn) {
      // 待ち時間が経過したら音を出す
      if (currentMillis - lastBuzzerStartTime >= interval) {
        digitalWrite(buzzerPin, HIGH);
        lastBuzzerStartTime = currentMillis;
        isBuzzerOn = true;
      }
    } else {
      // 音を出して20ms経ったら止める
      if (currentMillis - lastBuzzerStartTime >= 20) {
        digitalWrite(buzzerPin, LOW);
        isBuzzerOn = false;
      }
    }
  } else {
    // 範囲外なら音を消す
    digitalWrite(buzzerPin, LOW);
    isBuzzerOn = false;
  }

  displayNumber(currentDistance);
}

/*
void displayNumber(int num) {
  int d[4];
  d[0] = (num / 1000) % 10; // 千の位
  d[1] = (num / 100) % 10;  // 百の位
  d[2] = (num / 10) % 10;   // 十の位
  d[3] = num % 10;          // 一の位

  for (int i = 0; i < 4; i++) {



    // 74HC595にデータを送る
    digitalWrite(latchPin, LOW);
    shiftOut(dataPin, clockPin, MSBFIRST, numPatterns[d[i]]);
    digitalWrite(latchPin, HIGH);

    // 桁をONにする
    digitalWrite(digitPins[i], LOW);
    
    // 点灯時間（ここが短いと暗くなり、長いとチラつきます）
    delay(2);
    // 桁をOFFにする
    digitalWrite(digitPins[i], HIGH);
  }
}

*/

void echoISR() {
  // ピンの状態を読む
  if (digitalRead(echoPin) == HIGH) {
    // HIGHになった瞬間（音が返ってきたスタート）
    echoStartTime = micros();
  } else {
    // LOWになった瞬間（音が終わり）
    // 差分を計算して保存
    echoDuration = micros() - echoStartTime;
    newDistanceAvailable = true;
  }
}


void displayNumber(int num) {
  int d[4];
  
  // 各桁の数値を計算
  d[0] = (num / 1000) % 10;
  d[1] = (num / 100) % 10;
  d[2] = (num / 10) % 10;
  d[3] = num % 10;

  bool leadingZero = true; // 先頭ゼロフラグ

  for (int i = 0; i < 4; i++) {
    byte patternIndex = d[i];

    // 【ゼロサプレス処理】
    // 最下位桁(i=3)以外で、かつ「これまでずっと0」かつ「今の桁も0」なら消灯
    if (i < 3 && leadingZero && d[i] == 0) {
      patternIndex = 10; // 全消灯パターン(numPatternsの11番目)
    } else {
      leadingZero = false; // 0以外の数字が出たらフラグ解除
    }

    digitalWrite(latchPin, LOW);
    shiftOut(dataPin, clockPin, MSBFIRST, numPatterns[patternIndex]);
    digitalWrite(latchPin, HIGH);

    digitalWrite(digitPins[i], LOW); // 桁ON
    delay(1); 
    digitalWrite(digitPins[i], HIGH); // 桁OFF
  }
}