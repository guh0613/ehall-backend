from flask import Flask, request
from handlers.login_handler import cas_login_handler

app = Flask(__name__)


@app.route('/api/cas_login/<school_name>', methods=['POST'])
def cas_login(school_name):
    login_data = request.json
    return cas_login_handler(school_name, login_data)


if __name__ == '__main__':
    app.run()
