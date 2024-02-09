from configs import config


def get_cas_url(school_name: str) -> str:
    """return the CAS URL for the specified school."""
    return config.CAS_SERVER_URLS.get(school_name, None)
