import importlib


def cas_login_handler(school_name: str, login_data: dict):
    if 'username' not in login_data or 'password' not in login_data:
        return {
            'status': 'error',
            'message': 'Username and password are required'
        }, 400
    # import the service module and call the cas_authenticate function
    # if the school is not supported, the import will fail
    try:
        service = importlib.import_module(f'services.{school_name}.cas_auth_service')
    except ModuleNotFoundError:
        return {
            'status': 'error',
            'message': f'{school_name} is not supported'
        }, 400

    return service.cas_authenticate(school_name, login_data['username'], login_data['password'])
