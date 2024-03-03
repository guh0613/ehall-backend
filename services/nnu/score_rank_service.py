import requests

from utils.common_utils import get_ehallapp_url, get_ehall_url
from utils.nnu.cas_cache_utils import get_mod_auth_cas
from utils.request_utils import default_header


def get_score_rank(school_name: str, token: str, course_id: str, class_id: str, semester: str) -> tuple[dict, int]:
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

    # get userid
    query_user_info_url = ehall_url + '/jsonp/ywtb/info/getUserInfoAndSchoolInfo'
    response = s.get(query_user_info_url)
    user_id = response.json()['data']['userId']

    query_weu_url = ehall_url + '/appShow?appId=4768574631264620'
    response = s.get(query_weu_url)
    if response.status_code != 200:
        return {'status': 'retry', 'message': 'Failed to get user score.Please try again'}, 402

    s.headers['Accept'] = 'application/json, text/javascript, */*; q=0.01'

    query_score_url = ehallapp_url + '/jwapp/sys/cjcx/modules/cjcx/jxbcjtjcx.do'
    query_class_score_data = {
        'JXBID': class_id,
        'XNXQDM': semester,
        'TJLX': '01'
    }
    query_school_score_data = {
        'JXBID': '*',
        'XNXQDM': semester,
        'KCH': course_id,
        'TJLX': '02'
    }
    response_class_score = s.post(query_score_url, data=query_class_score_data)
    response_school_score = s.post(query_score_url, data=query_school_score_data)
    response_json1 = response_class_score.json()
    response_json2 = response_school_score.json()
    if response_json1['datas']['jxbcjtjcx']['extParams']['code'] != 1 or \
            response_json2['datas']['jxbcjtjcx']['extParams']['code'] != 1:
        return {'status': 'invalid', 'message': 'Failed to get score rank.Params are probably invalid'}, 400

    res_data = {"status": "OK", "message": "Score rank retrieved successfully", "data": {
        "class": {"highScore": response_json1['datas']['jxbcjtjcx']['rows'][0]['ZGF'],
                  "lowScore": response_json1['datas']['jxbcjtjcx']['rows'][0]['ZDF'],
                  "averageScore": response_json1['datas']['jxbcjtjcx']['rows'][0]['PJF']
                  },
        "school": {"highScore": response_json2['datas']['jxbcjtjcx']['rows'][0]['ZGF'],
                   "lowScore": response_json2['datas']['jxbcjtjcx']['rows'][0]['ZDF'],
                   "averageScore": response_json2['datas']['jxbcjtjcx']['rows'][0]['PJF']
                   }
    }
                }

    query_people_url = ehallapp_url + '/jwapp/sys/cjcx/modules/cjcx/jxbcjfbcx.do'
    query_class_people_data = {
        'JXBID': class_id,
        'XNXQDM': semester,
        'TJLX': '01',
        '*order': '+DJDM'
    }
    query_school_people_data = {
        'JXBID': '*',
        'XNXQDM': semester,
        'KCH': course_id,
        'TJLX': '02',
        '*order': '+DJDM'
    }
    response_class_people = s.post(query_people_url, data=query_class_people_data)
    response_school_people = s.post(query_people_url, data=query_school_people_data)
    response_json = {'class': response_class_people.json(), 'school': response_school_people.json()}
    if response_json['class']['datas']['jxbcjfbcx']['extParams']['code'] != 1 or \
            response_json['school']['datas']['jxbcjfbcx']['extParams']['code'] != 1:
        return {'status': 'invalid', 'message': 'Failed to get score rank.Params are probably invalid'}, 401

    res_data['data']['class']['numAbove90'] = response_json['class']['datas']['jxbcjfbcx']['rows'][0].get('DJSL', 0)
    res_data['data']['school']['numAbove90'] = response_json['school']['datas']['jxbcjfbcx']['rows'][0].get('DJSL', 0)

    # process class and school in one loop
    for i in range(len(response_json['class']['datas']['jxbcjfbcx']['rows'])):
        res_data['data']['class'][f'numAbove{90 - i * 10}' if i != 4 else 'numBelow60'] = response_json['class']['datas']['jxbcjfbcx']['rows'][i].get(
            'DJSL', 0)
        res_data['data']['school'][f'numAbove{90 - i * 10}' if i != 4 else 'numBelow60'] = response_json['school']['datas']['jxbcjfbcx']['rows'][i].get(
            'DJSL', 0)

    query_rank_url = ehallapp_url + '/jwapp/sys/cjcx/modules/cjcx/jxbxspmcx.do'
    query_class_rank_data = {
        'XH': user_id,
        'JXBID': class_id,
        'XNXQDM': semester,
        'TJLX': '01'
    }
    query_school_rank_data = {
        'XH': user_id,
        'JXBID': '*',
        'XNXQDM': semester,
        'KCH': course_id,
        'TJLX': '02'
    }
    response_class_rank = s.post(query_rank_url, data=query_class_rank_data)
    response_school_rank = s.post(query_rank_url, data=query_school_rank_data)
    response_json = {'class': response_class_rank.json(), 'school': response_school_rank.json()}
    if response_json['class']['datas']['jxbxspmcx']['extParams']['code'] != 1 or \
            response_json['school']['datas']['jxbxspmcx']['extParams']['code'] != 1:
        return {'status': 'invalid', 'message': 'Failed to get score rank.Params are probably invalid'}, 401

    res_data['data']['class']['rank'] = response_json['class']['datas']['jxbxspmcx']['rows'][0]['PM']
    res_data['data']['school']['rank'] = response_json['school']['datas']['jxbxspmcx']['rows'][0]['PM']

    res_data['data']['class']['totalPeopleNum'] = response_json['class']['datas']['jxbxspmcx']['rows'][0]['ZRS']
    res_data['data']['school']['totalPeopleNum'] = response_json['school']['datas']['jxbxspmcx']['rows'][0]['ZRS']

    return res_data, 200
