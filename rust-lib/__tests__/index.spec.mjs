import { test, expect } from 'vitest';
import { setPassword, getPassword } from '../index.js';

test('set and get password', () => {
  setPassword('hoge', 'fuga', 'bazbaaaaaz');
  const password = getPassword('hoge', 'fuga');
  expect(password).toEqual('bazbaaaaaz');
});