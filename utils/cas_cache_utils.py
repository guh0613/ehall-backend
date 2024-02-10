import re

from cachetools import TTLCache
import requests

from utils.common_utils import get_cas_url, get_ehall_url
from utils.request_utils import get_auth_headers, default_header

cache = TTLCache(maxsize=1024, ttl=1800)


def is_mod_auth_cas_expired(school_name, mod_auth_cas):
    result = query_user_info(school_name, mod_auth_cas)
    if result == 200:
        return False
    return True


# 假设的使用CASTGC刷新MOD_AUTH_CAS的方法
def refresh_mod_auth_cas(school_name, castgc):
    cas_url = get_cas_url(school_name)
    s = requests.Session()
    s.headers.update(get_auth_headers(school_name))
    s.cookies.set('CASTGC', castgc)
    auth_response = s.get(cas_url, verify=False)
    if auth_response.history:
        for resp in auth_response.history:
            location = resp.headers.get('Location')
            ticket = re.search(r'ticket=([^&#]+)', location).group(1)
            mod_auth_cas = "MOD_AUTH_" + ticket
            return mod_auth_cas
    else:
        return None
    return None


def get_mod_auth_cas(school_name, castgc):
    mod_auth_cas = cache.get(castgc)
    if mod_auth_cas is None or is_mod_auth_cas_expired(school_name, mod_auth_cas):
        mod_auth_cas = refresh_mod_auth_cas(school_name, castgc)
        if mod_auth_cas is None:
            return None
        cache[castgc] = mod_auth_cas
    return mod_auth_cas


def set_mod_auth_cas(castgc, mod_auth_cas):
    cache[castgc] = mod_auth_cas
    return True


def query_user_info(school_name: str, mod_auth_cas: str) -> int:
    ehall_url = get_ehall_url(school_name)
    # get user info
    s = requests.Session()
    s.cookies.set('MOD_AUTH_CAS', mod_auth_cas)
    s.headers.update(default_header)

    query_url = ehall_url + '//jsonp/ywtb/info/getUserInfoAndSchoolInfo.json'
    response = s.get(query_url, verify=False)

    # check if the response is valid, and get 'username', 'userid', 'userType', 'userDepartment',  'userSex'
    if response.status_code != 200:
        return 400
    try:
        user_info_orig = response.json()['data']
    except Exception:
        return 400
    user_info: dict = {'status': 'OK', 'message': 'User info retrieved successfully',
                       'data': {
                           'userName': user_info_orig['userName'], 'userId': user_info_orig['userId'],
                           'userType': user_info_orig['userTypeName'],
                           'userDepartment': user_info_orig['userDepartment'],
                           'userSex': user_info_orig['userSex']
                       }
                       }
    # if MOD_AUTH_CAS is invalid, the values in "data" will be null
    if user_info['data']['userName'] is None:
        return 401

    return 200
