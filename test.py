import serial
s = serial.Serial('/dev/pts/4', 115200)

s.write(b'^DN')
