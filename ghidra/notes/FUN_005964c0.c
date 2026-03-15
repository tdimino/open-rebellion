
int __cdecl FUN_005964c0(undefined4 param_1,undefined4 param_2,undefined4 param_3,uint param_4)

{
  undefined1 *puVar1;
  void *pvVar2;
  uint uVar3;
  int *piVar4;
  int iVar5;
  undefined4 *puVar6;
  int *piVar7;
  undefined1 *in_stack_00000020;
  int in_stack_00000024;
  undefined4 in_stack_00000028;
  undefined4 in_stack_0000002c;
  undefined4 in_stack_00000030;
  undefined4 in_stack_00000034;
  undefined4 in_stack_00000038;
  undefined4 in_stack_0000003c;
  undefined4 in_stack_00000040;
  undefined4 in_stack_00000044;
  undefined4 in_stack_00000048;
  undefined4 in_stack_0000004c;
  undefined4 in_stack_00000050;
  undefined4 in_stack_00000054;
  undefined4 in_stack_00000058;
  undefined4 in_stack_0000005c;
  undefined4 in_stack_00000060;
  undefined4 in_stack_00000064;
  undefined4 in_stack_00000068;
  undefined4 in_stack_0000006c;
  undefined4 in_stack_00000070;
  undefined4 in_stack_00000074;
  undefined4 in_stack_00000078;
  undefined4 in_stack_0000007c;
  undefined4 in_stack_00000080;
  undefined1 auStack_8c [12];
  undefined1 *puStack_80;
  undefined4 uStack_7c;
  undefined4 uStack_78;
  undefined4 uStack_74;
  undefined4 uStack_70;
  undefined4 uStack_6c;
  undefined4 uStack_68;
  undefined4 uStack_64;
  undefined4 uStack_60;
  undefined4 uStack_5c;
  undefined4 uStack_58;
  undefined4 uStack_54;
  undefined4 uStack_50;
  undefined4 uStack_4c;
  undefined4 uStack_48;
  undefined4 uStack_44;
  undefined4 uStack_40;
  undefined4 uStack_3c;
  undefined4 uStack_38;
  undefined4 uStack_34;
  undefined4 uStack_30;
  undefined4 uStack_2c;
  undefined4 uStack_28;
  undefined4 uStack_24;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00650e4a;
  local_c = ExceptionList;
  local_4 = 0;
  uStack_24 = 0x5964e9;
  ExceptionList = &local_c;
  pvVar2 = (void *)FUN_00597530();
  uStack_24 = 0x5964f0;
  uVar3 = FUN_00599c40(pvVar2,param_4);
  uStack_24 = 0x5964fc;
  pvVar2 = (void *)FUN_00618b70(0x418);
  puVar1 = in_stack_00000020;
  local_4._0_1_ = 1;
  if (pvVar2 == (void *)0x0) {
    piVar4 = (int *)0x0;
  }
  else {
    uStack_24 = in_stack_00000080;
    uStack_28 = in_stack_0000007c;
    uStack_2c = in_stack_00000078;
    uStack_30 = in_stack_00000074;
    uStack_34 = in_stack_00000070;
    uStack_38 = in_stack_0000006c;
    uStack_3c = in_stack_00000068;
    uStack_40 = in_stack_00000064;
    uStack_44 = in_stack_00000060;
    uStack_48 = in_stack_0000005c;
    uStack_4c = in_stack_00000058;
    uStack_50 = in_stack_00000054;
    uStack_54 = in_stack_00000050;
    uStack_58 = in_stack_0000004c;
    uStack_5c = in_stack_00000048;
    uStack_60 = in_stack_00000044;
    uStack_64 = in_stack_00000040;
    uStack_68 = in_stack_0000003c;
    uStack_6c = in_stack_00000038;
    uStack_70 = in_stack_00000034;
    uStack_74 = in_stack_00000030;
    uStack_78 = in_stack_0000002c;
    uStack_7c = in_stack_00000028;
    puStack_80 = in_stack_00000020;
    in_stack_00000020 = auStack_8c;
    FUN_005f2f90(auStack_8c,(int)&stack0x00000014);
    piVar4 = (int *)FUN_005b9c60(pvVar2,param_1,param_2,param_3,uVar3);
  }
  local_4._0_1_ = 0;
  iVar5 = FUN_00597530();
  if (*(int *)(iVar5 + 0x668) == 0) {
    piVar7 = (int *)FUN_005aaf90();
    uStack_24 = 0x596679;
    (**(code **)(*piVar7 + 100))();
  }
  else {
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      uStack_24 = 0x59663d;
      puVar6 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 2;
      if (puVar6 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(puVar6);
      }
      local_4._0_1_ = 0;
    }
    uStack_24 = 0x59666a;
    FUN_005afd40((int)piVar4);
  }
  if (in_stack_00000024 == 1) {
    if (DAT_006bc470 == (undefined4 *)0x0) {
      uStack_24 = 0x596694;
      puVar6 = (undefined4 *)FUN_00618b70(0x48);
      local_4._0_1_ = 3;
      if (puVar6 == (undefined4 *)0x0) {
        DAT_006bc470 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc470 = FUN_005a6290(puVar6);
      }
      local_4._0_1_ = 0;
    }
    uStack_24 = 0x5966c0;
    piVar7 = FUN_005a6360(DAT_006bc470,(uint)puVar1);
    if (piVar7 == (int *)0x0) goto LAB_0059678c;
    iVar5 = FUN_005b1b70(piVar4);
    if (iVar5 == 0) {
      FUN_005b69d0((int)piVar4);
      uStack_24 = 0x5966e4;
      FUN_005b8570(piVar7,(int)piVar4);
      iVar5 = FUN_00597530();
      if (*(int *)(iVar5 + 0x668) == 0) {
        uStack_24 = 0x59673d;
        pvVar2 = (void *)FUN_005aaf90();
        uStack_24 = 0x596744;
        iVar5 = FUN_005a9470(pvVar2,(int)piVar4);
      }
      else {
        if (DAT_006bc4a8 == (undefined4 *)0x0) {
          uStack_24 = 0x596705;
          puVar6 = (undefined4 *)FUN_00618b70(0x274);
          local_4._0_1_ = 4;
          if (puVar6 == (undefined4 *)0x0) {
            DAT_006bc4a8 = (undefined4 *)0x0;
          }
          else {
            DAT_006bc4a8 = FUN_005af5f0(puVar6);
          }
          local_4._0_1_ = 0;
        }
        uStack_24 = 0x596732;
        iVar5 = FUN_005af4b0(DAT_006bc4a8,(int)piVar4);
      }
      goto LAB_005967f0;
    }
    FUN_005b69b0((int)piVar4);
    iVar5 = FUN_00597530();
    if (*(int *)(iVar5 + 0x668) == 0) goto LAB_005967e3;
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      uStack_24 = 0x596773;
      puVar6 = (undefined4 *)FUN_00618b70(0x274);
      local_4 = CONCAT31(local_4._1_3_,5);
      if (puVar6 == (undefined4 *)0x0) goto LAB_005967cd;
      DAT_006bc4a8 = FUN_005af5f0(puVar6);
      goto LAB_005967cf;
    }
  }
  else {
LAB_0059678c:
    FUN_005b69b0((int)piVar4);
    iVar5 = FUN_00597530();
    if (*(int *)(iVar5 + 0x668) == 0) {
LAB_005967e3:
      uStack_24 = 0x5967e9;
      pvVar2 = (void *)FUN_005aaf90();
      uStack_24 = 0x5967f0;
      iVar5 = FUN_005a9340(pvVar2,piVar4);
      goto LAB_005967f0;
    }
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      uStack_24 = 0x5967b4;
      puVar6 = (undefined4 *)FUN_00618b70(0x274);
      local_4 = CONCAT31(local_4._1_3_,6);
      if (puVar6 == (undefined4 *)0x0) {
LAB_005967cd:
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(puVar6);
      }
LAB_005967cf:
      local_4._0_1_ = 0;
    }
  }
  uStack_24 = 0x5967e1;
  iVar5 = FUN_005af5c0(DAT_006bc4a8,(int)piVar4);
LAB_005967f0:
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x00000014);
  ExceptionList = local_c;
  return iVar5;
}

