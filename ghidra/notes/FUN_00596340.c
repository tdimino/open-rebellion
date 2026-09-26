
undefined4 __cdecl FUN_00596340(undefined4 param_1,undefined4 param_2)

{
  undefined2 uVar1;
  void *pvVar2;
  undefined4 *this;
  int iVar3;
  undefined4 uVar4;
  int *piVar5;
  void *this_00;
  undefined4 in_stack_00000018;
  undefined4 in_stack_0000001c;
  undefined4 in_stack_00000020;
  undefined4 in_stack_00000024;
  undefined4 in_stack_00000028;
  undefined4 in_stack_0000002c;
  undefined4 in_stack_00000030;
  undefined1 *in_stack_00000034;
  short in_stack_00000038;
  undefined1 auStack_44 [12];
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  undefined4 uStack_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00650de9;
  local_c = ExceptionList;
  local_4 = 0;
  uStack_20 = 0x596368;
  ExceptionList = &local_c;
  pvVar2 = (void *)FUN_00597530();
  uStack_20 = 0x59636f;
  uVar1 = FUN_005975d0(pvVar2,in_stack_00000038);
  uStack_20 = 0x596378;
  this = (undefined4 *)FUN_00618b70(100);
  local_4._0_1_ = 1;
  _in_stack_00000038 = this;
  if (this == (undefined4 *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    uStack_20 = in_stack_00000030;
    uStack_24 = in_stack_0000002c;
    uStack_28 = in_stack_00000028;
    uStack_2c = in_stack_00000024;
    uStack_30 = in_stack_00000020;
    uStack_34 = in_stack_0000001c;
    uStack_38 = in_stack_00000018;
    in_stack_00000034 = auStack_44;
    FUN_005f2f90(auStack_44,(int)&stack0x0000000c);
    pvVar2 = FUN_00596c40(this,param_1,param_2,uVar1);
  }
  local_4._0_1_ = 0;
  iVar3 = FUN_00597530();
  if (*(int *)(iVar3 + 0x668) == 0) {
    piVar5 = (int *)FUN_005aaf90();
    uStack_20 = 0x596483;
    (**(code **)(*piVar5 + 0x58))();
    uStack_20 = 0x596489;
    this_00 = (void *)FUN_005aaf90();
    uStack_20 = 0x596490;
    uVar4 = FUN_005a93e0(this_00,pvVar2);
  }
  else {
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      uStack_20 = 0x596406;
      _in_stack_00000038 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 2;
      if (_in_stack_00000038 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(_in_stack_00000038);
      }
      local_4._0_1_ = 0;
    }
    uStack_20 = 0x596433;
    FUN_005afdb0(pvVar2);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      uStack_20 = 0x596447;
      _in_stack_00000038 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 3;
      if (_in_stack_00000038 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(_in_stack_00000038);
      }
      local_4._0_1_ = 0;
    }
    uStack_20 = 0x596474;
    uVar4 = FUN_005af440(DAT_006bc4a8,pvVar2);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x0000000c);
  ExceptionList = local_c;
  return uVar4;
}

