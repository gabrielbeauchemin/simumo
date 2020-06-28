/*
 Tabs
 (c) 2009 By Xul.fr
 Freeware
*/

function selectTab(element) {
    let tabs = document.getElementById('tabs').getElementsByTagName("a");
    for (let i = 0; i < tabs.length; i++) {
        tabs[i].className = tabs[i].rel == element.rel ? "selected" : "";
    }

    updateVisualizationBox(document.getElementsByClassName("submenu-linkSelected")[0]);
}

function initTabs() {
    dragElement(document.getElementById("legendBox"));
}

function dragElement(elmnt) {
    var pos1 = 0, pos2 = 0, pos3 = 0, pos4 = 0;
    elmnt.onmousedown = dragMouseDown;

    function dragMouseDown(e) {
        e = e || window.event;
        e.preventDefault();
        // get the mouse cursor position at startup:
        pos3 = e.clientX;
        pos4 = e.clientY;
        document.onmouseup = closeDragElement;
        // call a function whenever the cursor moves:
        document.onmousemove = elementDrag;
    }

    function elementDrag(e) {
        e = e || window.event;
        e.preventDefault();
        // calculate the new cursor position:
        pos1 = pos3 - e.clientX;
        pos2 = pos4 - e.clientY;
        pos3 = e.clientX;
        pos4 = e.clientY;
        // set the element's new position:
        elmnt.style.top = (elmnt.offsetTop - pos2) + "px";
        elmnt.style.left = (elmnt.offsetLeft - pos1) + "px";
        //limit div inside it's parent
        if(elmnt.offsetTop > elmnt.parentNode.clientHeight + 20 - elmnt.clientHeight)
        {
            elmnt.style.top = elmnt.parentNode.clientHeight + 20 - elmnt.clientHeight + "px";
        }
        if(elmnt.offsetTop < 20)
        {
            elmnt.style.top = 20 + "px";
        }
        if(elmnt.offsetLeft > elmnt.parentNode.clientWidth - elmnt.clientWidth)
        {
            elmnt.style.left = elmnt.parentNode.clientWidth - elmnt.clientWidth + "px";
        }
        if(elmnt.offsetLeft < 0)
        {
            elmnt.style.left = 0 + "px";
        }

    }

    function closeDragElement() {
        /* stop moving when mouse button is released:*/
        document.onmouseup = null;
        document.onmousemove = null;
    }
}
