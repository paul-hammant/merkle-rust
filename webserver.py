import os
from bottle import route, run, template, app, response

@route('/', defaults={'path': ''})
@route('/<path:path>')
def all(path="/"):

    if path == "favicon.ico":
        return None
    data_path = "data/" + path
    if os.path.isfile(data_path):
        try:
            with open(data_path, "rb") as in_file:
                response.content_type = "application/json"
                return in_file.read()
        except IOError:
            return "nope"
    else:
        listdir = os.listdir(data_path)
        rv = file_contents(data_path + "/.sha1") + "\n"
        for item in listdir:
            if item.endswith(".sha1"):
                continue
            else:
                rv = rv + item + " "
                path_item = (data_path + "/" + item).replace("//", "/")
                if os.path.isfile(path_item):
                    rv = rv + file_contents(path_item + ".sha1") + "\n"
                else:
                    rv = rv + file_contents(path_item + "/.sha1") + "\n"
        response.content_type = "text/plain"
        return rv


def file_contents(file):
    with open(file, "rb") as in_file:
        return in_file.read()


run(host='localhost', port=8080)
