import { Component, Input } from '@angular/core';
import { ProgressBarMode } from '@angular/material/progress-bar';

import { IProgress } from './iprogress';

@Component({
  selector: 'app-progress-bar',
  templateUrl: './progress-bar.component.html',
  styleUrls: ['./progress-bar.component.scss']
})
export class ProgressBarComponent {
  @Input() public progress: IProgress | void;
  @Input() public mode: ProgressBarMode;

  constructor() { }

  public getPercentage(): number {
    if (this.progress) {
      let { cur, size } = this.progress;
      return Math.min(cur / size, 1);
    } else {
      return 0;
    }
  }

}
