import 'bootstrap';
import $ from 'jquery';
import './main.scss';

export function alert_clear() {
  $("#alert_win").hide();
  $("#alert_fail").hide();
}

export function alert_win() {
  $("#alert_win").show();
  $("#alert_fail").hide();
}

export function alert_fail() {
  $("#alert_win").hide();
  $("#alert_fail").show();
}

$(function () {
  alert_clear();
});


import("../crate/pkg").then(module => {
  module.run();
});
