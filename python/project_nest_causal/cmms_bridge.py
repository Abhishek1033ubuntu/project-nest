import logging
from dataclasses import dataclass
from typing import Dict, Any, Optional
import requests

logging.basicConfig(level=logging.INFO, format="%(asctime)s [%(levelname)s] %(message)s")
logger = logging.getLogger("CMMSIntegration")

@dataclass
class CausalFaultEvent:
    event_id: str
    root_cause_node: str
    confidence_score: float
    residual_shift: float
    asset_tag: str
    facility_zone: str

@dataclass
class RequisitionOrder:
    pr_number: str
    status: str
    part_sku: str
    description: str
    quantity: int
    target_bin: str

class CMMSIntegrationBridge:
    def __init__(self, erp_type: str, base_url: str, api_token: str):
        self.erp_type = erp_type.upper()
        self.base_url = base_url.rstrip("/")
        self.api_token = api_token

        self._part_bom_registry: Dict[str, Dict[str, Any]] = {
            "Impeller_Bearing_Line2": {
                "part_sku": "SKF-6208-2RS-C3",
                "description": "Deep Groove Ball Bearing 40x80x18mm",
                "quantity": 1,
                "unit_of_measure": "EA",
                "default_bin": "Aisle 3, Shelf B, Bin 12",
                "plant_code": "PLANT-1001"
            },
            "Discharge_Valve_3B": {
                "part_sku": "VALVE-SEAL-KIT-04",
                "description": "EPDM Gasket & O-Ring Replacement Pack",
                "quantity": 2,
                "unit_of_measure": "KT",
                "default_bin": "Aisle 1, Shelf D, Bin 04",
                "plant_code": "PLANT-1001"
            }
        }

    def process_causal_fault(self, fault: CausalFaultEvent) -> Optional[RequisitionOrder]:
        bom = self._part_bom_registry.get(fault.root_cause_node)
        if not bom:
            logger.warning(f"No BOM SKU mapping found for node: {fault.root_cause_node}")
            return None

        if self.erp_type == "MAXIMO":
            return self._create_maximo_purchase_requisition(fault, bom)
        elif self.erp_type == "SAP":
            return self._create_sap_purchase_requisition(fault, bom)
        else:
            raise ValueError(f"Unsupported ERP System: {self.erp_type}")

    def _create_maximo_purchase_requisition(self, fault: CausalFaultEvent, bom: Dict[str, Any]) -> RequisitionOrder:
        endpoint = f"{self.base_url}/maximo/api/os/mxpr"
        headers = {"apikey": self.api_token, "Content-Type": "application/json", "x-method-override": "SYNC"}
        payload = {
            "description": f"[Auto-Triggered project-nest] Predictive Replacement for {fault.root_cause_node}",
            "siteid": "BEDFORD",
            "prline": [{
                "itemnum": bom["part_sku"],
                "orderqty": bom["quantity"],
                "orderunit": bom["unit_of_measure"],
                "storeloc": "CENTRAL",
                "remark": f"Causal SCM Event: {fault.event_id}"
            }]
        }
        response = requests.post(endpoint, headers=headers, json=payload, timeout=5.0)
        response.raise_for_status()
        data = response.json()
        return RequisitionOrder(
            pr_number=data.get("prnum", "PR-MAXIMO-UNKNOWN"),
            status="SUBMITTED",
            part_sku=bom["part_sku"],
            description=bom["description"],
            quantity=bom["quantity"],
            target_bin=bom["default_bin"]
        )

    def _create_sap_purchase_requisition(self, fault: CausalFaultEvent, bom: Dict[str, Any]) -> RequisitionOrder:
        endpoint = f"{self.base_url}/sap/opu/odata/sap/API_PURCHASEREQUISITION_PROCESS_SRV/A_PurchaseRequisition"
        headers = {"Authorization": f"Bearer {self.api_token}", "Content-Type": "application/json"}
        payload = {
            "PurchaseRequisitionHeaderName": f"Predictive PR: {fault.asset_tag}",
            "to_PurchaseRequisitionItem": [{
                "Material": bom["part_sku"],
                "Plant": bom["plant_code"],
                "RequestedQuantity": str(bom["quantity"]),
                "BaseUnit": bom["unit_of_measure"]
            }]
        }
        response = requests.post(endpoint, headers=headers, json=payload, timeout=5.0)
        response.raise_for_status()
        data = response.json().get("d", {})
        return RequisitionOrder(
            pr_number=data.get("PurchaseRequisition", "PR-SAP-UNKNOWN"),
            status="APPROVED_AUTO",
            part_sku=bom["part_sku"],
            description=bom["description"],
            quantity=bom["quantity"],
            target_bin=bom["default_bin"]
        )
