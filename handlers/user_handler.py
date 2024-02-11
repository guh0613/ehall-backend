from services.user_info_service import get_user_info
from services.user_score_service import get_user_score


def user_info_handler(school_name: str, token: str) -> tuple[dict, int]:
    if token is None:
        return {'status': 'error', 'message': 'auth_token not found'}, 400

    return get_user_info(school_name, token)


def user_score_handler(school_name: str, token: str, requestdata: dict) -> tuple[dict, int]:
    if token is None:
        return {'status': 'error', 'message': 'auth_token not found'}, 400
    semester = requestdata.get('semester', "2022-2023-2,2023-2024-1")
    amount = requestdata.get('amount', 10)

    return get_user_score(school_name, token, semester, amount)
