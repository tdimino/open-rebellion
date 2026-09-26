
undefined4 __fastcall FUN_005df660(int param_1)

{
  undefined4 uVar1;
  
  uVar1 = 0;
  switch(*(undefined4 *)(param_1 + 0xc4)) {
  case 2:
    uVar1 = 0xdd05;
    break;
  case 4:
    return 0xdd04;
  case 5:
    return 0xdd03;
  case 6:
    return 0xdd06;
  }
  return uVar1;
}

