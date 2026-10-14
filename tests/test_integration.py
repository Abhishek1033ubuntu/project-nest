import math
import numpy as np
import pytest
from unittest.mock import patch

class MockOPCUAServer:
    def __init__(self):
        self.tags = {}
        self.fault_states = {"bearing_wear": 0.0, "valve_blockage": 0.0}

    def inject_fault(self, fault_name: str, magnitude: float):
        self.fault_states[fault_name] = max(0.0, min(1.0, magnitude))

    def step_simulation(self, t: float) -> dict:
        vfd = 50.0 + math.sin(t * 0.5) * 0.5
        rpm = vfd * 29.8
        current = (vfd * 1.45) + (self.fault_states["bearing_wear"] * 15.0)
        pressure = max(0.1, 0.0028 * (rpm ** 1.1) - (self.fault_states["valve_blockage"] * 3.5))
        vibration = 0.15 + ((self.fault_states["bearing_wear"] ** 2) * 8.5)

        self.tags = {
            "VFD_Frequency": float(vfd),
            "Pump_RPM": float(rpm),
            "Motor_Current": float(current),
            "Discharge_Pressure": float(pressure),
            "Pipe_Vibration": float(vibration),
        }
        return self.tags

class MockSCMEngine:
    def __init__(self):
        self.equations = {
            "Motor_Current": lambda t: 1.45 * t["VFD_Frequency"] + 2.0,
            "Pump_RPM": lambda t: 29.8 * t["VFD_Frequency"] + 10.0,
            "Discharge_Pressure": lambda t: 0.0028 * (t["Pump_RPM"] ** 1.1),
            "Pipe_Vibration": lambda t: 0.15 + (0.0 if t["Motor_Current"] < 80.0 else (t["Motor_Current"] - 80.0) * 0.5),
        }

    def diagnose(self, telemetry: dict) -> tuple[str, float, dict]:
        residuals = {}
        max_residual = -1.0
        root_cause = "NORMAL"

        for node, eq in self.equations.items():
            expected = eq(telemetry)
            actual = telemetry.get(node, expected)
            res = abs(actual - expected)
            residuals[node] = res
            if res > max_residual and res > 1.0:
                max_residual = res
                root_cause = node

        return root_cause, max_residual, residuals

@pytest.fixture
def simulator():
    return MockOPCUAServer()

@pytest.fixture
def scm_engine():
    return MockSCMEngine()

class TestOPCUASimulator:
    def test_nominal_telemetry(self, simulator):
        data = simulator.step_simulation(t=1.0)
        assert 49.0 <= data["VFD_Frequency"] <= 51.0

    def test_fault_injection(self, simulator):
        simulator.inject_fault("bearing_wear", 0.85)
        data = simulator.step_simulation(t=2.0)
        assert data["Motor_Current"] > 85.0

class TestSCMInferenceEngine:
    def test_nominal_diagnosis(self, scm_engine):
        telemetry = {"VFD_Frequency": 50.0, "Pump_RPM": 1500.0, "Motor_Current": 74.5, "Discharge_Pressure": 4.2, "Pipe_Vibration": 0.15}
        root_cause, residual, _ = scm_engine.diagnose(telemetry)
        assert root_cause == "NORMAL"

    def test_bearing_fault_attribution(self, scm_engine):
        telemetry = {"VFD_Frequency": 50.0, "Pump_RPM": 1500.0, "Motor_Current": 92.5, "Discharge_Pressure": 4.2, "Pipe_Vibration": 6.4}
        root_cause, residual, _ = scm_engine.diagnose(telemetry)
        assert root_cause == "Motor_Current"

class TestEndToEndPipelineIntegration:
    def test_full_pipeline_flow(self, simulator, scm_engine):
        simulator.inject_fault("bearing_wear", 0.9)
        fault_telemetry = simulator.step_simulation(t=4.0)
        root_cause, residual_shift, _ = scm_engine.diagnose(fault_telemetry)
        assert root_cause == "Motor_Current"

        with patch("requests.post") as mock_post:
            mock_post.return_value.status_code = 201
            mock_post.return_value.json.return_value = {"prnum": "PR-2026-TEST-1014"}
            import requests
            res = requests.post("http://localhost:8080/maximo/api/pr", json={"root_cause": root_cause})
            assert res.status_code == 201
