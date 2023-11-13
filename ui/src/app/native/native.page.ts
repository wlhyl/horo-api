import { Component, OnInit } from '@angular/core';
import { ActivatedRoute, Router } from '@angular/router';
import { HorostorageService } from '../services/horostorage/horostorage.service';
import { ApiService } from '../services/api/api.service';

@Component({
  selector: 'app-native',
  templateUrl: './native.page.html',
  styleUrls: ['./native.page.scss'],
})
export class NativePage implements OnInit {
  houses: Array<string> = [];
  horoData = this.storage.horoData;

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

  getHoro() {
    this.storage.horoData = this.horoData;
    this.router.navigate(['./image'], { relativeTo: this.route });
  }
}
