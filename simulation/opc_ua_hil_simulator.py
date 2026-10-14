import asyncio
import math
import random
from asyncua import Server

class PumpStationHILSimulator:
    def __init__(self, endpoint="opc.tcp://0.0.0.0:4840/freeopcua/server/"):
        self.server = Server()
        self.server.set_endpoint(endpoint)
        self.server.set_server_name("project-nest HIL Simulator")
        self.fault_states = {"bearing_wear": 0.0, "valve_blockage": 0.0, "vfd_drift": 0.0}

    async def initialize(self):
        await self.server.init()
        idx = await self.server.register_namespace("http://projectnest.industrial.io")
        objects = self.server.nodes.objects
        pump_folder = await objects.add_folder(idx, "PumpStation_Line2")

        self.node_vfd_freq = await pump_folder.add_variable(idx, "VFD_Frequency", 50.0)
        self.node_rpm = await pump_folder.add_variable(idx, "Pump_RPM", 1490.0)
        self.node_current = await pump_folder.add_variable(idx, "Motor_Current", 74.5)
        self.node_pressure = await pump_folder.add_variable(idx, "Discharge_Pressure", 4.2)
        self.node_vibration = await pump_folder.add_variable(idx, "Pipe_Vibration", 0.15)

        for node in [self.node_vfd_freq, self.node_rpm, self.node_current, self.node_pressure, self.node_vibration]:
            await node.set_writable()

    def inject_fault(self, fault_name: str, magnitude: float):
        if fault_name in self.fault_states:
            self.fault_states[fault_name] = max(0.0, min(1.0, magnitude))

    async def run_loop(self):
        async with self.server:
            step = 0
            while True:
                step += 1
                t = step * 0.1
                vfd_freq = 50.0 + math.sin(t * 0.5) * 0.5 + self.fault_states["vfd_drift"] + random.gauss(0, 0.05)
                rpm = (vfd_freq * 29.8) + random.gauss(0, 0.2)
                current = (vfd_freq * 1.45) + (self.fault_states["bearing_wear"] * 15.0) + random.gauss(0, 0.1)
                base_pressure = 0.0028 * (rpm ** 1.1)
                discharge_pressure = max(0.1, base_pressure - (self.fault_states["valve_blockage"] * 3.5) + random.gauss(0, 0.02))
                bearing_vib = (self.fault_states["bearing_wear"] ** 2) * 8.5
                vibration = 0.15 + bearing_vib + random.gauss(0, 0.01)

                await self.node_vfd_freq.write_value(float(vfd_freq))
                await self.node_rpm.write_value(float(rpm))
                await self.node_current.write_value(float(current))
                await self.node_pressure.write_value(float(discharge_pressure))
                await self.node_vibration.write_value(float(vibration))

                await asyncio.sleep(0.1)

if __name__ == "__main__":
    hil = PumpStationHILSimulator()
    loop = asyncio.get_event_loop()
    loop.run_until_complete(hil.initialize())
    loop.run_until_complete(hil.run_loop())
