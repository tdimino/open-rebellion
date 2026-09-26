
undefined4 __cdecl FUN_005961e0(undefined4 param_1,undefined4 param_2,short param_3)

{
  undefined2 uVar1;
  void *pvVar2;
  int iVar3;
  undefined4 *puVar4;
  undefined4 uVar5;
  int *piVar6;
  void *this;
  undefined4 in_stack_0000001c;
  undefined1 *in_stack_00000020;
  undefined1 auStack_2c [12];
  undefined4 uStack_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00650da9;
  local_c = ExceptionList;
  local_4 = 0;
  uStack_20 = 0x596208;
  ExceptionList = &local_c;
  pvVar2 = (void *)FUN_00597530();
  uStack_20 = 0x59620f;
  uVar1 = FUN_005975d0(pvVar2,param_3);
  uStack_20 = 0x596218;
  pvVar2 = (void *)FUN_00618b70(0x4c);
  local_4._0_1_ = 1;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    uStack_20 = in_stack_0000001c;
    in_stack_00000020 = auStack_2c;
    FUN_005f2f90(auStack_2c,(int)&stack0x00000010);
    pvVar2 = FUN_00596df0(pvVar2,param_1,param_2,uVar1);
  }
  local_4._0_1_ = 0;
  iVar3 = FUN_00597530();
  if (*(int *)(iVar3 + 0x668) == 0) {
    piVar6 = (int *)FUN_005aaf90();
    uStack_20 = 0x596305;
    (**(code **)(*piVar6 + 0x54))();
    uStack_20 = 0x59630b;
    this = (void *)FUN_005aaf90();
    uStack_20 = 0x596312;
    uVar5 = FUN_005a9450(this,pvVar2);
  }
  else {
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      uStack_20 = 0x596288;
      puVar4 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 2;
      if (puVar4 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(puVar4);
      }
      local_4._0_1_ = 0;
    }
    uStack_20 = 0x5962b5;
    FUN_005afdb0(pvVar2);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      uStack_20 = 0x5962c9;
      puVar4 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 3;
      if (puVar4 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(puVar4);
      }
      local_4._0_1_ = 0;
    }
    uStack_20 = 0x5962f6;
    uVar5 = FUN_005af490(DAT_006bc4a8,pvVar2);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x00000010);
  ExceptionList = local_c;
  return uVar5;
}

