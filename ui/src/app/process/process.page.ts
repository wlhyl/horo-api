import { Component, OnInit } from '@angular/core';
import { HorostorageService } from '../services/horostorage/horostorage.service';
import { ApiService } from '../services/api/api.service';
import { ProcessName } from '../type/enum/process';
import { ActivatedRoute, Router } from '@angular/router';

@Component({
  selector: 'app-process',
  templateUrl: './process.page.html',
  styleUrls: ['./process.page.scss'],
})
export class ProcessPage implements OnInit {
  houses: Array<string> = [];
  horoData = this.storage.horoData;
  processData = this.storage.processData;

  get processName(): string {
    switch (this.processData.process_name) {
      case ProcessName.Profection:
        return '小限';
      case ProcessName.Transit:
        return '行运';
      case ProcessName.SolarReturn:
        return '日返';
      case ProcessName.LunarReturn:
        return '月返';
    }
  }

  constructor(
    private api: ApiService,
    private router: Router,
    private route: ActivatedRoute,
    private storage: HorostorageService
  ) {}

  ngOnInit() {
    this.api.getHouses().subscribe({
      next: (response) => (this.houses = response),
    });
  }

  getProcess() {
    this.storage.horoData = this.horoData;
    this.storage.processData = this.processData;
    switch (this.processData.process_name) {
      case ProcessName.Profection:
        this.router.navigate(['./profection'], { relativeTo: this.route });
        break;
      case ProcessName.Transit:
        this.router.navigate(['./transit'], { relativeTo: this.route });
        break;
      case ProcessName.SolarReturn:
        break;
      case ProcessName.LunarReturn:
        break;
    }
  }

  pickerColumns = [
    {
      name: 'process',
      options: [
        {
          text: '小限',
          value: ProcessName.Profection,
        },
        {
          text: '行运',
          value: ProcessName.Transit,
        },
        {
          text: '日返',
          value: ProcessName.SolarReturn,
        },
        {
          text: '月返',
          value: ProcessName.LunarReturn,
        },
      ],
    },
  ];

  public pickerButtons = [
    {
      text: '取消',
      role: 'cancel',
    },
    {
      text: '确定',
      handler: (value: any) => {
        this.processData.process_name = value.process.value;
      },
    },
  ];
}
