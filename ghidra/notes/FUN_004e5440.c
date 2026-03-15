
undefined4 __fastcall FUN_004e5440(int *param_1)

{
  undefined4 uVar1;
  
  uVar1 = 0;
  switch(param_1[2]) {
  case 1:
    FUN_004e5530(param_1);
    param_1[2] = 2;
    return 0;
  case 2:
    FUN_004e5560((int)param_1);
    param_1[2] = 3;
    return 0;
  case 3:
    FUN_004e5710(param_1);
    param_1[2] = 4;
    return 0;
  case 4:
    FUN_004e57e0(param_1);
    param_1[2] = 5;
    return 0;
  case 5:
    (**(code **)(*param_1 + 0x20))();
    param_1[2] = 6;
    return 0;
  case 6:
    FUN_004e5870((int)param_1);
    param_1[2] = 0;
    uVar1 = 1;
  }
  param_1[2] = 1;
  return uVar1;
}

