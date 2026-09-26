
undefined4 __cdecl FUN_00437df0(int param_1)

{
  int iVar1;
  void *this;
  int iVar2;
  void *unaff_EBX;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d67b;
  local_c = ExceptionList;
  if (((DAT_006be3b8 != 0) && (iVar1 = *(int *)(DAT_006be3b8 + 0xc0), iVar1 != 0)) && (param_1 != 0)
     ) {
    ExceptionList = &local_c;
    this = (void *)FUN_00618b70(0x154);
    local_4 = 0;
    if (this == (void *)0x0) {
      DAT_006b28ac = (int *)0x0;
    }
    else {
      DAT_006b28ac = FUN_00437880(this,*(undefined4 *)(iVar1 + 0x1c),0,0,0xd2,0x105,iVar1,2,7,0x2a30
                                  ,0x62,0,5,param_1);
    }
    local_4 = 0xffffffff;
    if (DAT_006b28ac != (int *)0x0) {
      iVar2 = (**(code **)(*DAT_006b28ac + 0x50))();
      if (iVar2 != 0) {
        DAT_006b28ac[0x14] = 0x46000000;
        FUN_005ffce0(DAT_006b28ac,0);
        FUN_00606980(DAT_006b28ac,*(int *)(iVar1 + 0xcc),*(int *)(iVar1 + 0xd0),
                     *(int *)(iVar1 + 0xd4),*(int *)(iVar1 + 0xd8));
        FUN_00607ce0(DAT_006b28ac,2);
        FUN_006071a0(DAT_006b28ac);
        FUN_005f4f10((void *)(iVar1 + 0x6c),(int)DAT_006b28ac);
        (**(code **)(*DAT_006b28ac + 0x2c))(5);
        ExceptionList = unaff_EBX;
        return 1;
      }
      if (DAT_006b28ac != (int *)0x0) {
        (**(code **)*DAT_006b28ac)(1);
      }
      DAT_006b28ac = (int *)0x0;
    }
  }
  ExceptionList = local_c;
  return 0;
}

