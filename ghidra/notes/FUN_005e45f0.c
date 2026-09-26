
void __thiscall
FUN_005e45f0(void *this,uint param_1,uint param_2,uint param_3,uint param_4,uint param_5)

{
  undefined1 uVar1;
  undefined4 uVar2;
  uint *puVar3;
  void *pvVar4;
  undefined3 extraout_var;
  undefined4 *puVar5;
  undefined3 extraout_var_00;
  undefined3 extraout_var_01;
  undefined3 extraout_var_02;
  undefined3 extraout_var_03;
  undefined1 uVar6;
  void *local_c;
  undefined1 *puStack_8;
  undefined4 local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00654fb7;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  uVar2 = FUN_006037f0(6);
  puVar3 = (uint *)FUN_005ff440((int)this);
  pvVar4 = (void *)FUN_00618b70(0x28);
  local_4 = 0;
  if (pvVar4 == (void *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    uVar6 = 2;
    uVar1 = FUN_005e77c0(param_1);
    puVar5 = FUN_005fbd20(pvVar4,uVar2,CONCAT31(extraout_var,uVar1) + 0x4b1,uVar6);
  }
  local_4 = 0xffffffff;
  if ((puVar5 != (undefined4 *)0x0) && (puVar3 != (uint *)0x0)) {
    FUN_005fcc30(puVar5,puVar3,10,0x67,0,0,(undefined4 *)0x16,0xf);
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  pvVar4 = (void *)FUN_00618b70(0x28);
  local_4 = 1;
  if (pvVar4 == (void *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    uVar6 = 2;
    uVar1 = FUN_005e77c0(param_2);
    puVar5 = FUN_005fbd20(pvVar4,uVar2,CONCAT31(extraout_var_00,uVar1) + 0x4b6,uVar6);
  }
  local_4 = 0xffffffff;
  if ((puVar5 != (undefined4 *)0x0) && (puVar3 != (uint *)0x0)) {
    FUN_005fcc30(puVar5,puVar3,0x25,0x67,0,0,(undefined4 *)0x16,0xf);
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  pvVar4 = (void *)FUN_00618b70(0x28);
  local_4 = 2;
  if (pvVar4 == (void *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    uVar6 = 2;
    uVar1 = FUN_005e77c0(param_3);
    puVar5 = FUN_005fbd20(pvVar4,uVar2,CONCAT31(extraout_var_01,uVar1) + 0x4bb,uVar6);
  }
  local_4 = 0xffffffff;
  if ((puVar5 != (undefined4 *)0x0) && (puVar3 != (uint *)0x0)) {
    FUN_005fcc30(puVar5,puVar3,0x40,0x67,0,0,(undefined4 *)0x16,0xf);
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  pvVar4 = (void *)FUN_00618b70(0x28);
  local_4 = 3;
  if (pvVar4 == (void *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    uVar6 = 2;
    uVar1 = FUN_005e77c0(param_4);
    puVar5 = FUN_005fbd20(pvVar4,uVar2,CONCAT31(extraout_var_02,uVar1) + 0x4c0,uVar6);
  }
  local_4 = 0xffffffff;
  if ((puVar5 != (undefined4 *)0x0) && (puVar3 != (uint *)0x0)) {
    FUN_005fcc30(puVar5,puVar3,0x5b,0x67,0,0,(undefined4 *)0x16,0xf);
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  pvVar4 = (void *)FUN_00618b70(0x28);
  local_4 = 4;
  if (pvVar4 == (void *)0x0) {
    puVar5 = (undefined4 *)0x0;
  }
  else {
    uVar6 = 2;
    uVar1 = FUN_005e77c0(param_5);
    puVar5 = FUN_005fbd20(pvVar4,uVar2,CONCAT31(extraout_var_03,uVar1) + 0x4c5,uVar6);
  }
  local_4 = 0xffffffff;
  if ((puVar5 != (undefined4 *)0x0) && (puVar3 != (uint *)0x0)) {
    FUN_005fcc30(puVar5,puVar3,0x76,0x67,0,0,(undefined4 *)0x16,0xf);
    FUN_005fbfa0(puVar5);
    FUN_00618b60((undefined *)puVar5);
  }
  ExceptionList = local_c;
  return;
}

