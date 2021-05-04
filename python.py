# pythonnet
import clr

# OpenHardwareMonitor/OpenHardwareMonitorLib.dll
clr.AddReference(r'OpenHardwareMonitor/OpenHardwareMonitorLib')
from OpenHardwareMonitor.Hardware import Computer, SensorType

# pyserial
from serial import Serial
import serial.tools.list_ports

for port in serial.tools.list_ports.comports():
    if port.vid == 0x1209 and port.pid == 0x1776:
        print("Thelio Io at", port.device)
        serial = Serial(
            port=port.device,
            baudrate=115200,
            timeout=1,
        )
        serial.write(b"IoREVISION\r")
        for line in range(1, 5):
            print("Line", line, serial.read_until())

c = Computer()
c.CPUEnabled = True # get the Info about CPU
c.GPUEnabled = True # get the Info about GPU
c.Open()

max = 0
for i in range(0, len(c.Hardware)):
    h = c.Hardware[i]
    h.Update()
    for j in range(0, len(h.Sensors)):
        s = h.Sensors[j]
        if s.SensorType == SensorType.Temperature:
            v = s.get_Value()
            print(s.Identifier, s.Index, s.Name, v)
            if v > max:
                max = v
print("Max", max)
