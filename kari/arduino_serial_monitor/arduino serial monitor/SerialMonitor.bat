
@echo off
REM PowerShell を「閉じない」設定で起動
powershell.exe -NoProfile -NoExit -ExecutionPolicy Bypass -File "SerialMonitor.ps1"
REM PowerShellが終了してしまっても cmd のウィンドウは止める
pause
``
