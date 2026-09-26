
void __thiscall FUN_00445280(void *this,void *param_1,void *param_2)

{
  uint uVar1;
  int *piVar2;
  void *pvVar3;
  void *pvVar4;
  char *pcVar5;
  undefined4 uVar6;
  int iVar7;
  int iVar8;
  int iVar9;
  int local_4c;
  undefined2 local_48;
  undefined2 local_46;
  int local_44;
  int local_40;
  undefined4 local_3c [3];
  undefined4 local_30 [3];
  undefined4 local_24 [3];
  undefined4 local_18 [3];
  void *pvStack_c;
  undefined1 *puStack_8;
  int local_4;
  
  local_4 = 0xffffffff;
  puStack_8 = &LAB_0062eec0;
  pvStack_c = ExceptionList;
  ExceptionList = &pvStack_c;
  FUN_005f2f50(local_3c);
  local_46 = DAT_0065d424;
  local_4 = 0;
  local_48 = 0x8665;
  FUN_005f3010(local_3c,&local_48);
  pcVar5 = (char *)FUN_00583c40((int)local_3c);
  FUN_00601aa0((void *)((int)this + 0x130),pcVar5);
  pvVar4 = param_2;
  pvVar3 = param_1;
  FUN_0044a210((int)this);
  FUN_0044a2e0(this,2,pvVar3,pvVar4);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8666);
  FUN_005f3010(local_3c,&param_1);
  iVar9 = 3;
  FUN_0044b1d0(3,(int)local_3c,pvVar3);
  FUN_005f3c20(local_3c);
  uVar1 = *(uint *)(*(int *)((int)this + 300) + 0x50);
  if ((uVar1 & 0x10) == 0) {
    if ((uVar1 & 4) == 0) {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8771);
    }
    else {
      param_1 = (void *)CONCAT22(DAT_0065d424,0x8748);
    }
  }
  else {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x8744);
  }
  FUN_005f3010(local_3c,&param_1);
  FUN_0044b1d0(3,(int)local_3c,pvVar4);
  local_44 = 1;
  local_40 = -1;
  local_4._0_1_ = 1;
  local_4c = 0;
  FUN_004fd2b0(*(int **)((int)this + 300),&local_44,&local_4c);
  if ((local_40 == -1) && (local_4c != 0)) {
    param_1 = (void *)CONCAT22(DAT_0065d424,0x3814);
    FUN_005f2fc0(local_18,&param_1);
    local_4._0_1_ = 2;
    FUN_005f3c20(local_3c);
    param_1 = (void *)CONCAT22(param_1._2_2_,0x8766);
    FUN_005f2fc0(local_24,&param_1);
    local_4._0_1_ = 3;
    pcVar5 = (char *)FUN_00583c40((int)local_24);
    FUN_005f35b0(local_30,pcVar5);
    local_4._0_1_ = 4;
    FUN_005f30d0(local_3c,(int)local_30);
    local_4._0_1_ = 3;
    FUN_005f2ff0(local_30);
    iVar9 = 4;
    FUN_0044b1d0(4,(int)local_3c,pvVar3);
    FUN_005f3c20(local_3c);
    pcVar5 = (char *)FUN_00583c40((int)local_18);
    FUN_005f35b0(local_30,pcVar5);
    local_4._0_1_ = 5;
    FUN_005f30d0(local_3c,(int)local_30);
    FUN_005f31a0(local_3c,local_4c);
    local_4._0_1_ = 3;
    FUN_005f2ff0(local_30);
    FUN_0044b1d0(4,(int)local_3c,pvVar4);
    FUN_005f3c20(local_3c);
    local_4._0_1_ = 2;
    FUN_005f2ff0(local_24);
    local_4._0_1_ = 1;
    FUN_005f2ff0(local_18);
  }
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8651);
  FUN_005f3010(local_3c,&param_1);
  FUN_0044b1d0(iVar9 + 1,(int)local_3c,pvVar3);
  FUN_005f3c20(local_3c);
  uVar6 = FUN_004f2990(*(int *)((int)this + 300));
  FUN_005f31f0(local_3c,uVar6);
  FUN_0044b1d0(iVar9 + 1,(int)local_3c,pvVar4);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8667);
  FUN_005f3010(local_3c,&param_1);
  FUN_0044b1d0(iVar9 + 2,(int)local_3c,pvVar3);
  FUN_005f3c20(local_3c);
  piVar2 = *(int **)((int)this + 300);
  param_1 = (void *)(**(code **)(*piVar2 + 0x1e8))();
  iVar7 = (**(code **)(*piVar2 + 0x1dc))();
  FUN_005f31f0(local_3c,iVar7 - (int)param_1);
  FUN_0044b1d0(iVar9 + 2,(int)local_3c,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8668);
  FUN_005f3010(local_3c,&param_1);
  FUN_0044b1d0(iVar9 + 3,(int)local_3c,pvVar3);
  FUN_005f3c20(local_3c);
  piVar2 = *(int **)((int)this + 300);
  param_1 = (void *)(**(code **)(*piVar2 + 0x1e0))();
  iVar7 = (**(code **)(*piVar2 + 0x1ec))();
  FUN_005f31f0(local_3c,(int)param_1 - iVar7);
  FUN_0044b1d0(iVar9 + 3,(int)local_3c,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8658);
  FUN_005f3010(local_3c,&param_1);
  FUN_0044b1d0(iVar9 + 4,(int)local_3c,pvVar3);
  FUN_005f3c20(local_3c);
  piVar2 = *(int **)((int)this + 300);
  param_1 = (void *)(**(code **)(*piVar2 + 0x1e4))();
  iVar7 = (**(code **)(*piVar2 + 0x1d8))();
  FUN_005f31f0(local_3c,iVar7 - (int)param_1);
  FUN_0044b1d0(iVar9 + 4,(int)local_3c,param_2);
  param_1 = (void *)CONCAT22(DAT_0065d424,0x8671);
  FUN_005f3010(local_3c,&param_1);
  FUN_0044b1d0(iVar9 + 5,(int)local_3c,pvVar3);
  FUN_005f3c20(local_3c);
  piVar2 = *(int **)((int)this + 300);
  iVar7 = (**(code **)(*piVar2 + 0x1c4))();
  iVar8 = (**(code **)(*piVar2 + 0x1cc))();
  FUN_005f31f0(local_3c,iVar7 - iVar8);
  FUN_0044b1d0(iVar9 + 5,(int)local_3c,param_2);
  local_4 = (uint)local_4._1_3_ << 8;
  FUN_00619730();
  local_4 = 0xffffffff;
  FUN_005f2ff0(local_3c);
  ExceptionList = pvStack_c;
  return;
}

