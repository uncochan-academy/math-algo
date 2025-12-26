//距離系
const int trigPin = 9;
//割り込み用
const int echoPin = 2;
const int buzzerPin = 6;

//表示系
const int latchPin = 10;
const int clockPin = 13;
const int dataPin = 11;
const int digitPins[] = {8, 3, 4, 5}; // 12, 9, 8, 6番ピン

//0123456789消灯
const byte numPatterns[] = {
  0x3F, 0x06, 0x5B, 0x4F, 0x66, 0x6D, 0x7D, 0x07, 0x7F, 0x6F, 0x00
};

volatile unsigned long echoStartTime = 0;
volatile unsigned long echoDuration = 0;
volatile bool newDistanceAvailable = false;



unsigned long lastBuzzerStartTime = 0; 

unsigned long lastSensorTriggerTime = 0;
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

  // 割り込みの設定
  // echoPin(2番)の電圧が変わるたび(CHANGE)、echoISRという関数を自動実行する
  attachInterrupt(digitalPinToInterrupt(echoPin), echoISR, CHANGE);
  
  Serial.println("System Ready");
}

void loop() {
  unsigned long currentMillis = millis();

  if (currentMillis - lastSensorTriggerTime >= sensorInterval) {
    lastSensorTriggerTime = currentMillis;
    
    // 超音波を発射
    digitalWrite(trigPin, LOW);
    delayMicroseconds(2);
    digitalWrite(trigPin, HIGH);
    delayMicroseconds(10);
    digitalWrite(trigPin, LOW);
    
    // 超音波が返ってくるのを待たずに次の処理へ進む
    // 受信の計算は echoISR 関数
  }

  // 割り込みで計算されたデータを取得
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

  // ブザー制御セクション
  // 100cm以内に入ったら鳴らし始める
  if (currentDistance > 0 && currentDistance < 100) {
    
    // 距離2cm〜100cmに対して、間隔を50ms〜800msへ変化
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