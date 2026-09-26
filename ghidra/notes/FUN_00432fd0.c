
void __thiscall FUN_00432fd0(void *this,void *param_1,undefined4 param_2,undefined4 param_3)

{
  undefined4 *puVar1;
  int *piVar2;
  void *pvVar3;
  uint uVar4;
  int iVar5;
  int iVar6;
  uint local_14;
  uint uStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d153;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_14);
  uVar4 = 0;
  *(undefined4 *)((int)this + 200) = param_3;
  local_4 = 0;
  *(undefined4 *)((int)this + 0x40) = param_2;
  FUN_0058f0c0(this,param_1);
  FUN_005f4d90(param_1,(int)this + 0x20);
  FUN_005f4d90(param_1,(int)this + 0x24);
  FUN_005f4d90(param_1,(int)this + 0x3c);
  FUN_005f4d90(param_1,(undefined4 *)((int)this + 0x1c));
  FUN_005f4d90(param_1,(int)this + 0x30);
  FUN_005f4d90(param_1,(int)this + 0x34);
  FUN_004ece90((void *)((int)this + 0x28),param_1);
  FUN_004ece90((void *)((int)this + 0x2c),param_1);
  FUN_005f4990((void *)((int)this + 0x38),param_1);
  (**(code **)(*(int *)((int)this + 0x48) + 4))();
  FUN_005f4d90(param_1,&uStack_10);
  FUN_004ece90(&local_14,param_1);
  if (uStack_10 != 0) {
    do {
      puVar1 = (undefined4 *)FUN_00618b70(0x1c);
      local_4._0_1_ = 1;
      if (puVar1 == (undefined4 *)0x0) {
        piVar2 = (int *)0x0;
      }
      else {
        piVar2 = FUN_004ecc00(puVar1);
      }
      local_4 = (uint)local_4._1_3_ << 8;
      (**(code **)(*piVar2 + 8))(param_1);
      FUN_004f57b0((void *)((int)this + 0x48),piVar2);
      uVar4 = uVar4 + 1;
    } while (uVar4 < uStack_10);
  }
  pvVar3 = FUN_004f5940((void *)((int)this + 0x48),&local_14);
  *(void **)((int)this + 0x44) = pvVar3;
  iVar6 = (int)this + 0x50;
  iVar5 = 0x1e;
  do {
    FUN_005f4d90(param_1,iVar6);
    iVar6 = iVar6 + 4;
    iVar5 = iVar5 + -1;
  } while (iVar5 != 0);
  *(undefined4 *)((int)this + 0x1c) = 1;
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

