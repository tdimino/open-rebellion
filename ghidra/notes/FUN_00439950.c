
undefined4 __fastcall FUN_00439950(int *param_1)

{
  int *piVar1;
  int *this;
  int iVar2;
  
  iVar2 = param_1[3];
  if (iVar2 == 1) {
    (**(code **)(*param_1 + 0x20))();
    param_1[3] = 2;
  }
  else if (iVar2 == 2) {
    iVar2 = FUN_0049cb30(param_1 + 0xf);
    if (iVar2 != 0) {
      param_1[3] = 3;
      return 0;
    }
  }
  else if (iVar2 == 3) {
    piVar1 = param_1 + 0xf;
    this = param_1 + 6;
    FUN_0049df20(this,0x14,piVar1);
    FUN_0049df20(this,0x15,piVar1);
    FUN_0049df20(this,0x17,piVar1);
    param_1[99] = 5;
    for (iVar2 = thunk_FUN_005f5060((int)this); iVar2 != 0; iVar2 = *(int *)(iVar2 + 0x10)) {
      *(undefined4 *)(iVar2 + 0x1c) = 2;
    }
    iVar2 = FUN_005f5500(this,0x17);
    param_1[0x61] = iVar2;
    *(int **)(iVar2 + 0x54) = param_1;
    param_1[3] = 5;
    return 1;
  }
  return 0;
}

