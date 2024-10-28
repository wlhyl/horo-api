import {APP_INITIALIZER, NgModule} from '@angular/core';
import {BrowserModule} from '@angular/platform-browser';
import {RouteReuseStrategy} from '@angular/router';

// 数据双向绑定
// import { FormsModule } from '@angular/forms';

import {IonicModule, IonicRouteStrategy} from '@ionic/angular';

// http访问
import {provideHttpClient, withInterceptorsFromDi} from '@angular/common/http';

import {AppComponent} from './app.component';
import {AppRoutingModule} from './app-routing.module';
import {Horoconfig} from './services/config/horo-config.service';
import {ApiService} from './services/api/api.service';
import {appInit} from './services/init/app-init';

@NgModule({
  declarations: [AppComponent],
  bootstrap: [AppComponent],
  imports: [
    BrowserModule,
    IonicModule.forRoot(),
    AppRoutingModule], providers: [
    {provide: RouteReuseStrategy, useClass: IonicRouteStrategy},
    {
      provide: APP_INITIALIZER,
      useFactory: (config: Horoconfig, api: ApiService) => () => appInit(config, api),
      deps: [Horoconfig, ApiService],
      multi: true,
    },
    provideHttpClient(withInterceptorsFromDi()),
  ]
})
export class AppModule {
}
