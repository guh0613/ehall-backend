from Crypto.Cipher import AES
from Crypto.Util.Padding import pad
from base64 import b64encode
from urllib.parse import quote
import random


def aes_cbc_encrypt_url(data, key, iv):
    """
    return the encrypted data in base64 format, and url encoded.
    """
    cipher = AES.new(key, AES.MODE_CBC, iv)
    ct_bytes = cipher.encrypt(pad(data, AES.block_size))
    encrypted_b64 = b64encode(ct_bytes)
    encrypted_b64_url = quote(encrypted_b64.decode())
    return encrypted_b64_url


def generate_random_string(length: int):
    """ return a random string with the specified length."""
    chars = 'ABCDEFGHJKMNPQRSTWXYZabcdefhijkmnprstwxyz2345678'
    return ''.join(random.choice(chars) for _ in range(length))

