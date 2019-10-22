// Register the key handler 
document.addEventListener('keydown', function (e) {
    // Disable browser interaction
    // CTRL + A => Log out
    if (e.ctrlKey && e.keyCode == 76) {
        window.location.href = "/logout";
        e.preventDefault();
    }
    // CTRL + I => Index
    if (e.ctrlKey && e.keyCode == 73) {
        window.location.href = "/";
        e.preventDefault();
    }
}, false);

// Or with ALT + A
//if (e.altKey && e.keyCode == 65) {
//    window.location.href = "http://ourcodeworld.com"; 
//}