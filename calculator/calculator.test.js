const { add, subtract, multiply, divide, square } = require('./calculator');

test('adds 1 + 2 to equal 3', () => {
  expect(add(1, 2)).toBe(3);
});

test('subtracts 5 - 2 to equal 3', () => {
  expect(subtract(5, 2)).toBe(3);
});

test('multiplies 3 * 4 to equal 12', () => {
  expect(multiply(3, 4)).toBe(12);
});

test('divides 8 / 2 to equal 4', () => {
  expect(divide(8, 2)).toBe(4);
});

test('dividing by zero throws an error', () => {
  expect(() => divide(8, 0)).toThrow('Cannot divide by zero');
});

test('calculates the square of 3 to equal 9', () => {
  expect(square(3)).toBe(9);
});