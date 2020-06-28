// tool used for the timeline: https://simeydotme.github.io/jQuery-ui-Slider-Pips/#installation


function updateTimeline(min, max)
{
	$("#flat-slider").slider({
		min: min,
		max: max,
		range: true,
		values: [min, max],
		change: function() {updateVisualizationBox(document.getElementsByClassName("submenu-linkSelected")[0])},
	}).slider("pips", {
		rest: "label",
		step: (max - min)/10.0
	});

    document.getElementById("sliderUnit").innerHTML = "sec";
    document.getElementById("prevNext").removeAttribute("hidden");
}

function nextSliderRange()
{
	let timeValueMin = $("#flat-slider").slider("option", "min");
	let timeValueMax = $("#flat-slider").slider("option", "max");
	let timeValueBegin = $('#flat-slider').slider("option", "values")[0];
	let timeValueEnd = $('#flat-slider').slider("option", "values")[1];
	let offset = (timeValueMax - timeValueMin) * 0.025;
	if(timeValueEnd + offset < timeValueMax)
	{
		$('#flat-slider').slider("values", 0, timeValueBegin + offset);
		$('#flat-slider').slider("values", 1, timeValueEnd + offset);
	}
}

function previousSliderRange()
{
	let timeValueMin = $("#flat-slider").slider("option", "min");
	let timeValueMax = $("#flat-slider").slider("option", "max");
	let timeValueBegin = $('#flat-slider').slider("option", "values")[0];
	let timeValueEnd = $('#flat-slider').slider("option", "values")[1];
	let offset = (timeValueMax - timeValueMin) * 0.025;
	if(timeValueBegin - offset > timeValueMin)
	{
		$('#flat-slider').slider("values", 0, timeValueBegin - offset);
		$('#flat-slider').slider("values", 1, timeValueEnd - offset);
	}
}
