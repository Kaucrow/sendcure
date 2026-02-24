from dataclasses import dataclass
from typing import Optional


@dataclass
class Shipment:
    shipment_id: int
    guide_num: str
    client_cid: int
    receiver_cid: Optional[int]
    package_id: int
    delivery_id: Optional[int]
    status_id: int
    shipment_dt: str
    destination_address: str
    status_desc: Optional[str] = None
    client_name: Optional[str] = None
    package_desc: Optional[str] = None
    delivery_name: Optional[str] = None


@dataclass
class Delivery:
    delivery_id: int
    delivery_name: str
    vehicle_id: int
    vehicle_plate: Optional[str] = None


@dataclass
class StatusShipment:
    status_id: int
    status_desc: str


@dataclass
class Employee:
    employee_cid: int
    employee_name: str
    employee_email: str
    role: str
