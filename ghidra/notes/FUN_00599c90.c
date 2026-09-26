
undefined4 __thiscall FUN_00599c90(void *this,undefined4 param_1,undefined4 param_2)

{
  int iVar1;
  void *pvVar2;
  undefined4 *in_stack_00000018;
  float in_stack_0000001c;
  undefined4 in_stack_00000020;
  undefined4 in_stack_00000024;
  undefined4 in_stack_00000028;
  undefined4 in_stack_0000002c;
  undefined4 in_stack_00000030;
  int in_stack_00000034;
  undefined4 in_stack_00000038;
  undefined4 in_stack_0000003c;
  undefined4 *in_stack_00000040;
  undefined4 uStack_20;
  undefined4 *puVar3;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puStack_8 = &LAB_006517eb;
  local_c = ExceptionList;
  local_4._0_1_ = 2;
  local_4._1_3_ = 0;
  ExceptionList = &local_c;
  if (DAT_006bc4a8 != (void *)0x0) {
    ExceptionList = &local_c;
    FUN_0059a8b0(DAT_006bc4a8,1);
  }
  DAT_006bc4a8 = (undefined4 *)0x0;
  FUN_005aafa0();
  *(undefined4 **)((int)this + 0x668) = in_stack_00000040;
  if (in_stack_00000040 == (undefined4 *)0x0) {
    FUN_005be0f0();
    iVar1 = FUN_005aaf90();
    *(undefined4 *)(iVar1 + 0x9ec) = param_1;
    if (in_stack_00000034 == 1) {
      in_stack_00000040 = &uStack_20;
      FUN_005f2f90(&uStack_20,(int)&stack0x0000000c);
      param_2 = 0x1b;
    }
    else {
      in_stack_00000040 = &uStack_20;
      FUN_005f2f90(&uStack_20,(int)&stack0x0000000c);
    }
    local_4._0_1_ = 2;
    pvVar2 = (void *)FUN_005aaf90();
    FUN_0059a850(pvVar2,param_2);
    iVar1 = 0;
    uStack_20 = 0x599fcc;
    puVar3 = in_stack_00000018;
    pvVar2 = (void *)FUN_005aaf90();
    uStack_20 = 0x599fd3;
    FUN_005a8b10(pvVar2,iVar1,(float)puVar3);
    iVar1 = 1;
    uStack_20 = 0x599fdf;
    pvVar2 = (void *)FUN_005aaf90();
    uStack_20 = 0x599fe6;
    FUN_005a8b10(pvVar2,iVar1,in_stack_0000001c);
    iVar1 = 0;
    uStack_20 = 0x599ff1;
    pvVar2 = (void *)FUN_005aaf90();
    uStack_20 = 0x599ff8;
    FUN_005a8af0(pvVar2,iVar1,in_stack_00000020);
    iVar1 = 1;
    uStack_20 = 0x59a004;
    pvVar2 = (void *)FUN_005aaf90();
    uStack_20 = 0x59a00b;
    FUN_005a8af0(pvVar2,iVar1,in_stack_00000024);
    iVar1 = FUN_005aaf90();
    *(undefined4 *)(*(int *)(iVar1 + 8) + 0x24) = in_stack_00000028;
    iVar1 = FUN_005aaf90();
    *(undefined4 *)(*(int *)(iVar1 + 0xc) + 0x24) = in_stack_0000002c;
    iVar1 = FUN_005aaf90();
    *(undefined4 *)(iVar1 + 0x8e8) = in_stack_00000030;
    uStack_20 = 0x59a047;
    pvVar2 = (void *)FUN_005aaf90();
    uStack_20 = 0x59a04e;
    FUN_005ae1b0(pvVar2,in_stack_0000003c,in_stack_00000038);
    iVar1 = FUN_005aaf90();
    *(undefined4 *)(iVar1 + 0x898) = 0x43160000;
  }
  else {
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000040 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 3;
      if (in_stack_00000040 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000040);
      }
      local_4._0_1_ = 2;
    }
    uStack_20 = 0x599d28;
    FUN_005afb50(DAT_006bc4a8,0,in_stack_00000018);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 4;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 2;
    }
    uStack_20 = 0x599d70;
    FUN_005afb50(DAT_006bc4a8,1,in_stack_0000001c);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 5;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 2;
    }
    uStack_20 = 0x599db7;
    FUN_005afb30(DAT_006bc4a8,0,in_stack_00000020);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 6;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 2;
    }
    uStack_20 = 0x599dff;
    FUN_005afb30(DAT_006bc4a8,1,in_stack_00000024);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 7;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 2;
    }
    *(undefined4 *)(DAT_006bc4a8[0x93] + 0xb0) = 0;
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 8;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 2;
    }
    *(undefined4 *)(DAT_006bc4a8[0x94] + 0xb0) = 0;
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 9;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 2;
    }
    *(undefined4 *)(DAT_006bc4a8[0x93] + 0x24) = in_stack_00000028;
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 10;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 2;
    }
    *(undefined4 *)(DAT_006bc4a8[0x94] + 0x24) = in_stack_0000002c;
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 0xb;
      if (in_stack_00000018 != (undefined4 *)0x0) {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
        DAT_006bc4a8[0x11] = in_stack_00000030;
        goto LAB_0059a05d;
      }
      DAT_006bc4a8 = (undefined4 *)0x0;
    }
    DAT_006bc4a8[0x11] = in_stack_00000030;
  }
LAB_0059a05d:
  local_4._0_1_ = 1;
  FUN_005f2ff0((undefined4 *)&stack0x0000000c);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = local_c;
  return 1;
}

