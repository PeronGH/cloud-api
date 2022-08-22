import json
import genshinhelper
import sys
import os


def sign_ys(cookie: str) -> dict:
    sys.stdout = open(os.devnull, 'w')
    result = {}
    gs = genshinhelper.YuanShen(cookie)
    data: list = gs.daily_note

    for i, val in enumerate(data):
        if "message" in val:
            data.pop(i)

    result['data'] = data
    sys.stdout = sys.__stdout__
    return result


cookie = sys.argv[1]
encoder = json.JSONEncoder()
encoder.ensure_ascii = False
print(encoder.encode(sign_ys(cookie)))
