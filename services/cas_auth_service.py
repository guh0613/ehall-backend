import re

from utils.common_utils import get_cas_url
from utils.request_utils import get_auth_headers, get_auth_submit_headers
from utils.encryption_utils import aes_cbc_encrypt_url, generate_random_string
import requests


def cas_authenticate(school_name: str, username: str, password: str) -> tuple[dict, int]:
    cas_url = get_cas_url(school_name)
    if cas_url is None:
        return {
            'status': 'error',
            'message': f'No CAS URL found for {school_name}'
        }, 400
    s = requests.Session()
    s.headers.update(get_auth_headers(school_name))
    auth_response = s.get(cas_url,verify=False)
    # check the response body,and use regex to find the password salt and execution
    pattern = (r'<input type="hidden" id="pwdEncryptSalt" value="(.+?)" /><input type="hidden" id="execution" '
               r'name="execution" value="(.+?)" />')
    match = re.search(pattern, auth_response.text)
    if match is None:
        return {
            'status': 'error',
            'message': 'Failed to get password salt and execution'
        }, 500
    salt = match.group(1)
    execution = match.group(2)
    # encrypt the password, iv is the salt
    encrypted_password = aes_cbc_encrypt_url((generate_random_string(64)+password).encode(), salt.encode(), salt.encode())
    submit_data = {
        'username': username,
        'password': encrypted_password,
        'captcha': '',
        'lt': '',
        'cllt': 'userNameLogin',
        'dllt': 'generalLogin',
        'execution': execution,
        '_eventId': 'submit',
    }
    submit_response = s.post(cas_url, data=submit_data, headers=get_auth_submit_headers(school_name), verify=False)
    # if success, the response will have a location header, follow the redirect and get the ticket and castgc
    # check if the request was redirected
    if submit_response.history:
        for resp in submit_response.history:
            location = resp.headers.get('Location')
            ticket = re.search(r'ticket=(ST-\d+-\w+)', location).group(1)
            castgc = re.search(r'CASTGC=(TGT-\d+-\w+);', resp.headers.get('Set-Cookie')).group(1)
            mod_auth_cas = "MOD_AUTH_" + ticket
            return {'status': 'OK',
                    'message': 'Logged in successfully',
                    'castgc': castgc,
                    'mod_auth_cas': mod_auth_cas}, 200
    else:
        return {'status': 'error', 'message': 'Failed to login'}, 400
    return {
        'status': 'error',
        'message': 'Unknown error. Please try again'
    }, 400
