import serial

with serial.Serial('/tmp/ttyuartmock', 115200, timeout=0.1) as port:
    port.write(b'hello\n')
    data = port.readline()
    print(data)