import os
from bottle import route, run, template, app, response

@route('/', defaults={'path': ''})
@route('/<path:path>')
def all(path="/"):

    data_path = "data/" + path

    if not os.path.exists(data_path):
        response.status = 404
        return "not there"

    if os.path.isfile(data_path):
        try:
            with open(data_path, "rb") as in_file:
                response.content_type = "application/json"
                return in_file.read()
        except IOError:
            return "nope"
    else:
        listdir = sorted(os.listdir(data_path))
        sha1 = file_contents(data_path + "/.sha1") + "\n"
        contents = []
        for item in listdir:
            if item.endswith(".sha1"):
                continue
            else:
                line = item + " "
                path_item = (data_path + "/" + item).replace("//", "/")
                if os.path.isfile(path_item):
                    line += file_contents(path_item + ".sha1") + "\n"
                else:
                    line += file_contents(path_item + "/.sha1") + "\n"
            contents += line
        response.content_type = "text/plain"
        return sha1 + "\n" + "\n".join(sorted(contents))


def file_contents(file):
    with open(file, "rb") as in_file:
        return in_file.read()


run(host='localhost', port=8080)
