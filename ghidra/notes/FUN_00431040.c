
undefined4 __fastcall FUN_00431040(int *param_1)

{
  void *pvVar1;
  int iVar2;
  int *piVar3;
  undefined1 *local_14;
  undefined4 local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062cdb3;
  local_c = ExceptionList;
  if ((void *)param_1[4] == (void *)0x0) {
    ExceptionList = &local_c;
    (**(code **)(*param_1 + 4))();
    piVar3 = FUN_004f31b0(param_1[2]);
    param_1[4] = (int)piVar3;
  }
  else {
    local_14 = &stack0xffffffdc;
    ExceptionList = &local_c;
    FUN_004025b0((void *)param_1[4],(uint *)&stack0xffffffdc);
    pvVar1 = FUN_00403d30(param_1);
    param_1[3] = (int)pvVar1;
    if (pvVar1 == (void *)0x0) {
      local_14 = (undefined1 *)FUN_00618b70(0x138);
      local_4 = 0;
      if (local_14 == (undefined1 *)0x0) {
        pvVar1 = (void *)0x0;
      }
      else {
        pvVar1 = FUN_00433b10(local_14,param_1[2],(void *)param_1[4]);
      }
      local_4 = 0xffffffff;
      param_1[3] = (int)pvVar1;
      if (pvVar1 != (void *)0x0) {
        FUN_00433e00((int)pvVar1);
        *(int *)(param_1[3] + 0x40) = param_1[5];
        FUN_004f57b0(param_1,(void *)param_1[3]);
      }
    }
    local_14 = (undefined1 *)0x90;
    local_10 = 0x98;
    local_4 = 1;
    iVar2 = FUN_004f6010((void *)param_1[4],(uint *)&local_14,1);
    local_4 = 0xffffffff;
    FUN_00619730();
    param_1[4] = iVar2;
    if (iVar2 == 0) {
      param_1[3] = 0;
      ExceptionList = local_c;
      return 1;
    }
  }
  ExceptionList = local_c;
  return 0;
}

