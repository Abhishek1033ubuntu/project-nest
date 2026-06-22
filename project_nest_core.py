import os
import time
import random
import json
import sqlite3

# Global safety check for the modern MQTT networking library
try:
    import paho.mqtt.client as mqtt
    from paho.mqtt.enums import CallbackAPIVersion 
    MQTT_AVAILABLE = True
except ImportError:
    MQTT_AVAILABLE = False

class ProjectNestEcosystem:
    """
    Consolidated Enterprise Framework for Project N.E.S.T.
    (Neural Edge Support Toolkit)
    """
    def __init__(self, db_path="nest_central_server.db", broker_ip="127.0.0.1"):
        self.db_path = db_path
        self.broker_ip = broker_ip
        self.safe_current_limit = 10.0
        self.anomaly_counter = 0
        
        # Initialize Database Infrastructure
        self._init_database()
        
        # Initialize MQTT Client if library is installed
        if MQTT_AVAILABLE:
            self.client = mqtt.Client(
                callback_api_version=CallbackAPIVersion.VERSION2, 
                client_id="NEST_NODE_CONVEYOR_M01"
            )
        else:
            self.client = None

    def _init_database(self):
        """Builds on-premises time-series database tables."""
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        cursor.execute("""
        CREATE TABLE IF NOT EXISTS active_predictive_tickets (
            ticket_id TEXT PRIMARY KEY, factory_id TEXT, line_id TEXT,
            machine_id TEXT, timestamp_raised TEXT, rpm_lag TEXT,
            excess_current TEXT, status TEXT DEFAULT 'OPEN'
        )""")
        cursor.execute("""
        CREATE TABLE IF NOT EXISTS maintenance_history_logs (
            log_id INTEGER PRIMARY KEY AUTOINCREMENT, ticket_id TEXT,
            technician_id TEXT, action_taken TEXT, post_repair_current REAL,
            timestamp_closed TEXT DEFAULT CURRENT_TIMESTAMP
        )""")
        conn.commit()
        conn.close()

    def run_edge_node_intel(self, factory_id, line_id, machine_id, commanded_rpm, actual_rpm, current_draw):
        """
        MODULE 1: NEST-NODE (NPU Edge Layer)
        Monitors closed-loop telemetry and identifies subtle system drift.
        """
        rpm_drift = commanded_rpm - actual_rpm
        print(f"📡 [Node Ingestion] Cmd: {commanded_rpm}RPM | Act: {actual_rpm}RPM | Draw: {current_draw}A")
        
        # Predictive Threshold Verification
        if current_draw > self.safe_current_limit and rpm_drift > 5:
            self.anomaly_counter += 1
            print(f"    ⚠️ Anomaly verified. Progressive drift metric: {self.anomaly_counter}/3")
        else:
            if self.anomaly_counter > 0: self.anomaly_counter -= 1
            
        if self.anomaly_counter >= 3:
            ticket = {
                "ticket_id": f"NEST-TICK-{random.randint(1000, 9999)}",
                "factory_id": factory_id, "line_id": line_id, "machine_id": machine_id,
                "timestamp": time.strftime("%Y-%m-%d %H:%M:%S"),
                "metrics": {"rpm_lag": f"{rpm_drift} RPM", "excess_current": f"{round(current_draw - self.safe_current_limit, 2)} Amps"}
            }
            return json.dumps(ticket)
        return None

    def server_ingest_payload(self, ticket_json):
        """
        MODULE 2: CENTRAL SERVER (On-Premises Data Layer)
        Intercepts network alerts and registers them to secure database storage.
        """
        ticket = json.loads(ticket_json)
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        try:
            cursor.execute("""
            INSERT INTO active_predictive_tickets (ticket_id, factory_id, line_id, machine_id, timestamp_raised, rpm_lag, excess_current)
            VALUES (?, ?, ?, ?, ?, ?, ?)""", (
                ticket["ticket_id"], ticket["factory_id"], ticket["line_id"], ticket["machine_id"],
                ticket["timestamp"], ticket["metrics"]["rpm_lag"], ticket["metrics"]["excess_current"]
            ))
            conn.commit()
            print(f"\n📥 [Server Database] Successfully locked predictive ticket: {ticket['ticket_id']}")
        except sqlite3.IntegrityError:
            pass
        finally:
            conn.close()

    def run_tablet_resolution(self, ticket_id, tech_id, action, final_current):
        """
        MODULE 3: NEST-PAD (Human Interface/Training Loop Layer)
        Technician pulls data, executes physical fix, logs verified recovery metrics.
        """
        print(f"\n📱 [NEST-Pad Tablet] Dispatching Tech {tech_id} to resolve {ticket_id}...")
        conn = sqlite3.connect(self.db_path)
        cursor = conn.cursor()
        
        # Append to historical model training ledger
        cursor.execute("""
        INSERT INTO maintenance_history_logs (ticket_id, technician_id, action_taken, post_repair_current)
        VALUES (?, ?, ?, ?)""", (ticket_id, tech_id, action, final_current))
        
        # Close out active alarm status
        cursor.execute("UPDATE active_predictive_tickets SET status = 'RESOLVED' WHERE ticket_id = ?", (ticket_id,))
        conn.commit()
        
        # Confirm local dashboard is clear
        cursor.execute("SELECT ticket_id FROM active_predictive_tickets WHERE status = 'OPEN'")
        remaining = cursor.fetchall()
        conn.close()
        
        print(f"🎉 [System Success] Loop Closed. Active alarms remaining on floor: {len(remaining)}")

# --- SIMULATION PIPELINE ---
if __name__ == "__main__":
    print("====================================================")
    print("     PROJECT N.E.S.T. COMPLETE SYSTEM SIMULATION    ")
    print("====================================================\n")
    
    # Instantiate the system ecosystem
    nest = ProjectNestEcosystem()
    
    # 1. Simulate developing friction/misalignment inside the machine control loop
    error_data_stream = [
        {"cmd": 1500, "act": 1492, "current": 11.2},
        {"cmd": 1500, "act": 1490, "current": 11.8},
        {"cmd": 1500, "act": 1489, "current": 12.1}
    ]
    
    generated_ticket = None
    for data in error_data_stream:
        alert_payload = nest.run_edge_node_intel(
            factory_id="FAC-REWA-01", line_id="LINE-A", machine_id="CONVEYOR-M01",
            commanded_rpm=data["cmd"], actual_rpm=data["act"], current_draw=data["current"]
        )
        if alert_payload:
            generated_ticket = json.loads(alert_payload)
            # Route to server
            nest.server_ingest_payload(alert_payload)
            
    # 2. Simulate technician stepping in with the mobile tablet UI
    if generated_ticket:
        time.sleep(1)
        nest.run_tablet_resolution(
            ticket_id=generated_ticket["ticket_id"],
            tech_id="TECH-204",
            action="Realigned motor shaft coupling and re-tensioned drive belt assembly.",
            final_current=6.1
        )
    print("\n====================================================")
    print("      SIMULATION COMPLETE - FILES SAVED LOCALLY      ")
    print("====================================================")