let minMaxByMetric = {};

function getMinMaxLogInfo(log, metric) {
    if (minMaxByMetric[metric] == undefined) {
        let logJson = JSON.parse(log);
        let minValue = Number.POSITIVE_INFINITY;
        let maxValue = Number.NEGATIVE_INFINITY;
        let minTimestamp = Number.POSITIVE_INFINITY;
        let maxTimestamp = Number.NEGATIVE_INFINITY;
        logJson.forEach(function (entry) {
            minTimestamp = Math.min(timestampToSec(entry["timestamp"]), minTimestamp);
            maxTimestamp = Math.max(timestampToSec(entry["timestamp"]), maxTimestamp);
            entry["data"].forEach(function (data) {
                minValue = Math.min(data["value"], minValue);
                maxValue = Math.max(data["value"], maxValue);
            });
        });
        let minMaxInfo = {
            value: [minValue, maxValue],
            timestamp: [minTimestamp, maxTimestamp]
        }
        minMaxByMetric[metric] = minMaxInfo;
        return minMaxInfo;
    } else {
        return minMaxByMetric[metric];
    }
}

function parseLog(log, unitToSelect, min, max) {
    let logJson = JSON.parse(log);
    let parsedLog = []
    logJson.forEach(function (entry) {
        entry["data"].forEach(function (data) {
            if (data["resolution"] == unitToSelect) {
                let logEntry = {
                    lon: entry["lon"],
                    lat: entry["lat"],
                    metricType: entry["metric_type"],
                    dataType: data["type"],
                    unit: data["resolution"],
                    value: data["value"],
                    interpolation: (data["value"] - min) / (max -min) //normalised value
                }
                parsedLog.push(logEntry);
            }
        });
    });
    return parsedLog;
}

function secToTimestamp(sec) {
    let nbrHours = parseInt(sec / 3600.0);
    let nbrMinutes = parseInt((sec - (nbrHours * 3600.0)) / 60.0);
    let nbrSeconds = (sec - (nbrHours * 3600.0)) - (nbrMinutes * 60.0)
    let nbrMiliseconds =  (sec - Math.floor(sec)) * 100;
    return ("0" + nbrHours).slice(-2) + ":"
            + ("0" + nbrMinutes).slice(-2) + ":"
            + ("0" + nbrSeconds).slice(-2) + ":"
            + ("00" + nbrMiliseconds).slice(-3);
}

function timestampToSec(timestamp) {
    let split = timestamp.split(':');
    return parseInt(split[0]) * 60 * 60 + parseInt(split[1]) * 60 + parseInt(split[2]) + parseInt(split[3]) / 60;
}

function updateVisualizationBox(selectedMetric) {
    if (!selectedMetric) {
        return;
    }

    let selectedMetrics = document.getElementsByClassName("submenu-link");
    for(let i = 0, length = selectedMetrics.length; i < length; i++) //remove previous selected Metric
    {
        selectedMetrics[i].classList.remove("submenu-linkSelected");
    }
    selectedMetric.classList.add("submenu-linkSelected");

    $("body").css("cursor", "wait");
    let metrics = document.getElementsByName('metricSelection');

    let logPath = selectedMetric.getAttribute('data-logPath');
    let logUnit = selectedMetric.getAttribute('data-unit');

    let coloredPointsTab = document.getElementById('tabs').getElementsByTagName("a")[0];
    let heatMapTab = document.getElementById('tabs').getElementsByTagName("a")[1];
    let ScalablePointsTab = document.getElementById('tabs').getElementsByTagName("a")[2];

    let timeValueBegin = NaN;
    let timeValueEnd = NaN;
    let timeValueMin = NaN;
    let timeValueMax = NaN;

    let existTimeline = $('#flat-slider').attr('class') != 'unintialized';
    if (existTimeline) {
        timeValueBegin = $('#flat-slider').slider("option", "values")[0];
        timeValueEnd = $('#flat-slider').slider("option", "values")[1];
        timeValueMin = $("#flat-slider").slider("option", "min");
        timeValueMax = $("#flat-slider").slider("option", "max");
    }

    let gradient = []
    let colors = document.getElementById("legendColors").children;
    for (let i = 0, length = colors.length; i < length; i++) {
        gradient.push({
            r: parseInt(colors[i].getAttribute('data-red')),
            g: parseInt(colors[i].getAttribute('data-green')),
            b: parseInt(colors[i].getAttribute('data-blue'))
        });
    }

    let urlLog = !existTimeline
        ? "/logs?logPath=" + logPath
        : "/logs?logPath=" + logPath + "&min=" + secToTimestamp(timeValueBegin) + "&max=" + secToTimestamp(timeValueEnd);

    if (coloredPointsTab.className == "selected") {
        $.ajax({
            url: urlLog,
            cache: false,
            complete: function() {$("body").css("cursor", "default");},
            success: function (log) {
                let minMaxLogInfo = getMinMaxLogInfo(log, selectedMetric.innerText);
                let logMinValue = parseInt(minMaxLogInfo.value[0]);
                let logMaxValue = Math.ceil(minMaxLogInfo.value[1]);
                let logMinTimestamp = minMaxLogInfo.timestamp[0];
                let logMaxTimestamp = minMaxLogInfo.timestamp[1];
                if (!existTimeline) {
                    updateTimeline(logMinTimestamp, logMaxTimestamp);
                }
                loadColorGradient(logMinValue, logMaxValue, gradient);
                let parsedLog = parseLog(log, logUnit, logMinValue, logMaxValue);
                updateVisualizationLayer(parsedLog, "coloredPoints", gradient);
            }
        });

    } else if (heatMapTab.className == "selected") {
        $.ajax({
            url: urlLog,
            cache: false,
            complete: function() {$("body").css("cursor", "default");},
            success: function (log) {
                let minMaxLogInfo = getMinMaxLogInfo(log, selectedMetric.innerText);
                let logMinValue = parseInt(minMaxLogInfo.value[0]);
                let logMaxValue = Math.ceil(minMaxLogInfo.value[1]);
                let logMinTimestamp = minMaxLogInfo.timestamp[0];
                let logMaxTimestamp = minMaxLogInfo.timestamp[1];
                if (!existTimeline) {
                    updateTimeline(logMinTimestamp, logMaxTimestamp);
                }
                loadColorGradient(logMinValue, logMaxValue, gradient);
                let parsedLog = parseLog(log, logUnit, logMinValue, logMaxValue);
                updateVisualizationLayer(parsedLog, "heatMap", gradient);
            }
        });
    } else if (ScalablePointsTab.className == "selected") {
        $.ajax({
            url: urlLog,
            cache: false,
            complete: function() {$("body").css("cursor", "default");},
            success: function (log) {
                let minMaxLogInfo = getMinMaxLogInfo(log, selectedMetric.innerText);
                let logMinValue = parseInt(minMaxLogInfo.value[0]);
                let logMaxValue = Math.ceil(minMaxLogInfo.value[1]);
                let logMinTimestamp = minMaxLogInfo.timestamp[0];
                let logMaxTimestamp = minMaxLogInfo.timestamp[1];
                let pointMinSize = parseInt(document.getElementById("ScalablePointsMin").innerHTML);
                let pointMaxSize = parseInt(document.getElementById("ScalablePointsMax").innerHTML);
                if (!existTimeline) {
                    updateTimeline(logMinTimestamp, logMaxTimestamp);
                }
                let colorPointDiv = document.getElementById("ScalablePointsColor");
                let colorPoint = {
                    r: parseInt(colorPointDiv.getAttribute('data-red')),
                    g: parseInt(colorPointDiv.getAttribute('data-green')),
                    b: parseInt(colorPointDiv.getAttribute('data-blue')),
                    a: parseInt(colorPointDiv.getAttribute('data-alpha'))
                };
                loadScalablePointsLegend(logMinValue, logMaxValue, pointMinSize, pointMaxSize, colorPoint);
                let parsedLog = parseLog(log, logUnit, logMinValue, logMaxValue);
                updateVisualizationLayer(parsedLog, "scalablePoints", gradient);
            }
        });
    }

}
