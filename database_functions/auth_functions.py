from passlib.context import CryptContext

# Create a Passlib context for Argon2
pwd_context = CryptContext(schemes=["argon2"], deprecated="auto")

def hash_password(password: str):
    # Use the Passlib context to hash the password
    hashed_password = pwd_context.hash(password)
    return hashed_password

def verify_password(cnx, username: str, password: str) -> bool:
    cursor = cnx.cursor(buffered=True)
    print('checking pw')
    cursor.execute("SELECT Hashed_PW FROM Users WHERE Username = %s", (username,))
    result = cursor.fetchone()
    cursor.close()

    if not result:
        return False  # User not found

    stored_hashed_password = result[0]

    # Use the Passlib context to verify the password against the stored hash
    return pwd_context.verify(password, stored_hashed_password)

import jwt
from cryptography.hazmat.backends import default_backend
from cryptography.hazmat.primitives import serialization
from cryptography.hazmat.primitives.asymmetric.rsa import RSAPublicKey
from cryptography.hazmat.primitives.asymmetric import rsa
from jwt.algorithms import RSAAlgorithm
import requests
import base64

def fetch_azure_ad_public_keys(tenant_id):
    """ Fetches the public keys from Azure AD's discovery document. """
    open_id_config_url = f"https://login.microsoftonline.com/{tenant_id}/v2.0/.well-known/openid-configuration"
    config = requests.get(open_id_config_url).json()
    jwks_uri = config['jwks_uri']
    jwks = requests.get(jwks_uri).json()
    return jwks

def jwks_to_pem(jwks):
    """ Converts a JWKS to a PEM format. """
    for jwk in jwks.get('keys', []):
        if jwk['kty'] == 'RSA':
            public_num = rsa.RSAPublicNumbers(
                e=int.from_bytes(base64.urlsafe_b64decode(jwk['e'] + '=='), 'big'),
                n=int.from_bytes(base64.urlsafe_b64decode(jwk['n'] + '=='), 'big')
            )
            public_key = public_num.public_key(default_backend())
            pem = public_key.public_bytes(
                encoding=serialization.Encoding.PEM,
                format=serialization.PublicFormat.SubjectPublicKeyInfo
            )
            return pem
    raise Exception("Valid RSA public key not found in JWKS.")

def decode_id_token(id_token, tenant_id, client_id):
    """ Decodes and validates an ID token from Azure AD using a PEM-formatted public key. """
    jwks = fetch_azure_ad_public_keys(tenant_id)
    pem = jwks_to_pem(jwks)

    # Decode and validate the token
    try:
        # Options to validate iss, aud, and exp claims
        options = {
            'verify_exp': True,
            'verify_aud': True,
            'verify_iss': True
        }
        # Claims to validate
        validation = {
            'aud': client_id,
            'iss': f"https://login.microsoftonline.com/{tenant_id}/v2.0"
        }
        # Decode token using the PEM key
        decoded = jwt.decode(id_token, pem, algorithms=["RS256"], options=options, audience=validation['aud'], issuer=validation['iss'])
        return decoded
    except jwt.ExpiredSignatureError:
        raise Exception("The token has expired.")
    except jwt.InvalidTokenError as e:
        raise Exception(f"Token validation failed: {str(e)}")
    except Exception as e:
        raise Exception(f"Token decoding failed: {str(e)}")