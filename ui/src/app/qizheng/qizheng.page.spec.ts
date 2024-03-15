import { ComponentFixture, TestBed } from '@angular/core/testing';
import { QizhengPage } from './qizheng.page';

describe('QizhengPage', () => {
  let component: QizhengPage;
  let fixture: ComponentFixture<QizhengPage>;

  beforeEach(async(() => {
    fixture = TestBed.createComponent(QizhengPage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  }));

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
