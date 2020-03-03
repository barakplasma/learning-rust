import { sum_rs } from '../rust/src/lib.rs';
import { sum_js } from './sum';

let measurements = [];

function measure(tag, fn) {
  measurements.push(tag);
  window.performance.mark(tag+'start');
  fn();
  window.performance.mark(tag+'end');
}

function performance_report() {
  for (let m of measurements) {
    console.log(window.performance.measure(m, m+'start', m+'end'))
  }
}

function repeat(n, fn) {
  for (let i = 0; i<n; i++) {
    fn(i);
  }
}

// Easy
const easy = 5;
measure("sum_rs easy", () => console.log("easy rs sum of 1..5 = ", sum_rs(easy)))
measure("sum_js easy", () => console.log("easy js sum of 1..5 = ", sum_js(easy)))

const medium = 100
measure("sum_rs medium", () => console.log("medium rs sum of 1..100 = ", sum_rs(medium)))
measure("sum_js medium", () => console.log("medium js sum of 1..100 = ", sum_js(medium)))

const hard = medium
measure("sum_rs hard due to repetition", () => repeat(hard, sum_rs))
measure("sum_js hard due to repetition", () => repeat(hard, sum_js))

performance_report()