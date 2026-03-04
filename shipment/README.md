# Sendcure - shipment Subsystem

This folder contains the Python Textual UI for shipment operations in Sendcure.

## Run shipment

From repository root:

```bash
cd shipment
pip install -r requirements.txt
python main.py
```

Single command from repository root:

```bash
cd shipment && python main.py
```

## Requirements

- Python 3.11+
- `main-server` running on `http://localhost:8000`

## How to test

1) Start backend (`main-server`):

```bash
npm install
npm run dev
```

2) (Optional) Health check:

```bash
curl http://127.0.0.1:8000/health
```

3) Start shipment:

```bash
cd shipment && python main.py
```

4) Login:

- CI: `9876543`
- Password: `elatla`

## Notes

- Press `R` to refresh and `Q` to quit.


