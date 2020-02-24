import { Pager } from './pager';

describe('Pager', () => {
  it('should create an instance', () => {
    expect(new Pager([], 10)).toBeTruthy();
  });
});
