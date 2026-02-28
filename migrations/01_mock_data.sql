-- Employee
INSERT INTO
    employee (employee_cid, employee_email, employee_name, employee_passwd, phone_num)
VALUES
    /* Passwd: elatla */
    (9876543, 'elatla@elatla.com', 'El Atla', '$argon2id$v=19$m=65536,t=3,p=4$YjQ4NjA3YjYxNWJjMzk0ZjEyMGZhMDcxMDA2MTlkNWE$uzfrjQptIBYtObnauZ9c+YLO9ROnHRk4sLRc/3BskxM', '000-111-222');

-- Shipment status
INSERT INTO status_shipment (status_id, status_desc) VALUES 
(0, 'ON_COUNTER'),
(1, 'ON_TRANSIT'),
(2, 'DELIVERED');
(3, 'PICKED_UP');

-- Client
INSERT INTO
    client (client_cid, client_email, client_name, client_passwd, phone_num)
VALUES
    /* Passwd: juangarcia */
    (202020, 'juan.garcia@tiyasociados.com', 'Juan Garcia', '$argon2id$v=19$m=12,t=3,p=1$dHJ2bG95c2l4NTAwMDAwMA$Ezs9sAVZDnUGTt/1+kYzTw', '222-111-000');

-- Package
INSERT INTO package (package_id, package_desc, package_weight, package_width, package_length, package_height) VALUES 
(101, 'Mechanical Keyboard', 1.50, 35, 15, 5),
(102, 'Gaming Mouse', 0.20, 12, 7, 4),
(103, 'Monitor 24 inch', 4.50, 55, 40, 15);

INSERT INTO shipment (guide_num, sender_cid, receiver_cid, package_id, status_id, destination_address) VALUES 
('GD-789001', 202020, 202020, 101, 2, '67 St., Zimbabwe'), -- DELIVERED
('GD-789002', 202020, 202020, 102, 2, '67 St., Zimbabwe'), -- DELIVERED
('GD-789003', 202020, 202020, 103, 1, '67 St., Zimbabwe'); -- ON_TRANSIT