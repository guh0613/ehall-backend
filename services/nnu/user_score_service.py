import requests

from utils.nnu.cas_cache_utils import get_mod_auth_cas
from utils.common_utils import get_ehall_url, get_ehallapp_url
from utils.request_utils import default_header


def get_user_score(school_name: str, token: str, semester: str, amount: int) -> tuple[dict, int]:
    ehallapp_url = get_ehallapp_url(school_name)
    ehall_url = get_ehall_url(school_name)

    mod_auth_cas = get_mod_auth_cas(school_name, token)
    if mod_auth_cas is None:
        return {'status': 'error', 'message': 'Failed to get user score. authToken is probably invalid.'}, 401

    s = requests.Session()
    s.cookies.set('MOD_AUTH_CAS', mod_auth_cas)
    s.headers.update(default_header)
    if 'Referer' in s.headers:
        del s.headers['Referer']

    # get weu first,it will automatically be set-cookie
    query_weu_url = ehall_url + '/appShow?appId=4768574631264620'
    response = s.get(query_weu_url)
    if response.status_code != 200:
        return {'status': 'retry', 'message': 'Failed to get user score.Please try again'}, 402

    s.headers['Accept'] = 'application/json, text/javascript, */*; q=0.01'

    query_score_url = ehallapp_url + '/jwapp/sys/cjcx/modules/cjcx/xscjcx.do'
    query_score_data = get_query_score_data(semester, amount)
    response = s.post(query_score_url, data=query_score_data)
    if response.status_code != 200:
        return {'status': 'retry', 'message': 'Failed to get user score.Please try again'}, 402
    # get the response json
    original_json = response.json()
    # transform the json
    return transform_data(original_json), 200


def get_query_score_data(semester: str, amount: int):
    if semester == 'all':
        query_score_data = {
            'querySetting': '[{"name": "SFYX", "caption": "是否有效", "linkOpt": "AND", "builderList": "cbl_m_List","builder": "m_value_equal", "value": "1", "value_display": "是"},{"name": "SHOWMAXCJ", "caption": "显示最高成绩", "linkOpt": "AND", "builderList": "cbl_String","builder": "equal", "value": 0, "value_display": "否"}]',
            '*order': '-XNXQDM, -KCH, -KXH',
            'pageSize': amount,
            'pageNumber': 1
        }
        return query_score_data
    else:
        query_score_data = {
            'querySetting': '[{"name":"XNXQDM","value":"' + semester + '","linkOpt":"and","builder":"m_value_equal"},{"name":"SFYX","caption":"是否有效","linkOpt":"AND","builderList":"cbl_m_List","builder":"m_value_equal","value":"1","value_display":"是"}]',
            '*order': 'KCH,KXH',
            'pageSize': amount,
            'pageNumber': 1
        }
        return query_score_data


def transform_data(original_json):
    result = {
        "status": "OK",
        "message": "User score retrieved successfully",
        "totalCount": original_json["datas"]["xscjcx"]["totalSize"],
        "data": []
    }

    for row in original_json.get("datas", {}).get("xscjcx", {}).get("rows", []):
        course_data = {
            "courseName": row.get("XSKCM", "Unknown Course"),
            "courseID": row.get("KCH", ""),
            "classID": row.get("JXBID", ""),
            "examTime": row.get("KSSJ", ""),
            "totalScore": row.get("ZCJ", 0),
            "gradePoint": str(round(row.get("XFJD", 0), 1)),
            "regularScore": row.get("PSCJ", ""),
            "midScore": row.get("QZCJ", ""),
            "finalScore": row.get("QMCJ", ""),
            "regularPercent": row.get("PSCJXS", ""),
            "midPercent": row.get("QZCJXS", ""),
            "finalPercent": row.get("QMCJXS", ""),
            "courseType": row.get("KCXZDM_DISPLAY", ""),
            "courseCate": row.get("KCLBDM_DISPLAY", ""),
            "isRetake": row.get("CXCKDM_DISPLAY", ""),
            "credits": row.get("XF", 0),
            "gradeType": row.get("DJCJLXDM_DISPLAY", ""),
            "semester": row.get("XNXQDM", ""),
            "department": row.get("KKDWDM_DISPLAY", "")
        }

        other_scores = {}
        for i in range(1, 9):
            qtcj_key = f"QTCJ{i}"
            qtcj_value = row.get(qtcj_key)
            if qtcj_value:
                other_scores[f"otherScore{i}"] = qtcj_value
        course_data.update(other_scores)

        result["data"].append(course_data)

    return result
