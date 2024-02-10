from services.cas_auth_service import cas_authenticate


def cas_login_handler(school_name: str, login_data: dict):
    if 'username' not in login_data or 'password' not in login_data:
        return {
            'status': 'error',
            'message': 'Username and password are required'
        }, 400
    return cas_authenticate(school_name, login_data['username'], login_data['password'])
