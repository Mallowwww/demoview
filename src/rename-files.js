const fs = require('fs');
const path = require('path');

// @ts-ignore
function renameFiles(dir) {
  const files = fs.readdirSync(dir);

  for (const file of files) {
    const filePath = path.join(dir, file);
    const stat = fs.statSync(filePath);

    if (stat.isDirectory()) {
      renameFiles(filePath);
    } else if (file.startsWith('+')) {
      const newName = file.substring(1);
      fs.renameSync(filePath, path.join(dir, newName));
      console.log(`Renamed ${file} to ${newName}`);
    }
  }
}

const buildDir = './build'; // Adjust this to your build output directory
renameFiles(buildDir);