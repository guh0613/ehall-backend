from configs import config


def get_cas_url(school_name: str) -> str:
    """return the CAS URL for the specified school."""
    return config.CAS_SERVER_URLS.get(school_name, None)


def get_ehall_url(school_name: str) -> str:
    """return the eHall URL for the specified school."""
    return config.EHALL_SERVER_URLS.get(school_name, None)


def get_ehallapp_url(school_name: str) -> str:
    """return the eHallApp URL for the specified school."""
    return config.EHALLAPP_SERVER_URLS.get(school_name, None)
