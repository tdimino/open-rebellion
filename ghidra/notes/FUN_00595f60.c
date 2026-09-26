
int __cdecl FUN_00595f60(undefined4 param_1,undefined4 param_2,undefined4 param_3,uint param_4)

{
  void *pvVar1;
  uint uVar2;
  int *piVar3;
  int iVar4;
  undefined4 *puVar5;
  int *piVar6;
  undefined4 in_stack_00000020;
  undefined4 in_stack_00000024;
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
  undefined4 in_stack_00000084;
  undefined4 in_stack_00000088;
  undefined4 in_stack_0000008c;
  undefined4 in_stack_00000090;
  undefined4 in_stack_00000094;
  undefined4 in_stack_00000098;
  undefined4 in_stack_0000009c;
  undefined4 in_stack_000000a0;
  undefined4 in_stack_000000a4;
  undefined1 *in_stack_000000a8;
  undefined1 auStack_b0 [12];
  undefined4 uStack_a4;
  undefined4 uStack_a0;
  undefined4 uStack_9c;
  undefined4 uStack_98;
  undefined4 uStack_94;
  undefined4 uStack_90;
  undefined4 uStack_8c;
  undefined4 uStack_88;
  undefined4 uStack_84;
  undefined4 uStack_80;
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
  undefined1 *puStack_20;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  puStack_8 = &LAB_00650d69;
  local_c = ExceptionList;
  local_4 = 0;
  puStack_20 = (undefined1 *)0x595f88;
  ExceptionList = &local_c;
  pvVar1 = (void *)FUN_00597530();
  puStack_20 = (undefined1 *)0x595f8f;
  uVar2 = FUN_00599c40(pvVar1,param_4);
  puStack_20 = (undefined1 *)0x595f9b;
  pvVar1 = (void *)FUN_00618b70(0x448);
  local_4._0_1_ = 1;
  if (pvVar1 == (void *)0x0) {
    piVar3 = (int *)0x0;
  }
  else {
    puStack_20 = in_stack_000000a8;
    uStack_24 = in_stack_000000a4;
    uStack_28 = in_stack_000000a0;
    uStack_2c = in_stack_0000009c;
    uStack_30 = in_stack_00000098;
    uStack_34 = in_stack_00000094;
    uStack_38 = in_stack_00000090;
    uStack_3c = in_stack_0000008c;
    uStack_40 = in_stack_00000088;
    uStack_44 = in_stack_00000084;
    uStack_48 = in_stack_00000080;
    uStack_4c = in_stack_0000007c;
    uStack_50 = in_stack_00000078;
    uStack_54 = in_stack_00000074;
    uStack_58 = in_stack_00000070;
    uStack_5c = in_stack_0000006c;
    uStack_60 = in_stack_00000068;
    uStack_64 = in_stack_00000064;
    uStack_68 = in_stack_00000060;
    uStack_6c = in_stack_0000005c;
    uStack_70 = in_stack_00000058;
    uStack_74 = in_stack_00000054;
    uStack_78 = in_stack_0000004c;
    uStack_7c = in_stack_00000048;
    uStack_80 = in_stack_00000044;
    uStack_84 = in_stack_00000040;
    uStack_88 = in_stack_0000003c;
    uStack_8c = in_stack_00000038;
    uStack_90 = in_stack_00000034;
    uStack_94 = in_stack_00000030;
    uStack_98 = in_stack_0000002c;
    uStack_9c = in_stack_00000028;
    uStack_a0 = in_stack_00000024;
    uStack_a4 = in_stack_00000020;
    in_stack_000000a8 = auStack_b0;
    FUN_005f2f90(auStack_b0,(int)&stack0x00000014);
    piVar3 = FUN_005b05c0(pvVar1,param_1,param_2,param_3,uVar2);
  }
  local_4._0_1_ = 0;
  iVar4 = FUN_00597530();
  if (*(int *)(iVar4 + 0x668) == 0) {
    piVar6 = (int *)FUN_005aaf90();
    puStack_20 = (undefined1 *)0x5961a8;
    (**(code **)(*piVar6 + 0x60))();
    puStack_20 = (undefined1 *)0x5961ae;
    pvVar1 = (void *)FUN_005aaf90();
    puStack_20 = (undefined1 *)0x5961b5;
    iVar4 = FUN_005a9340(pvVar1,piVar3);
  }
  else {
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      puStack_20 = (undefined1 *)0x59612b;
      puVar5 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 2;
      if (puVar5 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(puVar5);
      }
      local_4._0_1_ = 0;
    }
    puStack_20 = (undefined1 *)0x596158;
    FUN_005afd70((int)piVar3);
    if (DAT_006bc4a8 == (undefined4 *)0x0) {
      puStack_20 = (undefined1 *)0x59616c;
      puVar5 = (undefined4 *)FUN_00618b70(0x274);
      local_4._0_1_ = 3;
      if (puVar5 == (undefined4 *)0x0) {
        DAT_006bc4a8 = (undefined4 *)0x0;
      }
      else {
        DAT_006bc4a8 = FUN_005af5f0(puVar5);
      }
      local_4._0_1_ = 0;
    }
    puStack_20 = (undefined1 *)0x596199;
    iVar4 = FUN_005af5c0(DAT_006bc4a8,(int)piVar3);
  }
  local_4 = 0xffffffff;
  FUN_005f2ff0((undefined4 *)&stack0x00000014);
  ExceptionList = local_c;
  return iVar4;
}

