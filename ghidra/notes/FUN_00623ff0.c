
int FUN_00623ff0(uint param_1,uint param_2,char *param_3,int param_4)

{
  uint uVar1;
  int iVar2;
  char *_Source;
  int iVar3;
  int iVar4;
  
  iVar3 = 0;
  iVar4 = 0x1a;
  _Source = (char *)0x0;
  do {
    iVar2 = (iVar4 + iVar3) / 2;
    uVar1 = *(uint *)(iVar2 * 0x2c + 0x6afa78);
    if (param_1 == uVar1) {
      if (param_2 < 8) {
        if (param_2 == 7) {
          _Source = (char *)(iVar2 * 0x2c + 0x6afa90);
        }
        else if (param_2 == 1) {
          _Source = (char *)(iVar2 * 0x2c + 0x6afa7c);
        }
        else if (param_2 == 3) {
          _Source = (char *)(iVar2 * 0x2c + 0x6afa88);
        }
      }
      else if (param_2 < 0x1002) {
        if (param_2 == 0x1001) {
          _Source = *(char **)(iVar2 * 0x2c + 0x6afa84);
        }
        else if (param_2 == 0xb) {
          _Source = (char *)(iVar2 * 0x2c + 0x6afa94);
        }
      }
      else if (param_2 == 0x1002) {
        _Source = *(char **)(iVar2 * 0x2c + 0x6afa8c);
      }
      else if (param_2 == 0x1004) {
        _Source = (char *)(iVar2 * 0x2c + 0x6afa9c);
      }
      if ((_Source != (char *)0x0) && (0 < param_4)) {
        _strncpy(param_3,_Source,param_4 - 1);
        param_3[param_4 + -1] = '\0';
        return 1;
      }
      break;
    }
    if (param_1 < uVar1) {
      iVar4 = iVar2 + -1;
    }
    else {
      iVar3 = iVar2 + 1;
    }
  } while (iVar3 <= iVar4);
  iVar4 = GetLocaleInfoA(param_1,param_2,param_3,param_4);
  return iVar4;
}

