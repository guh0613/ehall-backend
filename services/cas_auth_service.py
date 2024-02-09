import re
import time

from utils.common_utils import get_cas_url
from utils.request_utils import get_auth_headers
from utils.encryption_utils import aes_cbc_encrypt_url, random_string
import requests


def cas_authenticate(school_name: str, username: str = '', password: str = '', castgc: str = None) -> tuple[dict, int]:
    cas_url = get_cas_url(school_name)
    if cas_url is None:
        return {
            'status': 'error',
            'message': f'{school_name} is not supported'
        }, 400

    # create a session and get the auth page
    s = requests.Session()
    s.headers.update(get_auth_headers(school_name))

    # check if castgc is provided, if so, use it to authenticate
    if castgc is not None:
        s.cookies.set('CASTGC', castgc)
        auth_response = s.get(cas_url, verify=False)
        if auth_response.history:
            for resp in auth_response.history:
                location = resp.headers.get('Location')
                ticket = re.search(r'ticket=(ST-\d+-\w+)', location).group(1)
                mod_auth_cas = "MOD_AUTH_" + ticket
                return {'status': 'OK',
                        'message': 'Logged in successfully',
                        'mod_auth_cas': mod_auth_cas}, 200
        else:
            return {'status': 'error', 'message': 'Failed to login.CASTGC is probably invalid'}, 400

    # no castgc provided, use the username and password to authenticate
    auth_response = s.get(cas_url, verify=False)

    # check the response body,and use regex to find the password salt and execution
    pattern = (r'<input type="hidden" id="pwdEncryptSalt" value="(.+?)" /><input type="hidden" id="execution" '
               r'name="execution" value="(.+?)" />')
    match = re.search(pattern, auth_response.text)
    if match is None:
        return {
            'status': 'error',
            'message': 'Failed to get password salt and execution'
        }, 400
    salt = match.group(1)
    execution = match.group(2)

    # encrypt the password, iv is randomly generated
    encrypted_password = aes_cbc_encrypt_url((random_string(64) + password).encode(), salt.encode())
    submit_data = {
        'username': username,
        'password': encrypted_password,
        'captcha': '',
        '_eventId': 'submit',
        'cllt': 'userNameLogin',
        'dllt': 'generalLogin',
        'lt': '',
        'execution': execution,

    }

    # sleep for 2 seconds to avoid being blocked
    time.sleep(1)
    submit_response = s.post(cas_url, data=submit_data, headers=get_auth_headers(school_name), verify=False)
    # if success, the response will have a location header, follow the redirect and get the ticket and castgc
    # check if the request was redirected
    if submit_response.history:
        for resp in submit_response.history:
            location = resp.headers.get('Location')
            ticket = re.search(r'ticket=(ST-\d+-\w+)', location).group(1)
            castgc = resp.cookies.get('CASTGC')
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
