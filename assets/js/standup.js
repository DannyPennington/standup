
function reset() {
    const on = document.querySelectorAll('.on');
    on.forEach(el => el.classList.remove('on', 'active'));
}

function toggleState(elementId) {
    let elem = document.getElementById(elementId);
    if (elem.classList.contains('on')) {
        elem.classList.remove('on')
        elem.classList.remove('active')
    } else {
        elem.classList.add('on')
        activate(elementId) 
    }

}

function activate(elementId) {
    const active = document.querySelectorAll('.active');
    active.forEach(el => el.classList.remove('active'));
    let elem = document.getElementById(elementId);
    elem.classList.add('active')
}
