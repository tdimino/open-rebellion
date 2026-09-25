
undefined * __cdecl
FUN_00442590(uint param_1,void *param_2,uint param_3,undefined4 param_4,undefined4 param_5,
            void *param_6,undefined4 param_7,undefined4 param_8,int param_9)

{
  void *pvVar1;
  undefined4 uVar2;
  int iVar3;
  void *pvVar4;
  undefined4 *puVar5;
  void **ppvVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  uint uVar10;
  uint uVar11;
  uint uVar12;
  uint uVar13;
  uint uVar14;
  void *pvVar15;
  undefined *local_2c;
  undefined4 local_28;
  ushort uStack_24;
  ushort local_22;
  undefined2 uStack_1a;
  undefined2 local_18;
  undefined2 uStack_16;
  undefined2 local_14;
  undefined2 uStack_12;
  uint local_10;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062eb3b;
  local_c = ExceptionList;
  puVar5 = &local_28;
  for (iVar3 = 6; iVar3 != 0; iVar3 = iVar3 + -1) {
    *puVar5 = 0;
    puVar5 = puVar5 + 1;
  }
  local_2c = &DAT_006b28e0;
  *(undefined2 *)puVar5 = 0;
  ExceptionList = &local_c;
  iVar3 = FUN_00442790(param_1,(int)param_2,(undefined2 *)&local_28);
  pvVar15 = param_6;
  if (iVar3 != 1) {
    if (local_28._2_2_ == 0) {
      if (local_22 != 0) {
        FUN_00442590((uint)local_22,param_2,4,param_4,param_5,param_6,param_7,param_8,param_9);
      }
    }
    else {
      local_2c = FUN_00442590((uint)local_28._2_2_,param_2,8,param_4,param_5,param_6,param_7,param_8
                              ,param_9);
    }
    pvVar1 = (void *)FUN_005f5920(local_2c,(uint)uStack_24);
    if (pvVar1 == (void *)0x0) {
      if ((param_3 & 1) == 0) {
        param_2 = (void *)((byte)~(byte)param_3 >> 2 & 1);
      }
      else {
        param_2 = (void *)0x0;
      }
      pvVar4 = (void *)(uint)(((byte)param_3 & 2) == 2);
      pvVar1 = (void *)FUN_00618b70(0x90);
      local_4 = 0;
      if (pvVar1 == (void *)0x0) {
        pvVar1 = (void *)0x0;
      }
      else {
        uVar14 = (uint)uStack_24;
        uVar13 = CONCAT22(uStack_12,local_14);
        uVar12 = CONCAT22(local_14,uStack_16);
        uVar11 = CONCAT22(uStack_16,local_18);
        uVar10 = CONCAT22(local_18,uStack_1a);
        iVar9 = 0;
        iVar8 = 0;
        iVar7 = 0;
        ppvVar6 = &param_6;
        iVar3 = 0;
        uVar2 = FUN_006037f0(local_10 & 0xffff);
        pvVar1 = FUN_004abfb0(pvVar1,param_1,param_3,uVar2,ppvVar6,iVar3,iVar7,iVar8,iVar9,uVar10,
                              uVar11,uVar12,uVar13,uVar14,pvVar15,param_2,pvVar4);
      }
      local_4 = 0xffffffff;
      if (pvVar1 != (void *)0x0) {
        FUN_004abe10(pvVar1,param_7,param_8,param_9);
      }
      if (local_2c != (undefined *)0x0) {
        FUN_005f58d0(local_2c,(int)pvVar1);
      }
      if (pvVar1 == (void *)0x0) {
        ExceptionList = local_c;
        return local_2c;
      }
    }
    local_2c = (undefined *)((int)pvVar1 + 0x80);
  }
  ExceptionList = local_c;
  return local_2c;
}

