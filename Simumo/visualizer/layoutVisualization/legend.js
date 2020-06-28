function loadColorGradient(timeLineMin, timeLineMax, gradient) {
    let legendBoxContent = document.getElementById("legendBoxContent");
    legendBoxContent.innerHTML = "";
    let gradientStyle = "background-image: linear-gradient(to right,";
    for (let i = 0, length = gradient.length; i < length; i++) {
        gradientStyle += 'rgb(' +
            gradient[i].r.toString() + ',' +
            gradient[i].g.toString() + ',' +
            gradient[i].b.toString() + ')';
        if (i < length - 1) gradientStyle += ',';
    }

    gradientStyle += ')';

    //add color gradient
    legendBoxContent.innerHTML += `<div class=\"legendBoxContent\" style=\"${gradientStyle}\">`;
    //add scalar index
    let timeLineMiddle = Math.round((timeLineMin + timeLineMax) / 2.0)
    legendBoxContent.innerHTML += `<div class="beginLegendBox" style=>
                                ${timeLineMin}
                              </div>
                              <div class="middleLegendBox">
                                ${timeLineMiddle}
                              </div>
                              <div class = "endLegendBox">
                                ${timeLineMax}
                              </div>`;
}

function loadScalablePointsLegend(timeLineMin, timeLineMax, pointMinSize, pointMaxSize, pointColor) {
    let legendBoxContent = document.getElementById("legendBoxContent");
    legendBoxContent.innerHTML = "";

    //add sized points
    let ratioMinMaxPointSize = pointMaxSize / pointMinSize;
    function getSizePoint(interpolation) { return 3.0 + 3.0 * ratioMinMaxPointSize * interpolation; }
    let sizeSmallCircle = getSizePoint(0);
    let sizeMediumCircle = getSizePoint(0.5);
    let sizeLargeCircle = getSizePoint(1);
    let circleStyleSmall = `width: ${sizeSmallCircle}px; height: ${sizeSmallCircle}px; border-radius: 50%; background-color: rgba(${pointColor.r},${pointColor.g},${pointColor.b}, ${pointColor.b});`;
    let circleStyleMedium = `width: ${sizeMediumCircle}px; height: ${sizeMediumCircle}px; border-radius: 50%; background-color: rgba(${pointColor.r},${pointColor.g},${pointColor.b}, ${pointColor.b});`;
    let circleStyleLarge = `width: ${sizeLargeCircle}px; height: ${sizeLargeCircle}px; border-radius: 50%; background-color: rgba(${pointColor.r},${pointColor.g},${pointColor.b}, ${pointColor.b});`;

    //add scalar index
    let timeLineMiddle = Math.round((timeLineMin + timeLineMax) / 2.0)


    legendBoxContent.innerHTML += `<div style="display: flex; align-items: center;">  
                                   <div style="float: left;width: 30%;display: flex;justify-content: center;">  <div style="${circleStyleSmall}"><div style="height:100%; width:100%; margin-top:${sizeMediumCircle}px; font-weight: bold;">${timeLineMin}</div></div> </div> 
                                   <div style="float: left;width: 30%;display: flex;justify-content: center;">  <div  style="${circleStyleMedium}"><div style="height:100%; width:100%; margin-top:${sizeMediumCircle + 11}px; font-weight: bold; ">${timeLineMiddle}</div> </div> </div> 
                                   <div style="float: left;display: flex;justify-content: center;">  <div  style="${circleStyleLarge}"> <div style="height:100%; width:100%; margin-top:${sizeLargeCircle + 4}px; font-weight: bold;">${timeLineMax}</div> <div> </div>
                                   </div> </div>`;
}
