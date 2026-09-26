
void FUN_004a67a0(uint *param_1,int param_2,uint param_3,int param_4)

{
  undefined4 uVar1;
  void *pvVar2;
  undefined4 *puVar3;
  uint uVar4;
  int iVar5;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00637e4c;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar1 = FUN_006037f0(7);
  iVar5 = (-(uint)(param_4 != 0) & 10) + 0x17;
  uVar4 = -(uint)(param_2 != 1) & 0x32;
  if ((param_3 & 1) != 0) {
    pvVar2 = (void *)FUN_00618b70(0x28);
    puVar3 = (undefined4 *)0x0;
    local_4 = 0;
    if (pvVar2 != (void *)0x0) {
      puVar3 = FUN_005fbd20(pvVar2,uVar1,uVar4 + 0x28a4,2);
    }
    local_4 = 0xffffffff;
    FUN_005fd0f0(puVar3,param_1,0x19,iVar5);
    if (puVar3 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar3);
      FUN_00618b60((undefined *)puVar3);
    }
  }
  if ((param_3 & 2) != 0) {
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4 = 1;
    if (pvVar2 == (void *)0x0) {
      puVar3 = (undefined4 *)0x0;
    }
    else {
      puVar3 = FUN_005fbd20(pvVar2,uVar1,uVar4 + 0x28a5,2);
    }
    local_4 = 0xffffffff;
    FUN_005fd0f0(puVar3,param_1,0x29,iVar5);
    if (puVar3 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar3);
      FUN_00618b60((undefined *)puVar3);
    }
  }
  if ((param_3 & 4) != 0) {
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4 = 2;
    if (pvVar2 == (void *)0x0) {
      puVar3 = (undefined4 *)0x0;
    }
    else {
      puVar3 = FUN_005fbd20(pvVar2,uVar1,uVar4 + 0x28a6,2);
    }
    local_4 = 0xffffffff;
    FUN_005fd0f0(puVar3,param_1,0x39,iVar5);
    if (puVar3 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar3);
      FUN_00618b60((undefined *)puVar3);
    }
  }
  if ((param_3 & 0x40) != 0) {
    pvVar2 = (void *)FUN_00618b70(0x28);
    local_4 = 3;
    if (pvVar2 == (void *)0x0) {
      puVar3 = (undefined4 *)0x0;
    }
    else {
      puVar3 = FUN_005fbd20(pvVar2,uVar1,uVar4 + 0x28be,2);
    }
    local_4 = 0xffffffff;
    FUN_005fd0f0(puVar3,param_1,9,iVar5);
    if (puVar3 != (undefined4 *)0x0) {
      FUN_005fbfa0(puVar3);
      FUN_00618b60((undefined *)puVar3);
    }
  }
  ExceptionList = local_c;
  return;
}

