const { RoaringBitmap } = require('./index.js');

function main() {
  const rb = new RoaringBitmap();
  rb.insert(2);
  rb.insert(3);
  rb.insert(5);
  rb.insert(7);
  console.log("total bits set to true: ", rb.length);

  let rb1 = new RoaringBitmap();
  let rb2 = new RoaringBitmap();

  rb1.insert(1);
  console.log(rb1.isDisjoint(rb2), true);

  rb2.insert(1);
  console.log(rb1.isDisjoint(rb2), false);

  rb1 = new RoaringBitmap();
  rb2 = new RoaringBitmap();

  rb1.insert(1);
  console.log(rb1.isSubset(rb2), false);

  rb2.insert(1);
  console.log(rb1.isSubset(rb2), true);

  rb1.insert(2);
  console.log(rb1.isSubset(rb2), false);

  rb1 = new RoaringBitmap();
  rb2 = new RoaringBitmap();

  rb1.insert(1);
  console.log(rb2.isSuperset(rb1), false);

  rb2.insert(1);
  console.log(rb2.isSuperset(rb1), true);

  rb1.insert(2);
  console.log(rb2.isSuperset(rb1), false);
}

main();
