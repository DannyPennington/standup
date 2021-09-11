
function nothing() {
    console.log("this does nothing")
}

function toggleState(elementId) {
    let elem = document.getElementById(elementId);
    if (elem.classList.contains("on")) {
        elem.classList.remove("on")
    } else {
        elem.classList.add("on")
    }

}