async function logout() {
    var response = await fetch("/api/user/logout", {
        method: 'POST',
        headers: {'Content-Type': 'application/json'},
        mode: 'cors'
    }).catch(err => alert(err));
    if (response.ok) {
        window.open("/", "_self");
    } else {
        alert("HTTP Error: " + response.status);
    }
}

function cancel() {
    history.back();
}
