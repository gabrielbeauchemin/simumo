import argparse
import json
from json import JSONEncoder
from pprint import pprint

import networkx as nx
from osm_graph import OsmGraph
import matplotlib.pyplot as plt

"""
args parser configs
"""
parser = argparse.ArgumentParser(description="Fetch data from OsmGraph and write them in a json",
                                 formatter_class=argparse.ArgumentDefaultsHelpFormatter)
parser.add_argument("path", help="path to the file", type=str)
parser.add_argument("lon", help="longitude", type=float)
parser.add_argument("lat", help="latitude ", type=float)
parser.add_argument("zoom", help="latitude ", type=int)

"""
encoder from prettier jsonification

note :: 
    I was annoyed that my position was not on a single line,
    so i did this silly idea
"""


class OneLineList:
    list = None

    def __init__(self, l):
        self.list = list(l)


class FlatList(JSONEncoder):
    def default(self, o):
        if isinstance(o, OneLineList):
            return "##<{}>##".format(o.list)

    def encode(self, o):
        result = super().encode(o)
        result = result.replace('"##<', "").replace('>##"', "")
        return result


"""
execution
"""


def to_json_graph(lon, lat, zoom):
    print("fetching the graph")
    og = OsmGraph(lon, lat, zoom)
    # for verification
    print("showing the graph")
    show_graph_plt(og, lon, lat)
    return make_json_graph(og)

def make_json_graph(og):
    edges = [[k, v] for k, v in og.graph.edges().keys()]
    connected_node = {k for k, _ in edges}.union(k for _, k in edges)
    return {
        "nodes": {k: OneLineList(v) for k, v in og.pos.items()
                  if k in connected_node},
        "edges": [OneLineList([k, v]) for k, v in edges]
    }

def show_graph_plt(og, lon, lat):
    og.draw_graph()
    plt.show()


def execute(args):
    with open(args.path, "w") as file:
        data = to_json_graph(args.lon, args.lat, args.zoom)
        print("logging ")
        file.write(json.dumps(data, indent=4, cls=FlatList))


if __name__ == "__main__":
    execute(parser.parse_args())
