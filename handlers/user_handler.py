from services.user_info_service import get_user_info


def user_info_handler(school_name: str, mod_auth_cas: str) -> tuple[dict, int]:
    if mod_auth_cas is None:
        return {'status': 'error', 'message': 'Mod_auth_cas not found'}, 400
    # get user info
    return get_user_info(school_name, mod_auth_cas)
