const rust = import('../pkg');

rust
  .then(m => m.greet('username'))
  .catch(console.error);
