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
  instance.exports.move(2, 5, 3, 4); // B
  instance.exports.move(3, 2, 4, 3); // W
  instance.exports.move(4, 5, 5, 4); // B
  instance.exports.move(2, 1, 3, 2); // W
  instance.exports.move(1, 2, 2, 1); // B
  instance.exports.move(0, 1, 1, 2); // W
  instance.exports.move(5, 4, 6, 3); // B
  instance.exports.move(1, 0, 0, 1); // W
  instance.exports.move(2, 1, 1, 0); // B - Crowned

  let res = instance.exports.getPiece(1, 0);
  document.getElementById("container").innerText = res;
  console.log("At end, turn owner is " + instance.exports.getTurnOwner());
}).catch(console.error);
