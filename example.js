console.log("Hello", "Matcha!");
console.warn("Warning test!");
console.error("Error test!");

const path = "./log.txt";
try {
  const contents = await Matcha.readFile(path);
  console.log("Read from a file:", contents);
} catch (err) {
  console.error(`Unable to read file: '${path}'.`, err);
}

await Matcha.writeFile(path, "I can write to a file.");
const contents = await Matcha.readFile(path);
console.log(`Read from a file: '${path}':`, contents);
console.log("Removing file:", path);
Matcha.removeFile(path);
console.log("File removed");
