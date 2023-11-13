import { ComponentFixture, TestBed } from '@angular/core/testing';
import { NativePage } from './native.page';

describe('NativePage', () => {
  let component: NativePage;
  let fixture: ComponentFixture<NativePage>;

  beforeEach(async(() => {
    fixture = TestBed.createComponent(NativePage);
    component = fixture.componentInstance;
    fixture.detectChanges();
  }));

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
