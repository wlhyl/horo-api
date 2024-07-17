import { ComponentFixture, TestBed } from '@angular/core/testing';
import { PowerPage } from './power.page';

describe('PowerPage', () => {
  let component: PowerPage;
  let fixture: ComponentFixture<PowerPage>;

  beforeEach(async(() => {
    fixture = TestBed.createComponent(PowerPage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  }));

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
