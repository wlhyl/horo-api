import { Component, EventEmitter, Input, OnInit, Output } from '@angular/core';
import {
  PickerColumn,
  PickerColumnOption,
  PickerController,
  PickerOptions,
} from '@ionic/angular';

@Component({
  selector: 'horo-time-zone',
  templateUrl: './time-zone.component.html',
  styleUrls: ['./time-zone.component.scss'],
})
export class TimeZoneComponent implements OnInit {
  @Input()
  zone = 8;

  @Output()
  zoneChange = new EventEmitter<number>();

  get zoneString(): string | undefined {
    return this.zones.find((v) => v.value == this.zone)?.text;
  }
  private zones: Array<PickerColumnOption> = [...Array(25)].map((_, i) => {
    let value = i - 12;
    let s = '';
    if (value < 0) s = `西${-value}区`;
    else if (value == 0) s = '0时区';
    else s = `东${value}区`;
    return { text: s, value: i - 12 };
  });

  constructor(private pickerController: PickerController) {}

  ngOnInit() {}

  async openPicker() {
    const columns: PickerColumn[] = [
      {
        name: 'zone',
        options: this.zones,
        selectedIndex: this.zones.findIndex((v) => v.value == this.zone),
      },
    ];

    const pickerOptions: PickerOptions = {
      columns,
      buttons: [
        {
          text: '取消',
          role: 'cancel',
        },
        {
          text: '确定',
          handler: (value) => {
            this.zone = value.zone.value;

            this.emit();
          },
        },
      ],
    };

    const picker = await this.pickerController.create(pickerOptions);

    await picker.present();
  }

  private emit(): void {
    this.zoneChange.emit(this.zone);
  }
}
