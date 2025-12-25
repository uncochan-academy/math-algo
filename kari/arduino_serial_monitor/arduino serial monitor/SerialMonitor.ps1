
# Minimal serial monitor for Arduino (COM3, 9600). ASCII only to avoid mojibake.
$portName = "COM5"
$baudRate = 9600

$parity   = [System.IO.Ports.Parity]::None
$dataBits = 8
$stopBits = [System.IO.Ports.StopBits]::One

$p = [System.IO.Ports.SerialPort]::new($portName, $baudRate, $parity, $dataBits, $stopBits)
$p.NewLine     = "`r`n"   # Change to "`n" if Arduino prints LF only
$p.ReadTimeout = 500

$p.Open()
Write-Host "[$portName] opened @ $baudRate. Press Ctrl+C to stop."

while ($true) {
    try {
        $line = $p.ReadLine()
        if ($line -ne $null) { Write-Host $line }
    } catch {
        Start-Sleep -Milliseconds 10
    }
}

# Normally you will press Ctrl+C to stop the loop.
# If needed, you can manually close:
# $p.Close()
# $p.Dispose()
# Read-Host "Press Enter to exit"
``
