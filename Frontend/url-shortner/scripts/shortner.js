
function shortenUrl() {
    fetch('http://localhost:2300/short').then(response => {
        response.text().then(result => {
            result = JSON.parse(result);
            console.log(result.newId);
        });
    });
};