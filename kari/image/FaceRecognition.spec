# -*- mode: python ; coding: utf-8 -*-

import cv2
import os

# OpenCV HaarCascadeデータファイルのパスを取得
cv2_data = os.path.join(os.path.dirname(cv2.__file__), 'data')

a = Analysis(
    ['face_recognition_app.py'],
    pathex=[],
    binaries=[],
    datas=[
        (cv2_data, 'cv2/data'),  # OpenCVのデータファイルをバンドル
    ],
    hiddenimports=['cv2'],
    hookspath=[],
    hooksconfig={},
    runtime_hooks=[],
    excludes=[],
    noarchive=False,
    optimize=0,
)
pyz = PYZ(a.pure)

exe = EXE(
    pyz,
    a.scripts,
    a.binaries,
    a.datas,
    [],
    name='FaceRecognition',
    debug=False,
    bootloader_ignore_signals=False,
    strip=False,
    upx=True,
    upx_exclude=[],
    runtime_tmpdir=None,
    console=True,
    disable_windowed_traceback=False,
    argv_emulation=False,
    target_arch=None,
    codesign_identity=None,
    entitlements_file=None,
)
