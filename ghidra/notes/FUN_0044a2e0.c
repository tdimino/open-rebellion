
void __thiscall FUN_0044a2e0(void *this,undefined4 param_1,void *param_2,void *param_3)

{
  int iVar1;
  void *pvVar2;
  char *pcVar3;
  undefined *puVar4;
  undefined4 local_28;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f2ce;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_24);
  local_4 = 0;
  FUN_005f3c20(local_24);
  iVar1 = FUN_004067d0();
  if (iVar1 == 0) {
    local_28 = (void *)CONCAT22(DAT_0065d424,0x87b0);
    FUN_005f3010(local_24,&local_28);
  }
  else {
    FUN_005f35b0(local_18,s_Attached__006a8738);
    local_4._0_1_ = 1;
    FUN_005f30d0(local_24,(int)local_18);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f2ff0(local_18);
  }
  local_28 = (void *)FUN_00618b70(0x68);
  local_4._0_1_ = 2;
  if (local_28 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_0060ba00(local_28,param_1);
  }
  local_4._0_1_ = 0;
  pcVar3 = (char *)FUN_00583c40((int)local_24);
  FUN_005f35e0((void *)((int)pvVar2 + 0x14),pcVar3);
  FUN_005f59f0(param_2,(int)pvVar2);
  FUN_005f3c20(local_24);
  puVar4 = FUN_004f62d0(*(int *)(*(int *)((int)this + 300) + 0x1c));
  pcVar3 = (char *)FUN_00583c40((int)puVar4);
  FUN_005f35b0(local_18,pcVar3);
  local_4._0_1_ = 3;
  FUN_005f30d0(local_24,(int)local_18);
  local_4._0_1_ = 0;
  FUN_005f2ff0(local_18);
  pvVar2 = (void *)FUN_00618b70(0x68);
  local_4._0_1_ = 4;
  if (pvVar2 == (void *)0x0) {
    pvVar2 = (void *)0x0;
  }
  else {
    pvVar2 = FUN_0060ba00(pvVar2,param_1);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  pcVar3 = (char *)FUN_00583c40((int)local_24);
  FUN_005f35e0((void *)((int)pvVar2 + 0x14),pcVar3);
  FUN_005f59f0(param_3,(int)pvVar2);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_24);
  ExceptionList = local_c;
  return;
}

