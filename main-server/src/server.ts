import app from '@/app.js';
import { server } from '@global/constants.js';
import chalk from 'chalk';

const serverUrl = `http://${server.host}:${server.port}`;
const docsUrl = `${serverUrl}/docs/`;

app.listen(server.port, server.host, () => {
  console.log(` ▄▄▄▄ ▄▄▄▄▄ ▄▄  ▄▄ ▄▄▄▄   ▄▄▄▄ ▄▄ ▄▄ ▄▄▄▄  ▄▄▄▄▄
███▄▄ ██▄▄  ███▄██ ██▀██ ██▀▀▀ ██ ██ ██▄█▄ ██▄▄
▄▄██▀ ██▄▄▄ ██ ▀██ ████▀ ▀████ ▀███▀ ██ ██ ██▄▄▄
                              Main Server v0.1
  `);

  console.log(`Server listening on ${chalk.yellow(serverUrl)}.\n`);

  console.log(`View the (quite epic) documentation at ${chalk.cyan(docsUrl)}.\n`);
});