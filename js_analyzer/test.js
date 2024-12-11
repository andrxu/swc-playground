
// test.js

// A simple JavaScript module

// Exporting a constant
export const greeting = "Hello, World!";

// Importing the greeting constant from another module
import { greeting as importedGreeting } from './anotherModule.js';

function greet() {
    console.log(importedGreeting); // Using the imported greeting
}

// Exporting the greet function
export default greet;

// Calling the greet function
greet();