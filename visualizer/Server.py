from flask import Flask, render_template, request
import RenderOsmMap
from ParseLogs import get_logs_in_range
import yaml
import sys
import os

app = Flask(__name__,
            static_url_path="",
            static_folder="output",
            template_folder="output")


@app.route('/')
def send_visualization_layout():
    return render_template('layout.html')


@app.route('/logs')
def send_metric():
    logPath = request.args.get('logPath')
    min = request.args.get('min')
    max = request.args.get('max')
    return get_logs_in_range(logPath, min, max)


if __name__ == "__main__":
    with open(sys.argv[1], 'rt', encoding='utf8') as stream:
        try:
            config = yaml.load(stream)
            directoryPath = config['logs']['directory']
            for metric in config['logs']['metrics']:
                metric["logName"] = os.path.join(directoryPath, metric["logName"])
            RenderOsmMap.render_visualization(config['city'], config['logs']['metrics'], config['legends'])
        except yaml.YAMLError as exc:
            print("The Visualization config is not in valid Yaml")

    app.run("localhost")
