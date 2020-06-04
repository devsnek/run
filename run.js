#!/usr/bin/env node

'use strict';

const fs = require('fs');
const childProcess = require('child_process');

const types = {
  [process.argv[2]]: [process.argv[2], process.argv.slice(3)],
  'package.json': ['npm', ['run']],
  'Makefile': ['make', []],
  'Cargo.toml': ['cargo', ['run']],
  'binding.gyp': ['node-gyp', []],
  'gradlew': ['./gradlew', ['run']],
};

function exists(f) {
  try {
    return fs.statSync(f).isFile();
  } catch {
    return false;
  }
}

for (const type of Object.keys(types)) {
  if (exists(type)) {
    const [command, args] = types[type];
    childProcess.spawn(command, args.concat(process.argv.slice(2)), {
      stdio: 'inherit',
    });
    break;
  }
}
