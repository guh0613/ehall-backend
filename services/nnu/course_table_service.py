import requests

from utils.nnu.cas_cache_utils import get_mod_auth_cas
from utils.common_utils import get_ehall_url, get_ehallapp_url
from utils.request_utils import default_header


def get_course_table(school_name: str, token: str, semester: str) -> tuple[dict, int]:
    ehallapp_url = get_ehallapp_url(school_name, False)
    ehall_url = get_ehall_url(school_name)

    mod_auth_cas = get_mod_auth_cas(school_name, token)
    if mod_auth_cas is None:
        return {'status': 'error', 'message': 'Failed to get course table. auth_token is probably invalid.'}, 401

    s = requests.Session()
    s.cookies.set('MOD_AUTH_CAS', mod_auth_cas)
    s.headers.update(default_header)
    if 'Referer' in s.headers:
        del s.headers['Referer']

    query_weu_url = ehall_url + '/appShow?appId=4770397878132218'
    response = s.get(query_weu_url, verify=False)
    if response.status_code != 200:
        return {'status': 'retry', 'message': 'Failed to get course table.Please try again'}, 402


