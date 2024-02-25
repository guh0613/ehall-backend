from configs import config


def get_cas_url(school_name: str) -> str:
    """return the CAS URL for the specified school."""
    return config.CAS_SERVER_URLS.get(school_name, None)


def get_ehall_url(school_name: str) -> str:
    """return the eHall URL for the specified school."""
    return config.EHALL_SERVER_URLS.get(school_name, None)


def get_ehallapp_url(school_name: str, ishttps: bool = True) -> str:
    """return the eHallApp URL for the specified school."""
    if ishttps:
        return config.EHALLAPP_SERVER_URLS.get(school_name, None)
    else:
        # convert https url to http url
        url = config.EHALLAPP_SERVER_URLS.get(school_name, None)
        return url
