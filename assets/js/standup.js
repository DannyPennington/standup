
function reset() {
    const active = document.querySelectorAll('.on');
    active.forEach(el => el.classList.remove('on'));
}

function toggleState(elementId) {
    let elem = document.getElementById(elementId);
    if (elem.classList.contains("on")) {
        elem.classList.remove("on")
    } else {
        elem.classList.add("on")
    }

}