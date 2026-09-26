
undefined4 __thiscall FUN_0059a0b0(void *this,undefined4 param_1,undefined4 param_2)

{
  undefined *puVar1;
  int iVar2;
  void *pvVar3;
  undefined4 *in_stack_00000018;
  float in_stack_0000001c;
  undefined4 in_stack_00000020;
  undefined4 in_stack_00000024;
  undefined4 in_stack_00000028;
  int in_stack_0000002c;
  undefined4 in_stack_00000030;
  undefined4 in_stack_00000034;
  undefined4 *in_stack_00000038;
  undefined4 uStack_24;
  undefined4 *puVar4;
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  puVar1 = (undefined *)DAT_006bc4a8;
  puStack_8 = &LAB_00651865;
  local_c = ExceptionList;
  local_4._0_1_ = 0;
  local_4._1_3_ = 0;
  ExceptionList = &local_c;
  if (DAT_006bc4a8 != (undefined4 *)0x0) {
    ExceptionList = &local_c;
    FUN_005afa50((int)DAT_006bc4a8);
    FUN_00618b60(puVar1);
  }
  DAT_006bc4a8 = (undefined4 *)0x0;
  FUN_005aafa0();
  *(undefined4 **)((int)this + 0x668) = in_stack_00000038;
  if (in_stack_00000038 == (undefined4 *)0x0) {
    FUN_005bf320();
    iVar2 = FUN_005aaf90();
    *(undefined4 *)(iVar2 + 0x9ec) = param_1;
    if (in_stack_0000002c == 1) {
      in_stack_00000038 = &uStack_24;
      FUN_005f2f90(&uStack_24,(int)&stack0x0000000c);
      param_2 = 0x1b;
    }
    else {
      in_stack_00000038 = &uStack_24;
      FUN_005f2f90(&uStack_24,(int)&stack0x0000000c);
    }
    local_4 = (uint)local_4._1_3_ << 8;
    pvVar3 = (void *)FUN_005aaf90();
    FUN_0059a850(pvVar3,param_2);
    iVar2 = 0;
    uStack_24 = 0x59a360;
    puVar4 = in_stack_00000018;
    pvVar3 = (void *)FUN_005aaf90();
    uStack_24 = 0x59a367;
    FUN_005a8b10(pvVar3,iVar2,(float)puVar4);
    iVar2 = 1;
    uStack_24 = 0x59a373;
    pvVar3 = (void *)FUN_005aaf90();
    uStack_24 = 0x59a37a;
    FUN_005a8b10(pvVar3,iVar2,in_stack_0000001c);
    iVar2 = 0;
    uStack_24 = 0x59a385;
    pvVar3 = (void *)FUN_005aaf90();
    uStack_24 = 0x59a38c;
    FUN_005a8af0(pvVar3,iVar2,in_stack_00000020);
    iVar2 = 1;
    uStack_24 = 0x59a398;
    pvVar3 = (void *)FUN_005aaf90();
    uStack_24 = 0x59a39f;
    FUN_005a8af0(pvVar3,iVar2,in_stack_00000024);
    uStack_24 = 0x59a3ae;
    pvVar3 = (void *)FUN_005aaf90();
    uStack_24 = 0x59a3b5;
    FUN_005ae1b0(pvVar3,in_stack_00000034,in_stack_00000030);
    iVar2 = FUN_005aaf90();
    *(undefined4 *)(iVar2 + 0x8e8) = in_stack_00000028;
  }
  else {
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000038 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 1;
      if (in_stack_00000038 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000038);
      }
      local_4._0_1_ = 0;
    }
    uStack_24 = 0x59a14d;
    FUN_005afb50(DAT_006bc4a8,0,in_stack_00000018);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 2;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 0;
    }
    uStack_24 = 0x59a194;
    FUN_005afb50(DAT_006bc4a8,1,in_stack_0000001c);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 3;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 0;
    }
    uStack_24 = 0x59a1da;
    FUN_005afb30(DAT_006bc4a8,0,in_stack_00000020);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 4;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 0;
    }
    uStack_24 = 0x59a221;
    FUN_005afb30(DAT_006bc4a8,1,in_stack_00000024);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 5;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 0;
    }
    *(undefined4 *)(DAT_006bc4a8[0x93] + 0xb0) = 0;
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 6;
      if (in_stack_00000018 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
      }
      local_4._0_1_ = 0;
    }
    *(undefined4 *)(DAT_006bc4a8[0x94] + 0xb0) = 0;
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      in_stack_00000018 = (undefined4 *)FUN_00618b70(0x274);
      local_4 = CONCAT31(local_4._1_3_,7);
      if (in_stack_00000018 != (undefined4 *)0x0) {
        DAT_006bc4a8 = FUN_005af5f0(in_stack_00000018);
        DAT_006bc4a8[0x11] = in_stack_00000028;
        goto LAB_0059a3c4;
      }
      DAT_006bc4a8 = (undefined4 *)0x0;
    }
    DAT_006bc4a8[0x11] = in_stack_00000028;
  }
LAB_0059a3c4:
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x0000000c);
  ExceptionList = local_c;
  return 1;
}

