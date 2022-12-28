fetch('./ch03.wasm').then(response =>
  response.arrayBuffer()
).then(bytes => WebAssembly.instantiate(bytes, {
  env: {
    notify_piecemoved: (fX, fY, tX, tY, piece) => {
      console.log("A piece moved from (" + fX + "," + fY +
        ") to (" + tX + "," + tY + ")");
    },
    notify_piececrowned: (x, y) => {
      console.log("A piece was crowned at (" + x + "," + y + ")");
    },
  },
})).then(results => {
  console.log("Loaded wasm module");

  instance = results.instance;

  console.log("At start, turn owner is " +
    instance.exports.get_current_turn());

  instance.exports.move_piece(0, 5, 1, 4); // B
  instance.exports.move_piece(1, 2, 0, 3); // W
  instance.exports.move_piece(1, 4, 2, 3); // B
  instance.exports.move_piece(0, 3, 1, 4); // W
  instance.exports.move_piece(2, 3, 1, 2); // B
  instance.exports.move_piece(1, 4, 0, 5); // W
  instance.exports.move_piece(2, 5, 3, 4); // B
  instance.exports.move_piece(3, 2, 4, 3); // W
  instance.exports.move_piece(4, 5, 5, 4); // B
  instance.exports.move_piece(2, 1, 3, 2); // W
  instance.exports.move_piece(1, 2, 2, 1); // B
  instance.exports.move_piece(0, 1, 1, 2); // W
  instance.exports.move_piece(5, 4, 6, 3); // B
  instance.exports.move_piece(1, 0, 0, 1); // W
  instance.exports.move_piece(2, 1, 1, 0); // B - Crowned

  let res = instance.exports.get_piece(1, 0);
  document.getElementById("container").innerText = res;
  console.log("At end, turn owner is " + instance.exports.get_current_turn());
}).catch(console.error);
