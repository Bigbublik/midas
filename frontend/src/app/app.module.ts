import { BrowserModule } from '@angular/platform-browser';
import { NgModule } from '@angular/core';
import { ReactiveFormsModule } from '@angular/forms';
import { FlexLayoutModule } from '@angular/flex-layout';
import { HttpClientModule, HttpClientXsrfModule } from '@angular/common/http';

import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

import { MatDialogModule } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatButtonModule } from '@angular/material/button';
import { MatToolbarModule } from '@angular/material/toolbar';
import {
  MatSnackBarModule,
  MAT_SNACK_BAR_DEFAULT_OPTIONS
} from '@angular/material/snack-bar';
import { MatProgressBarModule } from '@angular/material/progress-bar';
import { MatTabsModule } from '@angular/material/tabs';
import { MatPaginatorModule } from '@angular/material/paginator';
import { MatTableModule } from '@angular/material/table';
import { MatSortModule } from '@angular/material/sort';
import { MatIconModule } from '@angular/material/icon';
import { MatListModule } from '@angular/material/list';
import { MatInputModule } from '@angular/material/input';

import * as am4core from '@amcharts/amcharts4/core';
import am4themes_animated from '@amcharts/amcharts4/themes/animated';
import { FontAwesomeModule } from '@fortawesome/angular-fontawesome';

import { SyncComponent } from './sync/sync.component';
import { IconSnackBarComponent } from './icon-snackbar/icon-snackbar.component';
import { SyncProgressComponent } from './sync-progress/sync-progress.component';
import { InfoComponent } from './info/info.component';
import { TradeObserverService } from './resources/trade-observer.service';
import { BookTickerComponent } from './info/book-ticker/book-ticker.component';
import { KeychainComponent } from './keychain/keychain.component';
import { EditDialogComponent } from './keychain/edit-dialog/edit-dialog.component';

@NgModule({
  declarations: [
    AppComponent,
    SyncComponent,
    IconSnackBarComponent,
    SyncProgressComponent,
    InfoComponent,
    BookTickerComponent,
    KeychainComponent,
    EditDialogComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    HttpClientXsrfModule,
    HttpClientModule,

    ReactiveFormsModule,
    FlexLayoutModule,
    MatDialogModule,
    MatFormFieldModule,
    MatInputModule,
    MatButtonModule,
    MatPaginatorModule,
    MatProgressBarModule,
    MatSnackBarModule,
    MatTableModule,
    MatTabsModule,
    MatToolbarModule,
    MatSortModule,
    MatIconModule,
    MatListModule,
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
