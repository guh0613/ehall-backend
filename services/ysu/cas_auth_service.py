import re

from utils.request_utils import general_cas_login


def cas_authenticate(school_name: str, username: str, password: str) -> tuple[dict, int]:
    submit_response = general_cas_login(school_name, username, password)
    # response object will be requests.Response() if the login failed
    if submit_response.status_code != 200:
        return {'status': 'retry', 'message': 'Failed to login.please try again'}, 402

    # if success, the response will have a location header, follow the redirect and get the ticket and castgc
    # check if the request was redirected
    if submit_response.history:
        for resp in submit_response.history:
            location = resp.headers.get('Location')
            ticket = re.search(r'ticket=([^&#]+)', location).group(1)
            castgc = resp.cookies.get('CASTGC')
            return {'status': 'OK',
                    'message': 'Login successful',
                    'auth_token': castgc}, 200
    else:
        return {'status': 'error', 'message': 'Failed to login.Please check your username or password'}, 400

    return {
        'status': 'retry',
        'message': 'Unknown error. Please try again'
    }, 402
