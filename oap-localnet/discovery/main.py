from fastapi import FastAPI, HTTPException, Body
from pydantic import BaseModel
from typing import List, Dict, Optional, Set
import collections
import time

app = FastAPI(title="OAP Mock Discovery", description="PSI Server for LocalNet")

# In-memory storage
contacts_db: Dict[str, str] = {}
system_logs = collections.deque(maxlen=50)

def log(msg: str):
    entry = f"[{time.strftime('%H:%M:%S')}] {msg}"
    print(entry)
    system_logs.append(entry)

class PsiRequest(BaseModel):
    identifiers: List[str]

class AddContactRequest(BaseModel):
    identifier: str
    did: str

@app.get("/health")
def health_check():
    return {"status": "ok"}

@app.get("/system/logs")
def get_logs():
    return list(system_logs)

@app.post("/psi/intersect")
def psi_intersect(req: PsiRequest):
    matches = {}
    for identifier in req.identifiers:
        if identifier in contacts_db:
            matches[identifier] = contacts_db[identifier]
    
    log(f"PSI Request for {len(req.identifiers)} ids, found {len(matches)} matches")
    return {"matches": matches}

@app.post("/admin/add-contact")
def add_contact(req: AddContactRequest):
    contacts_db[req.identifier] = req.did
    log(f"Linked {req.identifier} -> {req.did}")
    return {"status": "added", "identifier": req.identifier, "did": req.did}

@app.get("/admin/list")
def list_contacts():
    return contacts_db

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8081)
