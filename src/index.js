const fs = require('fs');

function readFile(filePath){
  try {
    const data = fs.readFileSync(filePath);
    return data.toString();
  } catch (error) {
    console.error(`Got an error trying to read the file: ${error.message}`);
  }
}

function main() {
  const fileData = readFile('./src/lines.txt');
  fileData
    .split('\n')
    .forEach((line, i) => {
    if (i % 2 === 0) {
      console.log(line);
    }
  })
}

main();