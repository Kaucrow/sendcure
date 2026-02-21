CREATE TABLE employee (
    employee_cid INT PRIMARY KEY,
    client_email VARCHAR(100) UNIQUE NOT NULL,
    client_name VARCHAR(100) NOT NULL,
    employee_passwd VARCHAR(100) NOT NULL,
    phone_num VARCHAR(20)
);

CREATE TABLE employee_role (
    employee_cid INT NOT NULL,
    role_id INT NOT NULL,

    FOREIGN KEY (employee_cid) REFERENCES employee(employee_cid),
    FOREIGN KEY (role_id) REFERENCES role(role_id)
);

CREATE TABLE role (
    role_id INT PRIMARY KEY AUTO_INCREMENT,
    role_desc VARCHAR(100)
);

CREATE TABLE client (
    client_cid INT PRIMARY KEY,
    client_email VARCHAR(100) UNIQUE NOT NULL,
    client_name VARCHAR(100) NOT NULL,
    client_passwd VARCHAR(100) NOT NULL,
    phone_num VARCHAR(20)
);

CREATE TABLE package (
    package_id INT PRIMARY KEY,
    package_desc TEXT,
    package_weight DECIMAL(10, 2),
    package_width VARCHAR(50),
    package_length VARCHAR(50),
    package_height VARCHAR(50)
);

CREATE TABLE vehicle (
    vehicle_id INT PRIMARY KEY,
    vehicle_desc VARCHAR(100),
    vehicle_plate VARCHAR(100)
);

CREATE TABLE delivery (
    delivery_id INT PRIMARY KEY AUTO_INCREMENT,
    delivery_name VARCHAR(100),
    vehicle_id INT,

    FOREIGN KEY (vehicle_id) REFERENCES vehicle(vehicle_id)
);

CREATE TABLE status_shipment (
    status_id INT PRIMARY KEY AUTO_INCREMENT,
    status_desc VARCHAR(100) 
);

CREATE TABLE shipment (
    shipment_id INT PRIMARY KEY AUTO_INCREMENT,
    guide_num VARCHAR(20) UNIQUE NOT NULL,
    client_cid INT,
    receiver_cid INT,
    package_id INT,
    delivery_id INT,
    status_id INT,
    shipment_dt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    destination_address TEXT NOT NULL,

    FOREIGN KEY (client_cid) REFERENCES client(client_cid),
    FOREIGN KEY (package_id) REFERENCES package(package_id),
    FOREIGN KEY (delivery_id) REFERENCES delivery(delivery_id),
    FOREIGN KEY (status_id) REFERENCES status_shipment(status_id)
);