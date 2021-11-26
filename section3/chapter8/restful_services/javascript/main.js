function getItems() {
    let call = apiCall("/item/get", "GET");
    call.send();
}

function renderItems(items, processType, elementId, processFunction) {
    let placeholder = "<div>";
    const itemsMeta = [];
    for (let i = 0; i < items.length; i++) {
        const title = items[i]["title"];
        const placeholderId = `${processType}-${title.replaceAll(" ", "-")}`;
        placeholder += '<div class="itemContainer">' + `<p>${title}</p>` +
            `<div id="${placeholderId}" class="actionButton">${processType}</div>` +
            "</div>";
        itemsMeta.push({id: placeholderId, title});
    }
    placeholder += "</div>";
    document.getElementById(elementId).innerHTML = placeholder;

    for (let i = 0; i < itemsMeta.length; i++) {
        document.getElementById(
            itemsMeta[i]["id"]
        ).addEventListener("click", processFunction);
    }
}

function createItem() {
    const title = document.getElementById("name");
    const call = apiCall("/item/create/" + title.value, "POST");
    call.send();
    document.getElementById("name").value = null;
}

function editItem() {
    const title = this.id.replaceAll("-", " ").replace("edit ", "");
    const json = {title, status: "done"};

    const call = apiCall("/item/edit", "PUT");
    call.send(JSON.stringify(json));
}

function deleteItem() {
    const title = this.id.replaceAll("-", " ").replace("delete ", "");
    const json = {title, status: "done"};
    const call = apiCall("/item/delete", "DELETE");
    call.send(JSON.stringify(json));
}

function apiCall(url, method) {
    let xhr = new XMLHttpRequest();
    xhr.withCredentials = true;

    xhr.addEventListener("readystatechange", function () {
        if (this.readyState === this.DONE) {
            renderItems(JSON.parse(this.responseText)["pending_items"], "edit", "pendingItems", editItem);
            renderItems(JSON.parse(this.responseText)["done_items"], "delete", "doneItems", deleteItem);
            document.getElementById("completeNum").innerHTML =
                JSON.parse(this.responseText)["done_item_count"];
            document.getElementById("pendingNum").innerHTML = JSON.parse(
                this.responseText)["pending_item_count"];
        }
    });
    xhr.open(method, `/api/v1${url}`);
    xhr.setRequestHeader("content-type", "application/json");
    xhr.setRequestHeader("user-token", localStorage.getItem("user-token"));
    return xhr;
}

document.getElementById("create-button").addEventListener("click", createItem);
document.getElementById("logout-button").addEventListener("click", () => window.location.replace("/logout"))

if (localStorage.getItem('user-token') === null) {
    window.location.replace(`${document.location.origin}/login`);
} else {
    getItems();
}