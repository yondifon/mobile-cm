import { describe, expect, test } from 'bun:test';
import cases from '../../spec/cases.json';
import { check, isCamtel, isMTN, isNexttel, isOrange } from '../src/index';

describe('check', () => {
  for (const { input, operator } of cases as { input: string; operator: string | null }[]) {
    test(`${JSON.stringify(input)} -> ${JSON.stringify(operator)}`, () => {
      expect(check(input)).toBe(operator as ReturnType<typeof check>);
      expect(isMTN(input)).toBe(operator === 'mtn');
      expect(isOrange(input)).toBe(operator === 'orange');
      expect(isNexttel(input)).toBe(operator === 'nexttel');
      expect(isCamtel(input)).toBe(operator === 'camtel');
    });
  }
});
