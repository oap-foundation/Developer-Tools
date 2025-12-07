from fastapi import FastAPI, HTTPException
from fastapi.staticfiles import StaticFiles
from fastapi.responses import HTMLResponse
import requests
import asyncio
from typing import Dict, Any, List

app = FastAPI(title="OAP LocalNet Dashboard")

SERVICES = {
    "relay1": "http://relay1:8080",
    "relay2": "http://relay2:8080",
    "relay3": "http://relay3:8080",
    "discovery": "http://discovery:8081",
    "ledger": "http://ledger:8082"
}

@app.get("/api/status")
async def get_status():
    status = {}
    for name, url in SERVICES.items():
        try:
            res = requests.get(f"{url}/health", timeout=1)
            status[name] = "online" if res.status_code == 200 else "error"
        except:
            status[name] = "offline"
    return status

@app.get("/api/logs")
async def get_logs():
    logs = {}
    for name, url in SERVICES.items():
        try:
            res = requests.get(f"{url}/system/logs", timeout=1)
            if res.status_code == 200:
                logs[name] = res.json()
            else:
                logs[name] = []
        except:
            logs[name] = []
    return logs

@app.post("/api/chaos/{relay_id}")
async def set_chaos(relay_id: str, config: Dict[str, Any]):
    if relay_id not in ["relay1", "relay2", "relay3"]:
        raise HTTPException(status_code=400, detail="Invalid relay")
    
    url = SERVICES[relay_id]
    try:
        res = requests.post(f"{url}/chaos", json=config, timeout=2)
        return res.json()
    except Exception as e:
        raise HTTPException(status_code=500, detail=str(e))

app.mount("/", StaticFiles(directory="static", html=True), name="static")

if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=3000)
