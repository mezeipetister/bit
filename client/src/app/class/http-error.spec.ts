import { HttpError } from './http-error';

describe('HttpError', () => {
  it('should create an instance', () => {
    expect(new HttpError("kind", "message")).toBeTruthy();
  });
});
