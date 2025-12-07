from fastapi import FastAPI, HTTPException, Body, Request, Response
from pydantic import BaseModel
from typing import List, Dict, Any, Optional
import time
import uuid
import collections
import random
import asyncio
import os

app = FastAPI(title="OAP Mock Relay", description="Introspectable, in-memory OATP Relay for LocalNet")

# In-memory storage
# { message_id: { ... } }
messages_db: Dict[str, Any] = {}

# System Logs (Circular Interval)
system_logs = collections.deque(maxlen=50)

def log(msg: str):
    entry = f"[{time.strftime('%H:%M:%S')}] {msg}"
    print(entry)
    system_logs.append(entry)

# Chaos Configuration
chaos_config = {
    "failure_rate": 0.0,
    "latency_ms": 0,
    "corrupt_bytes": False
}

class JweMessage(BaseModel):
    payload: str 

class InboxRequest(BaseModel):
    message: str 
    recipient: str 

class ChaosConfig(BaseModel):
    failure_rate: float
    latency_ms: int
    corrupt_bytes: bool

@app.middleware("http")
async def chaos_middleware(request: Request, call_next):
    # Skip chaos for system endpoints to avoid breaking the dashboard/controls
    if request.url.path.startswith("/system") or request.url.path.startswith("/chaos") or request.url.path.startswith("/health"):
        return await call_next(request)

    # 1. Latency
    if chaos_config["latency_ms"] > 0:
        delay = chaos_config["latency_ms"] / 1000.0
        await asyncio.sleep(delay)

    # 2. Failure
    if chaos_config["failure_rate"] > 0:
        if random.random() < chaos_config["failure_rate"]:
            log(f"Chaos: Simulated 500 Error for {request.method} {request.url.path}")
            return Response(content="Chaos Monkey says no!", status_code=500)

    response = await call_next(request)
    return response

@app.get("/health")
def health_check():
    return {"status": "ok", "uptime": "forever", "id": os.getenv("HOSTNAME", "unknown")}

@app.post("/chaos")
def configure_chaos(cfg: ChaosConfig):
    chaos_config["failure_rate"] = cfg.failure_rate
    chaos_config["latency_ms"] = cfg.latency_ms
    chaos_config["corrupt_bytes"] = cfg.corrupt_bytes
    log(f"Chaos Config Updated: {chaos_config}")
    return {"status": "updated", "config": chaos_config}

@app.get("/system/logs")
def get_logs():
    return list(system_logs)

@app.post("/inbox")
def receive_message(req: InboxRequest):
    msg_id = str(uuid.uuid4())
    
    content = req.message
    
    # 3. Corruption
    if chaos_config["corrupt_bytes"]:
        # Simulate bit-flip by just changing the string lightly if it's long enough
        if len(content) > 5:
            # Simple corruption: replace last char
            content = content[:-1] + "X"
            log(f"Chaos: Corrupted message content for {msg_id}")

    stored_msg = {
        "id": msg_id,
        "recipient": req.recipient,
        "content": content,
        "received_at": time.time(),
        "status": "stored"
    }
    messages_db[msg_id] = stored_msg
    log(f"Relay Received message for {req.recipient}: {msg_id}")
    return {"status": "accepted", "message_id": msg_id}

@app.get("/messages")
def list_messages(recipient: Optional[str] = None):
    if recipient:
        return [m for m in messages_db.values() if m["recipient"] == recipient]
    return list(messages_db.values())

@app.delete("/messages")
def clear_messages():
    messages_db.clear()
    return {"status": "cleared"}

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8080)
