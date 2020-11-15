import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { FlexLayoutModule } from '@angular/flex-layout';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

import { MatDialogModule } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { MatToolbarModule } from '@angular/material/toolbar';
import {
  MatSnackBarModule,
  MAT_SNACK_BAR_DEFAULT_OPTIONS
} from '@angular/material/snack-bar';
import { MatProgressBarModule } from '@angular/material/progress-bar';

import * as am4core from '@amcharts/amcharts4/core';
import am4themes_animated from '@amcharts/amcharts4/themes/animated';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

import { SyncComponent } from './sync/sync.component';
import { IconSnackBarComponent } from './icon-snackbar/icon-snackbar.component';
import { SyncProgressComponent } from './sync-progress/sync-progress.component';
import { InfoComponent } from './info/info.component';
import { TradeObserverService } from './trade-observer.service';

@NgModule({
  declarations: [
    AppComponent,
    SyncComponent,
    IconSnackBarComponent,
    SyncProgressComponent,
    InfoComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,

    FlexLayoutModule,
    MatDialogModule,
    MatButtonModule,
    MatProgressBarModule,
    MatSnackBarModule,
    MatToolbarModule,
    FontAwesomeModule,
  ],
  providers: [
    { provide: MAT_SNACK_BAR_DEFAULT_OPTIONS, useValue: { duration: 5000 } }
  ],
  bootstrap: [AppComponent]
})
export class AppModule {
  constructor(tradeObserver: TradeObserverService) {
    am4core.useTheme(am4themes_animated);
    tradeObserver.connect();
  }
}
