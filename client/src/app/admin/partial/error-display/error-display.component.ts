import { Component, OnChanges, Input } from '@angular/core';
import { Model } from 'src/app/class/model';

@Component({
  selector: 'error-display',
  templateUrl: './error-display.component.html',
  styleUrls: ['./error-display.component.css']
})
export class ErrorDisplayComponent implements OnChanges {
  // @Input() isLoading: boolean;
  // @Input() isOk: boolean;
  @Input() model: Model<any>;

  ngOnChanges() { }

}
