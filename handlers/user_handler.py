from services.user_info_service import get_user_info


def user_info_handler(school_name: str, token: str) -> tuple[dict, int]:
    if token is None:
        return {'status': 'error', 'message': 'auth_token not found'}, 400
    # get user info
    return get_user_info(school_name, token)
