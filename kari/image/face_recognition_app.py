"""
顔認識プログラム
顔の特徴を学習して名前をつけます
OpenCVのみを使用したシンプルな実装
"""

import cv2
import numpy as np
import os
import pickle
import sys


def get_cascade_path(filename):
    """HaarCascadeファイルのパスを取得（PyInstaller対応）"""
    if getattr(sys, 'frozen', False):
        # PyInstallerでパッケージ化されている場合
        base_path = sys._MEIPASS
        cascade_path = os.path.join(base_path, 'cv2', 'data', filename)
        if not os.path.exists(cascade_path):
            # フォールバック: OpenCVのデータディレクトリから直接取得
            cascade_path = cv2.data.haarcascades + filename
    else:
        # 通常のPython実行の場合
        cascade_path = cv2.data.haarcascades + filename
    return cascade_path


class SimpleFaceRecognizer:
    """シンプルな顔認識システム"""

    def __init__(self):
        cascade_path = get_cascade_path('haarcascade_frontalface_default.xml')
        self.face_cascade = cv2.CascadeClassifier(cascade_path)
        if self.face_cascade.empty():
            raise RuntimeError(f"Failed to load cascade classifier from {cascade_path}")
        self.recognizer = cv2.face.LBPHFaceRecognizer_create()
        self.known_faces = {}  # {name: [face_images]}
        self.is_trained = False
        self.label_to_name = {}
        self.name_to_label = {}
        self.data_file = "face_data.pkl"

    def add_face(self, name, face_image):
        """顔画像を追加"""
        if name not in self.known_faces:
            self.known_faces[name] = []
        self.known_faces[name].append(face_image)
        print(f"{name}の顔画像を追加しました（合計: {len(self.known_faces[name])}枚）")

    def train(self):
        """学習を実行"""
        if not self.known_faces:
            print("学習データがありません")
            return False

        faces = []
        labels = []

        # ラベルマッピングを作成
        self.name_to_label = {name: i for i, name in enumerate(self.known_faces.keys())}
        self.label_to_name = {i: name for name, i in self.name_to_label.items()}

        # 学習データを準備
        for name, face_images in self.known_faces.items():
            label = self.name_to_label[name]
            for face_img in face_images:
                faces.append(face_img)
                labels.append(label)

        # 学習実行
        self.recognizer.train(faces, np.array(labels))
        self.is_trained = True
        print(f"\n学習完了！ {len(self.known_faces)}人、合計{len(faces)}枚の顔画像で学習しました")
        return True

    def recognize(self, face_image):
        """顔を認識"""
        if not self.is_trained:
            return "Unknown", 0

        label, confidence = self.recognizer.predict(face_image)
        name = self.label_to_name.get(label, "Unknown")

        # 信頼度が低い場合はUnknown
        # LBPHでは低い値ほど一致度が高い（距離の概念）
        if confidence > 70:  # 閾値（調整可能）
            return "Unknown", confidence
        else:
            return name, confidence

    def save_data(self):
        """学習データを保存"""
        data = {
            'known_faces': self.known_faces,
            'name_to_label': self.name_to_label,
            'label_to_name': self.label_to_name,
            'is_trained': self.is_trained
        }
        with open(self.data_file, 'wb') as f:
            pickle.dump(data, f)

        if self.is_trained:
            self.recognizer.save("face_recognizer.yml")

        print(f"データを保存しました: {self.data_file}")

    def load_data(self):
        """学習データを読み込み"""
        if not os.path.exists(self.data_file):
            print("保存されたデータがありません")
            return False

        with open(self.data_file, 'rb') as f:
            data = pickle.load(f)

        self.known_faces = data.get('known_faces', {})
        self.name_to_label = data.get('name_to_label', {})
        self.label_to_name = data.get('label_to_name', {})
        self.is_trained = data.get('is_trained', False)

        if self.is_trained and os.path.exists("face_recognizer.yml"):
            self.recognizer.read("face_recognizer.yml")

        print(f"データを読み込みました: {len(self.known_faces)}人分")
        return True


def run_face_recognition():
    """顔認識アプリのメイン関数"""

    recognizer = SimpleFaceRecognizer()
    cap = cv2.VideoCapture(0)

    if not cap.isOpened():
        print("エラー: カメラを開けませんでした")
        return

    # 保存されたデータを読み込み
    recognizer.load_data()

    print("\n=== 顔認識プログラム（追跡機能付き） ===")
    print("操作方法:")
    print("  Escキー or qキー: 終了")
    print("  nキー: 新しい人を登録（名前入力→顔を5回撮影）")
    print("  tキー: 学習を実行")
    print("  sキー: データを保存")
    print("  rキー: 認識モードON/OFF切り替え")
    print("  cキー: すべてのデータをクリア")
    print("  kキー: 追跡モードON/OFF切り替え")
    print("  +/-キー: 認識状態の維持時間を調整")
    print("\n※一度認識すると約2秒間その状態を維持します")
    print("※追跡モードONで顔を見失っても位置を予測します")
    print("\nカメラ起動中...")

    recognition_mode = False
    tracking_mode = False
    screenshot_count = 0
    frame_counter = 0

    # 認識状態の維持システム
    recognized_name = "Unknown"
    last_recognition_frame = 0
    MAINTAIN_FRAMES = 60  # この数のフレーム（約2秒）認識状態を維持
    recognition_history = []  # 最近の認識結果を保存
    HISTORY_SIZE = 5  # 履歴のサイズ

    # 追跡システム
    tracker = None
    tracking_active = False
    face_bbox = None
    position_history = []
    MAX_HISTORY = 10
    last_detection_frame = 0
    PREDICTION_THRESHOLD = 60  # 約2秒
    last_valid_position = None
    MAX_MOVEMENT_PER_FRAME = 50

    def create_tracker():
        """CSRTトラッカーを作成"""
        return cv2.legacy.TrackerCSRT_create()

    def predict_next_position():
        """過去の位置から次の位置を予測"""
        if len(position_history) < 3:
            return None
        recent = position_history[-3:]
        velocities = []
        for i in range(len(recent) - 1):
            vx = recent[i+1][0] - recent[i][0]
            vy = recent[i+1][1] - recent[i][1]
            velocities.append((vx, vy))
        avg_vx = sum(v[0] for v in velocities) / len(velocities)
        avg_vy = sum(v[1] for v in velocities) / len(velocities)
        last_pos = position_history[-1]
        predicted_x = int(last_pos[0] + avg_vx)
        predicted_y = int(last_pos[1] + avg_vy)
        return (predicted_x, predicted_y)

    def is_reasonable_movement(old_pos, new_pos):
        """移動が現実的な範囲内かチェック"""
        if old_pos is None or new_pos is None:
            return True
        distance = np.sqrt((new_pos[0] - old_pos[0])**2 + (new_pos[1] - old_pos[1])**2)
        return distance <= MAX_MOVEMENT_PER_FRAME

    while True:
        ret, frame = cap.read()

        if not ret:
            print("エラー: フレームを読み込めませんでした")
            break

        frame = cv2.flip(frame, 1)
        gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
        frame_counter += 1

        result = frame.copy()

        face_detected_now = False
        tracking_success = False
        recognized_now = False

        # 顔検出を試みる（追跡モードOFFまたは定期的に実行）
        if not tracking_mode or not tracking_active or frame_counter % 10 == 0:
            faces = recognizer.face_cascade.detectMultiScale(gray, 1.3, 5)

            if len(faces) > 0:
                # 最大の顔を選択（または最も近い顔）
                if last_valid_position is not None and len(faces) > 1 and tracking_mode:
                    # 追跡モードで複数顔がある場合、最も近い顔を選択
                    min_distance = float('inf')
                    selected_face = None
                    for (x, y, w, h) in faces:
                        face_center = (x + w//2, y + h//2)
                        distance = np.sqrt((face_center[0] - last_valid_position[0])**2 +
                                         (face_center[1] - last_valid_position[1])**2)
                        if distance < min_distance:
                            min_distance = distance
                            selected_face = (x, y, w, h)
                    if min_distance <= MAX_MOVEMENT_PER_FRAME * 2:
                        (x, y, w, h) = selected_face
                    else:
                        (x, y, w, h) = max(faces, key=lambda f: f[2] * f[3])
                else:
                    (x, y, w, h) = max(faces, key=lambda f: f[2] * f[3])

                new_center = (x + w//2, y + h//2)

                # 移動が妥当かチェック
                if not is_reasonable_movement(last_valid_position, new_center):
                    continue

                face_bbox = (x, y, w, h)
                face_detected_now = True
                last_detection_frame = frame_counter
                last_valid_position = new_center

                # 追跡モードONの場合、トラッカーを初期化
                if tracking_mode:
                    tracker = create_tracker()
                    tracker.init(frame, face_bbox)
                    tracking_active = True

                    # 位置履歴に追加
                    position_history.append(new_center)
                    if len(position_history) > MAX_HISTORY:
                        position_history.pop(0)

        # 追跡モードONで顔検出失敗時はトラッキングを試みる
        elif tracking_mode and tracking_active:
            tracking_success, bbox = tracker.update(frame)

            if tracking_success:
                x, y, w, h = [int(v) for v in bbox]
                new_center = (x + w//2, y + h//2)

                # 移動が妥当かチェック
                if not is_reasonable_movement(last_valid_position, new_center):
                    tracking_success = False
                    tracking_active = False
                else:
                    face_bbox = (x, y, w, h)
                    last_valid_position = new_center

                    # 位置履歴に追加
                    position_history.append(new_center)
                    if len(position_history) > MAX_HISTORY:
                        position_history.pop(0)

        # 顔の領域を取得（検出・追跡・予測のいずれか）
        display_bbox = None
        display_status = ""
        use_prediction = False

        if face_detected_now:
            display_bbox = face_bbox
            display_status = "DETECTED"
        elif tracking_success:
            display_bbox = face_bbox
            display_status = "TRACKING"
        elif tracking_mode and face_bbox:
            # 予測を試みる
            frames_since_detection = frame_counter - last_detection_frame
            if frames_since_detection < PREDICTION_THRESHOLD:
                predicted_pos = predict_next_position()
                if predicted_pos:
                    x, y, w, h = face_bbox
                    center_x, center_y = predicted_pos
                    x = center_x - w//2
                    y = center_y - h//2
                    display_bbox = (x, y, w, h)
                    display_status = "PREDICTED"
                    use_prediction = True

        # 顔が検出・追跡・予測されている場合の処理
        if display_bbox:
            x, y, w, h = display_bbox

            # 顔の領域を抽出して認識
            if not use_prediction:  # 予測時は認識しない
                face_roi = gray[y:y+h, x:x+w]
                face_roi_resized = cv2.resize(face_roi, (200, 200))

                if recognition_mode and recognizer.is_trained:
                    # 認識モード
                    name, confidence = recognizer.recognize(face_roi_resized)

                    # 認識結果を履歴に追加（多数決で安定化）
                    if name != "Unknown":
                        recognition_history.append(name)
                        if len(recognition_history) > HISTORY_SIZE:
                            recognition_history.pop(0)

                        # 履歴から最も多い名前を選択
                        if recognition_history:
                            most_common = max(set(recognition_history), key=recognition_history.count)
                            recognized_name = most_common
                            last_recognition_frame = frame_counter
                            recognized_now = True

            # 表示処理
            if recognition_mode and recognizer.is_trained:
                # 表示する名前を決定（維持システム）
                frames_since_recognition = frame_counter - last_recognition_frame
                if frames_since_recognition <= MAINTAIN_FRAMES and recognized_name != "Unknown":
                    # 維持期間内
                    display_name = recognized_name

                    # 色とラベルを状態に応じて変更
                    if display_status == "DETECTED":
                        if recognized_now:
                            color = (0, 255, 0)  # 緑（認識中）
                            label = f"{display_name} ({confidence:.1f} | Detected)"
                        else:
                            color = (0, 200, 200)  # シアン（維持中）
                            label = f"{display_name} (Maintained)"
                    elif display_status == "TRACKING":
                        color = (0, 255, 255)  # 黄（追跡中）
                        label = f"{display_name} (Tracking)"
                    elif display_status == "PREDICTED":
                        color = (0, 0, 255)  # 赤（予測中）
                        label = f"{display_name} (Predicted)"
                    else:
                        color = (0, 200, 200)  # シアン
                        label = f"{display_name}"
                else:
                    # 維持期間を過ぎたまたは新規認識
                    if not use_prediction and 'name' in locals() and name != "Unknown":
                        color = (0, 255, 0)  # 緑
                        label = f"{name} ({confidence:.1f})"
                    else:
                        color = (0, 0, 255)  # 赤
                        label = "Unknown"

                cv2.rectangle(result, (x, y), (x+w, y+h), color, 2)
                cv2.putText(result, label, (x, y - 10),
                           cv2.FONT_HERSHEY_SIMPLEX, 0.7, color, 2)
            else:
                # 通常モード（認識OFF）
                # 追跡状態に応じて色分け
                if display_status == "DETECTED":
                    color = (0, 255, 0)  # 緑
                    label = "Face (Detected)"
                elif display_status == "TRACKING":
                    color = (0, 255, 255)  # 黄
                    label = "Face (Tracking)"
                elif display_status == "PREDICTED":
                    color = (0, 0, 255)  # 赤
                    label = "Face (Predicted)"
                else:
                    color = (255, 0, 0)  # 青
                    label = "Face"

                cv2.rectangle(result, (x, y), (x+w, y+h), color, 2)
                cv2.putText(result, label, (x, y - 10),
                           cv2.FONT_HERSHEY_SIMPLEX, 0.6, color, 2)

        # 情報を表示
        mode_text = "Recognition: ON" if recognition_mode else "Recognition: OFF"
        mode_color = (0, 255, 0) if recognition_mode else (128, 128, 128)
        cv2.putText(result, mode_text, (10, 40),
                   cv2.FONT_HERSHEY_SIMPLEX, 0.8, mode_color, 2)

        # 追跡モードの表示
        track_text = "Tracking: ON" if tracking_mode else "Tracking: OFF"
        track_color = (0, 255, 0) if tracking_mode else (128, 128, 128)
        cv2.putText(result, track_text, (10, 75),
                   cv2.FONT_HERSHEY_SIMPLEX, 0.8, track_color, 2)

        trained_text = f"Trained: {len(recognizer.known_faces)} people"
        cv2.putText(result, trained_text, (10, 110),
                   cv2.FONT_HERSHEY_SIMPLEX, 0.6, (255, 255, 0), 2)

        if not recognizer.is_trained:
            cv2.putText(result, "Press 't' to train after adding faces", (10, 145),
                       cv2.FONT_HERSHEY_SIMPLEX, 0.5, (0, 255, 255), 1)

        # 維持時間の表示
        if recognition_mode and recognizer.is_trained:
            maintain_text = f"Maintain Time: {MAINTAIN_FRAMES} frames (~{MAINTAIN_FRAMES/30:.1f}s)"
            cv2.putText(result, maintain_text, (10, 145),
                       cv2.FONT_HERSHEY_SIMPLEX, 0.5, (200, 200, 200), 1)

        # 移動軌跡を描画（追跡モードONの場合）
        if tracking_mode and len(position_history) > 1:
            for i in range(1, len(position_history)):
                cv2.line(result, position_history[i-1], position_history[i],
                        (255, 100, 100), 2)

        # 凡例
        if tracking_mode:
            cv2.putText(result, "Green: Detected | Yellow: Tracking | Red: Predicted",
                       (10, result.shape[0] - 40),
                       cv2.FONT_HERSHEY_SIMPLEX, 0.5, (255, 255, 255), 1)

        # 操作説明
        help_text = "n:New | t:Train | s:Save | r:Recog | k:Track | c:Clear | +/-:Time | q:Exit"
        cv2.putText(result, help_text, (10, result.shape[0] - 15),
                   cv2.FONT_HERSHEY_SIMPLEX, 0.5, (255, 255, 255), 1)

        cv2.imshow('Face Recognition', result)

        key = cv2.waitKey(1) & 0xFF

        if key == 27 or key == ord('q'):
            print("終了します")
            break

        elif key == ord('n'):
            # 新しい人を登録
            print("\n=== 新しい人を登録 ===")
            name = input("名前を入力してください: ").strip()

            if not name:
                print("名前が入力されていません")
                continue

            print(f"\n{name}さんの顔を撮影します")
            print("カメラに顔を向けてください...")
            print("5枚撮影します。様々な角度や表情で撮影すると認識精度が上がります")

            captured_count = 0
            while captured_count < 5:
                ret, frame = cap.read()
                if not ret:
                    continue

                frame = cv2.flip(frame, 1)
                gray = cv2.cvtColor(frame, cv2.COLOR_BGR2GRAY)
                display = frame.copy()

                faces = recognizer.face_cascade.detectMultiScale(gray, 1.3, 5)

                if len(faces) > 0:
                    # 最大の顔を選択
                    (x, y, w, h) = max(faces, key=lambda f: f[2] * f[3])

                    cv2.rectangle(display, (x, y), (x+w, y+h), (0, 255, 0), 2)
                    cv2.putText(display, f"Capturing {captured_count+1}/5", (x, y - 10),
                               cv2.FONT_HERSHEY_SIMPLEX, 0.7, (0, 255, 0), 2)

                    # 顔画像を抽出して保存
                    face_roi = gray[y:y+h, x:x+w]
                    face_roi_resized = cv2.resize(face_roi, (200, 200))
                    recognizer.add_face(name, face_roi_resized)

                    captured_count += 1
                    print(f"撮影 {captured_count}/5 完了")

                    cv2.imshow('Face Recognition', display)
                    cv2.waitKey(500)  # 0.5秒待機
                else:
                    cv2.putText(display, "No face detected", (50, 50),
                               cv2.FONT_HERSHEY_SIMPLEX, 1, (0, 0, 255), 2)

                cv2.imshow('Face Recognition', display)
                if cv2.waitKey(1) & 0xFF == 27:
                    break

            print(f"\n{name}さんの登録が完了しました！")
            print("'t'キーを押して学習を実行してください")

        elif key == ord('t'):
            # 学習を実行
            print("\n学習を開始します...")
            if recognizer.train():
                recognition_mode = True

        elif key == ord('s'):
            # データを保存
            recognizer.save_data()

        elif key == ord('r'):
            # 認識モードの切り替え
            if recognizer.is_trained:
                recognition_mode = not recognition_mode
                status = "ON" if recognition_mode else "OFF"
                print(f"認識モード: {status}")
            else:
                print("まず学習を実行してください（'t'キー）")

        elif key == ord('k'):
            # 追跡モードの切り替え
            tracking_mode = not tracking_mode
            status = "ON" if tracking_mode else "OFF"
            print(f"追跡モード: {status}")
            if not tracking_mode:
                # 追跡モードOFF時はトラッカーをリセット
                tracker = None
                tracking_active = False
                position_history = []
                last_valid_position = None

        elif key == ord('c'):
            # データをクリア
            confirm = input("\nすべてのデータをクリアしますか？ (yes/no): ")
            if confirm.lower() == 'yes':
                recognizer.known_faces = {}
                recognizer.is_trained = False
                recognizer.name_to_label = {}
                recognizer.label_to_name = {}
                recognition_mode = False
                recognition_history = []
                recognized_name = "Unknown"
                print("すべてのデータをクリアしました")
            else:
                print("キャンセルしました")

        elif key == ord('+') or key == ord('='):
            MAINTAIN_FRAMES += 10
            print(f"認識維持時間を延長: {MAINTAIN_FRAMES} フレーム (約{MAINTAIN_FRAMES/30:.1f}秒)")

        elif key == ord('-') or key == ord('_'):
            MAINTAIN_FRAMES = max(10, MAINTAIN_FRAMES - 10)
            print(f"認識維持時間を短縮: {MAINTAIN_FRAMES} フレーム (約{MAINTAIN_FRAMES/30:.1f}秒)")

    cap.release()
    cv2.destroyAllWindows()


def main():
    """メイン関数"""
    run_face_recognition()


if __name__ == "__main__":
    main()
