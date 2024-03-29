import axios from "axios"
// import React from "react"
import {DeviceUUID} from "../assets/device-uuid.min.js"
export function getCookie(cname) {
  let name = cname + "=";
  let decodedCookie = decodeURIComponent(document.cookie);
  let ca = decodedCookie.split(';');
  for (let i = 0; i < ca.length; i++) {
    let c = ca[i];
    while (c.charAt(0) === ' ') {
      c = c.substring(1);
    }
    if (c.indexOf(name) === 0) {
      return c.substring(name.length, c.length);
    }
  }
  return "";
}
export function setCookie(name,value,days) {
  var expires = "";
  if (days) {
      var date = new Date();
      date.setTime(date.getTime() + (days*24*60*60*1000));
      expires = "; expires=" + date.toUTCString();
  }
  document.cookie = name + "=" + (value || "")  + expires + "; path=/; SameSite=Strict";
}
export const sendLogin = (_data) => {
  return new Promise((resolve, reject) => {
    const axios = require('axios');
    let data = JSON.stringify(_data);
    let config = {
      method: 'post',
      url: 'https://mailvalidator.dup.company/login',
      data: data
    };

    axios(config)
      .then((response) => {
        resolve(response.data)
      })
      .catch((error) => {
        reject(error)
      });
  })
}
export const uuid = ()=> new DeviceUUID().get();
export const Client = axios.create({
  baseURL: !process.env.NODE_ENV || process.env.NODE_ENV === 'development' ? 'https://mailvalidator.dup.company/' : '/',
  timeout: 1000,
  headers: {
    'Content-Type': 'application/json',
    // 'Cookie': '____ads='+getCookie("____ads")
  },
  xsrfCookieName:"____ads",
  // xsrfHeaderName:"X-FUCK-OF"
});
