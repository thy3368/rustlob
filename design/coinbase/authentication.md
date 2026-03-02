# API Authentication

> Source: https://docs.cdp.coinbase.com/api-reference/authentication.md

---

## Overview

Coinbase Developer Platform (CDP) uses server and client API keys to authenticate access.

- **Secret API Keys**: For server-to-server communication (REST APIs)
- **Client API Keys**: For client-side communication (JSON-RPC)

---

## Prerequisites

1. Log into an existing CDP account at https://portal.cdp.coinbase.com
2. Create an API key from the API keys dashboard

---

## Step 1: Create an API Key

1. Navigate to your API keys dashboard
2. Select your desired project from the top dropdown
3. Click **Create API key** and give it a name

### Configuration Options

- **IP Allowlist**: Restrict key to specific IP addresses
- **Permissions**: Configure granular permissions (trade, transfer, etc.)
- **Signature Algorithm**: Choose Ed25519 (Recommended) or ECDSA

### Save Your Credentials

After creating, save the following securely:
- **API Key ID** (e.g., `organizations/{org_id}/apiKeys/{key_id}`)
- **Secret Key** (PEM format)

<Warning>
  Never include Secret API key information in your code. Use environment variables instead.
</Warning>

---

## Step 2: Generate JWT

CDP APIs use JWT for authentication. JWTs are valid for 2 minutes.

### Environment Setup

```bash
export KEY_NAME="organizations/{org_id}/apiKeys/{key_id}"
export KEY_SECRET="-----BEGIN EC PRIVATE KEY-----\nYOUR PRIVATE KEY\n-----END EC PRIVATE KEY-----"
export REQUEST_METHOD="GET"
export REQUEST_PATH="/api/v3/brokerage/accounts"
export REQUEST_HOST="api.coinbase.com"
```

### Python Example

```python
import jwt
from cryptography.hazmat.primitives import serialization
import time
import secrets
import os

key_name = os.getenv('KEY_NAME')  
key_secret = os.getenv('KEY_SECRET') 

def build_jwt(uri):
    private_key_bytes = key_secret.encode('utf-8')
    private_key = serialization.load_pem_private_key(private_key_bytes, password=None)
    
    jwt_payload = {
        'sub': key_name,
        'iss': "cdp",
        'nbf': int(time.time()),
        'exp': int(time.time()) + 120,
        'uri': uri,
    }
    
    jwt_token = jwt.encode(
        jwt_payload,
        private_key,
        algorithm='ES256',
        headers={'kid': key_name, 'nonce': secrets.token_hex()},
    )
    return jwt_token

# Generate
uri = f"GET api.coinbase.com/api/v3/brokerage/accounts"
jwt_token = build_jwt(uri)

# Export
import os
os.environ['JWT'] = jwt_token
```

### JavaScript Example

```javascript
const { sign } = require("jsonwebtoken");
const crypto = require("crypto");

const keyName = process.env.KEY_NAME;
const keySecret = process.env.KEY_SECRET;
const algorithm = "ES256";
const uri = `GET api.coinbase.com/api/v3/brokerage/accounts`;

const token = sign(
  {
    iss: "cdp",
    nbf: Math.floor(Date.now() / 1000),
    exp: Math.floor(Date.now() / 1000) + 120,
    sub: keyName,
    uri,
  },
  keySecret,
  {
    algorithm,
    header: {
      kid: keyName,
      nonce: crypto.randomBytes(16).toString("hex"),
    },
  }
);

console.log("export JWT=" + token);
```

---

## Step 3: Make Authenticated Requests

```bash
# Using JWT
curl -X GET "https://api.coinbase.com/api/v3/brokerage/accounts" \
  -H "Authorization: Bearer $JWT"
```

```python
import requests

response = requests.get(
    "https://api.coinbase.com/api/v3/brokerage/accounts",
    headers={"Authorization": f"Bearer {jwt_token}"}
)
print(response.json())
```

---

## JWT Structure

### Header
```json
{
  "alg": "ES256",
  "typ": "JWT",
  "kid": "organizations/{org_id}/apiKeys/{key_id}",
  "nonce": "random-hex-string"
}
```

### Payload
```json
{
  "sub": "organizations/{org_id}/apiKeys/{key_id}",
  "iss": "cdp",
  "nbf": 1699900000,
  "exp": 1699900100,
  "uri": "GET api.coinbase.com/api/v3/brokerage/accounts"
}
```

---

## Required Dependencies

| Language | Dependencies |
|----------|-------------|
| Python | `PyJWT`, `cryptography` |
| JavaScript | `jsonwebtoken` |
| TypeScript | `jsonwebtoken`, `@types/jsonwebtoken` |
| Go | `gopkg.in/go-jose/go-jose.v2` |
| Ruby | `jwt`, `openssl` |
| PHP | `firebase/php-jwt` |
| Java | `nimbus-jose-jwt`, `bcpkix-jdk18on` |
| C# | `Microsoft.IdentityModel.Tokens`, `System.IdentityModel.Tokens.Jwt` |

---

## Security Best Practices

1. **Never commit secrets** to version control
2. **Use environment variables** for API keys
3. **Rotate keys regularly**
4. **Set IP allowlists** when possible
5. **Use minimum required permissions**
6. **JWTs expire in 2 minutes** - regenerate for each request
