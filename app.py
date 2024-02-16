from flask import Flask, request
from handlers.login_handler import cas_login_handler
from handlers.user_handler import user_info_handler, user_score_handler, score_rank_handler

app = Flask(__name__)


@app.route('/api/<school_name>/cas_login', methods=['POST'])
def cas_login(school_name):
    login_data = request.json
    return cas_login_handler(school_name, login_data)


@app.route('/api/<school_name>/user/info', methods=['GET'])
def user_info(school_name):
    auth_token = request.headers.get('Authorization')
    return user_info_handler(school_name, auth_token)


@app.route('/api/<school_name>/user/score', methods=['GET', 'POST'])
def user_score(school_name):
    auth_token = request.headers.get('Authorization')
    if request.method == 'GET':
        return user_score_handler(school_name, auth_token, {})
    score_request_data = request.json
    return user_score_handler(school_name, auth_token, score_request_data)


@app.route('/api/<school_name>/user/score_rank', methods=['POST'])
def score_rank(school_name):
    auth_token = request.headers.get('Authorization')
    score_rank_request_data = request.json
    return score_rank_handler(school_name, auth_token, score_rank_request_data)


if __name__ == '__main__':
    app.run()
