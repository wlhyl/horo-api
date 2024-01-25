import { Component, EventEmitter, OnInit, Output } from '@angular/core';

@Component({
  selector: 'horo-date-time-change',
  templateUrl: './date-time-change.component.html',
  styleUrls: ['./date-time-change.component.scss'],
})
export class DateTimeChangeComponent implements OnInit {
  stepUnit = '年';

  private step = {
    year: 0,
    month: 0,
    day: 0,
    hour: 0,
    minute: 0,
    second: 0,
  };

  @Output()
  private chagedStep = new EventEmitter<{
    year: number;
    month: number;
    day: number;
    hour: number;
    minute: number;
    second: number;
  }>();

  constructor() {}

  ngOnInit() {}

  changeStepUnit() {
    switch (this.stepUnit) {
      case '年':
        this.stepUnit = '月';
        break;
      case '月':
        this.stepUnit = '日';
        break;
      case '日':
        this.stepUnit = '时';
        break;
      case '时':
        this.stepUnit = '分';
        break;
      case '分':
        this.stepUnit = '秒';
        break;
      default:
        this.stepUnit = '年';
    }
  }

  changeStep(n: number) {
    switch (this.stepUnit) {
      case '年':
        this.step = {
          year: n,
          month: 0,
          day: 0,
          hour: 0,
          minute: 0,
          second: 0,
        };
        break;
      case '月':
        this.step = {
          year: 0,
          month: n,
          day: 0,
          hour: 0,
          minute: 0,
          second: 0,
        };
        break;
      case '日':
        this.step = {
          year: 0,
          month: 0,
          day: n,
          hour: 0,
          minute: 0,
          second: 0,
        };
        break;
      case '时':
        this.step = {
          year: 0,
          month: 0,
          day: 0,
          hour: n,
          minute: 0,
          second: 0,
        };
        break;
      case '分':
        this.step = {
          year: 0,
          month: 0,
          day: 0,
          hour: 0,
          minute: n,
          second: 0,
        };
        break;
      default:
        this.step = {
          year: 0,
          month: 0,
          day: 0,
          hour: 0,
          minute: 0,
          second: n,
        };
    }

    this.chagedStep.emit(this.step);
  }
}
