
void __thiscall FUN_0044a620(void *this,uint param_1,void *param_2,void *param_3)

{
  bool bVar1;
  uint uVar2;
  void *pvVar3;
  char *pcVar4;
  uint *puVar5;
  undefined4 uVar6;
  undefined4 local_20;
  undefined4 local_1c;
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f336;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_18);
  local_4 = 0;
  local_20 = (void *)CONCAT22(DAT_0065d424,0x8651);
  FUN_005f3010(local_18,&local_20);
  local_20 = (void *)FUN_00618b70(0x68);
  uVar2 = param_1;
  local_4._0_1_ = 1;
  if (local_20 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_0060ba00(local_20,param_1);
  }
  local_4._0_1_ = 0;
  pcVar4 = (char *)FUN_00583c40((int)local_18);
  FUN_005f35e0((void *)((int)pvVar3 + 0x14),pcVar4);
  FUN_005f59f0(param_2,(int)pvVar3);
  FUN_005f3c20(local_18);
  puVar5 = FUN_004025b0(*(void **)((int)this + 300),&param_1);
  local_20 = (void *)0x10;
  local_1c = 0x40;
  local_4._0_1_ = 2;
  if ((*puVar5 >> 0x18 < 0x10) || (0x3f < *puVar5 >> 0x18)) {
    bVar1 = false;
  }
  else {
    bVar1 = true;
  }
  FUN_00619730();
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  if (bVar1) {
    uVar6 = FUN_004f2990(*(int *)((int)this + 300));
  }
  else {
    uVar6 = 0;
  }
  FUN_005f31f0(local_18,uVar6);
  param_2 = (void *)FUN_00618b70(0x68);
  local_4._0_1_ = 3;
  if (param_2 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_0060ba00(param_2,uVar2);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  pcVar4 = (char *)FUN_00583c40((int)local_18);
  FUN_005f35e0((void *)((int)pvVar3 + 0x14),pcVar4);
  FUN_005f59f0(param_3,(int)pvVar3);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_18);
  ExceptionList = local_c;
  return;
}

