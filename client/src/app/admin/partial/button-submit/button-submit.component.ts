import { Component, Input, OnInit, OnChanges } from '@angular/core';
import { Model } from 'src/app/class/model';

@Component({
  selector: 'button-submit',
  templateUrl: './button-submit.component.html',
  styleUrls: ['./button-submit.component.css']
})
export class ButtonSubmitComponent implements OnChanges {
  // @Input() isLoading: boolean;
  // @Input() isOk: boolean;
  @Input() model: Model<any>;
  @Input() name?: string = 'Mentés';

  ngOnChanges() { }
}
