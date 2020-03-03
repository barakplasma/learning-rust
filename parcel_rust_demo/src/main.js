import { answer } from '../rust/src/lib.rs';

console.log("Hello Console!")
console.log("The answer to your question is ", answer());
document.getElementById("result").innerText = answer();
