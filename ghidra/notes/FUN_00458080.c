
void __thiscall FUN_00458080(void *this,int param_1,void *param_2)

{
  int iVar1;
  uint *puVar2;
  void *pvVar3;
  undefined4 local_28;
  undefined2 uStack_24;
  undefined2 uStack_22;
  undefined2 uStack_20;
  undefined2 uStack_1e;
  undefined2 uStack_1c;
  undefined2 uStack_1a;
  HDC local_18;
  undefined4 local_14;
  void *pvStack_10;
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_00630562;
  pvStack_c = ExceptionList;
  iVar1 = param_1 * 0x38;
  ExceptionList = &pvStack_c;
  FUN_004ece30(&local_28);
  local_4 = 0;
  FUN_0042dbe0(&local_14);
  local_4._0_1_ = 1;
  puVar2 = FUN_004025b0(param_2,(uint *)&local_18);
  local_4._0_1_ = 2;
  FUN_004f26d0((void *)((int)this + iVar1 + 0x17c),puVar2);
  local_4._0_1_ = 1;
  FUN_00619730();
  FUN_004f26d0((void *)((int)this + iVar1 + 0x178),&local_28);
  FUN_004f26d0((void *)((int)this + iVar1 + 0x180),&local_28);
  local_18 = GetDC(*(HWND *)((int)this + 0x18));
  if (*(int *)((int)this + iVar1 + 0x188) != 0) goto LAB_00458239;
  if (param_1 == 1) {
    pvStack_10 = (void *)FUN_00618b70(0x34);
    local_4 = CONCAT31(local_4._1_3_,3);
    if (pvStack_10 == (void *)0x0) {
LAB_0045820d:
      pvVar3 = (void *)0x0;
    }
    else {
      uStack_1a = DAT_0065d424;
      uStack_1c = 0x1829;
      pvVar3 = FUN_00601700(pvStack_10,&uStack_1c,0,0,0,0,0,0);
    }
LAB_0045820f:
    local_4._0_1_ = 1;
  }
  else {
    if (param_1 == 2) {
      pvStack_10 = (void *)FUN_00618b70(0x34);
      local_4 = CONCAT31(local_4._1_3_,4);
      if (pvStack_10 == (void *)0x0) goto LAB_0045820d;
      uStack_1e = DAT_0065d424;
      uStack_20 = 0x1831;
      pvVar3 = FUN_00601700(pvStack_10,&uStack_20,0,0,0,0,0,0);
      goto LAB_0045820f;
    }
    pvVar3 = (void *)0x0;
    if (param_1 == 3) {
      pvStack_10 = (void *)FUN_00618b70(0x34);
      local_4 = CONCAT31(local_4._1_3_,5);
      if (pvStack_10 == (void *)0x0) goto LAB_0045820d;
      uStack_22 = DAT_0065d424;
      uStack_24 = 0x1833;
      pvVar3 = FUN_00601700(pvStack_10,&uStack_24,0,0,0,0,0,0);
      goto LAB_0045820f;
    }
  }
  FUN_00601b30(pvVar3,5,1);
  FUN_00601c60(pvVar3,10);
  FUN_00601b80(pvVar3,local_18);
  *(void **)((int)this + iVar1 + 0x188) = pvVar3;
LAB_00458239:
  if (*(int *)((int)this + iVar1 + 0x194) == 0) {
    pvStack_10 = (void *)FUN_00618b70(0x34);
    local_4._0_1_ = 6;
    if (pvStack_10 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_00601880(pvStack_10,(char *)0x0,0,0,0,0,0,0);
    }
    local_4._0_1_ = 1;
    FUN_00601b30(pvVar3,5,0x39);
    FUN_00601c60(pvVar3,10);
    FUN_00601c90(pvVar3,0x2ffffff,(HWND)0x0);
    *(void **)((int)this + iVar1 + 0x194) = pvVar3;
  }
  if (*(int *)((int)this + iVar1 + 400) == 0) {
    pvStack_10 = (void *)FUN_00618b70(0x34);
    local_4._0_1_ = 7;
    if (pvStack_10 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_00601880(pvStack_10,(char *)0x0,0,0,0,0,0,0);
    }
    local_4._0_1_ = 1;
    FUN_00601b30(pvVar3,5,0x2f);
    FUN_00601c60(pvVar3,10);
    FUN_00601c90(pvVar3,0x2ffffff,(HWND)0x0);
    *(void **)((int)this + iVar1 + 400) = pvVar3;
  }
  if (*(int *)((int)this + iVar1 + 0x18c) == 0) {
    pvStack_10 = (void *)FUN_00618b70(0x34);
    local_4._0_1_ = 8;
    if (pvStack_10 == (void *)0x0) {
      pvVar3 = (void *)0x0;
    }
    else {
      pvVar3 = FUN_00601880(pvStack_10,(char *)0x0,0,0,0,0,0,0);
    }
    local_4._0_1_ = 1;
    FUN_00601b30(pvVar3,5,0x10);
    FUN_00601c60(pvVar3,10);
    FUN_00601c90(pvVar3,0x2ffffff,(HWND)0x0);
    *(void **)((int)this + iVar1 + 0x18c) = pvVar3;
  }
  pvVar3 = param_2;
  puVar2 = FUN_004591b0(param_2,&param_2);
  local_4._0_1_ = 9;
  FUN_00457f30(this,param_1,pvVar3,puVar2);
  local_4._0_1_ = 1;
  FUN_00619730();
  FUN_00457b40(this,param_1,pvVar3,0);
  ReleaseDC(*(HWND *)((int)this + 0x18),local_18);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_00619730();
  ExceptionList = pvStack_c;
  return;
}

