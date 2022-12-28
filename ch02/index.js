fetch('./checkers.wasm').then(response =>
  response.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes, {
  events: {
    piecemoved: (fX, fY, tX, tY, piece) => {
      console.log("A piece (" + piece + ") moved from (" + fX + "," + fY +
        ") to (" + tX + "," + tY + ")");
    },
    piececrowned: (x, y) => {
      console.log("A piece was crowned at (" + x + "," + y + ")");
    },
  },
})).then(results => {
  console.log("Loaded wasm module");

  instance = results.instance;

  instance.exports.initBoard();
  console.log("At start, turn owner is " +
    instance.exports.getTurnOwner());

  instance.exports.move(0, 5, 1, 4); // B
  instance.exports.move(1, 2, 0, 3); // W
  instance.exports.move(1, 4, 2, 3); // B
  instance.exports.move(0, 3, 1, 4); // W
  instance.exports.move(2, 3, 1, 2); // B
  instance.exports.move(1, 4, 0, 5); // W
  instance.exports.move(1, 2, 0, 0); // B - this will be crowned
  instance.exports.move(3, 2, 4, 3); // W
  instance.exports.move(0, 0, 0, 2); // W

  let res = instance.exports.getPiece(0, 2);
  document.getElementById("container").innerText = res;
  console.log("At end, turn owner is " + instance.exports.getTurnOwner());
}).catch(console.error);
