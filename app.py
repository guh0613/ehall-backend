from flask import Flask, request
from handlers.login_handler import cas_login_handler
from handlers.user_handler import user_info_handler

app = Flask(__name__)


@app.route('/api/<school_name>/cas_login', methods=['POST'])
def cas_login(school_name):
    login_data = request.json
    return cas_login_handler(school_name, login_data)


@app.route('/api/<school_name>/user/info', methods=['GET'])
def user_info(school_name):
    mod_auth_cas = request.headers.get('Authorization')
    return user_info_handler(school_name, mod_auth_cas)


if __name__ == '__main__':
    app.run()
