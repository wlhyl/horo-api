import { TestBed } from '@angular/core/testing';

import { QizhengConfigService } from './qizheng-config.service';

describe('QizhengConfigService', () => {
  let service: QizhengConfigService;

  beforeEach(() => {
    TestBed.configureTestingModule({});
    service = TestBed.inject(QizhengConfigService);
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
