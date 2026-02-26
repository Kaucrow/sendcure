CREATE TABLE employee (
    employee_cid INT PRIMARY KEY,
    employee_email VARCHAR(100) UNIQUE NOT NULL,
    employee_name VARCHAR(100) NOT NULL,
    employee_passwd VARCHAR(255) NOT NULL,
    phone_num VARCHAR(20)
);

CREATE TABLE client (
    client_cid INT PRIMARY KEY,
    client_email VARCHAR(100) UNIQUE NOT NULL,
    client_name VARCHAR(100) NOT NULL,
    client_passwd VARCHAR(255) NOT NULL,
    phone_num VARCHAR(20)
);

CREATE TABLE package (
    package_id INT PRIMARY KEY,
    package_desc TEXT,
    package_weight DECIMAL(10, 2),
    package_width INT,
    package_length INT,
    package_height INT
);

CREATE TABLE status_shipment (
    status_id SERIAL PRIMARY KEY,
    status_desc VARCHAR(100)
);

CREATE TABLE shipment (
    shipment_id SERIAL PRIMARY KEY,
    guide_num VARCHAR(20) UNIQUE NOT NULL,
    sender_cid INT,
    receiver_cid INT,
    package_id INT,
    status_id INT,
    shipment_dt TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    destination_address TEXT NOT NULL,

    FOREIGN KEY (sender_cid) REFERENCES client(client_cid),
    FOREIGN KEY (package_id) REFERENCES package(package_id),
    FOREIGN KEY (status_id) REFERENCES status_shipment(status_id)
);