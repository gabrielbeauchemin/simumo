import ndjson
from pathlib import Path
import json

parsed_logs = dict()  # cached passed log files parsed


def parse_log(path):
    with open(path) as f:
        return ndjson.load(f)


def find_entries_in_log(log, min, max):
    def timestamp_to_sec(timestamp):
        hour, min, sec, mili = timestamp.split(":")
        return (int(hour) * 60 * 60) + (int(min) * 60) + int(sec) + int(mili) / 60

    if not min or not max:
        return log
    else:
        return [x for x in log if timestamp_to_sec(min) < timestamp_to_sec(x["timestamp"]) < timestamp_to_sec(max)]


def get_logs_in_range(logPath, min, max):
    if logPath in parsed_logs.keys():
        entries = find_entries_in_log(parsed_logs[logPath], min, max)
        return json.dumps(entries, sort_keys=True, indent=4)
    else:
        log = Path(logPath)
        if log.is_file():
            parsed_log = parse_log(logPath)
            parsed_logs[logPath] = parsed_log
            entries = find_entries_in_log(parsed_log, min, max)
            return json.dumps(entries, sort_keys=True, indent=4)
        else:
            return ""
