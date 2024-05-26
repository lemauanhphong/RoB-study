const puppeteer = require('puppeteer');

(async () => {
    const browser = await puppeteer.launch({
        // args: ["--no-sandbox"],
        executablePath: "/usr/bin/google-chrome-stable",
        headless: false,
        devtools: true
    });
    const page = await browser.newPage();

    page.evaluateOnNewDocument(() => {
        const originalMethods = {
            showDirectoryPicker: showDirectoryPicker,
            showOpenFilePicker: showOpenFilePicker,
            removeEntry: FileSystemFileHandle.prototype.removeEntry,
            createWritable: FileSystemFileHandle.prototype.createWritable,
            write: FileSystemWritableFileStream.prototype.write,
            close: FileSystemWritableFileStream.prototype.close,
            getFile: FileSystemFileHandle.prototype.getFile,
        };
        
        showDirectoryPicker = async () => {
            const directoryHandle = await originalMethods.showDirectoryPicker.call(this);
            console.log(`showDirectoryPicker was called on ${directoryHandle.kind} - ${directoryHandle.name}`);
            return directoryHandle;
        }

        showOpenFilePicker = async () => {
            const fileHandle = await originalMethods.showOpenFilePicker.call(this);
            for (let i in fileHandle)
                console.log(`showDirectoryPicker was called on ${fileHandle[i].kind} - ${fileHandle[i].name}`);
            return fileHandle;
        }

        FileSystemFileHandle.prototype.removeEntry = async (target, options) => {
            console.log(`removeEntry was called on ${target} with options ${JSON.stringify(options)}`);
            return originalMethods.removeEntry.call(window, target, options);
        }

        let currentWritableFile = null;
        FileSystemFileHandle.prototype.createWritable = async function(options) {
            const writableStream = await originalMethods.createWritable.call(this, options);
            console.log(`createWritable was called on ${this.name} with options ${JSON.stringify(options)}`);
            currentWritableFile = this.name;
            return writableStream;
        };

        FileSystemWritableFileStream.prototype.write = async function(content) {
            console.log(`write was called on ${this.currentWritableFile} with content ${JSON.stringify(content)}`);
            return originalMethods.write.call(this, content);
        };

        FileSystemWritableFileStream.prototype.close = async function() {
            console.log(`close was called on ${this.currentWritableFile}`);
            currentWritableFile = null;
            return originalMethods.close.call(this);
        };

        FileSystemFileHandle.prototype.getFile = async function(options) {
            const file = await originalMethods.getFile.call(this, options);
            console.log(`getFile was called on ${this.name} with options ${JSON.stringify(options)}`);
            return file;
        };
    })

    const url = process.argv[2]; 
    if (!url) {
        console.log('Command line: node app.js <url>');
        process.exit(1);
    }

    await page.goto(url);

    new Promise(r => setTimeout(r, 10000000))
    .then(async () => {await browser.close()})
    
})();