const sleep = require('sleep');
const test = require('tape');

module.exports = (liza) => {

  test('Verify Host', (t) => {
    const Host_Doc = {
      host_doc:{
      kyc_proof: "DOC # QuarnnnnvltuenblergjasnvAfs"
    }}
    t.plan(2)
    const verified = liza.call("host", "main", "register_as_host", Host_Doc);
    console.log("verified:: ",verified);
    t.equal(verified.Ok.length, 46)

    sleep.sleep(5);
    const is_verified = liza.call("host", "main", "is_registered_as_host", {});
    console.log("is verified?:: ",is_verified);
    t.equal(is_verified.Ok.addresses.length, 1)

  })
}
