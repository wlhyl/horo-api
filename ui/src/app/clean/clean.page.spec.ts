import { ComponentFixture, TestBed } from '@angular/core/testing';
import { CleanPage } from './clean.page';

describe('CleanPage', () => {
  let component: CleanPage;
  let fixture: ComponentFixture<CleanPage>;

  beforeEach(async(() => {
    fixture = TestBed.createComponent(CleanPage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  }));

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
