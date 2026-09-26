
void __thiscall FUN_00438f30(void *this,int param_1,int param_2,int param_3)

{
  HWND pHVar1;
  char *pcVar2;
  void *pvVar3;
  int iVar4;
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *local_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062d7b0;
  local_c = ExceptionList;
  pHVar1 = *(HWND *)((int)this + 0x18);
  iVar4 = param_2;
  if (9999 < param_2) {
    iVar4 = 9999;
  }
  if (9999 < param_3) {
    param_3 = 9999;
  }
  ExceptionList = &local_c;
  if (*(int *)((int)this + 300) != 0) {
    ExceptionList = &local_c;
    FUN_005f2f50(local_30);
    local_4 = 0;
    if (param_1 == 0) {
      param_2 = CONCAT22(DAT_0065d424,0x3816);
      FUN_005f3010(local_30,&param_2);
    }
    else {
      param_2 = CONCAT22(DAT_0065d424,0x3815);
      FUN_005f2fc0(local_18,&param_2);
      local_4._0_1_ = 1;
      pcVar2 = (char *)FUN_00583c40((int)local_18);
      FUN_005f35b0(local_24,pcVar2);
      local_4._0_1_ = 2;
      pvVar3 = FUN_005f31a0(local_30,iVar4);
      FUN_005f30d0(pvVar3,(int)local_24);
      local_4._0_1_ = 1;
      FUN_005f2ff0(local_24);
      local_4 = (uint)local_4._1_3_ << 8;
      FUN_005f2ff0(local_18);
    }
    pcVar2 = (char *)FUN_00583c40((int)local_30);
    FUN_00601aa0(*(void **)((int)this + 300),pcVar2);
    FUN_00601dc0(*(void **)((int)this + 300),pHVar1,1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_30);
  }
  if (*(int *)((int)this + 0x130) != 0) {
    FUN_005f2f50(local_30);
    local_4 = 3;
    if (param_1 == 0) {
      param_3 = CONCAT22(DAT_0065d424,0x3816);
      FUN_005f3010(local_30,&param_3);
    }
    else {
      param_1 = CONCAT22(DAT_0065d424,0x3815);
      FUN_005f2fc0(local_24,&param_1);
      local_4._0_1_ = 4;
      pcVar2 = (char *)FUN_00583c40((int)local_24);
      FUN_005f35b0(local_18,pcVar2);
      local_4._0_1_ = 5;
      pvVar3 = FUN_005f31a0(local_30,param_3);
      FUN_005f30d0(pvVar3,(int)local_18);
      local_4._0_1_ = 4;
      FUN_005f2ff0(local_18);
      local_4 = CONCAT31(local_4._1_3_,3);
      FUN_005f2ff0(local_24);
    }
    pcVar2 = (char *)FUN_00583c40((int)local_30);
    FUN_00601aa0(*(void **)((int)this + 0x130),pcVar2);
    FUN_00601dc0(*(void **)((int)this + 0x130),pHVar1,1);
    local_4 = 0xffffffff;
    FUN_005f2ff0(local_30);
  }
  ExceptionList = local_c;
  return;
}

