
undefined4 __cdecl
FUN_00416c50(undefined4 param_1,int param_2,undefined4 param_3,undefined4 param_4)

{
  HWND hWnd;
  void *this;
  int iVar1;
  void *unaff_ESI;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062addb;
  local_c = ExceptionList;
  if ((param_2 != 0) && (DAT_006b146c == (int *)0x0)) {
    ExceptionList = &local_c;
    this = (void *)FUN_00618b70(0x120);
    local_4 = 0;
    if (this == (void *)0x0) {
      DAT_006b146c = (int *)0x0;
    }
    else {
      DAT_006b146c = (int *)FUN_00416b90(this,*(undefined4 *)(param_2 + 0x1c),0,0,0x19c,0xb0,param_2
                                         ,0xc,0xc,0x297f,0xd,0x44000000,1,param_1,param_3,param_4);
    }
    local_4 = 0xffffffff;
    if (DAT_006b146c != (int *)0x0) {
      iVar1 = (**(code **)(*DAT_006b146c + 0x50))();
      if (iVar1 != 0) {
        FUN_005ffce0(DAT_006b146c,0);
        FUN_005ffeb0((int)DAT_006b146c);
        hWnd = (HWND)DAT_006b146c[6];
        (**(code **)(*DAT_006b146c + 0x2c))(5);
        SetWindowPos(hWnd,(HWND)0x0,0,0,0,0,0x43);
        SendMessageA(*(HWND *)(param_2 + 0x18),0x479,0,0);
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

