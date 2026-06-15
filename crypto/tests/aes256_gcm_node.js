#!/usr/bin/env node

const crypto = require('crypto');

const args = process.argv.slice(2);
const mode = args[0];
const keyHex = args[1];
const nonceHex = args[2];
const aadHex = args[3];
const dataHex = args[4];

const key = Buffer.from(keyHex, 'hex');
const nonce = Buffer.from(nonceHex, 'hex');
const aad = Buffer.from(aadHex, 'hex');
const data = Buffer.from(dataHex, 'hex');

if (mode === 'encrypt') {
  const cipher = crypto.createCipheriv('aes-256-gcm', key, nonce);
  cipher.setAAD(aad);
  
  const encrypted = Buffer.concat([cipher.update(data), cipher.final()]);
  const tag = cipher.getAuthTag();
  
  console.log(encrypted.toString('hex'));
  console.log(tag.toString('hex'));
} else if (mode === 'decrypt') {
  const ciphertext = data.slice(0, -16);
  const tag = data.slice(-16);
  
  const decipher = crypto.createDecipheriv('aes-256-gcm', key, nonce);
  decipher.setAAD(aad);
  decipher.setAuthTag(tag);
  
  try {
    const decrypted = Buffer.concat([decipher.update(ciphertext), decipher.final()]);
    console.log(decrypted.toString('hex'));
  } catch (e) {
    console.error('Decryption failed:', e.message);
    process.exit(1);
  }
} else {
  console.error('Usage: node aes256_gcm_node.js <encrypt|decrypt> <key> <nonce> <aad> <data>');
  process.exit(1);
}
