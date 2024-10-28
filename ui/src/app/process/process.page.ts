import {Component, OnInit} from '@angular/core';
import {HorostorageService} from '../services/horostorage/horostorage.service';
import {ProcessName} from '../type/enum/process';
import {ActivatedRoute, Router} from '@angular/router';
import {Horoconfig} from '../services/config/horo-config.service';
import {Title} from '@angular/platform-browser';

@Component({
  selector: 'app-process',
  templateUrl: './process.page.html',
  styleUrls: ['./process.page.scss'],
})
export class ProcessPage implements OnInit {
  readonly houses: Array<string> = this.config.houses;
  horaData = this.storage.horoData;
  processData = this.storage.processData;
  title = '推运';

  currentProcess = this.processData.process_name

  get processName(): string {
    return ProcessName.name(this.currentProcess);
  }

  constructor(
    private router: Router,
    private route: ActivatedRoute,
    private storage: HorostorageService,
    private config: Horoconfig,
    private titleService: Title
  ) {
  }

  ngOnInit() {
    this.titleService.setTitle(this.title);
  }


  get processNameEnum(): typeof ProcessName {
    return ProcessName;
  }

  getProcess() {
    this.storage.horoData = this.horaData;
    this.storage.processData = this.processData;
    const path = ProcessName.path(this.processData.process_name);
    this.router.navigate([path], {
      relativeTo: this.route,
    });
  }

  processOptions = [
    ProcessName.Firdaria,
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


  onIonChange(event: CustomEvent) {
    this.currentProcess = event.detail.value;
  }

  onDidDismiss(event: CustomEvent) {
    if (event.detail.data === null) {
      this.currentProcess = this.processData.process_name
    } else {
      this.processData.process_name = event.detail.data;
    }
  }
}
