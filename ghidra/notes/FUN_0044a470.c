
void __thiscall FUN_0044a470(void *this,void *param_1,void *param_2,void *param_3)

{
  uint uVar1;
  int iVar2;
  void *pvVar3;
  char *pcVar4;
  void *pvVar5;
  undefined4 local_28;
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062f306;
  local_c = ExceptionList;
  ExceptionList = &local_c;
  FUN_005f2f50(local_24);
  local_4 = 0;
  FUN_005f3c20(local_24);
  iVar2 = FUN_004067d0();
  if (iVar2 == 0) {
    local_28 = (void *)CONCAT22(DAT_0065d424,0x8666);
    FUN_005f3010(local_24,&local_28);
  }
  else {
    FUN_005f35b0(local_18,s_Status__006a8744);
    local_4._0_1_ = 1;
    FUN_005f30d0(local_24,(int)local_18);
    local_4 = (uint)local_4._1_3_ << 8;
    FUN_005f2ff0(local_18);
  }
  local_28 = (void *)FUN_00618b70(0x68);
  pvVar5 = param_1;
  local_4._0_1_ = 2;
  if (local_28 == (void *)0x0) {
    pvVar3 = (void *)0x0;
  }
  else {
    pvVar3 = FUN_0060ba00(local_28,param_1);
  }
  local_4._0_1_ = 0;
  pcVar4 = (char *)FUN_00583c40((int)local_24);
  FUN_005f35e0((void *)((int)pvVar3 + 0x14),pcVar4);
  FUN_005f59f0(param_2,(int)pvVar3);
  FUN_005f3c20(local_24);
  param_1 = (void *)FUN_00618b70(0x68);
  local_4._0_1_ = 3;
  if (param_1 == (void *)0x0) {
    pvVar5 = (void *)0x0;
  }
  else {
    pvVar5 = FUN_0060ba00(param_1,pvVar5);
  }
  local_4 = (uint)local_4._1_3_ << 8;
  uVar1 = *(uint *)(*(int *)((int)this + 300) + 0x50);
  if ((uVar1 & 0x10) == 0) {
    if ((uVar1 & 4) == 0) {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8757);
    }
    else {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8763);
    }
  }
  else {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8744);
  }
  FUN_005f3010(local_24,&param_1);
  pcVar4 = (char *)FUN_00583c40((int)local_24);
  FUN_005f35e0((void *)((int)pvVar5 + 0x14),pcVar4);
  FUN_005f59f0(param_3,(int)pvVar5);
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_24);
  ExceptionList = local_c;
  return;
}

