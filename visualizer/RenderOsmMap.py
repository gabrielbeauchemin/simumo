import jinja2
from distutils.dir_util import copy_tree
import os
import shutil

dirname = os.path.dirname(__file__)  # absolute path to this script folder


def render_ol_map(map_output_path, city):
    # copy directory containing the outputmap and its ressources
    geocoder_path = os.path.join(dirname, "ol-geocoder/template")
    map_output_path = os.path.join(dirname, "output")
    if os.path.isdir(map_output_path):
        shutil.rmtree(map_output_path)  # in case the command was run before
    copy_tree(geocoder_path, map_output_path)

    # generate javascript file (OlMapTemplate.j2) from template so that:
    # - the map generated is zoomed at the requested city
    # - the map contains heatmap point to represent the selected metric
    template_env = jinja2.Environment(loader=jinja2.FileSystemLoader(map_output_path))
    template = template_env.get_template("OlMapTemplate.j2")
    map_rendered = template.render(initialLocation="\"" + city + "\"")
    with open(map_output_path + "/OlMapTemplate.js", "wb") as text_file:
        map_rendered = template.render(initialLocation="\"" + city + "\"").encode("utf-8")
        text_file.write(map_rendered)
    os.remove(map_output_path + "/OlMapTemplate.j2")


def render_layout(map_output_path, metrics, legend):
    # copy directory containing the outputmap and its ressources
    layout_path = os.path.join(dirname, "layoutVisualization")

    copy_tree(layout_path, map_output_path)

    # generate html layout from the file layout.j2 so that
    # - the timeline has the good minimum, maximum and interval incrementation,
    # - the metrics selector contains all the requested metrics
    # - the legend of the metrics si the one requested (not done for now)
    template_env = jinja2.Environment(loader=jinja2.FileSystemLoader(map_output_path))
    template = template_env.get_template('layout.j2')

    with open(map_output_path + "/layout.html", "wb") as text_file:
        map_rendered = template.render(metrics=metrics, legend=legend).encode("utf-8")
        text_file.write(map_rendered)
    os.remove(map_output_path + '/layout.j2')


def render_visualization(city, metrics, legend):
    map_output_path = os.path.join(dirname, "output")
    render_ol_map(map_output_path, city)
    render_layout(map_output_path, metrics, legend)
