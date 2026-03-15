
undefined4 __cdecl FUN_00417260(int param_1,undefined4 param_2,undefined4 param_3)

{
  void *this;
  int iVar1;
  void *unaff_ESI;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062ae1b;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  if (DAT_006b146c != (int *)0x0) {
    ExceptionList = &local_c;
    FUN_00416d90();
  }
  if (param_1 != 0) {
    this = (void *)FUN_00618b70(0x120);
    local_4 = 0;
    if (this == (void *)0x0) {
      DAT_006b146c = (int *)0x0;
    }
    else {
      DAT_006b146c = (int *)FUN_004171c0(this,*(undefined4 *)(param_1 + 0x1c),0,0,0x19c,0xb0,param_1
                                         ,0xc,0xc,0x297d,0xd,0x44000000,1,0x472,param_2,param_3);
    }
    local_4 = 0xffffffff;
    if (DAT_006b146c != (int *)0x0) {
      iVar1 = (**(code **)(*DAT_006b146c + 0x50))();
      if (iVar1 != 0) {
        FUN_005ffce0(DAT_006b146c,0);
        FUN_005ffeb0((int)DAT_006b146c);
        SetWindowPos((HWND)DAT_006b146c[6],(HWND)0x0,0,0,0,0,3);
        (**(code **)(*DAT_006b146c + 0x2c))(5);
        ExceptionList = unaff_ESI;
        return 1;
      }
      if (DAT_006b146c != (int *)0x0) {
        (**(code **)*DAT_006b146c)(1);
      }
      DAT_006b146c = (int *)0x0;
    }
  }
  ExceptionList = local_c;
  return 0;
}

