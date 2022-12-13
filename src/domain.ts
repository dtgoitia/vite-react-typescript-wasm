import init, { CounterState } from "./../@rsw/my-wasm-test/pkg";

export async function initialize(): Promise<CounterState> {
  return init()
    .then(_ => {
      const counter = CounterState.new();
      return counter;
    })
}
