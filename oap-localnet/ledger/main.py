from fastapi import FastAPI, HTTPException, Body
from pydantic import BaseModel
from typing import Dict, Any, Optional
import collections
import time

app = FastAPI(title="OAP Mock Ledger", description="OAPP Ledger for LocalNet")

balances_db: Dict[str, int] = {}
transactions_db: Dict[str, Any] = {}
system_logs = collections.deque(maxlen=50)

def log(msg: str):
    entry = f"[{time.strftime('%H:%M:%S')}] {msg}"
    print(entry)
    system_logs.append(entry)

class TransferRequest(BaseModel):
    sender_did: str
    recipient_did: str
    amount: int
    signature: str 

@app.get("/health")
def health_check():
    return {"status": "ok"}

@app.get("/system/logs")
def get_logs():
    return list(system_logs)

@app.get("/balance/{did}")
def get_balance(did: str):
    return {"did": did, "balance": balances_db.get(did, 0)}

@app.get("/faucet/{did}")
def faucet(did: str):
    amount = 1000 * 1000000 
    current = balances_db.get(did, 0)
    balances_db[did] = current + amount
    log(f"Faucet funded {did} with 1000 OAP")
    return {"status": "funded", "did": did, "new_balance": balances_db[did]}

@app.post("/transfer")
def transfer(req: TransferRequest):
    sender_bal = balances_db.get(req.sender_did, 0)
    if sender_bal < req.amount:
        raise HTTPException(status_code=400, detail="Insufficient funds")
    
    balances_db[req.sender_did] -= req.amount
    balances_db[req.recipient_did] = balances_db.get(req.recipient_did, 0) + req.amount
    
    tx_id = f"tx-{len(transactions_db) + 1}"
    tx = {
        "id": tx_id,
        "sender": req.sender_did,
        "recipient": req.recipient_did,
        "amount": req.amount,
        "status": "confirmed"
    }
    transactions_db[tx_id] = tx
    
    log(f"Transfer {req.amount} from {req.sender_did} to {req.recipient_did}")
    return tx

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8082)
