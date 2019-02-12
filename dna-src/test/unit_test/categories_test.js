const test = require('tape');

const App_Config_1 = {
  ui_hash: "QuarnnnnvltuenblergjasnvAfs",
  dna_list: ["QweAFioina","QtavsFdvva"]
}

const App_Config_2 = {
  ui_hash: "QrafVSdvzv98vlsurhvsdfvser",
  dna_list: ["Q4farvrvsdf","Q4fdfvrbas"]
}

module.exports = (app) => {

  test('Category test', (t) => {

    const app_address = app.call("provider", "main", "register_app", App_Config_1);
    t.equal(app_address.Ok.length, 46)
    console.log("APP ADDRESS:: ",app_address.Ok);

    const app_address2 = app.call("provider", "main", "register_app", App_Config_2);
    t.equal(app_address2.Ok.length, 46)
    console.log("APP ADDRESS:: ",app_address2.Ok);

    const result1 = app.call("categories", "main", "add_category", {category:"Zo", tag:"El", hash:app_address.Ok})
    console.log("add Categories:: ",result1);
    t.equal(result1.Ok, null)

    const result2 = app.call("categories", "main", "add_category", {category:"Zo", tag:"El", hash:app_address2.Ok})
    console.log("add Categories:: ",result2);
    t.equal(result2.Ok, null)

    const result3 = app.call("categories", "main", "get_apps_by_category", {category:"Zo"})
    console.log("Get Categories:: ",result3);
    console.log("Get Categories:: ",JSON.stringify(result3.Ok[0].entry));
    t.equal(result3.Ok.length, 2)

    t.end()
  })
}
