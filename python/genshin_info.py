import json
import genshinhelper
import sys
import os


def ys_info(cookie: str) -> dict:
    sys.stdout = open(os.devnull, 'w')
    result = {}
    gs = genshinhelper.YuanShen(cookie)
    result['data'] = gs.user_data
    sys.stdout = sys.__stdout__
    return result


cookie = sys.argv[1]
encoder = json.JSONEncoder()
encoder.ensure_ascii = False
print(encoder.encode(ys_info(cookie)))
