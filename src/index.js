import 'bootstrap';
import './main.scss';

import("../crate/pkg").then(module => {
  module.run();
});
