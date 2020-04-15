const { fibonacci: fibonacciWasm } = require('../pkg')
const fibonacciJs = require('./fibonacci')

const run = async () => {
  const total = 45

  console.log('Starting!');

  console.time('wasm-time');
  console.log(fibonacciWasm(total));
  console.timeEnd('wasm-time');

  console.time('js-time');
  console.log(fibonacciJs(total));
  console.timeEnd('js-time');
}

run()
