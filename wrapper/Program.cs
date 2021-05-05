using System;
using OpenHardwareMonitor.Hardware;

namespace wrapper
{
    class Program
    {
        static void Main(string[] args)
        {
            var c = new Computer();
            c.CPUEnabled = true;
            c.GPUEnabled = true;
            c.Open();

            while (true) {
                //TODO: support some commands if necessary
                Console.ReadLine();

                var max = 0.0F;
                foreach (var h in c.Hardware) {
                    h.Update();
                    foreach (var s in h.Sensors) {
                        if (s.SensorType == SensorType.Temperature) {
                            var v = s.Value ?? 0;
                            //Console.WriteLine($"{s.Identifier} {s.Index} {s.Name} {v}");
                            if (v > max) {
                                max = v;
                            }
                        }
                    }
                }

                Console.WriteLine($"{max}");
            }
        }
    }
}
