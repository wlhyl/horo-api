import { Component, OnInit } from '@angular/core';
import { HorostorageService } from '../services/horostorage/horostorage.service';
import { ApiService } from '../services/api/api.service';
import { ProcessName } from '../type/enum/process';
import { ActivatedRoute, Router } from '@angular/router';
import { Horoconfig } from '../services/config/horo-config.service';
import { Title } from '@angular/platform-browser';

@Component({
  selector: 'app-process',
  templateUrl: './process.page.html',
  styleUrls: ['./process.page.scss'],
})
export class ProcessPage implements OnInit {
  readonly houses: Array<string> = this.config.houses;
  horoData = this.storage.horoData;
  processData = this.storage.processData;
  title = '推运';

  get processName(): string {
    return ProcessName.name(this.processData.process_name);
  }

  constructor(
    private api: ApiService,
    private router: Router,
    private route: ActivatedRoute,
    private storage: HorostorageService,
    private config: Horoconfig,
    private titleService: Title
  ) {}

  ngOnInit() {
    this.titleService.setTitle(this.title);
  }

  get processNameEnum(): typeof ProcessName {
    return ProcessName;
  }

  getProcess() {
    this.storage.horoData = this.horoData;
    this.storage.processData = this.processData;
    const path = ProcessName.path(this.processData.process_name);
    this.router.navigate([path], {
      relativeTo: this.route,
    });
  }

  private options = [
    ProcessName.Profection,
    ProcessName.Transit,
    ProcessName.SolarReturn,
    ProcessName.LunarReturn,
    ProcessName.SolarcomparNative,
    ProcessName.NativecomparSolar,
    ProcessName.LunarcomparNative,
    ProcessName.NativecomparLunar,
  ].map((process_name) => {
    return {
      text: ProcessName.name(process_name),
      value: process_name,
    };
  });

  pickerColumns = [
    {
      name: 'process',
      options: this.options,
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
