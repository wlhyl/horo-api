import {Component, EventEmitter, Input, OnChanges, OnInit, Output, SimpleChanges} from '@angular/core';

@Component({
  selector: 'horo-date-time',
  templateUrl: './date-time.component.html',
  styleUrls: ['./date-time.component.scss'],
})
export class DateTimeComponent implements OnInit, OnChanges {
  @Input()
  year = 0

  @Input()
  month = 1;
  @Input()
  day = 1;
  @Input()
  hour = 0;
  @Input()
  minute = 0;
  @Input()
  second = 0;

  @Output()
  yearChange = new EventEmitter<number>();

  @Output()
  monthChange = new EventEmitter<number>();

  @Output()
  dayChange = new EventEmitter<number>();

  @Output()
  hourChange = new EventEmitter<number>();

  @Output()
  minuteChange = new EventEmitter<number>();

  @Output()
  secondChange = new EventEmitter<number>();

  years = [...Array(200)].map((_, index) => 1900 + index);

  months = [...Array(12)].map((_, index) => 1 + index);

  get days() {
    // month从0开始
    // 获取本月最后一天: 等于，下一个月第0天
    // 本月: month-1
    // 下一个月: (month-1) + 1= month
    let lastDay = new Date(this.currentValue.year, this.currentValue.month, 0).getDate();
    return [...Array(lastDay)].map((_, index) => index + 1);
  }

  hours = [...Array(24)].map((_, index) => index);

  minutes = [...Array(60)].map((_, index) => index);

  seconds = [...Array(60)].map((_, index) => index);

  currentValue = {
    year: 0,
    month: 0,
    day: 0,
    hour: 0,
    minute: 0,
    second: 0,
  }

  isOpen = false

  constructor() {
  }

  ngOnInit() {
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (changes['year']) {
      this.currentValue.year = changes['year'].currentValue
    }

    if (changes['month']) {
      this.currentValue.month = changes['month'].currentValue
    }

    if (changes['day']) {
      this.currentValue.day = changes['day'].currentValue
    }


    if (changes['hour']) {
      this.currentValue.hour = changes['hour'].currentValue
    }

    if (changes['minute']) {
      this.currentValue.minute = changes['minute'].currentValue
    }

    if (changes['second']) {
      this.currentValue.second = changes['second'].currentValue
    }
  }


  onIonChangeYear(event: CustomEvent) {
    this.currentValue.year = event.detail.value;
  }

  onIonChangeMonth(event: CustomEvent) {
    this.currentValue.month = event.detail.value;
  }

  onIonChangeDay(event: CustomEvent) {
    this.currentValue.day = event.detail.value;
  }

  onIonChangeHour(event: CustomEvent) {
    this.currentValue.hour = event.detail.value;
  }

  onIonChangeMinute(event: CustomEvent) {
    this.currentValue.minute = event.detail.value;
  }

  onIonChangeSecond(event: CustomEvent) {
    this.currentValue.second = event.detail.value;
  }

  onDidDismiss(event: CustomEvent) {
    this.isOpen = false

    if (event.detail.data === null) {
      this.currentValue.year = this.year
      this.currentValue.month = this.month
      this.currentValue.day = this.day
      this.currentValue.hour = this.hour
      this.currentValue.minute = this.minute
      this.currentValue.second = this.second
    } else {
      this.year = event.detail.data.year
      this.month = event.detail.data.month
      this.day = event.detail.data.day
      this.hour = event.detail.data.hour
      this.minute = event.detail.data.minute
      this.second = event.detail.data.second

      this.emit();
    }
  }


  nowDate() {
    let t = new Date();
    this.year = t.getFullYear();
    this.month = t.getMonth() + 1;
    this.day = t.getDate();
    this.hour = t.getHours();
    this.minute = t.getMinutes();
    this.second = t.getSeconds();
    this.emit();
  }

  private emit(): void {
    this.yearChange.emit(this.year);
    this.monthChange.emit(this.month);
    this.dayChange.emit(this.day);
    this.hourChange.emit(this.hour);
    this.minuteChange.emit(this.minute);
    this.secondChange.emit(this.second);
  }
}
