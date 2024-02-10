from services.cas_auth_service import cas_authenticate


def cas_login_handler(school_name: str, login_data: dict):
    # the post data must have username and password, or castgc instead.
    if 'CASTGC' in login_data:
        return cas_authenticate(school_name, castgc=login_data['CASTGC'])
    else:
        if 'username' not in login_data or 'password' not in login_data:
            return {
                'status': 'error',
                'message': 'Username and password are required'
            }, 400
        # authenticate the user
        return cas_authenticate(school_name, login_data['username'], login_data['password'])
