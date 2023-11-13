import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import {
  PickerColumn,
  PickerColumnOption,
  PickerController,
  PickerOptions,
} from '@ionic/angular';

@Component({
  selector: 'horo-date-time',
  templateUrl: './date-time.component.html',
  styleUrls: ['./date-time.component.scss'],
})
export class DateTimeComponent implements OnInit {
  @Input()
  year = 0;
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

  private years = [...Array(200)].map((_, index) => {
    let year = 1900 + index;
    let v = {
      text: `${year}`,
      value: year,
    };
    return v;
  });
  private months: Array<PickerColumnOption> = [...Array(12)].map((_, index) => {
    let month = 1 + index;
    let v = {
      text: `${month}`,
      value: month,
    };
    return v;
  });

  private get days(): Array<PickerColumnOption> {
    // month从0开始
    // 获取本月最后一天: 等于，下一个月第0天
    // 本月: month-1
    // 下一个月: (month-1) + 1= month
    let n = new Date(this.year, this.month, 0).getDate();
    let days = [...Array(n)].map((_, index) => {
      let day = index + 1;
      return {
        text: `${day}`,
        value: day,
      };
    });
    return days;
  }

  private hours: Array<PickerColumnOption> = [...Array(24)].map((_, index) => {
    let v = {
      text: `${index}`,
      value: index,
    };
    return v;
  });

  private minutes: Array<PickerColumnOption> = [...Array(60)].map(
    (_, index) => {
      let v = {
        text: `${index}`,
        value: index,
      };
      return v;
    }
  );

  private seconds: Array<PickerColumnOption> = [...Array(60)].map(
    (_, index) => {
      let v = {
        text: `${index}`,
        value: index,
      };
      return v;
    }
  );

  constructor(private pickerController: PickerController) {}
  ngOnInit() {}

  async openPicker() {
    const columns: PickerColumn[] = [
      {
        name: 'year',
        options: this.years,
        // columnWidth: "1000em",
        selectedIndex: this.years.findIndex((v) => v.value == this.year),
      },
      {
        name: 'month',
        options: this.months,
        columnWidth:"2em",
        selectedIndex: this.months.findIndex((v) => v.value == this.month),
      },
      {
        name: 'day',
        options: this.days,
        selectedIndex: this.days.findIndex((v) => v.value == this.day),
      },
      {
        name: 'hour',
        options: this.hours,
        selectedIndex: this.hours.findIndex((v) => v.value == this.hour),
      },
      {
        name: 'minute',
        options: this.minutes,
        selectedIndex: this.minutes.findIndex((v) => v.value == this.minute),
      },
      {
        name: 'second',
        options: this.seconds,
        selectedIndex: this.seconds.findIndex((v) => v.value == this.second),
      },
    ];

    const pickerOptions: PickerOptions = {
      // columns: JSON.parse(JSON.stringify(columns)),
      columns,
      buttons: [
        {
          text: '取消',
          role: 'cancel',
        },
        {
          text: '确定',
          handler: (value) => {
            this.year = value.year.value;
            this.month = value.month.value;
            this.day = value.day.value;
            this.hour = value.hour.value;
            this.minute = value.minute.value;
            this.second = value.second.value;
            this.emit();
          },
        },
      ],
    };

    const picker = await this.pickerController.create(pickerOptions);

    picker.addEventListener('ionPickerColChange', async (event: any) => {
      const column: PickerColumn = event.detail;
      let year: number = picker.columns[0].options.find((v) => v.selected)
        ?.value!!;
      let month: number = picker.columns[1].options.find((v) => v.selected)
        ?.value!!;

      let dayIndex = picker.columns[2].selectedIndex!!;

      if (column.name === 'year' || column.name == 'month') {
        // month从0开始
        // 获取本月最后一天: 等于，下一个月第0天
        let n = new Date(year, month, 0).getDate();
        let days = [...Array(n)].map((_, index) => {
          let day = index + 1;
          return {
            text: `${day}`,
            value: day,
          };
        });

        picker.columns[2] = {
          name: 'day',
          options: days,
          selectedIndex: dayIndex < n ? dayIndex : n - 1,
        };
        // 不加此行，不能更新列
        picker.columns = JSON.parse(JSON.stringify(picker.columns));
      }
    });

    await picker.present();
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
