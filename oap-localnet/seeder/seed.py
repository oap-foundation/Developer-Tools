import sys
import requests
import time
try:
    import nacl.signing
    import nacl.encoding
except ImportError:
    print("pynacl not installed. Install it with: pip install pynacl")
    sys.exit(1)

LEDGER_URL = "http://ledger:8082"

def generate_identity(name):
    signing_key = nacl.signing.SigningKey.generate()
    verify_key = signing_key.verify_key
    
    secret_hex = signing_key.encode(encoder=nacl.encoding.HexEncoder).decode('utf-8')
    public_hex = verify_key.encode(encoder=nacl.encoding.HexEncoder).decode('utf-8')
    
    # Mock DID
    did = f"did:key:{public_hex[:16]}" # Simplified
    
    print(f"--- {name} Identity ---")
    print(f"DID: {did}")
    print(f"Secret Key: {secret_hex}")
    print(f"Public Key: {public_hex}")
    
    # Auto-fund
    try:
        res = requests.get(f"{LEDGER_URL}/faucet/{did}", timeout=5)
        if res.status_code == 200:
            print(f"Funded: 1000 OAP (Balance: {res.json().get('new_balance')})")
        else:
            print(f"Funding Failed: {res.status_code}")
    except Exception as e:
        print(f"Funding Failed (Ledger unreachable?): {e}")

    print(f"----------------------\n")

def wait_for_ledger():
    print("Waiting for Ledger...")
    for _ in range(10):
        try:
            if requests.get(f"{LEDGER_URL}/health", timeout=2).status_code == 200:
                print("Ledger is up!")
                return
        except:
            time.sleep(1)
    print("Warning: Ledger not reachable, proceeding without funding.")

if __name__ == "__main__":
    print("OAP LocalNet Seeder - Generating Dev Identities...\n")
    
    wait_for_ledger()
    
    generate_identity("Alice")
    generate_identity("Bob")
    generate_identity("Mallory")
    
    print("Seeding complete.")
