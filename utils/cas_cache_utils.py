import re

from cachetools import TTLCache
import requests

from utils.common_utils import get_cas_url, get_ehall_url
from utils.request_utils import get_auth_headers, default_header

cache = TTLCache(maxsize=1024, ttl=1800)


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
    if mod_auth_cas is None:
        mod_auth_cas = refresh_mod_auth_cas(school_name, castgc)
        if mod_auth_cas is None:
            return None
        cache[castgc] = mod_auth_cas
    return mod_auth_cas


def set_mod_auth_cas(castgc, mod_auth_cas):
    cache[castgc] = mod_auth_cas
    return True
