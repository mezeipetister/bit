import { Notification } from './notification';

describe('Notification', () => {
  it('should create an instance', () => {
    expect(new Notification(0, new Date(), false, "", "")).toBeTruthy();
  });
});
