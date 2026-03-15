
void __cdecl FUN_00614530(int param_1,HWND param_2)

{
  UINT uID;
  CHAR local_200 [256];
  CHAR local_100 [256];
  
  if (param_1 == 0) {
    return;
  }
  if (DAT_006be6fc != 1) {
    return;
  }
  if (param_1 < -0x7fffbffa) {
    if (param_1 == -0x7fffbffb) {
      uID = 0xde0c;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7fffbfff) {
      uID = 0xde53;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7ff8fff1) {
    if (param_1 == -0x7ff8fff2) {
      uID = 0xde42;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7ffbfe10) {
      uID = 0xde8b;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789fffa) {
    if (param_1 == -0x7789fffb) {
      uID = 0xde00;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7ff8ffa9) {
      uID = 0xde17;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ffeb) {
    if (param_1 == -0x7789ffec) {
      uID = 0xde03;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789fff6) {
      uID = 0xde02;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ffc8) {
    if (param_1 == -0x7789ffc9) {
      uID = 0xde0a;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ffd8) {
      uID = 0xde08;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ffa0) {
    if (param_1 == -0x7789ffa1) {
      uID = 0xde11;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ffa6) {
      uID = 0xde0d;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ff91) {
    if (param_1 == -0x7789ff92) {
      uID = 0xde13;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff9c) {
      uID = 0xde12;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ff7d) {
    if (param_1 == -0x7789ff7e) {
      uID = 0xde16;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff88) {
      uID = 0xde15;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ff69) {
    if (param_1 == -0x7789ff6a) {
      uID = 0xde1a;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff6f) {
      uID = 0xde18;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ff55) {
    if (param_1 == -0x7789ff56) {
      uID = 0xde1c;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff60) {
      uID = 0xde1b;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ff32) {
    if (param_1 == -0x7789ff33) {
      uID = 0xde1f;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff4c) {
      uID = 0xde1d;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ff23) {
    if (param_1 == -0x7789ff24) {
      uID = 0xde23;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff2e) {
      uID = 0xde21;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff2c) {
      uID = 0xde24;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff29) {
      uID = 0xde22;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ff1e) {
    if (param_1 == -0x7789ff1f) {
      uID = 0xde29;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff22) {
      uID = 0xde84;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ff0f) {
    if (param_1 == -0x7789ff10) {
      uID = 0xde2b;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff1a) {
      uID = 0xde2a;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789ff00) {
    if (param_1 == -0x7789ff01) {
      uID = 0xde3b;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789ff06) {
      uID = 0xde2d;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789fee7) {
    if (param_1 == -0x7789fee8) {
      uID = 0xde32;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789fefc) {
      uID = 0xde2f;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789fec9) {
    if (param_1 == -0x7789feca) {
      uID = 0xde34;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789fede) {
      uID = 0xde33;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789feb5) {
    if (param_1 == -0x7789feb6) {
      uID = 0xde39;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789fec4) {
      uID = 0xde35;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789fec3) {
      uID = 0xde36;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789fec0) {
      uID = 0xde37;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789feab) {
    if (param_1 == -0x7789feac) {
      uID = 0xde3f;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789feb1) {
      uID = 0xde3e;
      goto LAB_00614e74;
    }
  }
  else if (param_1 < -0x7789fe97) {
    if (param_1 == -0x7789fe98) {
      uID = 0xde41;
      goto LAB_00614e74;
    }
    if (param_1 == -0x7789fea2) {
      uID = 0xde40;
      goto LAB_00614e74;
    }
  }
  else {
    if (param_1 < -0x7789fe6f) {
      if (param_1 == -0x7789fe70) {
        uID = 0xde07;
        goto LAB_00614e74;
      }
      switch(param_1) {
      case -0x7789fe84:
        uID = 0xde43;
        break;
      default:
        goto switchD_006148cc_caseD_8876017d;
      case -0x7789fe82:
        uID = 0xde44;
        break;
      case -0x7789fe80:
        uID = 0xde45;
        break;
      case -0x7789fe7d:
        uID = 0xde47;
      }
      goto LAB_00614e74;
    }
    if (param_1 < -0x7789fe5b) {
      if (param_1 == -0x7789fe5c) {
        uID = 0xde4b;
        goto LAB_00614e74;
      }
      if (param_1 == -0x7789fe66) {
        uID = 0xde4a;
        goto LAB_00614e74;
      }
    }
    else if (param_1 < -0x7789fe4c) {
      if (param_1 == -0x7789fe4d) {
        uID = 0xde85;
        goto LAB_00614e74;
      }
      if (param_1 == -0x7789fe52) {
        uID = 0xde4c;
        goto LAB_00614e74;
      }
    }
    else if (param_1 < -0x7789fe3d) {
      if (param_1 == -0x7789fe3e) {
        uID = 0xde4e;
        goto LAB_00614e74;
      }
      if (param_1 == -0x7789fe48) {
        uID = 0xde4d;
        goto LAB_00614e74;
      }
    }
    else if (param_1 < -0x7789fe29) {
      if (param_1 == -0x7789fe2a) {
        uID = 0xde50;
        goto LAB_00614e74;
      }
      if (param_1 == -0x7789fe34) {
        uID = 0xde4f;
        goto LAB_00614e74;
      }
    }
    else if (param_1 < -0x7789fe15) {
      if (param_1 == -0x7789fe16) {
        uID = 0xde52;
        goto LAB_00614e74;
      }
      if (param_1 == -0x7789fe20) {
        uID = 0xde51;
        goto LAB_00614e74;
      }
    }
    else if (param_1 < -0x7789fdf7) {
      if (param_1 == -0x7789fdf8) {
        uID = 0xde55;
        goto LAB_00614e74;
      }
      if (param_1 == -0x7789fe02) {
        uID = 0xde54;
        goto LAB_00614e74;
      }
    }
    else if (param_1 < -0x7789fde3) {
      if (param_1 == -0x7789fde4) {
        uID = 0xde57;
        goto LAB_00614e74;
      }
      if (param_1 == -0x7789fde7) {
        uID = 0xde56;
        goto LAB_00614e74;
      }
    }
    else {
      if (param_1 < -0x7789fd7f) {
        if (param_1 == -0x7789fd80) {
          uID = 0xde88;
          goto LAB_00614e74;
        }
        switch(param_1) {
        case -0x7789fdd0:
          uID = 0xde59;
          break;
        case -0x7789fdcf:
          uID = 0xde14;
          break;
        case -0x7789fdce:
          uID = 0xde09;
          break;
        case -0x7789fdcd:
          uID = 0xde27;
          break;
        case -0x7789fdcc:
          uID = 0xde48;
          break;
        case -0x7789fdcb:
          uID = 0xde28;
          break;
        case -0x7789fdca:
          uID = 0xde49;
          break;
        case -0x7789fdc9:
          uID = 0xde06;
          break;
        case -0x7789fdc8:
          uID = 0xde20;
          break;
        case -0x7789fdc7:
          uID = 0xde2c;
          break;
        case -0x7789fdc6:
          uID = 0xde0f;
          break;
        case -0x7789fdc5:
          uID = 0xde0e;
          break;
        case -0x7789fdc4:
          uID = 0xde30;
          break;
        case -0x7789fdc3:
          uID = 0xde31;
          break;
        case -0x7789fdc2:
          uID = 0xde01;
          break;
        case -0x7789fdc1:
          uID = 0xde1e;
          break;
        case -0x7789fdc0:
          uID = 0xde26;
          break;
        case -0x7789fdbf:
          uID = 0xde46;
          break;
        case -0x7789fdbe:
          uID = 0xde2e;
          break;
        case -0x7789fdbd:
          uID = 0xde19;
          break;
        case -0x7789fdbc:
          uID = 0xde38;
          break;
        case -0x7789fdbb:
          uID = 0xde0b;
          break;
        case -0x7789fdba:
          uID = 0xde3a;
          break;
        case -0x7789fdb9:
          uID = 0xde05;
          break;
        case -0x7789fdb8:
          uID = 0xde3c;
          break;
        case -0x7789fdb7:
          uID = 0xde04;
          break;
        case -0x7789fdb6:
          uID = 0xde25;
          break;
        case -0x7789fdb5:
          uID = 0xde58;
          break;
        case -0x7789fdb4:
          uID = 0xde10;
          break;
        case -0x7789fdb3:
          uID = 0xde3d;
          break;
        case -0x7789fdb2:
          uID = 0xde83;
          break;
        case -0x7789fdb1:
          uID = 0xde82;
          break;
        case -0x7789fdb0:
          uID = 0xde86;
          break;
        default:
          goto switchD_006148cc_caseD_8876017d;
        case -0x7789fd94:
          uID = 0xde87;
        }
        goto LAB_00614e74;
      }
      if (param_1 < -0x7789fd57) {
        if (param_1 == -0x7789fd58) {
          uID = 0xde8a;
          goto LAB_00614e74;
        }
        if (param_1 == -0x7789fd6c) {
          uID = 0xde89;
          goto LAB_00614e74;
        }
      }
      else if (param_1 < -0x7789fd3e) {
        if (param_1 == -0x7789fd3f) {
          uID = 0xde94;
          goto LAB_00614e74;
        }
        if (param_1 == -0x7789fd44) {
          uID = 0xde5a;
          goto LAB_00614e74;
        }
        if (param_1 == -0x7789fd43) {
          uID = 0xde5b;
          goto LAB_00614e74;
        }
      }
      else {
        if (param_1 < -0x7789fd11) {
          if (param_1 == -0x7789fd12) {
            uID = 0xde73;
            goto LAB_00614e74;
          }
          switch(param_1) {
          case -0x7789fd3a:
            uID = 0xde5e;
            break;
          case -0x7789fd39:
            uID = 0xde5f;
            break;
          case -0x7789fd38:
            uID = 0xde60;
            break;
          case -0x7789fd37:
            uID = 0xde61;
            break;
          case -0x7789fd36:
            uID = 0xde5c;
            break;
          case -0x7789fd35:
            uID = 0xde5d;
            break;
          case -0x7789fd34:
            uID = 0xde62;
            break;
          case -0x7789fd33:
            uID = 0xde63;
            break;
          default:
            goto switchD_006148cc_caseD_8876017d;
          case -0x7789fd30:
            uID = 0xde64;
            break;
          case -0x7789fd2f:
            uID = 0xde67;
            break;
          case -0x7789fd2e:
            uID = 0xde68;
            break;
          case -0x7789fd2d:
            uID = 0xde69;
            break;
          case -0x7789fd2c:
            uID = 0xde6a;
            break;
          case -0x7789fd2b:
            uID = 0xde6b;
            break;
          case -0x7789fd29:
            uID = 0xde66;
            break;
          case -0x7789fd28:
            uID = 0xde65;
            break;
          case -0x7789fd27:
            uID = 0xde95;
            break;
          case -0x7789fd26:
            uID = 0xde6c;
            break;
          case -0x7789fd25:
            uID = 0xde6d;
            break;
          case -0x7789fd24:
            uID = 0xde6e;
            break;
          case -0x7789fd23:
            uID = 0xde96;
            break;
          case -0x7789fd22:
            uID = 0xde6f;
            break;
          case -0x7789fd21:
            uID = 0xdea0;
            break;
          case -0x7789fd20:
            uID = 0xdea1;
            break;
          case -0x7789fd1f:
            uID = 0xdea2;
            break;
          case -0x7789fd1e:
            uID = 0xdea3;
            break;
          case -0x7789fd1c:
            uID = 0xde70;
            break;
          case -0x7789fd1b:
            uID = 0xde71;
            break;
          case -0x7789fd1a:
            uID = 0xde72;
            break;
          case -0x7789fd19:
            uID = 0xde97;
          }
          goto LAB_00614e74;
        }
        if (param_1 < 1) {
          switch(param_1) {
          case -0x7789fd08:
            uID = 0xde98;
            break;
          case -0x7789fd07:
            uID = 0xde99;
            break;
          case -0x7789fd06:
            uID = 0xde9a;
            break;
          case -0x7789fd05:
            uID = 0xde9b;
            break;
          default:
            goto switchD_006148cc_caseD_8876017d;
          case -0x7789fcfe:
            uID = 0xde9c;
            break;
          case -0x7789fcfd:
            uID = 0xde9d;
            break;
          case -0x7789fcfc:
            uID = 0xde9e;
            break;
          case -0x7789fcfb:
            uID = 0xde9f;
            break;
          case -0x7789fcf3:
            uID = 0xde74;
            break;
          case -0x7789fcf2:
            uID = 0xde75;
            break;
          case -0x7789fcf1:
            uID = 0xde76;
            break;
          case -0x7789fcf0:
            uID = 0xde77;
            break;
          case -0x7789fcef:
            uID = 0xde78;
            break;
          case -0x7789fcee:
            uID = 0xde79;
            break;
          case -0x7789fced:
            uID = 0xde7a;
            break;
          case -0x7789fcec:
            uID = 0xde7b;
            break;
          case -0x7789fceb:
            uID = 0xde7c;
            break;
          case -0x7789fcea:
            uID = 0xde7d;
            break;
          case -0x7789fce9:
            uID = 0xde7e;
            break;
          case -0x7789fce8:
            uID = 0xde7f;
            break;
          case -0x7789fce7:
            uID = 0xde80;
            break;
          case -0x7789fce6:
            uID = 0xde8c;
            break;
          case -0x7789fce5:
            uID = 0xde8d;
            break;
          case -0x7789fce4:
            uID = 0xde8e;
            break;
          case -0x7789fce3:
            uID = 0xde8f;
            break;
          case -0x7789fce2:
            uID = 0xde90;
            break;
          case -0x7789fce1:
            uID = 0xde91;
            break;
          case -0x7789fce0:
            uID = 0xde92;
            break;
          case -0x7789fcdf:
            uID = 0xde93;
          }
          goto LAB_00614e74;
        }
      }
    }
  }
switchD_006148cc_caseD_8876017d:
  uID = 0xde81;
LAB_00614e74:
  if (DAT_006be6d4 != (int *)0x0) {
    (**(code **)(*DAT_006be6d4 + 0x28))(DAT_006be6d4);
  }
  if (DAT_006be704 != (HINSTANCE)0x0) {
    LoadStringA(DAT_006be704,0xddff,local_200,0xff);
    LoadStringA(DAT_006be704,uID,local_100,0xff);
    MessageBoxExA(param_2,local_100,local_200,0x10010,0x409);
  }
  return;
}

