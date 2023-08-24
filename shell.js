// const fs = require('fs');

// // Replace 'path/to/your/directory' with the actual path of the directory you want to read
// const directoryPath = 'src/templates/gitignore';

// fs.readdir(directoryPath, (err, files) => {
//     if (err) {
//         console.error('Error reading directory:', err);
//         return;
//     }

//     const fileNames = files.filter(file => fs.statSync(directoryPath + '/' + file).isFile());
//     console.log('File names in the directory:', fileNames);
// });


const fs = require('fs');
const path = require('path');

// Replace 'directoryPath' with the path of the directory you want to loop through
const directoryPath = 'src/templates/gitignore';

// Read the contents of the directory
fs.readdir(directoryPath, (err, files) => {
    if (err) {
        console.error('Error reading directory:', err);
        return;
    }

    // Loop through the files and print their names
    files.forEach(file => {
        const filePath = path.join(directoryPath, file);
        fs.stat(filePath, (err, stats) => {
            if (err) {
                console.error('Error getting file stats:', err);
                return;
            }
            if (stats.isFile()) {
                console.log(`"${file}",`);
            }
        });
    });
});
