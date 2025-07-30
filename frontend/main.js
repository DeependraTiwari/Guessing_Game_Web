import init, { check_guess, generate_secret } from '../pkg/guessing_game.js';

let secret;

async function initGame() {
  await init(); // Load WASM module
  secret = generate_secret(100);
  console.log("Secret number:", secret);
}

window.check = function () {
  const input = document.getElementById("guess");
  const result = document.getElementById("result");
  const guess = parseInt(input.value);
  result.textContent = check_guess(guess, secret);
};

initGame();
