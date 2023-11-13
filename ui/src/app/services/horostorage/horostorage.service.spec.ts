import { TestBed } from '@angular/core/testing';

import { HorostorageService } from './horostorage.service';

describe('HorostorageService', () => {
  let service: HorostorageService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(HorostorageService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
