
int FUN_0046cab0(int param_1)

{
  int iVar1;
  
  switch(*(undefined4 *)(param_1 + 0x2c)) {
  case 0x51:
    return 7;
  case 0x52:
    return 9;
  case 0x53:
    iVar1 = *(int *)(param_1 + 0x18);
    if (iVar1 == 0x20) {
      return 4;
    }
    if (iVar1 == 0x21) {
      return 5;
    }
    if (iVar1 == 0x22) {
      return 6;
    }
    break;
  case 0x54:
    return 10;
  case 0x55:
    return 1;
  case 0x56:
    return 8;
  case 0x57:
    return 3;
  case 0x58:
    return 2;
  case 0x61:
    return 0xd;
  case 0x62:
    return 0xc;
  case 99:
    return 0xb;
  case 0x69:
    return 0xf;
  case 0x6a:
    return 0xe;
  }
  DAT_006a8794 = DAT_006a8794 + 1;
  return DAT_006a8794;
}

